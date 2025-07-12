mod http; // 声明（引入）该模块下的子模块，私有成员，使用 pub use http::get; 进行重新导入
pub use http::get; // 重新导出 get 方法，模块使用 utils::get(); （相当于当前模块生命该方法）


// pub mod http; // 模块使用 utils::http::get();

pub fn test()
{
    println!("Hello, test crate!");
}