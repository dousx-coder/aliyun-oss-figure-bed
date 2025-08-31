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
    let arg1 = &args[1];
    let start = if arg1 == "md" { 2 } else { 1 };
    let client = Client::from_env().unwrap();
    let endpoint = client
        .get_bucket_base()
        .endpoint()
        .to_url()
        .to_string()
        .replace("https://", "")
        .replace("/", "");
    let bucket = client.get_bucket_base().get_name().to_string();
    for i in start..args.len() {
        let file = &args[i];
        let ext = match Path::new(file).extension() {
            None => {
                panic!();
            }
            Some(ext) => ext.to_string_lossy(),
        };
        let timestamp = Local::now().format("%Y/%m/%d/%H-%M-%S-%3f").to_string();
        let uuid_simple = Uuid::new_v4().simple();
        let fs = format!("{timestamp}-{uuid_simple}.{ext}");
        let md = format!("markdown/{fs}");
        let url = format!("https://{bucket}.{endpoint}/{md}");
        match client.put_file(PathBuf::from(file), md).await {
            Ok(_) => {
                if arg1 == "md" {
                    // 输出md可引用的路径
                    println!("![{fs}]({url})");
                } else {
                    println!("{url}");
                }
            }
            Err(e) => {
                eprintln!("上传异常: {}", e);
            }
        }
    }
}
