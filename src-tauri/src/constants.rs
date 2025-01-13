pub struct AppConfig;

impl AppConfig {
    // 基础配置
    pub const BASE_URL: &'static str = "https://safe.ydyrx.top";
    pub const APP_ID: &'static str = "10001";
    
    // AES密钥，密钥这里如果是rsa交换的话，是随机生成的
    pub const AES_KEY: &'static str = "NdXWjNahQTCp0hOKtLtL4rHV";
    
    // API 超时设置 (秒)
    pub const API_TIMEOUT: i64 = 300; // 5分钟
    
    // RSA加密的API列表，也可以把所有方法都加进来
    pub const RSA_APIS: [&'static str; 6] = [
        "GetToken",
        "UserLogin",
        "UserReduceMoney",
        "UserReduceVipNumber",
        "UserReduceVipTime",
        "GetVipData",
    ];
    
    // RSA公钥
    pub const RSA_PUBLIC_KEY: &'static str = "\
    -----BEGIN PUBLIC KEY-----\n\
    MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCljJVkvwbQTChbpwefzCgYSI/L\n\
    6PNmbcmb+RpPcxkjGwGrKOS5/cSQCiTS6GsobQqsI/BiQ40WDvXKOoywv+6aMCAs\n\
    wO4wHUL7fYVmaKbKxlovoxQoDu4pm7eC/zvxrxFXgFaj9Vtoh1MoPr876BzfCpRs\n\
    cf32mz+HIldJjtcZ/QIDAQAB\n\
    -----END PUBLIC KEY-----\n";
} 