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