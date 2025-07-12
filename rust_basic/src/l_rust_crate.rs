use crate::l_rust_math::*; // 导入 add 方法
pub fn rust_crate()
{
    add(12, 3);
}

/* mod 模块 */
// 1. pub(crate) 控制可访问权限
// pub(crate) mod name {
//
// }

// 2.
// pub(super) mod name {
//
// }

// 3.
// mod parent {
//     pub struct PubStruct;
//     struct PriStruct;
//
//     struct Demo {
//         sub_pub: child::PubSubStruct,
//         sub_pri: child::PriSubStruct,
//     }
//
//     pub mod child {
//         // use crate::l_rust_crate::parent::PubStruct; // crate：绝对路径，crate表示整个项目的根目录（main文件）
//         use super::{PriStruct, PubStruct}; // super：相对路径，指上一级目录 ../（这里指的是 parent）
//         pub struct PubSubStruct {
//             property: PubStruct,
//             property_pri: PriStruct,
//             /*
//                 子模块可以访问父模块的私有成员 - *1
//                 不可以访问同级模块的私有成员   - *2
//                 父模块不能引用子模块的私有成员 - *3
//             */
//         }
//         /*
//             pub(super) 指仅可以给父级模块使用
//             pub(crate) 。。。
//         */
//         pub(super) struct PriSubStruct;
//     }
// }