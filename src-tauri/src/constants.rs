pub struct AppConfig;

impl AppConfig {
    // 基础配置
    pub const BASE_URL: &'static str = "http://127.0.0.1:18888";
    pub const APP_ID: &'static str = "10001";
    
    // AES密钥
    pub const AES_KEY: &'static str = "NdXWjNahQTCp0hOKtLtL4rHV";
    
    // API 超时设置 (秒)
    pub const API_TIMEOUT: i64 = 300; // 5分钟
    
    
} 