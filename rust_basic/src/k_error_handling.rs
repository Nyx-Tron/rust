// pub fn error_handling() -> Result<(), String>
// {
//     /* 简单粗暴错误处理 */
//     // panic!("panic!");
//
//     /* Result */
//     let age = 20;
//     // 1. do_something(age);                                    //  不建议
//     // 2. let result = do_something(age).unwrap();        // panic! 不建议
//     // 3. let result = do_something(age).expect("Error"); // panic! 不好用
//     /* 4. 太麻烦
//         match do_something(age) {
//             Ok(_) => println!("all is ok"),
//             Err(err) => println!("error: {}", err),
//         }
//     */
//     // let result = do_something(age)?; // 语法糖 main 和 do_something 函数的Err返回函数类型匹配，就可以用 ?
//     // println!("{}", result);
//     // Ok(())
// }
//
// fn do_something(age: u8) -> Result<String, String> {
//     if age > 20 {
//         Ok(String::from("Ok"))
//     } else {
//         Err(String::from("Err"))
//     }
// }

/* 如果 main 和 do_something 之间返回的的 Err 类型不匹配， 如何处理？ */
type MyResult<T> = Result<T, MyError>; // type 定义别名
// 例如 type str = String; 从此可以使用 str 代替 String
pub fn error_handling() -> MyResult<()>
{
    let age = 20;
    // 1.
    // do_something(age)?;
    // Ok(())
    // 2. 使用闭包
    do_something(age).map_err(|err| format!("this is a error: {}", err))?; // err错误从 i32 转换成了 String
    Ok(())
}

fn do_something(age: u8) -> Result<String, i32> {
    if age > 20 {
        Ok(String::from("Ok"))
    } else {
        Err(404)
    }
}

#[derive(Debug)]
pub struct MyError(String);
impl From<i32> for MyError {
    fn from(code: i32) -> Self {
        MyError(String::from("something went wrong"))
    }
}
impl From<String> for MyError {
    fn from(value: String) -> Self {
        MyError(value)
    }
}





/* 常见错误处理的方法 */
// use std::io;
//
// enum AppError {
//     DatabaseError(String),
//     JwtError(String),
//     Unauthorized,
//     BadRequest(i32),
//     IoError(String),
// }
//
// impl From<io::Error> for AppError {
//     fn from(value: io::Error) -> Self {
//         Self::IoError(format!("io error: {}", value))
//     }
// }