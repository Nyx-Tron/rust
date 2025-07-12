use std::{fs};
use std::time::Duration;

#[tokio::main]
pub async fn asynchronous()
{
    // fs 读写文件
    let result = tokio::fs::read_to_string("src/main.rs").await;
    match result {
        Ok(content) => {
            println!("File content:\n{}", content);
        },
        Err(error) => {
            println!("Error reading file: {}", error);
        }
    }

    // say_hello().await;

    // let handle = tokio::spawn(async {
    //     println!("Hello, world!");
    // });
    // handle.await.unwrap();

    /* 两个常见的宏：
        1. tokio::join!()
        2. tokio::select!()
    */
    // 实现异步， 同时运行这两个函数
    tokio::join!(
        say_hello(),
        say_goodbye(),
    );

    /*
        select!：
            如果任何一个函数有结果后，便会返回，其他函数保留进度并结束运行
            通常用于《超时处理》
    */
    tokio::select!(
        result = say_hello() => {
            println!("{result}");
        },
        _ = tokio::time::sleep(tokio::time::Duration::from_millis(500)) => {
            println!("over time!");
        }
    );
}

async fn say_hello() -> &'static str
{
    tokio::time::sleep(Duration::from_secs(5)).await;
    "Hello, world!"
}

async fn say_goodbye()
{
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("goodbye!");
}