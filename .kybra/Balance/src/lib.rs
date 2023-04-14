#![allow(warnings, unused)]
use ic_cdk::api::call::CallResult;
use kybra_vm_value_derive::{CdkActTryFromVmValue, CdkActTryIntoVmValue};
use rand::{rngs::StdRng, Rng, SeedableRng};
use rustpython_derive::{pyclass, PyPayload};
use rustpython_vm::{
    builtins::{
        PyBaseException, PyBytes, PyDict, PyGenerator, PyIntRef, PyList, PyListRef, PyStr, PyTuple,
        PyTupleRef,
    },
    class::PyClassImpl,
    convert::ToPyObject,
    function::IntoFuncArgs,
    protocol::{PyIter, PyIterReturn},
    py_serde::{deserialize, serialize},
    AsObject, PyObject, PyObjectRef, PyRef, VirtualMachine,
};
use serde::de::{DeserializeSeed, Visitor};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, SerializeTuple};
use slotmap::Key;
use std::str::FromStr;
thread_local! { static RNG_REF_CELL : std :: cell :: RefCell < StdRng > = std :: cell :: RefCell :: new (SeedableRng :: from_seed ([0u8 ; 32])) ; }
static mut _KYBRA_INTERPRETER_OPTION: Option<rustpython_vm::Interpreter> = None;
static mut _KYBRA_SCOPE_OPTION: Option<rustpython_vm::scope::Scope> = None;
fn _kybra_custom_getrandom(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    RNG_REF_CELL.with(|rng_ref_cell| {
        let mut rng = rng_ref_cell.borrow_mut();
        rng.fill(_buf);
    });
    Ok(())
}
getrandom::register_custom_getrandom!(_kybra_custom_getrandom);
pub trait CdkActTryIntoVmValue<Context, VmValue> {
    fn try_into_vm_value(self, context: Context) -> Result<VmValue, CdkActTryIntoVmValueError>;
}
#[derive(Debug)]
pub struct CdkActTryIntoVmValueError(pub String);
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for () {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(vm.ctx.none())
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for bool {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for ic_cdk::export::candid::Empty
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        panic!("Empty cannot be converted into PyObjectRef");
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for ic_cdk::export::candid::Func
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(vm
            .ctx
            .new_tuple(vec![
                self.principal.try_into_vm_value(vm).unwrap(),
                self.method.try_into_vm_value(vm).unwrap(),
            ])
            .into())
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for ic_cdk::export::Principal
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        let principal_class = _kybra_unwrap_rust_python_result(
            vm.run_block_expr(
                vm.new_scope_with_builtins(),
                r#"
from kybra import Principal

Principal
                    "#,
            ),
            vm,
        );
        let from_str =
            _kybra_unwrap_rust_python_result(principal_class.get_attr("from_str", vm), vm);
        let principal_instance =
            _kybra_unwrap_rust_python_result(vm.invoke(&from_str, (self.to_text(),)), vm);
        Ok(principal_instance)
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for ic_cdk::api::call::RejectionCode
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        match self {
            ic_cdk::api::call::RejectionCode::NoError => {
                let dict = vm.ctx.new_dict();
                dict.set_item("NoError", vm.ctx.none(), vm);
                Ok(dict.into())
            }
            ic_cdk::api::call::RejectionCode::SysFatal => {
                let dict = vm.ctx.new_dict();
                dict.set_item("SysFatal", vm.ctx.none(), vm);
                Ok(dict.into())
            }
            ic_cdk::api::call::RejectionCode::SysTransient => {
                let dict = vm.ctx.new_dict();
                dict.set_item("SysTransient", vm.ctx.none(), vm);
                Ok(dict.into())
            }
            ic_cdk::api::call::RejectionCode::DestinationInvalid => {
                let dict = vm.ctx.new_dict();
                dict.set_item("DestinationInvalid", vm.ctx.none(), vm);
                Ok(dict.into())
            }
            ic_cdk::api::call::RejectionCode::CanisterReject => {
                let dict = vm.ctx.new_dict();
                dict.set_item("CanisterReject", vm.ctx.none(), vm);
                Ok(dict.into())
            }
            ic_cdk::api::call::RejectionCode::CanisterError => {
                let dict = vm.ctx.new_dict();
                dict.set_item("CanisterError", vm.ctx.none(), vm);
                Ok(dict.into())
            }
            ic_cdk::api::call::RejectionCode::Unknown => {
                let dict = vm.ctx.new_dict();
                dict.set_item("Unknown", vm.ctx.none(), vm);
                Ok(dict.into())
            }
        }
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for ic_cdk::export::candid::Reserved
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(vm.ctx.none())
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for ic_cdk::timer::TimerId
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.data().as_ffi().to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for ic_cdk::api::stable::StableMemoryError
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        match self {
            ic_cdk::api::stable::StableMemoryError::OutOfMemory => {
                let dict = vm.ctx.new_dict();
                dict.set_item("OutOfMemory", vm.ctx.none(), vm);
                Ok(dict.into())
            }
            ic_cdk::api::stable::StableMemoryError::OutOfBounds => {
                let dict = vm.ctx.new_dict();
                dict.set_item("OutOfBounds", vm.ctx.none(), vm);
                Ok(dict.into())
            }
        }
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for String {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for ic_stable_structures::btreemap::InsertError
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        match self {
            ic_stable_structures::btreemap::InsertError::KeyTooLarge { given, max } => {
                let dict = vm.ctx.new_dict();
                let key_too_large_dict = vm.ctx.new_dict();
                key_too_large_dict.set_item("given", given.try_into_vm_value(vm).unwrap(), vm);
                key_too_large_dict.set_item("max", max.try_into_vm_value(vm).unwrap(), vm);
                dict.set_item("KeyTooLarge", key_too_large_dict.into(), vm);
                Ok(dict.into())
            }
            ic_stable_structures::btreemap::InsertError::ValueTooLarge { given, max } => {
                let dict = vm.ctx.new_dict();
                let value_too_large_dict = vm.ctx.new_dict();
                value_too_large_dict.set_item("given", given.try_into_vm_value(vm).unwrap(), vm);
                value_too_large_dict.set_item("max", max.try_into_vm_value(vm).unwrap(), vm);
                dict.set_item("ValueTooLarge", value_too_large_dict.into(), vm);
                Ok(dict.into())
            }
        }
    }
}
impl<T> CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for (T,)
where
    T: for<'a> CdkActTryIntoVmValue<
        &'a rustpython::vm::VirtualMachine,
        rustpython::vm::PyObjectRef,
    >,
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        self.0.try_into_vm_value(vm)
    }
}
impl<T> CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for Box<T>
where
    T: for<'a> CdkActTryIntoVmValue<
        &'a rustpython::vm::VirtualMachine,
        rustpython::vm::PyObjectRef,
    >,
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        (*self).try_into_vm_value(vm)
    }
}
impl<T> CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for Option<T>
where
    T: for<'a> CdkActTryIntoVmValue<
        &'a rustpython::vm::VirtualMachine,
        rustpython::vm::PyObjectRef,
    >,
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        match self {
            Some(value) => Ok(value.try_into_vm_value(vm).unwrap()),
            None => Ok(().to_pyobject(vm)),
        }
    }
}
impl<T, K> CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for Result<T, K>
where
    T: for<'a> CdkActTryIntoVmValue<
        &'a rustpython::vm::VirtualMachine,
        rustpython::vm::PyObjectRef,
    >,
    K: for<'a> CdkActTryIntoVmValue<
        &'a rustpython::vm::VirtualMachine,
        rustpython::vm::PyObjectRef,
    >,
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        match self {
            Ok(ok) => {
                let dict = vm.ctx.new_dict();
                dict.set_item("ok", ok.try_into_vm_value(vm).unwrap(), vm);
                Ok(dict.into())
            }
            Err(err) => {
                let dict = vm.ctx.new_dict();
                dict.set_item("err", err.try_into_vm_value(vm).unwrap(), vm);
                Ok(dict.into())
            }
        }
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for f64 {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for f32 {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for ic_cdk::export::candid::Int
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(vm.ctx.new_int(self.0).into())
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for i128 {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for i64 {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for i32 {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for i16 {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for i8 {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for ic_cdk::export::candid::Nat
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(vm.ctx.new_int(self.0).into())
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for u128 {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for u64 {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for usize {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for u32 {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for u16 {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for u8 {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
trait KybraTryIntoVec {}
impl KybraTryIntoVec for () {}
impl KybraTryIntoVec for bool {}
impl KybraTryIntoVec for String {}
impl KybraTryIntoVec for ic_cdk::export::candid::Empty {}
impl KybraTryIntoVec for ic_cdk::export::candid::Reserved {}
impl KybraTryIntoVec for ic_cdk::export::candid::Func {}
impl KybraTryIntoVec for ic_cdk::export::Principal {}
impl KybraTryIntoVec for ic_cdk::timer::TimerId {}
impl KybraTryIntoVec for ic_cdk::api::call::RejectionCode {}
impl KybraTryIntoVec for f64 {}
impl KybraTryIntoVec for f32 {}
impl KybraTryIntoVec for ic_cdk::export::candid::Int {}
impl KybraTryIntoVec for i128 {}
impl KybraTryIntoVec for i64 {}
impl KybraTryIntoVec for i32 {}
impl KybraTryIntoVec for i16 {}
impl KybraTryIntoVec for i8 {}
impl KybraTryIntoVec for ic_cdk::export::candid::Nat {}
impl KybraTryIntoVec for u128 {}
impl KybraTryIntoVec for u64 {}
impl KybraTryIntoVec for usize {}
impl KybraTryIntoVec for u32 {}
impl KybraTryIntoVec for u16 {}
impl<T> KybraTryIntoVec for Option<T> {}
impl<T> KybraTryIntoVec for Vec<T> {}
impl<T> CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for Vec<T>
where
    T: KybraTryIntoVec,
    T: for<'a> CdkActTryIntoVmValue<
        &'a rustpython::vm::VirtualMachine,
        rustpython::vm::PyObjectRef,
    >,
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        try_into_vm_value_generic_array(self, vm)
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for Vec<u8>
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(vm.ctx.new_bytes(self).into())
    }
}
fn try_into_vm_value_generic_array<T>(
    generic_array: Vec<T>,
    vm: &rustpython::vm::VirtualMachine,
) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError>
where
    T: for<'a> CdkActTryIntoVmValue<
        &'a rustpython::vm::VirtualMachine,
        rustpython::vm::PyObjectRef,
    >,
{
    let py_object_refs = generic_array
        .into_iter()
        .map(|item| item.try_into_vm_value(vm).unwrap())
        .collect::<Vec<PyObjectRef>>();
    Ok(vm.ctx.new_list(py_object_refs).into())
}
pub trait CdkActTryFromVmValue<T, Context> {
    fn try_from_vm_value(self, context: Context) -> Result<T, CdkActTryFromVmValueError>;
}
#[derive(Debug)]
pub struct CdkActTryFromVmValueError(pub String);
impl CdkActTryFromVmValue<(), &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<(), CdkActTryFromVmValueError> {
        Ok(())
    }
}
impl CdkActTryFromVmValue<bool, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<bool, CdkActTryFromVmValueError> {
        match self.try_into_value(vm) {
            Ok(value) => Ok(value),
            Err(err) => Err(CdkActTryFromVmValueError(
                "Could not convert PyObjectRef to bool".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<ic_cdk::export::candid::Empty, &rustpython::vm::VirtualMachine>
    for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<ic_cdk::export::candid::Empty, CdkActTryFromVmValueError> {
        panic!("PyObjectRef cannot be converted into Empty");
    }
}
impl CdkActTryFromVmValue<ic_cdk::export::candid::Func, &rustpython::vm::VirtualMachine>
    for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<ic_cdk::export::candid::Func, CdkActTryFromVmValueError> {
        let tuple_self: PyTupleRef = _kybra_unwrap_rust_python_result(self.try_into_value(vm), vm);
        let principal = tuple_self.get(0).unwrap();
        let method = tuple_self.get(1).unwrap();
        Ok(ic_cdk::export::candid::Func {
            principal: principal.clone().try_from_vm_value(vm).unwrap(),
            method: method.clone().try_from_vm_value(vm).unwrap(),
        })
    }
}
impl CdkActTryFromVmValue<ic_cdk::export::Principal, &rustpython::vm::VirtualMachine>
    for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<ic_cdk::export::Principal, CdkActTryFromVmValueError> {
        let to_str = _kybra_unwrap_rust_python_result(self.get_attr("to_str", vm), vm);
        let result = _kybra_unwrap_rust_python_result(vm.invoke(&to_str, ()), vm);
        let result_string: String = _kybra_unwrap_rust_python_result(result.try_into_value(vm), vm);
        Ok(ic_cdk::export::Principal::from_text(result_string).unwrap())
    }
}
impl CdkActTryFromVmValue<ic_cdk::export::candid::Reserved, &rustpython::vm::VirtualMachine>
    for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<ic_cdk::export::candid::Reserved, CdkActTryFromVmValueError> {
        Ok(ic_cdk::export::candid::Reserved)
    }
}
impl CdkActTryFromVmValue<ic_cdk::timer::TimerId, &rustpython::vm::VirtualMachine>
    for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<ic_cdk::timer::TimerId, CdkActTryFromVmValueError> {
        let vm_value_as_u64: u64 = _kybra_unwrap_rust_python_result(self.try_into_value(vm), vm);
        Ok(ic_cdk::timer::TimerId::from(slotmap::KeyData::from_ffi(
            vm_value_as_u64,
        )))
    }
}
impl CdkActTryFromVmValue<String, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<String, CdkActTryFromVmValueError> {
        match self.try_into_value(vm) {
            Ok(value) => Ok(value),
            Err(err) => Err(CdkActTryFromVmValueError(
                "Could not convert PyObjectRef to String".to_string(),
            )),
        }
    }
}
impl<T> CdkActTryFromVmValue<(T,), &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef
where
    rustpython::vm::PyObjectRef:
        for<'a> CdkActTryFromVmValue<T, &'a rustpython::vm::VirtualMachine>,
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<(T,), CdkActTryFromVmValueError> {
        Ok((self.try_from_vm_value(vm).unwrap(),))
    }
}
impl<T> CdkActTryFromVmValue<Box<T>, &rustpython::vm::VirtualMachine>
    for rustpython::vm::PyObjectRef
where
    rustpython::vm::PyObjectRef:
        for<'a> CdkActTryFromVmValue<T, &'a rustpython::vm::VirtualMachine>,
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Box<T>, CdkActTryFromVmValueError> {
        match self.try_from_vm_value(vm) {
            Ok(value) => Ok(Box::new(value)),
            Err(err) => Err(err),
        }
    }
}
impl<T> CdkActTryFromVmValue<Option<T>, &rustpython::vm::VirtualMachine>
    for rustpython::vm::PyObjectRef
where
    rustpython::vm::PyObjectRef:
        for<'a> CdkActTryFromVmValue<T, &'a rustpython::vm::VirtualMachine>,
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Option<T>, CdkActTryFromVmValueError> {
        if self.is(&vm.ctx.none()) {
            Ok(None)
        } else {
            match self.try_from_vm_value(vm) {
                Ok(value) => Ok(Some(value)),
                Err(err) => Err(err),
            }
        }
    }
}
impl CdkActTryFromVmValue<f64, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<f64, CdkActTryFromVmValueError> {
        match self.try_into_value(vm) {
            Ok(value) => Ok(value),
            Err(err) => Err(CdkActTryFromVmValueError(
                "Could not convert PyObjectRef to f64".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<f32, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<f32, CdkActTryFromVmValueError> {
        match self.try_into_value(vm) {
            Ok(value) => Ok(value),
            Err(err) => Err(CdkActTryFromVmValueError(
                "Could not convert PyObjectRef to f32".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<ic_cdk::export::candid::Int, &rustpython::vm::VirtualMachine>
    for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<ic_cdk::export::candid::Int, CdkActTryFromVmValueError> {
        let int_result: Result<PyIntRef, _> = self.try_into_value(vm);
        match int_result {
            Ok(int) => Ok(ic_cdk::export::candid::Int(int.as_bigint().clone())),
            Err(_) => Err(CdkActTryFromVmValueError(
                "PyObjectRef is not a PyIntRef".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<i128, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<i128, CdkActTryFromVmValueError> {
        match self.try_into_value(vm) {
            Ok(value) => Ok(value),
            Err(err) => Err(CdkActTryFromVmValueError(
                "Could not convert PyObjectRef to i128".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<i64, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<i64, CdkActTryFromVmValueError> {
        match self.try_into_value(vm) {
            Ok(value) => Ok(value),
            Err(err) => Err(CdkActTryFromVmValueError(
                "Could not convert PyObjectRef to i64".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<i32, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<i32, CdkActTryFromVmValueError> {
        match self.try_into_value(vm) {
            Ok(value) => Ok(value),
            Err(err) => Err(CdkActTryFromVmValueError(
                "Could not convert PyObjectRef to i32".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<i16, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<i16, CdkActTryFromVmValueError> {
        match self.try_into_value(vm) {
            Ok(value) => Ok(value),
            Err(err) => Err(CdkActTryFromVmValueError(
                "Could not convert PyObjectRef to i16".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<i8, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<i8, CdkActTryFromVmValueError> {
        match self.try_into_value(vm) {
            Ok(value) => Ok(value),
            Err(err) => Err(CdkActTryFromVmValueError(
                "Could not convert PyObjectRef to i8".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<ic_cdk::export::candid::Nat, &rustpython::vm::VirtualMachine>
    for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<ic_cdk::export::candid::Nat, CdkActTryFromVmValueError> {
        let int_result: Result<PyIntRef, _> = self.try_into_value(vm);
        match int_result {
            Ok(int) => {
                Ok(ic_cdk::export::candid::Nat::from_str(&int.as_bigint().to_string()).unwrap())
            }
            Err(_) => Err(CdkActTryFromVmValueError(
                "PyObjectRef is not a PyIntRef".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<u128, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<u128, CdkActTryFromVmValueError> {
        match self.try_into_value(vm) {
            Ok(value) => Ok(value),
            Err(err) => Err(CdkActTryFromVmValueError(
                "Could not convert PyObjectRef to u128".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<u64, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<u64, CdkActTryFromVmValueError> {
        match self.try_into_value(vm) {
            Ok(value) => Ok(value),
            Err(err) => Err(CdkActTryFromVmValueError(
                "Could not convert PyObjectRef to u64".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<usize, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<usize, CdkActTryFromVmValueError> {
        match self.try_into_value(vm) {
            Ok(value) => Ok(value),
            Err(err) => Err(CdkActTryFromVmValueError(
                "Could not convert PyObjectRef to usize".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<u32, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<u32, CdkActTryFromVmValueError> {
        match self.try_into_value(vm) {
            Ok(value) => Ok(value),
            Err(err) => Err(CdkActTryFromVmValueError(
                "Could not convert PyObjectRef to u32".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<u16, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<u16, CdkActTryFromVmValueError> {
        match self.try_into_value(vm) {
            Ok(value) => Ok(value),
            Err(err) => Err(CdkActTryFromVmValueError(
                "Could not convert PyObjectRef to u16".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<u8, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<u8, CdkActTryFromVmValueError> {
        match self.try_into_value(vm) {
            Ok(value) => Ok(value),
            Err(err) => Err(CdkActTryFromVmValueError(
                "Could not convert PyObjectRef to u8".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<Vec<bool>, &rustpython::vm::VirtualMachine>
    for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<bool>, CdkActTryFromVmValueError> {
        Ok(_kybra_unwrap_rust_python_result(
            self.try_into_value(vm),
            vm,
        ))
    }
}
impl CdkActTryFromVmValue<Vec<String>, &rustpython::vm::VirtualMachine>
    for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<String>, CdkActTryFromVmValueError> {
        Ok(_kybra_unwrap_rust_python_result(
            self.try_into_value(vm),
            vm,
        ))
    }
}
impl CdkActTryFromVmValue<Vec<f64>, &rustpython::vm::VirtualMachine>
    for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<f64>, CdkActTryFromVmValueError> {
        Ok(_kybra_unwrap_rust_python_result(
            self.try_into_value(vm),
            vm,
        ))
    }
}
impl CdkActTryFromVmValue<Vec<f32>, &rustpython::vm::VirtualMachine>
    for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<f32>, CdkActTryFromVmValueError> {
        Ok(_kybra_unwrap_rust_python_result(
            self.try_into_value(vm),
            vm,
        ))
    }
}
impl CdkActTryFromVmValue<Vec<i128>, &rustpython::vm::VirtualMachine>
    for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<i128>, CdkActTryFromVmValueError> {
        Ok(_kybra_unwrap_rust_python_result(
            self.try_into_value(vm),
            vm,
        ))
    }
}
impl CdkActTryFromVmValue<Vec<i64>, &rustpython::vm::VirtualMachine>
    for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<i64>, CdkActTryFromVmValueError> {
        Ok(_kybra_unwrap_rust_python_result(
            self.try_into_value(vm),
            vm,
        ))
    }
}
impl CdkActTryFromVmValue<Vec<i32>, &rustpython::vm::VirtualMachine>
    for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<i32>, CdkActTryFromVmValueError> {
        Ok(_kybra_unwrap_rust_python_result(
            self.try_into_value(vm),
            vm,
        ))
    }
}
impl CdkActTryFromVmValue<Vec<i16>, &rustpython::vm::VirtualMachine>
    for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<i16>, CdkActTryFromVmValueError> {
        Ok(_kybra_unwrap_rust_python_result(
            self.try_into_value(vm),
            vm,
        ))
    }
}
impl CdkActTryFromVmValue<Vec<i8>, &rustpython::vm::VirtualMachine>
    for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<i8>, CdkActTryFromVmValueError> {
        Ok(_kybra_unwrap_rust_python_result(
            self.try_into_value(vm),
            vm,
        ))
    }
}
impl CdkActTryFromVmValue<Vec<u128>, &rustpython::vm::VirtualMachine>
    for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<u128>, CdkActTryFromVmValueError> {
        Ok(_kybra_unwrap_rust_python_result(
            self.try_into_value(vm),
            vm,
        ))
    }
}
impl CdkActTryFromVmValue<Vec<u64>, &rustpython::vm::VirtualMachine>
    for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<u64>, CdkActTryFromVmValueError> {
        Ok(_kybra_unwrap_rust_python_result(
            self.try_into_value(vm),
            vm,
        ))
    }
}
impl CdkActTryFromVmValue<Vec<u32>, &rustpython::vm::VirtualMachine>
    for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<u32>, CdkActTryFromVmValueError> {
        Ok(_kybra_unwrap_rust_python_result(
            self.try_into_value(vm),
            vm,
        ))
    }
}
impl CdkActTryFromVmValue<Vec<u16>, &rustpython::vm::VirtualMachine>
    for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<u16>, CdkActTryFromVmValueError> {
        Ok(_kybra_unwrap_rust_python_result(
            self.try_into_value(vm),
            vm,
        ))
    }
}
impl CdkActTryFromVmValue<Vec<u8>, &rustpython::vm::VirtualMachine>
    for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<u8>, CdkActTryFromVmValueError> {
        Ok(_kybra_unwrap_rust_python_result(
            self.try_into_value(vm),
            vm,
        ))
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
impl<T> CdkActTryFromVmValue<Vec<T>, &rustpython::vm::VirtualMachine>
    for rustpython::vm::PyObjectRef
where
    T: KybraTryFromVec,
    rustpython::vm::PyObjectRef:
        for<'a> CdkActTryFromVmValue<T, &'a rustpython::vm::VirtualMachine>,
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<T>, CdkActTryFromVmValueError> {
        try_from_vm_value_generic_array(self, vm)
    }
}
fn try_from_vm_value_generic_array<T>(
    py_object_ref: rustpython::vm::PyObjectRef,
    vm: &rustpython::vm::VirtualMachine,
) -> Result<Vec<T>, CdkActTryFromVmValueError>
where
    rustpython::vm::PyObjectRef:
        for<'a> CdkActTryFromVmValue<T, &'a rustpython::vm::VirtualMachine>,
{
    let py_list: PyListRef = _kybra_unwrap_rust_python_result(py_object_ref.try_into_value(vm), vm);
    let vec = py_list.borrow_vec();
    Ok(vec
        .iter()
        .map(|item| item.clone().try_from_vm_value(vm).unwrap())
        .collect())
}
#[ic_cdk_macros::init]
#[candid::candid_method(init)]
fn _kybra_init() {
    unsafe {
        let _kybra_interpreter = rustpython_vm::Interpreter::with_init(Default::default(), |vm| {
            vm.add_native_modules(rustpython_stdlib::get_module_inits());
            vm.add_frozen(rustpython_vm::py_freeze!(dir = "python_source"));
        });
        let _kybra_scope = _kybra_interpreter.enter(|vm| vm.new_scope_with_builtins());
        _kybra_interpreter.enter(|vm| {
            Ic::make_class(&vm.ctx);
            _kybra_unwrap_rust_python_result(
                vm.builtins.set_attr("_kybra_ic", vm.new_pyobj(Ic {}), vm),
                vm,
            );
            let result = vm.run_code_string(
                _kybra_scope.clone(),
                &format!("from {} import *", "grantInitialBalance"),
                "".to_owned(),
            );
            if let Err(err) = result {
                let err_string: String = err.to_pyobject(vm).repr(vm).unwrap().to_string();
                panic!("{}", err_string);
            }
        });
        _KYBRA_INTERPRETER_OPTION = Some(_kybra_interpreter);
        _KYBRA_SCOPE_OPTION = Some(_kybra_scope);
        #[pyclass(module = false, name = "ic")]
        #[derive(Debug, PyPayload)]
        struct Ic {}
        #[pyclass]
        impl Ic {
            #[pymethod]
            fn _kybra_accept_message(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::call::accept_message()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_arg_data_raw(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::call::arg_data_raw()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_arg_data_raw_size(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::call::arg_data_raw_size()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_caller(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::caller().try_into_vm_value(vm).unwrap()
            }
            #[pymethod]
            fn _kybra_candid_decode(
                &self,
                candid_encoded_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let candid_encoded: Vec<u8> =
                    candid_encoded_py_object_ref.try_from_vm_value(vm).unwrap();
                let candid_args: candid::IDLArgs =
                    candid::IDLArgs::from_bytes(&candid_encoded).unwrap();
                let candid_string = candid_args.to_string();
                candid_string.try_into_vm_value(vm).unwrap()
            }
            #[pymethod]
            fn _kybra_candid_encode(
                &self,
                candid_string_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let candid_string: String =
                    candid_string_py_object_ref.try_from_vm_value(vm).unwrap();
                let candid_args: candid::IDLArgs = candid_string.parse().unwrap();
                let candid_encoded: Vec<u8> = candid_args.to_bytes().unwrap();
                candid_encoded.try_into_vm_value(vm).unwrap()
            }
            #[pymethod]
            fn _kybra_canister_balance(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::canister_balance()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_canister_balance128(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::canister_balance128()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_clear_timer(
                &self,
                timer_id_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let timer_id: ic_cdk::timer::TimerId =
                    timer_id_py_object_ref.try_from_vm_value(vm).unwrap();
                ic_cdk::timer::clear_timer(timer_id)
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_data_certificate(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::data_certificate()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_id(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::id().try_into_vm_value(vm).unwrap()
            }
            #[pymethod]
            fn _kybra_method_name(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::call::method_name()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_msg_cycles_accept(
                &self,
                max_amount_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let max_amount: u64 = max_amount_py_object_ref.try_from_vm_value(vm).unwrap();
                ic_cdk::api::call::msg_cycles_accept(max_amount)
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_msg_cycles_accept128(
                &self,
                max_amount_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let max_amount: u128 = max_amount_py_object_ref.try_from_vm_value(vm).unwrap();
                ic_cdk::api::call::msg_cycles_accept128(max_amount)
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_msg_cycles_available(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::call::msg_cycles_available()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_msg_cycles_available128(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::call::msg_cycles_available128()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_msg_cycles_refunded(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::call::msg_cycles_refunded()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_msg_cycles_refunded128(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::call::msg_cycles_refunded128()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_notify_raw(
                &self,
                canister_id_py_object_ref: PyObjectRef,
                method_py_object_ref: PyObjectRef,
                args_raw_py_object_ref: PyObjectRef,
                payment_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let canister_id_principal: ic_cdk::export::Principal =
                    canister_id_py_object_ref.try_from_vm_value(vm).unwrap();
                let method_string: String = method_py_object_ref.try_from_vm_value(vm).unwrap();
                let args_raw_vec: Vec<u8> = args_raw_py_object_ref.try_from_vm_value(vm).unwrap();
                let payment: u128 = payment_py_object_ref.try_from_vm_value(vm).unwrap();
                let notify_result = ic_cdk::api::call::notify_raw(
                    canister_id_principal,
                    &method_string,
                    &args_raw_vec,
                    payment,
                );
                notify_result.try_into_vm_value(vm).unwrap()
            }
            #[pymethod]
            fn _kybra_performance_counter(
                &self,
                counter_type_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let counter_type: u32 = counter_type_py_object_ref.try_from_vm_value(vm).unwrap();
                ic_cdk::api::call::performance_counter(counter_type)
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_print(&self, param_py_object_ref: PyObjectRef, vm: &VirtualMachine) {
                let param_string: String = param_py_object_ref.try_into_value(vm).unwrap();
                ic_cdk::println!("{:#?}", param_string);
            }
            #[pymethod]
            fn _kybra_reject(
                &self,
                reject_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let reject_message: String = reject_py_object_ref.try_from_vm_value(vm).unwrap();
                ic_cdk::api::call::reject(reject_message.as_str())
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_reject_code(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::call::reject_code()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_reject_message(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::call::reject_message()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_reply(
                &self,
                first_called_function_name_py_object_ref: PyObjectRef,
                reply_value_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let first_called_function_name: String = first_called_function_name_py_object_ref
                    .try_from_vm_value(vm)
                    .unwrap();
                match &first_called_function_name[..] {
                    _ => panic!("This cannot happen"),
                }
            }
            #[pymethod]
            fn _kybra_reply_raw(
                &self,
                buf_vector_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let buf_vector: Vec<u8> = buf_vector_py_object_ref.try_from_vm_value(vm).unwrap();
                ic_cdk::api::call::reply_raw(&buf_vector)
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_set_certified_data(
                &self,
                data_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) {
                let data: Vec<u8> = data_py_object_ref.try_from_vm_value(vm).unwrap();
                ic_cdk::api::set_certified_data(&data)
                    .try_into_vm_value(vm)
                    .unwrap();
            }
            #[pymethod]
            fn _kybra_set_timer(
                &self,
                delay_py_object_ref: PyObjectRef,
                func_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let delay_as_u64: u64 = delay_py_object_ref.try_from_vm_value(vm).unwrap();
                let delay = core::time::Duration::new(delay_as_u64, 0);
                let closure = move || unsafe {
                    let _kybra_interpreter = _KYBRA_INTERPRETER_OPTION.as_mut().unwrap();
                    let _kybra_scope = _KYBRA_SCOPE_OPTION.as_mut().unwrap();
                    let vm = &_kybra_interpreter.vm;
                    _kybra_unwrap_rust_python_result(vm.invoke(&func_py_object_ref, ()), vm);
                };
                ic_cdk::timer::set_timer(delay, closure)
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_set_timer_interval(
                &self,
                interval_py_object_ref: PyObjectRef,
                func_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let interval_as_u64: u64 = interval_py_object_ref.try_from_vm_value(vm).unwrap();
                let interval = core::time::Duration::new(interval_as_u64, 0);
                let closure = move || unsafe {
                    let _kybra_interpreter = _KYBRA_INTERPRETER_OPTION.as_mut().unwrap();
                    let _kybra_scope = _KYBRA_SCOPE_OPTION.as_mut().unwrap();
                    let vm = &_kybra_interpreter.vm;
                    _kybra_unwrap_rust_python_result(vm.invoke(&func_py_object_ref, ()), vm);
                };
                ic_cdk::timer::set_timer_interval(interval, closure)
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_stable_bytes(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::stable::stable_bytes()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_stable_grow(
                &self,
                new_pages_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let new_pages: u32 = new_pages_py_object_ref.try_from_vm_value(vm).unwrap();
                ic_cdk::api::stable::stable_grow(new_pages)
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_stable_read(
                &self,
                offset_py_object_ref: PyObjectRef,
                length_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let offset: u32 = offset_py_object_ref.try_from_vm_value(vm).unwrap();
                let length: u32 = length_py_object_ref.try_from_vm_value(vm).unwrap();
                let mut buf: Vec<u8> = vec![0; length as usize];
                ic_cdk::api::stable::stable_read(offset, &mut buf);
                buf.try_into_vm_value(vm).unwrap()
            }
            #[pymethod]
            fn _kybra_stable_size(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::stable::stable_size()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_stable_write(
                &self,
                offset_py_object_ref: PyObjectRef,
                buf_vector_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) {
                let offset: u32 = offset_py_object_ref.try_from_vm_value(vm).unwrap();
                let buf_vector: Vec<u8> = buf_vector_py_object_ref.try_from_vm_value(vm).unwrap();
                let buf: &[u8] = &buf_vector[..];
                ic_cdk::api::stable::stable_write(offset, buf);
            }
            #[pymethod]
            fn _kybra_stable_b_tree_map_contains_key(
                &self,
                memory_id_py_object_ref: PyObjectRef,
                key_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm).unwrap();
                match memory_id {
                    _ => panic!(
                        "memory_id {} does not have an associated StableBTreeMap",
                        memory_id
                    ),
                }
            }
            #[pymethod]
            fn _kybra_stable_b_tree_map_get(
                &self,
                memory_id_py_object_ref: PyObjectRef,
                key_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm).unwrap();
                match memory_id {
                    _ => panic!(
                        "memory_id {} does not have an associated StableBTreeMap",
                        memory_id
                    ),
                }
            }
            #[pymethod]
            fn _kybra_stable_b_tree_map_insert(
                &self,
                memory_id_py_object_ref: PyObjectRef,
                key_py_object_ref: PyObjectRef,
                value_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm).unwrap();
                match memory_id {
                    _ => panic!(
                        "memory_id {} does not have an associated StableBTreeMap",
                        memory_id
                    ),
                }
            }
            #[pymethod]
            fn _kybra_stable_b_tree_map_is_empty(
                &self,
                memory_id_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm).unwrap();
                match memory_id {
                    _ => panic!(
                        "memory_id {} does not have an associated StableBTreeMap",
                        memory_id
                    ),
                }
            }
            #[pymethod]
            fn _kybra_stable_b_tree_map_items(
                &self,
                memory_id_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> Vec<PyObjectRef> {
                let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm).unwrap();
                match memory_id {
                    _ => panic!(
                        "memory_id {} does not have an associated StableBTreeMap",
                        memory_id
                    ),
                }
            }
            #[pymethod]
            fn _kybra_stable_b_tree_map_keys(
                &self,
                memory_id_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> Vec<PyObjectRef> {
                let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm).unwrap();
                match memory_id {
                    _ => panic!(
                        "memory_id {} does not have an associated StableBTreeMap",
                        memory_id
                    ),
                }
            }
            #[pymethod]
            fn _kybra_stable_b_tree_map_len(
                &self,
                memory_id_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm).unwrap();
                match memory_id {
                    _ => panic!(
                        "memory_id {} does not have an associated StableBTreeMap",
                        memory_id
                    ),
                }
            }
            #[pymethod]
            fn _kybra_stable_b_tree_map_remove(
                &self,
                memory_id_py_object_ref: PyObjectRef,
                key_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm).unwrap();
                match memory_id {
                    _ => panic!(
                        "memory_id {} does not have an associated StableBTreeMap",
                        memory_id
                    ),
                }
            }
            #[pymethod]
            fn _kybra_stable_b_tree_map_values(
                &self,
                memory_id_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> Vec<PyObjectRef> {
                let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm).unwrap();
                match memory_id {
                    _ => panic!(
                        "memory_id {} does not have an associated StableBTreeMap",
                        memory_id
                    ),
                }
            }
            #[pymethod]
            fn _kybra_stable64_grow(
                &self,
                new_pages_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let new_pages: u64 = new_pages_py_object_ref.try_from_vm_value(vm).unwrap();
                ic_cdk::api::stable::stable64_grow(new_pages)
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_stable64_read(
                &self,
                offset_py_object_ref: PyObjectRef,
                length_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let offset: u64 = offset_py_object_ref.try_from_vm_value(vm).unwrap();
                let length: u64 = length_py_object_ref.try_from_vm_value(vm).unwrap();
                let mut buf: Vec<u8> = vec![0; length as usize];
                ic_cdk::api::stable::stable64_read(offset, &mut buf);
                buf.try_into_vm_value(vm).unwrap()
            }
            #[pymethod]
            fn _kybra_stable64_size(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::stable::stable64_size()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_stable64_write(
                &self,
                offset_py_object_ref: PyObjectRef,
                buf_vector_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) {
                let offset: u64 = offset_py_object_ref.try_from_vm_value(vm).unwrap();
                let buf_vector: Vec<u8> = buf_vector_py_object_ref.try_from_vm_value(vm).unwrap();
                let buf: &[u8] = &buf_vector[..];
                ic_cdk::api::stable::stable64_write(offset, buf);
            }
            #[pymethod]
            fn _kybra_time(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::time().try_into_vm_value(vm).unwrap()
            }
            #[pymethod]
            fn _kybra_trap(&self, message_py_object_ref: PyObjectRef, vm: &VirtualMachine) {
                let message: String = message_py_object_ref.try_from_vm_value(vm).unwrap();
                ic_cdk::api::trap(&message);
            }
        }
    }
}
#[ic_cdk_macros::post_upgrade]
fn _kybra_post_upgrade() {
    unsafe {
        let _kybra_interpreter = rustpython_vm::Interpreter::with_init(Default::default(), |vm| {
            vm.add_native_modules(rustpython_stdlib::get_module_inits());
            vm.add_frozen(rustpython_vm::py_freeze!(dir = "python_source"));
        });
        let _kybra_scope = _kybra_interpreter.enter(|vm| vm.new_scope_with_builtins());
        _kybra_interpreter.enter(|vm| {
            Ic::make_class(&vm.ctx);
            _kybra_unwrap_rust_python_result(
                vm.builtins.set_attr("_kybra_ic", vm.new_pyobj(Ic {}), vm),
                vm,
            );
            let result = vm.run_code_string(
                _kybra_scope.clone(),
                &format!("from {} import *", "grantInitialBalance"),
                "".to_owned(),
            );
            if let Err(err) = result {
                let err_string: String = err.to_pyobject(vm).repr(vm).unwrap().to_string();
                panic!("{}", err_string);
            }
        });
        _KYBRA_INTERPRETER_OPTION = Some(_kybra_interpreter);
        _KYBRA_SCOPE_OPTION = Some(_kybra_scope);
        #[pyclass(module = false, name = "ic")]
        #[derive(Debug, PyPayload)]
        struct Ic {}
        #[pyclass]
        impl Ic {
            #[pymethod]
            fn _kybra_accept_message(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::call::accept_message()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_arg_data_raw(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::call::arg_data_raw()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_arg_data_raw_size(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::call::arg_data_raw_size()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_caller(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::caller().try_into_vm_value(vm).unwrap()
            }
            #[pymethod]
            fn _kybra_candid_decode(
                &self,
                candid_encoded_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let candid_encoded: Vec<u8> =
                    candid_encoded_py_object_ref.try_from_vm_value(vm).unwrap();
                let candid_args: candid::IDLArgs =
                    candid::IDLArgs::from_bytes(&candid_encoded).unwrap();
                let candid_string = candid_args.to_string();
                candid_string.try_into_vm_value(vm).unwrap()
            }
            #[pymethod]
            fn _kybra_candid_encode(
                &self,
                candid_string_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let candid_string: String =
                    candid_string_py_object_ref.try_from_vm_value(vm).unwrap();
                let candid_args: candid::IDLArgs = candid_string.parse().unwrap();
                let candid_encoded: Vec<u8> = candid_args.to_bytes().unwrap();
                candid_encoded.try_into_vm_value(vm).unwrap()
            }
            #[pymethod]
            fn _kybra_canister_balance(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::canister_balance()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_canister_balance128(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::canister_balance128()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_clear_timer(
                &self,
                timer_id_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let timer_id: ic_cdk::timer::TimerId =
                    timer_id_py_object_ref.try_from_vm_value(vm).unwrap();
                ic_cdk::timer::clear_timer(timer_id)
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_data_certificate(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::data_certificate()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_id(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::id().try_into_vm_value(vm).unwrap()
            }
            #[pymethod]
            fn _kybra_method_name(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::call::method_name()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_msg_cycles_accept(
                &self,
                max_amount_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let max_amount: u64 = max_amount_py_object_ref.try_from_vm_value(vm).unwrap();
                ic_cdk::api::call::msg_cycles_accept(max_amount)
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_msg_cycles_accept128(
                &self,
                max_amount_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let max_amount: u128 = max_amount_py_object_ref.try_from_vm_value(vm).unwrap();
                ic_cdk::api::call::msg_cycles_accept128(max_amount)
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_msg_cycles_available(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::call::msg_cycles_available()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_msg_cycles_available128(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::call::msg_cycles_available128()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_msg_cycles_refunded(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::call::msg_cycles_refunded()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_msg_cycles_refunded128(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::call::msg_cycles_refunded128()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_notify_raw(
                &self,
                canister_id_py_object_ref: PyObjectRef,
                method_py_object_ref: PyObjectRef,
                args_raw_py_object_ref: PyObjectRef,
                payment_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let canister_id_principal: ic_cdk::export::Principal =
                    canister_id_py_object_ref.try_from_vm_value(vm).unwrap();
                let method_string: String = method_py_object_ref.try_from_vm_value(vm).unwrap();
                let args_raw_vec: Vec<u8> = args_raw_py_object_ref.try_from_vm_value(vm).unwrap();
                let payment: u128 = payment_py_object_ref.try_from_vm_value(vm).unwrap();
                let notify_result = ic_cdk::api::call::notify_raw(
                    canister_id_principal,
                    &method_string,
                    &args_raw_vec,
                    payment,
                );
                notify_result.try_into_vm_value(vm).unwrap()
            }
            #[pymethod]
            fn _kybra_performance_counter(
                &self,
                counter_type_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let counter_type: u32 = counter_type_py_object_ref.try_from_vm_value(vm).unwrap();
                ic_cdk::api::call::performance_counter(counter_type)
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_print(&self, param_py_object_ref: PyObjectRef, vm: &VirtualMachine) {
                let param_string: String = param_py_object_ref.try_into_value(vm).unwrap();
                ic_cdk::println!("{:#?}", param_string);
            }
            #[pymethod]
            fn _kybra_reject(
                &self,
                reject_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let reject_message: String = reject_py_object_ref.try_from_vm_value(vm).unwrap();
                ic_cdk::api::call::reject(reject_message.as_str())
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_reject_code(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::call::reject_code()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_reject_message(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::call::reject_message()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_reply(
                &self,
                first_called_function_name_py_object_ref: PyObjectRef,
                reply_value_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let first_called_function_name: String = first_called_function_name_py_object_ref
                    .try_from_vm_value(vm)
                    .unwrap();
                match &first_called_function_name[..] {
                    _ => panic!("This cannot happen"),
                }
            }
            #[pymethod]
            fn _kybra_reply_raw(
                &self,
                buf_vector_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let buf_vector: Vec<u8> = buf_vector_py_object_ref.try_from_vm_value(vm).unwrap();
                ic_cdk::api::call::reply_raw(&buf_vector)
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_set_certified_data(
                &self,
                data_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) {
                let data: Vec<u8> = data_py_object_ref.try_from_vm_value(vm).unwrap();
                ic_cdk::api::set_certified_data(&data)
                    .try_into_vm_value(vm)
                    .unwrap();
            }
            #[pymethod]
            fn _kybra_set_timer(
                &self,
                delay_py_object_ref: PyObjectRef,
                func_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let delay_as_u64: u64 = delay_py_object_ref.try_from_vm_value(vm).unwrap();
                let delay = core::time::Duration::new(delay_as_u64, 0);
                let closure = move || unsafe {
                    let _kybra_interpreter = _KYBRA_INTERPRETER_OPTION.as_mut().unwrap();
                    let _kybra_scope = _KYBRA_SCOPE_OPTION.as_mut().unwrap();
                    let vm = &_kybra_interpreter.vm;
                    _kybra_unwrap_rust_python_result(vm.invoke(&func_py_object_ref, ()), vm);
                };
                ic_cdk::timer::set_timer(delay, closure)
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_set_timer_interval(
                &self,
                interval_py_object_ref: PyObjectRef,
                func_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let interval_as_u64: u64 = interval_py_object_ref.try_from_vm_value(vm).unwrap();
                let interval = core::time::Duration::new(interval_as_u64, 0);
                let closure = move || unsafe {
                    let _kybra_interpreter = _KYBRA_INTERPRETER_OPTION.as_mut().unwrap();
                    let _kybra_scope = _KYBRA_SCOPE_OPTION.as_mut().unwrap();
                    let vm = &_kybra_interpreter.vm;
                    _kybra_unwrap_rust_python_result(vm.invoke(&func_py_object_ref, ()), vm);
                };
                ic_cdk::timer::set_timer_interval(interval, closure)
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_stable_bytes(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::stable::stable_bytes()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_stable_grow(
                &self,
                new_pages_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let new_pages: u32 = new_pages_py_object_ref.try_from_vm_value(vm).unwrap();
                ic_cdk::api::stable::stable_grow(new_pages)
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_stable_read(
                &self,
                offset_py_object_ref: PyObjectRef,
                length_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let offset: u32 = offset_py_object_ref.try_from_vm_value(vm).unwrap();
                let length: u32 = length_py_object_ref.try_from_vm_value(vm).unwrap();
                let mut buf: Vec<u8> = vec![0; length as usize];
                ic_cdk::api::stable::stable_read(offset, &mut buf);
                buf.try_into_vm_value(vm).unwrap()
            }
            #[pymethod]
            fn _kybra_stable_size(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::stable::stable_size()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_stable_write(
                &self,
                offset_py_object_ref: PyObjectRef,
                buf_vector_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) {
                let offset: u32 = offset_py_object_ref.try_from_vm_value(vm).unwrap();
                let buf_vector: Vec<u8> = buf_vector_py_object_ref.try_from_vm_value(vm).unwrap();
                let buf: &[u8] = &buf_vector[..];
                ic_cdk::api::stable::stable_write(offset, buf);
            }
            #[pymethod]
            fn _kybra_stable_b_tree_map_contains_key(
                &self,
                memory_id_py_object_ref: PyObjectRef,
                key_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm).unwrap();
                match memory_id {
                    _ => panic!(
                        "memory_id {} does not have an associated StableBTreeMap",
                        memory_id
                    ),
                }
            }
            #[pymethod]
            fn _kybra_stable_b_tree_map_get(
                &self,
                memory_id_py_object_ref: PyObjectRef,
                key_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm).unwrap();
                match memory_id {
                    _ => panic!(
                        "memory_id {} does not have an associated StableBTreeMap",
                        memory_id
                    ),
                }
            }
            #[pymethod]
            fn _kybra_stable_b_tree_map_insert(
                &self,
                memory_id_py_object_ref: PyObjectRef,
                key_py_object_ref: PyObjectRef,
                value_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm).unwrap();
                match memory_id {
                    _ => panic!(
                        "memory_id {} does not have an associated StableBTreeMap",
                        memory_id
                    ),
                }
            }
            #[pymethod]
            fn _kybra_stable_b_tree_map_is_empty(
                &self,
                memory_id_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm).unwrap();
                match memory_id {
                    _ => panic!(
                        "memory_id {} does not have an associated StableBTreeMap",
                        memory_id
                    ),
                }
            }
            #[pymethod]
            fn _kybra_stable_b_tree_map_items(
                &self,
                memory_id_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> Vec<PyObjectRef> {
                let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm).unwrap();
                match memory_id {
                    _ => panic!(
                        "memory_id {} does not have an associated StableBTreeMap",
                        memory_id
                    ),
                }
            }
            #[pymethod]
            fn _kybra_stable_b_tree_map_keys(
                &self,
                memory_id_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> Vec<PyObjectRef> {
                let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm).unwrap();
                match memory_id {
                    _ => panic!(
                        "memory_id {} does not have an associated StableBTreeMap",
                        memory_id
                    ),
                }
            }
            #[pymethod]
            fn _kybra_stable_b_tree_map_len(
                &self,
                memory_id_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm).unwrap();
                match memory_id {
                    _ => panic!(
                        "memory_id {} does not have an associated StableBTreeMap",
                        memory_id
                    ),
                }
            }
            #[pymethod]
            fn _kybra_stable_b_tree_map_remove(
                &self,
                memory_id_py_object_ref: PyObjectRef,
                key_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm).unwrap();
                match memory_id {
                    _ => panic!(
                        "memory_id {} does not have an associated StableBTreeMap",
                        memory_id
                    ),
                }
            }
            #[pymethod]
            fn _kybra_stable_b_tree_map_values(
                &self,
                memory_id_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> Vec<PyObjectRef> {
                let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm).unwrap();
                match memory_id {
                    _ => panic!(
                        "memory_id {} does not have an associated StableBTreeMap",
                        memory_id
                    ),
                }
            }
            #[pymethod]
            fn _kybra_stable64_grow(
                &self,
                new_pages_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let new_pages: u64 = new_pages_py_object_ref.try_from_vm_value(vm).unwrap();
                ic_cdk::api::stable::stable64_grow(new_pages)
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_stable64_read(
                &self,
                offset_py_object_ref: PyObjectRef,
                length_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyObjectRef {
                let offset: u64 = offset_py_object_ref.try_from_vm_value(vm).unwrap();
                let length: u64 = length_py_object_ref.try_from_vm_value(vm).unwrap();
                let mut buf: Vec<u8> = vec![0; length as usize];
                ic_cdk::api::stable::stable64_read(offset, &mut buf);
                buf.try_into_vm_value(vm).unwrap()
            }
            #[pymethod]
            fn _kybra_stable64_size(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::stable::stable64_size()
                    .try_into_vm_value(vm)
                    .unwrap()
            }
            #[pymethod]
            fn _kybra_stable64_write(
                &self,
                offset_py_object_ref: PyObjectRef,
                buf_vector_py_object_ref: PyObjectRef,
                vm: &VirtualMachine,
            ) {
                let offset: u64 = offset_py_object_ref.try_from_vm_value(vm).unwrap();
                let buf_vector: Vec<u8> = buf_vector_py_object_ref.try_from_vm_value(vm).unwrap();
                let buf: &[u8] = &buf_vector[..];
                ic_cdk::api::stable::stable64_write(offset, buf);
            }
            #[pymethod]
            fn _kybra_time(&self, vm: &VirtualMachine) -> PyObjectRef {
                ic_cdk::api::time().try_into_vm_value(vm).unwrap()
            }
            #[pymethod]
            fn _kybra_trap(&self, message_py_object_ref: PyObjectRef, vm: &VirtualMachine) {
                let message: String = message_py_object_ref.try_from_vm_value(vm).unwrap();
                ic_cdk::api::trap(&message);
            }
        }
    }
}
#[ic_cdk_macros::pre_upgrade]
fn _kybra_pre_upgrade() {
    unsafe {
        let _kybra_interpreter = _KYBRA_INTERPRETER_OPTION.as_mut().unwrap();
        let _kybra_scope = _KYBRA_SCOPE_OPTION.as_mut().unwrap();
        _kybra_interpreter.enter(|vm| {});
    }
}
#[ic_cdk_macros::query()]
#[candid::candid_method(query)]
async fn balanceOf(_cdk_user_defined_who: candid::Principal) -> candid::Nat {
    unsafe {
        let _kybra_interpreter = _KYBRA_INTERPRETER_OPTION.as_mut().unwrap();
        let _kybra_scope = _KYBRA_SCOPE_OPTION.as_mut().unwrap();
        let vm = &_kybra_interpreter.vm;
        let method_py_object_ref =
            _kybra_unwrap_rust_python_result(_kybra_scope.globals.get_item("balanceOf", vm), vm);
        let invoke_result = vm.invoke(
            &method_py_object_ref,
            (_cdk_user_defined_who.try_into_vm_value(vm).unwrap(),),
        );
        match invoke_result {
            Ok(py_object_ref) => {
                let _kybra_final_return_value =
                    _kybra_async_result_handler(vm, &py_object_ref, vm.ctx.none()).await;
                _kybra_final_return_value.try_from_vm_value(vm).unwrap()
            }
            Err(err) => {
                let err_string: String = err.to_pyobject(vm).repr(vm).unwrap().to_string();
                panic!("{}", err_string);
            }
        }
    }
}
#[doc = r" A marker type to match unconstrained callback arguments"]
#[derive(Debug, Clone, Copy, PartialEq, candid :: Deserialize)]
pub struct ArgToken;
impl candid::CandidType for ArgToken {
    fn _ty() -> candid::types::Type {
        candid::types::Type::Empty
    }
    fn idl_serialize<S: candid::types::Serializer>(&self, _serializer: S) -> Result<(), S::Error> {
        unimplemented!("Token is not serializable")
    }
}
pub fn _kybra_unwrap_rust_python_result<T>(
    rust_python_result: Result<T, PyRef<PyBaseException>>,
    vm: &rustpython::vm::VirtualMachine,
) -> T {
    match rust_python_result {
        Ok(ok) => ok,
        Err(err) => {
            let err_string: String = err.to_pyobject(vm).repr(vm).unwrap().to_string();
            panic!("{}", err_string);
        }
    }
}
# [async_recursion :: async_recursion (? Send)]
async fn _kybra_async_result_handler(
    vm: &rustpython::vm::VirtualMachine,
    py_object_ref: &rustpython::vm::PyObjectRef,
    arg: PyObjectRef,
) -> rustpython::vm::PyObjectRef {
    if _kybra_is_generator(vm, &py_object_ref) == false {
        return py_object_ref.clone();
    }
    let send_result = vm.call_method(&py_object_ref, "send", (arg.clone(),));
    let py_iter_return =
        _kybra_unwrap_rust_python_result(PyIterReturn::from_pyresult(send_result, vm), vm);
    match py_iter_return {
        PyIterReturn::Return(returned_py_object_ref) => {
            if _kybra_is_generator(vm, &returned_py_object_ref) == true {
                let recursed_py_object_ref =
                    _kybra_async_result_handler(vm, &returned_py_object_ref, vm.ctx.none()).await;
                return _kybra_async_result_handler(vm, py_object_ref, recursed_py_object_ref)
                    .await;
            }
            let name: String =
                _kybra_unwrap_rust_python_result(returned_py_object_ref.get_attr("name", vm), vm)
                    .try_from_vm_value(vm)
                    .unwrap();
            let args: Vec<PyObjectRef> = _kybra_unwrap_rust_python_result(
                _kybra_unwrap_rust_python_result(returned_py_object_ref.get_attr("args", vm), vm)
                    .try_into_value(vm),
                vm,
            );
            match &name[..] {
                "call" => _kybra_async_result_handler_call(vm, py_object_ref, &args).await,
                "call_with_payment" => {
                    _kybra_async_result_handler_call_with_payment(vm, py_object_ref, &args).await
                }
                "call_with_payment128" => {
                    _kybra_async_result_handler_call_with_payment128(vm, py_object_ref, &args).await
                }
                "call_raw" => _kybra_async_result_handler_call_raw(vm, py_object_ref, &args).await,
                "call_raw128" => {
                    _kybra_async_result_handler_call_raw128(vm, py_object_ref, &args).await
                }
                _ => panic!("async operation not supported"),
            }
        }
        PyIterReturn::StopIteration(returned_py_object_ref_option) => {
            match returned_py_object_ref_option {
                Some(returned_py_object_ref) => returned_py_object_ref,
                None => vm.ctx.none(),
            }
        }
    }
}
fn _kybra_is_generator(vm: &rustpython::vm::VirtualMachine, py_object_ref: &PyObjectRef) -> bool {
    if let Ok(_) = py_object_ref.get_attr("send", vm) {
        true
    } else {
        false
    }
}
async fn _kybra_async_result_handler_call(
    vm: &rustpython::vm::VirtualMachine,
    py_object_ref: &PyObjectRef,
    args: &Vec<PyObjectRef>,
) -> PyObjectRef {
    let canister_id_principal: ic_cdk::export::Principal =
        args[0].clone().try_from_vm_value(vm).unwrap();
    let qualname: String = args[1].clone().try_from_vm_value(vm).unwrap();
    let cross_canister_call_function_name = format!("_kybra_call_{}", qualname.replace(".", "_"));
    let call_result_instance = match &cross_canister_call_function_name[..] {
        _ => panic!("cross canister function does not exist"),
    };
    _kybra_async_result_handler(vm, py_object_ref, call_result_instance).await
}
async fn _kybra_async_result_handler_call_with_payment(
    vm: &rustpython::vm::VirtualMachine,
    py_object_ref: &PyObjectRef,
    args: &Vec<PyObjectRef>,
) -> PyObjectRef {
    let canister_id_principal: ic_cdk::export::Principal =
        args[0].clone().try_from_vm_value(vm).unwrap();
    let qualname: String = args[1].clone().try_from_vm_value(vm).unwrap();
    let cross_canister_call_with_payment_function_name =
        format!("_kybra_call_with_payment_{}", qualname.replace(".", "_"));
    let call_result_instance = match &cross_canister_call_with_payment_function_name[..] {
        _ => panic!("cross canister function does not exist"),
    };
    _kybra_async_result_handler(vm, py_object_ref, call_result_instance).await
}
async fn _kybra_async_result_handler_call_with_payment128(
    vm: &rustpython::vm::VirtualMachine,
    py_object_ref: &PyObjectRef,
    args: &Vec<PyObjectRef>,
) -> PyObjectRef {
    let canister_id_principal: ic_cdk::export::Principal =
        args[0].clone().try_from_vm_value(vm).unwrap();
    let qualname: String = args[1].clone().try_from_vm_value(vm).unwrap();
    let cross_canister_call_with_payment128_function_name =
        format!("_kybra_call_with_payment128_{}", qualname.replace(".", "_"));
    let call_result_instance = match &cross_canister_call_with_payment128_function_name[..] {
        _ => panic!("cross canister function does not exist"),
    };
    _kybra_async_result_handler(vm, py_object_ref, call_result_instance).await
}
async fn _kybra_async_result_handler_call_raw(
    vm: &rustpython::vm::VirtualMachine,
    py_object_ref: &PyObjectRef,
    args: &Vec<PyObjectRef>,
) -> PyObjectRef {
    let canister_id_principal: ic_cdk::export::Principal =
        args[0].clone().try_from_vm_value(vm).unwrap();
    let method_string: String = args[1].clone().try_from_vm_value(vm).unwrap();
    let args_raw_vec: Vec<u8> = args[2].clone().try_from_vm_value(vm).unwrap();
    let payment: u64 = args[3].clone().try_from_vm_value(vm).unwrap();
    let call_raw_result = ic_cdk::api::call::call_raw(
        canister_id_principal,
        &method_string,
        &args_raw_vec,
        payment,
    )
    .await;
    _kybra_async_result_handler(
        vm,
        py_object_ref,
        _kybra_create_call_result_instance(vm, call_raw_result),
    )
    .await
}
async fn _kybra_async_result_handler_call_raw128(
    vm: &rustpython::vm::VirtualMachine,
    py_object_ref: &PyObjectRef,
    args: &Vec<PyObjectRef>,
) -> PyObjectRef {
    let canister_id_principal: ic_cdk::export::Principal =
        args[0].clone().try_from_vm_value(vm).unwrap();
    let method_string: String = args[1].clone().try_from_vm_value(vm).unwrap();
    let args_raw_vec: Vec<u8> = args[2].clone().try_from_vm_value(vm).unwrap();
    let payment: u128 = args[3].clone().try_from_vm_value(vm).unwrap();
    let call_raw_result = ic_cdk::api::call::call_raw128(
        canister_id_principal,
        &method_string,
        &args_raw_vec,
        payment,
    )
    .await;
    _kybra_async_result_handler(
        vm,
        py_object_ref,
        _kybra_create_call_result_instance(vm, call_raw_result),
    )
    .await
}
fn _kybra_create_call_result_instance<T>(
    vm: &rustpython::vm::VirtualMachine,
    call_result: CallResult<T>,
) -> PyObjectRef
where
    T: for<'a> CdkActTryIntoVmValue<
        &'a rustpython::vm::VirtualMachine,
        rustpython::vm::PyObjectRef,
    >,
{
    let canister_result_class = _kybra_unwrap_rust_python_result(
        vm.run_block_expr(
            vm.new_scope_with_builtins(),
            r#"
from kybra import CanisterResult

CanisterResult
                "#,
        ),
        vm,
    );
    match call_result {
        Ok(ok) => {
            let method_result = vm.invoke(
                &canister_result_class,
                (ok.try_into_vm_value(vm).unwrap(), vm.ctx.none()),
            );
            _kybra_unwrap_rust_python_result(method_result, vm)
        }
        Err(err) => {
            let err_string = format!(
                "Rejection code {rejection_code}, {error_message}",
                rejection_code = (err.0 as i32).to_string(),
                error_message = err.1
            );
            let method_result = vm.invoke(
                &canister_result_class,
                (vm.ctx.none(), err_string.try_into_vm_value(vm).unwrap()),
            );
            _kybra_unwrap_rust_python_result(method_result, vm)
        }
    }
}
#[inline]
pub fn kybra_serialize<S>(
    vm: &VirtualMachine,
    pyobject: &PyObject,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    KybraPyObjectSerializer { pyobject, vm }.serialize(serializer)
}
#[inline]
pub fn kybra_deserialize<'de, D>(
    vm: &'de VirtualMachine,
    deserializer: D,
) -> Result<<KybraPyObjectDeserializer as DeserializeSeed>::Value, D::Error>
where
    D: serde::Deserializer<'de>,
{
    KybraPyObjectDeserializer { vm }.deserialize(deserializer)
}
pub struct KybraPyObjectSerializer<'s> {
    pyobject: &'s PyObject,
    vm: &'s VirtualMachine,
}
impl<'s> KybraPyObjectSerializer<'s> {
    pub fn new(vm: &'s VirtualMachine, pyobject: &'s PyObjectRef) -> Self {
        KybraPyObjectSerializer { pyobject, vm }
    }
    fn clone_with_object(&self, pyobject: &'s PyObjectRef) -> KybraPyObjectSerializer {
        KybraPyObjectSerializer {
            pyobject,
            vm: self.vm,
        }
    }
}
impl<'s> serde::Serialize for KybraPyObjectSerializer<'s> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let serialize_seq_elements =
            |serializer: S, elements: &[PyObjectRef]| -> Result<S::Ok, S::Error> {
                let mut seq = serializer.serialize_seq(Some(elements.len()))?;
                seq.serialize_element(&"LIST")?;
                for e in elements {
                    seq.serialize_element(&self.clone_with_object(e))?;
                }
                seq.end()
            };
        let serialize_tuple_elements =
            |serializer: S, elements: &[PyObjectRef]| -> Result<S::Ok, S::Error> {
                let mut tup = serializer.serialize_tuple(elements.len())?;
                tup.serialize_element(&"TUPLE")?;
                for e in elements {
                    tup.serialize_element(&self.clone_with_object(e))?;
                }
                tup.end()
            };
        let serialize_bytes_elements =
            |serializer: S, elements: &[u8]| -> Result<S::Ok, S::Error> {
                let mut seq = serializer.serialize_seq(Some(elements.len()))?;
                seq.serialize_element(&"BYTES")?;
                for e in elements {
                    seq.serialize_element(e.into())?;
                }
                seq.end()
            };
        if let Some(s) = self.pyobject.payload::<PyStr>() {
            serialize(self.vm, self.pyobject, serializer)
        } else if self.pyobject.fast_isinstance(self.vm.ctx.types.float_type) {
            serialize(self.vm, self.pyobject, serializer)
        } else if self.pyobject.fast_isinstance(self.vm.ctx.types.bool_type) {
            serialize(self.vm, self.pyobject, serializer)
        } else if self.pyobject.fast_isinstance(self.vm.ctx.types.int_type) {
            serialize(self.vm, self.pyobject, serializer)
        } else if let Some(list) = self.pyobject.payload_if_subclass::<PyList>(self.vm) {
            serialize_seq_elements(serializer, &list.borrow_vec())
        } else if let Some(tuple) = self.pyobject.payload_if_subclass::<PyTuple>(self.vm) {
            serialize_tuple_elements(serializer, tuple)
        } else if let Some(bytes) = self.pyobject.payload_if_subclass::<PyBytes>(self.vm) {
            serialize_bytes_elements(serializer, bytes.as_bytes())
        } else if self.pyobject.fast_isinstance(self.vm.ctx.types.dict_type) {
            let dict: PyRef<PyDict> = self.pyobject.to_owned().downcast().unwrap();
            let pairs: Vec<_> = dict.into_iter().collect();
            let mut map = serializer.serialize_map(Some(pairs.len()))?;
            for (key, e) in &pairs {
                map.serialize_entry(&self.clone_with_object(key), &self.clone_with_object(e))?;
            }
            map.end()
        } else if self.vm.is_none(self.pyobject) {
            serialize(self.vm, self.pyobject, serializer)
        } else {
            let class = self.pyobject.class();
            let class_name = class.name();
            if class_name.to_string() == "Principal" {
                let to_str = _kybra_unwrap_rust_python_result(
                    self.pyobject.get_attr("to_str", self.vm),
                    self.vm,
                );
                let to_str_invoke_result =
                    _kybra_unwrap_rust_python_result(self.vm.invoke(&to_str, ()), self.vm);
                let to_str_invoke_string: String = _kybra_unwrap_rust_python_result(
                    to_str_invoke_result.try_into_value(self.vm),
                    self.vm,
                );
                return serializer
                    .serialize_str(&format!("KYBRA::Principal::{}", to_str_invoke_string));
            }
            Err(serde::ser::Error::custom(format!(
                "Object of type '{}' is not serializable",
                self.pyobject.class()
            )))
        }
    }
}
#[derive(Clone)]
pub struct KybraPyObjectDeserializer<'c> {
    vm: &'c VirtualMachine,
}
impl<'c> KybraPyObjectDeserializer<'c> {
    pub fn new(vm: &'c VirtualMachine) -> Self {
        KybraPyObjectDeserializer { vm }
    }
}
impl<'de> DeserializeSeed<'de> for KybraPyObjectDeserializer<'de> {
    type Value = PyObjectRef;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(self.clone())
    }
}
impl<'de> Visitor<'de> for KybraPyObjectDeserializer<'de> {
    type Value = PyObjectRef;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a type that can deserialise in Python")
    }
    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(self.vm.ctx.new_bool(value).into())
    }
    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(self.vm.ctx.new_int(value).into())
    }
    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(self.vm.ctx.new_int(value).into())
    }
    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(self.vm.ctx.new_float(value).into())
    }
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if value.starts_with("KYBRA::Principal::") {
            let principal_class = _kybra_unwrap_rust_python_result(
                self.vm.run_block_expr(
                    self.vm.new_scope_with_builtins(),
                    r#"
from kybra import Principal

Principal
                        "#,
                ),
                self.vm,
            );
            let from_str = _kybra_unwrap_rust_python_result(
                principal_class.get_attr("from_str", self.vm),
                self.vm,
            );
            let principal_string = value.to_string().replace("KYBRA::Principal::", "");
            let principal_instance = _kybra_unwrap_rust_python_result(
                self.vm.invoke(&from_str, (principal_string,)),
                self.vm,
            );
            Ok(principal_instance)
        } else {
            self.visit_string(value.to_owned())
        }
    }
    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(self.vm.ctx.new_str(value).into())
    }
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(self.vm.ctx.none())
    }
    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut seq_type = "".to_string();
        if let Some(first_value) = access.next_element_seed(self.clone())? {
            let first_value_string: String =
                _kybra_unwrap_rust_python_result(first_value.try_into_value(self.vm), self.vm);
            seq_type = first_value_string;
        }
        if seq_type == "BYTES" {
            let mut seq = Vec::with_capacity(access.size_hint().unwrap_or(0));
            while let Some(value) = access.next_element_seed(self.clone())? {
                let value_u8: u8 = value.try_from_vm_value(self.vm).unwrap();
                seq.push(value_u8);
            }
            Ok(self.vm.ctx.new_bytes(seq).into())
        } else {
            let mut seq = Vec::with_capacity(access.size_hint().unwrap_or(0));
            while let Some(value) = access.next_element_seed(self.clone())? {
                seq.push(value);
            }
            if seq_type == "TUPLE" {
                Ok(self.vm.ctx.new_tuple(seq).into())
            } else {
                Ok(self.vm.ctx.new_list(seq).into())
            }
        }
    }
    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: serde::de::MapAccess<'de>,
    {
        let dict = self.vm.ctx.new_dict();
        while let Some((key_obj, value)) = access.next_entry_seed(self.clone(), self.clone())? {
            _kybra_unwrap_rust_python_result(dict.set_item(&*key_obj, value, self.vm), self.vm);
        }
        Ok(dict.into())
    }
}
use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};
type Memory = VirtualMemory<DefaultMemoryImpl>;
thread_local! { static MEMORY_MANAGER : RefCell < MemoryManager < DefaultMemoryImpl >> = RefCell :: new (MemoryManager :: init (DefaultMemoryImpl :: default ())) ; }
candid::export_service!();
#[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
fn _kybra_export_candid() -> String {
    __export_service()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn _kybra_write_candid_to_disk() {
        std::fs::write("index.did", _kybra_export_candid()).unwrap();
    }
}
