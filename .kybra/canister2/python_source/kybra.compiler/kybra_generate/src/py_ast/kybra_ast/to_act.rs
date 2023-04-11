use crate::generators::vm_value_conversion::{try_from_vm_value, try_into_vm_value};
use cdk_framework::{AbstractCanisterTree, ActCanisterMethod, ActDataType, ToAct};

use super::KybraAst;

impl ToAct for KybraAst {
    fn to_act(&self) -> AbstractCanisterTree {
        let query_methods: Vec<ActCanisterMethod> = self
            .canister_methods
            .iter()
            .filter(|method| match method {
                ActCanisterMethod::QueryMethod { .. } => true,
                ActCanisterMethod::UpdateMethod(_) => false,
            })
            .cloned()
            .collect();
        let update_methods: Vec<ActCanisterMethod> = self
            .canister_methods
            .iter()
            .filter(|method| match method {
                ActCanisterMethod::QueryMethod { .. } => false,
                ActCanisterMethod::UpdateMethod(_) => true,
            })
            .cloned()
            .collect();

        let arrays: Vec<ActDataType> = self
            .canister_types
            .iter()
            .filter(|act| match act {
                ActDataType::Array(_) => true,
                _ => false,
            })
            .cloned()
            .collect();
        let funcs: Vec<ActDataType> = self
            .canister_types
            .iter()
            .filter(|act| match act {
                ActDataType::Func(_) => true,
                _ => false,
            })
            .cloned()
            .collect();
        let options: Vec<ActDataType> = self
            .canister_types
            .iter()
            .filter(|act| match act {
                ActDataType::Option(_) => true,
                _ => false,
            })
            .cloned()
            .collect();
        let primitives: Vec<ActDataType> = self
            .canister_types
            .iter()
            .filter(|act| match act {
                ActDataType::Primitive(_) => true,
                _ => false,
            })
            .cloned()
            .collect();
        let records: Vec<ActDataType> = self
            .canister_types
            .iter()
            .filter(|act| match act {
                ActDataType::Record(_) => true,
                _ => false,
            })
            .cloned()
            .collect();
        let tuples: Vec<ActDataType> = self
            .canister_types
            .iter()
            .filter(|act| match act {
                ActDataType::Tuple(_) => true,
                _ => false,
            })
            .cloned()
            .collect();
        let type_refs: Vec<ActDataType> = self
            .canister_types
            .iter()
            .filter(|act| match act {
                ActDataType::TypeRef(_) => true,
                _ => false,
            })
            .cloned()
            .collect();
        let variants: Vec<ActDataType> = self
            .canister_types
            .iter()
            .filter(|act| match act {
                ActDataType::Variant(_) => true,
                _ => false,
            })
            .cloned()
            .collect();

        let heartbeat_method = self.heartbeat.clone();
        let init_method = self.init_method.clone();
        let inspect_message_method = self.inspect_method.clone();
        let post_upgrade_method = self.post_upgrade.clone();
        let pre_upgrade_method = self.pre_upgrade.clone();

        let external_canisters = self.external_canisters.clone();

        let try_into_vm_value_impls = try_into_vm_value::generate_try_into_vm_value_impls();
        let try_from_vm_value_impls = try_from_vm_value::generate_try_from_vm_value_impls();

        AbstractCanisterTree {
            cdk_name: "kybra".to_string(),
            body: self.rust_code.clone(),
            update_methods,
            query_methods,
            heartbeat_method,
            init_method,
            inspect_message_method,
            post_upgrade_method,
            pre_upgrade_method,
            arrays,
            funcs,
            options,
            primitives,
            records,
            try_from_vm_value_impls,
            try_into_vm_value_impls,
            tuples,
            type_refs,
            variants,
            external_canisters,
            keywords: crate::get_python_keywords(),
            header: self.header.clone(),
        }
    }
}
