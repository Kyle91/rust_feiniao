// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod crypto;
mod api;
mod constants;
use api::FeiNiaoAPI;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;

// 创建一个全局的 API 实例
lazy_static::lazy_static! {
    static ref API: Arc<TokioMutex<FeiNiaoAPI>> = {
        let api = FeiNiaoAPI::new();
        Arc::new(TokioMutex::new(api))
    };
}

#[tauri::command]
async fn get_token() -> Result<String, String> {
    let mut api = API.lock().await;
    api.get_token().await
}

#[tauri::command]
async fn login(username: String, password: String) -> Result<String, String> {
    let mut api = API.lock().await;
    let params = json!({
        "username": username,
        "password": password,
    });
    let response = api.call_api("Login", Some(params)).await?;
    Ok(response.Msg)
}

#[tauri::command]
async fn register(
    _username: String,
    _password: String,
    _super_password: String,
    _qq: String,
    _email: String,
    _phone: String,
    _bind_info: Option<String>,
) -> Result<String, String> {
    Ok("注册功能待实现".to_string())
}

#[tauri::command]
async fn card_login(_card_number: String) -> Result<String, String> {
    Ok("卡密登录功能待实现".to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            login,
            register,
            card_login,
            get_token,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
