pub fn generate_vec_impls() -> proc_macro2::TokenStream {
    quote::quote! {
        impl CdkActTryFromVmValue<Vec<bool>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<bool>, CdkActTryFromVmValueError> {
                Ok(_kybra_unwrap_rust_python_result(self.try_into_value(vm), vm))
            }
        }

        impl CdkActTryFromVmValue<Vec<String>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<String>, CdkActTryFromVmValueError> {
                Ok(_kybra_unwrap_rust_python_result(self.try_into_value(vm), vm))
            }
        }

        impl CdkActTryFromVmValue<Vec<f64>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<f64>, CdkActTryFromVmValueError> {
                Ok(_kybra_unwrap_rust_python_result(self.try_into_value(vm), vm))
            }
        }

        impl CdkActTryFromVmValue<Vec<f32>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<f32>, CdkActTryFromVmValueError> {
                Ok(_kybra_unwrap_rust_python_result(self.try_into_value(vm), vm))
            }
        }

        impl CdkActTryFromVmValue<Vec<i128>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<i128>, CdkActTryFromVmValueError> {
                Ok(_kybra_unwrap_rust_python_result(self.try_into_value(vm), vm))
            }
        }

        impl CdkActTryFromVmValue<Vec<i64>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<i64>, CdkActTryFromVmValueError> {
                Ok(_kybra_unwrap_rust_python_result(self.try_into_value(vm), vm))
            }
        }

        impl CdkActTryFromVmValue<Vec<i32>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<i32>, CdkActTryFromVmValueError> {
                Ok(_kybra_unwrap_rust_python_result(self.try_into_value(vm), vm))
            }
        }

        impl CdkActTryFromVmValue<Vec<i16>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<i16>, CdkActTryFromVmValueError> {
                Ok(_kybra_unwrap_rust_python_result(self.try_into_value(vm), vm))
            }
        }

        impl CdkActTryFromVmValue<Vec<i8>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<i8>, CdkActTryFromVmValueError> {
                Ok(_kybra_unwrap_rust_python_result(self.try_into_value(vm), vm))
            }
        }

        impl CdkActTryFromVmValue<Vec<u128>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<u128>, CdkActTryFromVmValueError> {
                Ok(_kybra_unwrap_rust_python_result(self.try_into_value(vm), vm))
            }
        }

        impl CdkActTryFromVmValue<Vec<u64>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<u64>, CdkActTryFromVmValueError> {
                Ok(_kybra_unwrap_rust_python_result(self.try_into_value(vm), vm))
            }
        }

        impl CdkActTryFromVmValue<Vec<u32>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<u32>, CdkActTryFromVmValueError> {
                Ok(_kybra_unwrap_rust_python_result(self.try_into_value(vm), vm))
            }
        }

        impl CdkActTryFromVmValue<Vec<u16>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<u16>, CdkActTryFromVmValueError> {
                Ok(_kybra_unwrap_rust_python_result(self.try_into_value(vm), vm))
            }
        }

        impl CdkActTryFromVmValue<Vec<u8>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<u8>, CdkActTryFromVmValueError> {
                Ok(_kybra_unwrap_rust_python_result(self.try_into_value(vm), vm))
            }
        }

        trait KybraTryFromVec {}

        impl<T> KybraTryFromVec for Vec<T> {}

        impl KybraTryFromVec for () {}

        impl<T> KybraTryFromVec for Option<T> {}

        impl KybraTryFromVec for ic_cdk::export::candid::Empty {}

        impl KybraTryFromVec for ic_cdk::export::candid::Reserved {}

        impl KybraTryFromVec for ic_cdk::export::candid::Func {}

        impl KybraTryFromVec for ic_cdk::export::Principal {}

        impl KybraTryFromVec for ic_cdk::timer::TimerId {}

        impl KybraTryFromVec for ic_cdk::export::candid::Int {}

        impl KybraTryFromVec for ic_cdk::export::candid::Nat {}

        impl<T> CdkActTryFromVmValue<Vec<T>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef
        where
            T: KybraTryFromVec,
            rustpython::vm::PyObjectRef: for<'a> CdkActTryFromVmValue<T, &'a rustpython::vm::VirtualMachine>
        {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<T>, CdkActTryFromVmValueError> {
                try_from_vm_value_generic_array(self, vm)
            }
        }

        fn try_from_vm_value_generic_array<T>(py_object_ref: rustpython::vm::PyObjectRef, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<T>, CdkActTryFromVmValueError>
        where
            rustpython::vm::PyObjectRef: for<'a> CdkActTryFromVmValue<T, &'a rustpython::vm::VirtualMachine>
        {
            let py_list: PyListRef = _kybra_unwrap_rust_python_result(py_object_ref.try_into_value(vm), vm);
            let vec = py_list.borrow_vec();

            Ok(vec.iter().map(|item| {
                item.clone().try_from_vm_value(vm).unwrap()
            }).collect())
        }
    }
}
