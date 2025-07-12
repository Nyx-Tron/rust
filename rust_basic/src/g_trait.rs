use std::mem;

pub fn trait_function() {
    let player = Player::new("Tom".to_string());
    let game = Game::new(player);
    game.play();

    let game2 = Game2::new(Box::new(Newborn));
    game2.play();
}

trait Person {
    fn get_name(&self) -> String;
    fn say(&self) {
        println!("hello, {}", self.get_name());
    }
}

struct Player {
    name: String,
}
impl Player {
    fn new(name: String) -> Self {
        Self {
            name
        }
    }
}
impl Person for Player {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    // fn get_name(&mut self) -> String { // 高阶用法，将结构体中的量取出
    //     mem::take(&mut self.name)
    // }
}

struct Newborn; // 空结构体

impl Person for Newborn {
    fn get_name(&self) -> String {
        String::from("unknown")
    }
}

struct Port{
    name: String,
}

impl Person for Port {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

// struct Game<T: Person + ...> { // 泛型
//     person: T,
// }
struct Game<T> // 第二种约束方法
where T: Person
{ // 泛型
    person: T,
}
impl<T: Person> Game<T> {
    fn new(person: T) -> Self {
        Self {
            person
        }
    }
    fn play(&self) {
        println!("{} playing game ...", self.person.get_name()); // 需要让T和Person有某种关联
    }
}

// trait 对象
struct Game2 {
    person: Box<dyn Person>, // 编译时 trait对象 未知大小
}
impl Game2 {
    fn new(person: Box<dyn Person>) -> Self{
        Game2{
            person,
        }
    }
    fn play(&self) {
        println!("{} playing game2 ...", self.person.get_name());
    }
}


