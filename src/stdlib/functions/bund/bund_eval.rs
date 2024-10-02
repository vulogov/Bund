extern crate log;
// use multistackvm::multistackvm::{VM, StackOps};
// use easy_error::{Error, bail};
//
//
// pub fn bund_eval_base(vm: &mut VM, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
//     match op {
//         StackOps::FromStack => {
//             if vm.stack.current_stack_len() < 1 {
//                 bail!("Stack is too shallow for inline {}", &err_prefix);
//             }
//         }
//         StackOps::FromWorkBench => {
//             if vm.stack.current_stack_len() < 1 {
//                 bail!("Stack is too shallow for inline {}", &err_prefix);
//             }
//             if vm.stack.workbench.len() < 1 {
//                 bail!("Workbench is too shallow for inline {}", &err_prefix);
//             }
//         }
//     }
//     let snippet_val = match op {
//         StackOps::FromStack => vm.stack.pull(),
//         StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
//     };
//     match snippet_val {
//         Some(snippet_val) => {
//             match snippet_val.cast_string() {
//                 Ok(snippet) => {
//
//                 }
//                 Err(err) => {
//                     bail!("{} returns: NO DATA", &err_prefix);
//                 }
//             }
//         }
//         None => {
//             bail!("{} returns: NO DATA", &err_prefix);
//         }
//     }
// }
//
pub fn init_stdlib() {

}
