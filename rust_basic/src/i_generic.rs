/* 先看问题： */
// pub fn generic()
// {
//     println!("1 + 2 = {}", add(1, 2));
//     println!("1 + 2 = {}", add_u8(1u8, 2u8));
// }
//
// fn add(a: i32, b: i32) -> i32{
//     a + b
// }
//
// fn add_u8(a: u8, b: u8) -> u8{
//     a + b
// }


/* 使用泛型 */
// use std::ops::Add;
//
// pub fn generic()
// {
//     println!("1 + 2 = {}", add(2, 3));
//
//     let point_a = Point { x: 1, y: 2 };
//     let point_b = Point { x: 2, y: 3 };
//     let result = add(point_a.clone(), point_b.clone());
//     println!("point_a + point_b = {:?}", result);
//     println!("{:?} + {:?} = {:?}", point_a, point_b, result);
// }
// fn add<T: Add<Output = T>>(a: T, b: T) -> T {
//     a + b // a, b 必须同时满足 add trait 特征才能实现相加
// }
//
// #[derive(Debug, Copy, Clone)]
// struct Point<T = i32> { // 泛型默认值为 i32
//     x: T,
//     y: T,
// }
// impl Add for Point {
//     type Output = Self;
//
//     fn add(self, other: Self) -> Self {
//         Point { x: self.x + other.x, y: self.y + other.y }
//     }
// }


/* 不使用 clone */
use std::ops::Add;

pub fn generic()
{
    let point_a = Point { x: 1, y: 2 };
    let point_b = Point { x: 2, y: 3 };
    println!("point_a = {:?}, point_b = {:?}", point_a, point_b);
    println!("result = {:?}", add(&point_a, &point_b));
}
fn add<T>(a: &T, b: &T) -> T
where
    for<'a> &'a T: Add<Output = T>, // &T: Bound Conditions
{
    a + b // a, b 必须同时满足 add trait 特征才能实现相加
}

#[derive(Debug)]
struct Point<T = i32> { // 泛型默认值为 i32
    x: T,
    y: T,
}

impl Add for &Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}