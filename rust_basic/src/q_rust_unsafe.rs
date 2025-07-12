use std::collections::HashMap;
use std::marker::PhantomPinned;
use std::mem::MaybeUninit;
use std::pin::Pin;
use std::{mem};

/* 使用Pin，是存在问题的 */
/*
    Unpin !Unpin
    大部分自定义的变量类型都是 Unpin 的，而 Pin 一般针对于 !Unpin 类型，Unpin 一般是无法固定的。
*/
// pub fn rust_unsafe()
// {
//     let map = HashMap::from([
//         (String::from("Nyx"), String::from("Handsome boy")),
//         (String::from("xyz"), String::from("Pretty girl")),
//     ]);
//     let engine = MySelfRefEngine::new(map);
//     println!("Translate result: {}", engine.translate("Nyx"));
//     println!("Translate result: {}", engine.translate("xyz"));
// }
// const DEFAULT_VALUE: &str = "";
//
// struct MySelfRefEngine {
//     map: HashMap<String, String>,
//     translator: PtrTranslator,
// }
// impl MySelfRefEngine {
//     fn new(map: HashMap<String, String>) -> Pin<Box<Self>> {
//         let mut engine = Box::pin(Self{
//             map,
//             translator: PtrTranslator{
//                 map: ptr::null()
//             },
//         });
//
//         let map_ptr: *const HashMap<String, String> = &engine.map;
//         engine.translator.map = map_ptr;
//
//         engine
//     }
//     fn translate(&self, text: &str) -> &str {
//         println!("Translation map size: {}", self.map.len());
//         println!("Translator: {}", text);
//         self.translator.translate(text).unwrap_or(DEFAULT_VALUE)
//     }
// }
//
// struct PtrTranslator {
//     map: *const HashMap<String, String>,
// }
// impl PtrTranslator {
//     fn translate(&self, text: &str) -> Option<&str> {
//         unsafe { &*self.map }.get(text).map(|s| s.as_str())
//     }
// }




/* 修改后 fixed */
// pub fn rust_unsafe()
// {
//     let map = HashMap::from([
//         (String::from("Nyx"), String::from("Handsome boy")),
//         (String::from("xyz"), String::from("Pretty girl")),
//     ]);
//     let engine = MySelfRefEngine::new(map);
//     println!("Translate result: {}", engine.translate("Nyx"));
//     println!("Translate result: {}", engine.translate("xyz"));
// }
// const DEFAULT_VALUE: &str = "";
//
// struct MySelfRefEngine {
//     map: HashMap<String, String>,
//     translator: PtrTranslator,
//     _marker: PhantomPinned, // 是整个结构体为 !Unpin 类型，被 Pin 固定后，就没有办法正常引用了
// }
// impl MySelfRefEngine {
//     fn new(map: HashMap<String, String>) -> Pin<Box<Self>> {
//         let mut engine = Box::pin(Self{
//             map,
//             translator: PtrTranslator{
//                 map: ptr::null()
//             },
//             _marker: PhantomPinned,
//         });
//
//         let map_ptr: *const HashMap<String, String> = &engine.map;
//         unsafe{
//             let reference = engine.as_mut().get_unchecked_mut();
//             reference.translator.map = map_ptr;
//         }
//         engine
//     }
//     fn translate(&self, text: &str) -> &str {
//         println!("Translation map size: {}", self.map.len());
//         println!("Translator: {}", text);
//         self.translator.translate(text).unwrap_or(DEFAULT_VALUE)
//     }
// }
//
// struct PtrTranslator {
//     map: *const HashMap<String, String>,
// }
// impl PtrTranslator {
//     fn translate(&self, text: &str) -> Option<&str> {
//         unsafe { &*self.map }.get(text).map(|s| s.as_str())
//     }
// }


/*  */
pub fn rust_unsafe()
{
    let map = HashMap::from([
        (String::from("Nyx"), String::from("Handsome boy")),
        (String::from("xyz"), String::from("Pretty girl")),
    ]);
    let engine = MySelfRefEngine2::new(map);
    println!("Translate result: {}", engine.translate("Nyx"));
    println!("Translate result: {}", engine.translate("xyz"));
}
const DEFAULT_VALUE: &str = "";

struct RefTranslator<'a> {
    map: &'a HashMap<String, String>,
}

impl<'a> RefTranslator<'a> {
    fn translate(&self, text: &str) -> Option<&str> {
        unsafe { &*self.map }.get(text).map(|s| s.as_str())
    }
}

struct MySelfRefEngine2 {
    map: HashMap<String, String>,
    translator: RefTranslator<'static>,
    _marker: PhantomPinned,
}

impl MySelfRefEngine2 {
    fn new(map: HashMap<String, String>) -> Pin<Box<Self>> {
        let mut engine = Box::pin(Self{
            map,
            translator: RefTranslator {
                map: unsafe{ MaybeUninit::uninit().assume_init() }
            },
            _marker: PhantomPinned,
        });
        // let map_ptr: *const HashMap<String, String> = &engine.map;
        // unsafe {
        //     let reference = engine.as_mut().get_unchecked_mut();
        //     reference.translator.map = &*map_ptr;
        // }
        let map_ptr = unsafe {
            mem::transmute(&engine.map)
        };
        unsafe {
            let reference = engine.as_mut().get_unchecked_mut();
            reference.translator.map = map_ptr;
        }
        engine
    }

    fn translate(&self, text: &str) -> &str {
        println!("Translation map size: {}", self.map.len());
        println!("Translator: {}", text);
        self.translator.translate(text).unwrap_or(DEFAULT_VALUE)
    }
}