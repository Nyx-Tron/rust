pub fn basic_types() {
    let bool = true;
    let num = 20;
    let c = 'b';
    let size = size_of::<char>();
    println!("The value of bool is: {}", bool);
    println!("The value of num is: {num}");
    println!("The value of char is: {}, its size is {}", c, size_of_val(&c)); // Unicode 编码
    println!("The value of size is: {}", size);
}