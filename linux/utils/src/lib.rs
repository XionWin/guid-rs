// #[macro_export]
// macro_rules! foreach_ptr {
//     ($data: expr, $len: expr, $iter_fun: tt) => {
//         unsafe {
//             for ptr in std::slice::from_raw_parts($data, $len) {
//                 $iter_fun(ptr);
//             }
//         }
//     };
// }

extern crate proc_macro;

// use crate::proc_macro::TokenStream;

// #[proc_macro_attribute]
// pub fn trace(_attr: TokenStream, item: TokenStream) -> TokenStream {
//     dbg!(_attr);
//     let item = dbg!(item);
//     item
// }