use aliyun_oss_client::file::Files;
use aliyun_oss_client::Client;
use chrono::Local;
use futures::stream::{self, StreamExt};
use std::env;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let is_md = &args[1] == "md";
    let start = if is_md { 2 } else { 1 };

    for i in start..args.len() {
        let file = &args[i];
        if !Path::new(&file).exists() {
            panic!();
        }
        if let None = Path::new(&file).extension() {
            panic!();
        }
    }

    let client = Client::from_env().unwrap();
    let endpoint = client
        .get_bucket_base()
        .endpoint()
        .to_url()
        .to_string()
        .replace("https://", "")
        .replace("/", "");
    let bucket = client.get_bucket_base().get_name().to_string();
    let client = Arc::new(client);

    let tasks: Vec<_> = (start..args.len())
        .map(|i| {
            let file = args[i].clone();
            let ext = match Path::new(&file).extension() {
                None => {
                    panic!();
                }
                Some(ext) => ext.to_string_lossy().to_string(),
            };
            let timestamp = Local::now().format("%Y/%m/%d/%H-%M-%S-%3f").to_string();
            let uuid_simple = Uuid::new_v4().simple();
            let fs = format!("{timestamp}-{uuid_simple}.{ext}");
            let md = format!("markdown/{fs}");
            let url = format!("https://{bucket}.{endpoint}/{md}");
            let tc = client.clone();

            async move {
                match tc.put_file(PathBuf::from(&file), md).await {
                    Ok(_) => {
                        if is_md {
                            Some(format!("![{fs}]({url})"))
                        } else {
                            Some(url)
                        }
                    }
                    Err(e) => {
                        eprintln!("上传异常: {}", e);
                        None
                    }
                }
            }
        })
        .collect();

    // 使用 stream 控制并发数
    let results: Vec<_> = stream::iter(tasks)
        .buffer_unordered(3) // 最多同时运行3个任务
        .collect()
        .await;

    for result in results {
        match result {
            Some(output) => println!("{}", output),
            None => {}
        }
    }
}
