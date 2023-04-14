use cdk_framework::{ActDataType, ToTokenStream};
use quote::{format_ident, quote};
use syn::Ident;

use crate::py_ast::kybra_types::StableBTreeMapNode;

pub fn generate_stable_b_tree_map(
    stable_b_tree_map_nodes: &Vec<StableBTreeMapNode>,
) -> proc_macro2::TokenStream {
    let stable_b_tree_maps_and_impls =
        generate_global_stable_b_tree_maps_and_impls(stable_b_tree_map_nodes);
    let stable_b_tree_maps: Vec<proc_macro2::TokenStream> = stable_b_tree_maps_and_impls
        .iter()
        .map(|stable_b_tree_map_and_impl| stable_b_tree_map_and_impl.0.clone())
        .collect();
    let stable_b_tree_impls: Vec<proc_macro2::TokenStream> = stable_b_tree_maps_and_impls
        .iter()
        .map(|stable_b_tree_map_and_impl| stable_b_tree_map_and_impl.1.clone())
        .collect();

    // let storable_impls = generate_storable_impls(stable_b_tree_map_nodes);
    // let bounded_storable_impls = generate_bounded_storable_impls(stable_b_tree_map_nodes);

    quote! {
        use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
        use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
        use std::{borrow::Cow, cell::RefCell};
        use candid::{CandidType, Decode, Deserialize, Encode};

        // TODO prefix everything

        type Memory = VirtualMemory<DefaultMemoryImpl>;

        thread_local! {
            static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

            #(#stable_b_tree_maps)*
        }

        #(#stable_b_tree_impls)*
    }
}

fn generate_global_stable_b_tree_maps_and_impls(
    stable_b_tree_map_nodes: &Vec<StableBTreeMapNode>,
) -> Vec<(proc_macro2::TokenStream, proc_macro2::TokenStream)> {
    stable_b_tree_map_nodes
        .iter()
        .map(|stable_b_tree_map_node| {
            let map_name_ident = ref_cell_ident(stable_b_tree_map_node.memory_id);
            let memory_id = stable_b_tree_map_node.memory_id;

            let (key_wrapper_type_name, key_wrapper_type) = generate_wrapper_type(&stable_b_tree_map_node.key_type, memory_id, "Key");
            let (value_wrapper_type_name, value_wrapper_type) = generate_wrapper_type(&stable_b_tree_map_node.value_type, memory_id, "Value");

            let key_try_into_vm_value_impl = generate_try_into_vm_value_impl(&key_wrapper_type_name);
            let key_storable_impl = generate_storable_impl(&key_wrapper_type_name);
            let key_bounded_storable_impl = generate_bounded_storable_impl(&key_wrapper_type_name, stable_b_tree_map_node.max_key_size);

            let value_try_into_vm_value_impl = generate_try_into_vm_value_impl(&value_wrapper_type_name);
            let value_storable_impl = generate_storable_impl(&value_wrapper_type_name);
            let value_bounded_storable_impl = generate_bounded_storable_impl(&value_wrapper_type_name, stable_b_tree_map_node.max_value_size);

            (
                quote! {
                    static #map_name_ident: RefCell<StableBTreeMap<Memory, #key_wrapper_type_name, #value_wrapper_type_name>> = RefCell::new(StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(#memory_id))),));
                },
                quote! {
                    #key_wrapper_type
                    #key_try_into_vm_value_impl
                    #key_storable_impl
                    #key_bounded_storable_impl

                    #value_wrapper_type
                    #value_try_into_vm_value_impl
                    #value_storable_impl
                    #value_bounded_storable_impl
                }
            )
        })
        .collect()
}

pub fn ref_cell_ident(memory_id: u8) -> proc_macro2::Ident {
    format_ident!("STABLE_B_TREE_MAP_{}_REF_CELL", memory_id)
}

pub fn generate_wrapper_type(
    act_data_type: &ActDataType,
    memory_id: u8,
    key_or_value: &str,
) -> (Ident, proc_macro2::TokenStream) {
    let key_type = &act_data_type.to_token_stream(&vec![]); // TODO do we need the keyword lists?
    let wrapper_struct_name_ident =
        format_ident!("StableBTreeMap{}{}Type", memory_id, key_or_value);

    (
        wrapper_struct_name_ident.clone(),
        quote! {
            #[derive(CandidType, Deserialize, CdkActTryFromVmValue)]
            struct #wrapper_struct_name_ident(#key_type);
        },
    )
}

pub fn generate_try_into_vm_value_impl(wrapper_type_name: &Ident) -> proc_macro2::TokenStream {
    quote::quote! {
        impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for #wrapper_type_name {
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                Ok(self.0.try_into_vm_value(vm).unwrap())
            }
        }
    }
}

fn generate_storable_impl(wrapper_type_name: &Ident) -> proc_macro2::TokenStream {
    // match act_data_type {
    //     ActDataType::Array(_) => quote! {},
    //     ActDataType::Func(_) => quote! {},
    //     ActDataType::Option(_) => quote! {},
    //     ActDataType::Primitive(primitive) => match &primitive.act_type {
    //         cdk_framework::nodes::data_type_nodes::LiteralOrTypeAlias::Literal(literal) => {
    //             let wrapper_type_name_ident = format_ident!("{}", wrapper_type_name);

    //             quote! {
    //                 impl Storable for #wrapper_type_name_ident {
    //                     fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
    //                         Cow::Owned(candid::Encode!(self).unwrap())
    //                     }

    //                     fn from_bytes(bytes: Vec<u8>) -> Self {
    //                         candid::Decode!(&bytes, Self).unwrap()
    //                     }
    //                 }
    //             }
    //         }
    //         cdk_framework::nodes::data_type_nodes::LiteralOrTypeAlias::TypeAlias(_) => todo!(),
    //     },
    //     ActDataType::Record(_) => quote! {},
    //     ActDataType::Tuple(_) => quote! {},
    //     ActDataType::TypeRef(_) => quote! {},
    //     ActDataType::Variant(_) => quote! {},
    // }

    // let wrapper_type_name_ident = format_ident!("{}", wrapper_type_name);

    quote! {
        impl Storable for #wrapper_type_name {
            fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
                Cow::Owned(candid::Encode!(self).unwrap())
            }

            fn from_bytes(bytes: Vec<u8>) -> Self {
                candid::Decode!(&bytes, Self).unwrap()
            }
        }
    }
}

fn generate_bounded_storable_impl(
    wrapper_type_name: &Ident,
    max_size: u32,
) -> proc_macro2::TokenStream {
    quote! {
        impl BoundedStorable for #wrapper_type_name {
            fn max_size() -> u32 {
                #max_size
            }
        }
    }
}
