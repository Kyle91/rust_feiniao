use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use cipher::{
    block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit,
};
use rand::thread_rng;
use rand::RngCore;
use openssl::rsa::{Rsa, Padding};
use openssl::pkey::Public;

type Aes192CbcEnc = cbc::Encryptor<aes::Aes192>;
type Aes192CbcDec = cbc::Decryptor<aes::Aes192>;

pub struct CryptoUtil;

impl CryptoUtil {
    // 生成 AES-192 密钥
    pub fn generate_aes_key() -> String {
        let mut key = vec![0u8; 24];
        thread_rng().fill_bytes(&mut key);
        String::from_utf8(key).unwrap_or_else(|_| "NdXWjNahQTCp0hOKtLtL4rHV".to_string())
    }

    // 使用公钥加密
    pub fn rsa_encrypt(data: &[u8], rsa: &Rsa<Public>) -> Result<String, String> {
        let mut buf = vec![0; rsa.size() as usize];
        let encrypted_len = rsa.public_encrypt(data, &mut buf, Padding::PKCS1)
            .map_err(|e| format!("RSA加密失败: {}", e))?;
        Ok(BASE64.encode(&buf[..encrypted_len]))
    }

    // 使用公钥解密（仅用于签名验证）
    pub fn rsa_decrypt_with_public_key(encrypted_data: &str, rsa: &Rsa<Public>) -> Result<Vec<u8>, String> {
        let encrypted_bytes = BASE64.decode(encrypted_data)
            .map_err(|e| e.to_string())?;
        let mut buf = vec![0; rsa.size() as usize];
        let decrypted_len = rsa.public_decrypt(&encrypted_bytes, &mut buf, Padding::PKCS1)
            .map_err(|e| format!("RSA解密失败: {}", e))?;
        Ok(buf[..decrypted_len].to_vec())
    }

    // AES-192-CBC 加密
    pub fn aes_encrypt(data: &[u8], key: &[u8]) -> Result<String, String> {
        let iv = [0u8; 16];
        let cipher = Aes192CbcEnc::new(key.into(), &iv.into());
        
        let mut buffer = vec![0u8; data.len() + 16];
        buffer[..data.len()].copy_from_slice(data);
        
        let ciphertext = cipher
            .encrypt_padded_mut::<Pkcs7>(&mut buffer, data.len())
            .map_err(|e| e.to_string())?;

        Ok(BASE64.encode(ciphertext))
    }

    // AES-192-CBC 解密
    pub fn aes_decrypt(encrypted_data: &str, key: &[u8]) -> Result<Vec<u8>, String> {
        let encrypted_bytes = BASE64.decode(encrypted_data)
            .map_err(|e| e.to_string())?;

        let iv = [0u8; 16];
        let cipher = Aes192CbcDec::new(key.into(), &iv.into());
        
        let mut buffer = encrypted_bytes.clone();
        let decrypted = cipher
            .decrypt_padded_mut::<Pkcs7>(&mut buffer)
            .map_err(|e| e.to_string())?;

        Ok(decrypted.to_vec())
    }

    // MD5 哈希
    pub fn md5_hash(data: &[u8]) -> String {
        format!("{:X}", md5::compute(data))
    }
} 