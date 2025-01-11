use reqwest::{Client, header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT}};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::crypto::CryptoUtil;
use crate::constants::AppConfig;
use rand::Rng;
use std::sync::Arc;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};

// 全局单例
static API_INSTANCE: Lazy<Arc<Mutex<FeiNiaoAPI>>> = Lazy::new(|| {
    Arc::new(Mutex::new(FeiNiaoAPI::new()))
});

#[derive(Debug)]
pub struct FeiNiaoAPI {
    client: Client,
    token: Option<String>,
    aes_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub Msg: String,
    pub Data: Option<Value>,
    pub Time: i64,
    pub Status: i32,
}

impl FeiNiaoAPI {
    pub fn new() -> Self {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)  // 允许自签名证书
            .build()
            .unwrap_or_else(|_| Client::new());

        Self {
            client,
            token: None,
            aes_key: None,
        }
    }

    // 获取实例
    pub fn get_instance() -> Arc<Mutex<FeiNiaoAPI>> {
        API_INSTANCE.clone()
    }

    fn get_timestamp(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; WOW64)"));
        
        if let Some(token) = &self.token {
            headers.insert("Token", HeaderValue::from_str(token).unwrap());
        }
        
        headers
    }


    // 生成随机 Status
    fn generate_status(&self) -> i32 {
        rand::thread_rng().gen_range(10000..99999)
    }

    // 验证时间戳
    fn validate_timestamp(&self, response_time: i64) -> Result<(), String> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        if (current_time - response_time).abs() > AppConfig::API_TIMEOUT {
            return Err("响应时间戳异常".to_string());
        }
        Ok(())
    }

    async fn request(&mut self, api_name: &str, params: Option<Value>) -> Result<ApiResponse, String> {
        // 1. 构建基础请求数据
        let mut request_data = json!({
            "Time": self.get_timestamp(),
            "Status": self.generate_status(),
            "Api": api_name,
        });

        // 2. 添加其他参数
        if let Some(p) = params {
            request_data["Data"] = p;
        }

        println!("初始请求数据: {}", request_data);

        // 3. 序列化请求数据
        let request_str = serde_json::to_string(&request_data)
            .map_err(|e| format!("序列化请求数据失败: {}", e))?;

        // 4. AES加密
        let aes_key_bytes = AppConfig::AES_KEY.as_bytes();

        let encrypted_data = CryptoUtil::aes_encrypt(
            request_str.as_bytes(),
            aes_key_bytes,
        )?;
        println!("加密后的数据: {}", encrypted_data);

        // 5. 计算签名
        let sign_data = format!("{}{}", encrypted_data, AppConfig::AES_KEY);
        let sign = CryptoUtil::md5_hash(sign_data.as_bytes());
        println!("请求签名: {}", sign);

        // 6. 构建最终请求数据
        let final_request = json!({
            "a": encrypted_data,
            "b": sign
        });

        println!("完整请求: {}", final_request);

        // 7. 发送请求
        let url = format!("{}/Api?AppId={}", AppConfig::BASE_URL, AppConfig::APP_ID);
        println!("请求url: {}", url);
        let response = self.client
            .post(&url)
            .headers(self.build_headers())
            .json(&final_request)
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?;

        // 8. 解析响应
        let response_text = response.text().await
            .map_err(|e| format!("读取响应失败: {}", e))?;
        println!("原始响应: {}", response_text);
        let response_data: Value = serde_json::from_str(&response_text)
            .map_err(|e| format!("解析响应失败: {}", e))?;

        // 9. 验证响应签名
        let response_data_str = response_data["a"].as_str()
            .ok_or("Missing encrypted data in response")?;
        let response_sign = response_data["b"].as_str()
            .ok_or("Missing sign in response")?;
        let res_sign_data = format!("{}{}", response_data_str, AppConfig::AES_KEY);
        let verify_sign = CryptoUtil::md5_hash(res_sign_data.as_bytes());
        if verify_sign != response_sign {
            return Err("响应签名验证失败".to_string());
        }

        // 10. 解密响应数据
        let decrypted_data = CryptoUtil::aes_decrypt(
            response_data_str,
            aes_key_bytes,
        )?;

        // 11. 解析解密后的数据
        let decrypted_str = String::from_utf8(decrypted_data)
            .map_err(|e| format!("响应数据解码失败: {}", e))?;
        println!("解密后的响应: {}", decrypted_str);
        let response: ApiResponse = serde_json::from_str(&decrypted_str)
            .map_err(|e| format!("解析解密后的响应失败: {}", e))?;

        // 12. 验证时间戳和Status
        if let Some(response_time) = response.Data.as_ref()
            .and_then(|d| d.get("Time"))
            .and_then(|t| t.as_i64()) {
            self.validate_timestamp(response_time)?;
        }

        Ok(response)
    }

    pub async fn get_token(&mut self) -> Result<String, String> {
        let response = self.request("GetToken", None).await?;
        
        if let Some(data) = response.Data {
            if let Some(token) = data.get("Token").and_then(Value::as_str) {
                self.token = Some(token.to_string());
                return Ok(token.to_string());
            }
        }
        Err("Token not found in response".to_string())
    }

    pub async fn call_api(&mut self, api_name: &str, params: Option<Value>) -> Result<ApiResponse, String> {
        if self.token.is_none() {
            return Err("Token not set. Please call get_token first.".to_string());
        }
        
        self.request(api_name, params).await
    }
} 