use aliyun_oss_client::file::Files;
use aliyun_oss_client::Client;
use chrono::Local;
use std::env;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    // 第0个参数是程序名
    let file = &args[1];
    let ext = match Path::new(file).extension() {
        None => {
            panic!();
        }
        Some(ext) => ext.to_string_lossy(),
    };

    let client = Client::from_env().unwrap();
    let timestamp = Local::now().format("%Y/%m/%d/%H-%M-%S-%3f").to_string();
    let uuid_simple = Uuid::new_v4().simple();
    let fs = format!("markdown/{timestamp}-{uuid_simple}.{ext}");
    // 这里再次同环境变量中获取配置，这里的变量名同Client::from_env()
    // Client没有对外开发获取配置的方法，暂时先这样获取
    let endpoint = env::var("ALIYUN_ENDPOINT").unwrap();
    let bucket = env::var("ALIYUN_BUCKET").unwrap();
    let url = format!("https://{bucket}.{endpoint}/{fs}");
    let _ = client.put_file(PathBuf::from(file), fs).await;
    println!("{url}");
}
