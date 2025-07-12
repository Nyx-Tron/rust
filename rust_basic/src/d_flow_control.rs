pub fn flow_control() {
    /* for 循环 */
    for i in 0..10 {
        print!("{} ", i);
    }
    println!();

    let vector = vec!(String::from("Nyx"), String::from("Jack"));
    for name in vector {
        println!("{}", name);
    }

    /* while 循环 */
    let mut i = 0;
    while i < 10 {
        print!("{} ", i);
        i += 1;
    }

    /* loop + if */
    let mut i = 0;
    loop {
        println!("hello");
        if i > 3 {
            break;
        }
        i += 1;
    }

    /* match 模式匹配 */
    let user = User::new(String::from("1"), String::from("Nyx"), Gender::MALE);
    user.print();
}

#[derive(Debug)]
struct User {
    id: String,
    name: String,
    gender: Gender,
}

#[derive(Debug)]
enum Gender {
    MALE,
    FEMALE,
    UNKNOWN,
}

impl User {
    fn new(id: String, name: String, gender: Gender) -> Self {
        Self{id, name, gender}
    }

    fn print(&self) {
        println!("{:#?}", self);

        match self.gender {
            Gender::MALE => { // arm
                println!("{} is a male", self.name);
            },
            Gender::FEMALE => {
                println!("{} is a female", self.name);
            },
            _ => {
                println!("{} is unknown", self.name);
            }
        }
    }
}