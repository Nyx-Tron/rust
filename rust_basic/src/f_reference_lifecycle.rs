use std::collections::HashMap;

pub fn reference_lifecycle() {
    /* 迭代器 */
    // let mut list = Vec::new();
    // list.push(1);
    // list.push(2);
    // list.push(3);

    // for 循环迭代便利
    // for item in list {
    //     println!("item is {}", item);
    // }

    // 迭代器 1 不消耗所有权
    // println!("不消耗所有权迭代器： ");
    // for item in list.iter() {
    //     print!("item is {} ", item);
    // }

    // 迭代器 2 消耗所有权
    // println!("消耗所有权迭代器： ");
    // for item in list.into_iter() {
    //     print!("item is {} ", item);
    // }

    // 迭代器 + 闭包
    // list.iter().for_each(|item|
    //     print!("item is {} ", item)
    // )



    /* 引用 */
    // 不可变引用
    // let num = 5;
    // let ref_num = &num;

    // 可变引用,
    // let mut num = 6;
    // let ref_num_2 = &num;
    // let mut_num = &mut num; // Cannot borrow immutable local variable `num` as mutable
    // println!("num: {}, ref_num_2: {}, mut_num: {}", num, ref_num, mut_num); // cannot borrow `num` as immutable because it is also borrowed as mutable
    // // 可变引用和只能存在一个， 不可变引用可以存在多个，但是不能和可变引用同时存在。

    // 内存拷贝方式传参，低效
    let query = String::from("name=tom&age=18");
    let qp = QueryParser::form_string(query.clone());
    println!("params: {:#?}", qp);

    // 使用引用高效传参
    let query = "name=tom&age=18";
    let qp = EnhancedQueryParser::form_string(& query); // 此时传入的生命周期为 在整个main函数中有效
    println!("string {:?}", query);
    println!("params: {:#?}", qp);
}

#[derive(Debug)]
struct QueryParser {
    query: String,
    params: HashMap<String, String>
}

impl QueryParser {
    fn form_string(query: String) -> Self {
        let params: HashMap<String, String> = query.split("&")
            .map(|item|{
                let mut parts = item.split("=");
                let key = String::from(parts.next().unwrap());
                let value = String::from(parts.next().unwrap());
                (key, value)
            })
            .collect();
        QueryParser {
            query,
            params,
        }
    }
}


#[derive(Debug)]
struct EnhancedQueryParser<'a> {
    query: &'a str,
    params: HashMap<&'a str, &'a str>
}

impl<'a> EnhancedQueryParser<'a> {
    fn form_string(query: &'a str) -> Self {
        let params:HashMap<&'a str, &'a str> = query.split("&")
            .map(|kv|{
                let mut parts = kv.split("=");
                (
                    parts.next().unwrap(),
                    parts.next().unwrap()
                )
            })
            .collect();

        return Self {
            query,
            params,
        }
    }
}