use quote::quote;

use crate::py_ast::PyAst;
use cdk_framework::{nodes::ActPreUpgradeMethod, CanisterMethodType};

impl PyAst<'_> {
    pub fn build_pre_upgrade_method(&self) -> ActPreUpgradeMethod {
        let pre_upgrade_function_defs =
            self.get_function_def_of_type(CanisterMethodType::PreUpgrade);

        if pre_upgrade_function_defs.len() > 1 {
            todo!();
        }

        let call_to_pre_upgrade_py_function = match pre_upgrade_function_defs.get(0) {
            Some(pre_upgrade_function_def) => {
                pre_upgrade_function_def.generate_call_to_py_function()
            }
            None => quote!(),
        };

        let body = quote! {
            unsafe {
                let _kybra_interpreter = _KYBRA_INTERPRETER_OPTION.as_mut().unwrap();
                let _kybra_scope = _KYBRA_SCOPE_OPTION.as_mut().unwrap();

                _kybra_interpreter.enter(|vm| {
                    #call_to_pre_upgrade_py_function
                });
            }
        };

        ActPreUpgradeMethod { body }
    }
}
