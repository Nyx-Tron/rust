use crate::k_error_handling::MyError;

mod b_variable;
mod a_basic_types;
mod c_complex_types;
mod d_flow_control;
mod e_ownership;
mod f_reference_lifecycle;
mod g_trait;
mod h_smart_pointer;
mod i_generic;
mod j_closure;
mod k_error_handling;
mod l_rust_crate;
mod l_rust_math;
mod utils;
mod m_rust_test;
mod n_rust_thread;
mod o_asynchronous;
mod p_macro;
mod q_rust_unsafe;

fn main()
{
    // 基础
    // a_basic_types::basic_types();
    // b_variable::variable();
    // c_complex_types::complex_types();
    // d_flow_control::flow_control();
    // e_ownership::ownership();
    // f_reference_lifecycle::reference_lifecycle();
    // g_trait::trait_function();
    // h_smart_pointer::smart_pointer();

    // 高级
    // i_generic::generic();
    // j_closure::closure();
    // m_rust_test::rust_test();
    // n_rust_thread::rust_thread();
    // o_asynchronous::asynchronous();
    // p_macro::rust_macro();
    q_rust_unsafe::rust_unsafe();

}



// fn main() -> Result<(), MyError>
// {
//     // 高级
//     let _ = k_error_handling::error_handling()?;
//     utils::get(); // 模块使用
//     Ok(())
// }



