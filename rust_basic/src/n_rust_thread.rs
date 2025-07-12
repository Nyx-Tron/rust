use std::{any, thread};
use std::sync::{mpsc, Arc, Mutex}; // 锁
use std::time::Instant;

pub fn rust_thread()
{
    // let person = Person {name: String::from("Free")};
    // 1. 创建线程 spawn
    /*
        closure may outlive the current function.
        这里没有使用 move，所以编译器会优先使用引用 person ；
        一旦 rust_thread 先于子线程结束，person 就变成了悬垂指针，存在内存风险.
    */
    // let handle = thread::spawn(move || {
    //     println!("Hello, world!");
    //     println!("Hello, {:?}", person);
    // });
    /*
        move 关键字：
        move converts any variables captured by reference or mutable reference to variables captured by value.
    */
    // let handle = thread::spawn(move || {
    //     println!("Hello, spawn!");
    //     println!("Hello, {:?}", person);
    // });
    // handle.join().unwrap();
    // ------------------------------------------------------------------------------------------------------------

    // 2. 创建线程 scope（作用域线程）
    /*
        作用域线程：在 main 退出之前，该线程一定会被执行完成
        scope 引用变量；spawn 一般使用 move 来拷贝本地变量
    */
    // thread::scope(|scope| {
    //     scope.spawn(|| {
    //         println!("Hello, scope!");
    //         println!("Hello, {:?}", person);
    //     });
    // });
    // ------------------------------------------------------------------------------------------------------------

    // ex. 计算累加和：1 + 2 + 3 + ... + total = ?

    // 使用单线程：
    // let total = 1000000000u64;
    // let mut sum = 0;
    // let start = Instant::now(); // 测量时间 start
    // for i in 1.total {
    //     sum += i;
    // }
    // println!("sum = {}", sum);
    // println!("total times: {}", start.elapsed().as_millis()); // 测量时间 total times


    // 对比多线程
    const TOTAL: u64 = 1000_000_000;
    count_time(|| {
        let mut sum = 0;
        for i in 1..TOTAL {
            sum += i;
        }
        println!("simple thread's sum = {}", sum);
    });

    /*
        move 后相当于 sum 复制了一份
    */
    count_time(|| {
        let mut sum = 0;
        let chunk_size = TOTAL / 16;
        let mut handles = vec![];

        for i in 0..16 {
            let start = i * chunk_size;
            let end = if i == 15 {
                TOTAL
            } else {
                (i + 1) * chunk_size
            };

            let handle = thread::spawn(move || {
                let mut local_sum = 0;
                for i in start..end {
                    local_sum += i;
                }
                sum += local_sum;
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("parallel thread's sum: {}", sum);
    });
    /*
        使用 锁 解决
        spawn
    */
    count_time(|| {
        let mut sum = Arc::new(Mutex::new(0)); // Arc：多重所有权机制，帮助多个线程拥有一份 sum 的所有权
        let chunk_size = TOTAL / 16;
        let mut handles = vec![];

        for i in 0..16 {
            let start = i * chunk_size;
            let end = if i == 15 {
                TOTAL
            } else {
                (i + 1) * chunk_size
            };

            let sum_clone = Arc::clone(&sum);
            let handle = thread::spawn(move || {
                let mut local_sum = 0;
                for i in start..end {
                    local_sum += i;
                }
                let mut sum = sum_clone.lock().unwrap();
                *sum += local_sum;
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("parallel thread's sum: {}", sum.lock().unwrap());
    });
    /*
        使用 锁 解决
        scope
    */
    count_time(|| {
        let chunk_size = TOTAL / 16;
        let sum = Mutex::new(0);

        thread::scope(|scope| {
            for i in 0..16 {
                let start = i * chunk_size;
                let end = if i == 15 {
                    TOTAL
                } else {
                    (i + 1) * chunk_size
                };

                let sum_ref = &sum;
                scope.spawn(move || {
                    let mut local_sum = 0;
                    for i in start..end {
                        local_sum += i;
                    }
                    let mut sum = sum_ref.lock().unwrap();
                    *sum += local_sum;
                });
            }
        });
        println!("scope thread's sum = {}", sum.lock().unwrap());
    });
    /*
        使用 通道 channel 解决
    */
    count_time(|| {
        let chunk_size = TOTAL / 16;
        let (tx, rx) = mpsc::channel();

        for i in 0..16 {
            let start = i * chunk_size;
            let end = if i == 15 {
                TOTAL
            } else {
                (i + 1) * chunk_size
            };

            let tx = tx.clone();

            thread::spawn(move || {
                let mut local_sum = 0;
                for i in start..end {
                    local_sum += i;
                }

                tx.send(local_sum).unwrap();
            });
        }

        drop(tx); // drop掉一开始的那个 tx

        let mut sum = 0;
        while let Ok(local_sum) = rx.recv() {
            sum += local_sum;
        }
        println!("channel' method sum = {}", sum);
    });
}

#[derive(Debug)]
struct Person {
    name: String,
}

// 统计函数 f 用时
fn count_time<F: FnOnce()>(f: F) {
    let name = any::type_name::<F>(); // 获得 String 类型的函数名
    let start = Instant::now(); // 测量时间 start
    f();
    println!("{}'s total time: {}", name, start.elapsed().as_millis()); // 测量时间 total times
}