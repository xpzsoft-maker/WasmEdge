use crate::{
    types::{Mutability, ValType},
    wasmedge, Error, Value, WasmEdgeResult,
};

#[derive(Debug)]
pub struct Global {
    pub(crate) ctx: *mut wasmedge::WasmEdge_GlobalInstanceContext,
    pub(crate) registered: bool,
}
impl Global {
    pub fn create(ty: &mut GlobalType, val: Value) -> WasmEdgeResult<Self> {
        let ctx = unsafe {
            wasmedge::WasmEdge_GlobalInstanceCreate(ty.ctx, wasmedge::WasmEdge_Value::from(val))
        };
        ty.registered = true;
        match ctx.is_null() {
            true => Err(Error::OperationError(String::from(
                "fail to create Global instance",
            ))),
            false => Ok(Self {
                ctx,
                registered: false,
            }),
        }
    }

    pub fn get_type(&self) -> WasmEdgeResult<GlobalType> {
        let ty_ctx = unsafe { wasmedge::WasmEdge_GlobalInstanceGetGlobalType(self.ctx) };
        match ty_ctx.is_null() {
            true => Err(Error::OperationError(String::from(
                "fail to get type info from the Global instance",
            ))),
            false => Ok(GlobalType {
                ctx: ty_ctx as *mut _,
                registered: true,
            }),
        }
    }

    pub fn get_value(&self) -> Value {
        let val = unsafe { wasmedge::WasmEdge_GlobalInstanceGetValue(self.ctx) };
        Value::from(val)
    }

    pub fn set_value(&mut self, val: Value) {
        unsafe {
            wasmedge::WasmEdge_GlobalInstanceSetValue(self.ctx, wasmedge::WasmEdge_Value::from(val))
        }
    }
}
impl Drop for Global {
    fn drop(&mut self) {
        if !self.registered && !self.ctx.is_null() {
            unsafe { wasmedge::WasmEdge_GlobalInstanceDelete(self.ctx) };
        }
    }
}

#[derive(Debug)]
pub struct GlobalType {
    pub(crate) ctx: *mut wasmedge::WasmEdge_GlobalTypeContext,
    pub(crate) registered: bool,
}
impl GlobalType {
    /// Create a GlobalType instance
    pub fn create(val_ty: ValType, mutable: Mutability) -> WasmEdgeResult<Self> {
        let ctx = unsafe {
            wasmedge::WasmEdge_GlobalTypeCreate(
                wasmedge::WasmEdge_ValType::from(val_ty),
                wasmedge::WasmEdge_Mutability::from(mutable),
            )
        };
        match ctx.is_null() {
            true => Err(Error::OperationError(String::from(
                "fail to create GlobalType instance",
            ))),
            false => Ok(Self {
                ctx,
                registered: false,
            }),
        }
    }

    /// Get the value type from a GlobalType instance
    pub fn get_value_type(&self) -> ValType {
        let val = unsafe { wasmedge::WasmEdge_GlobalTypeGetValType(self.ctx) };
        val.into()
    }

    /// Get the mutability value from a GlobalType instance.
    pub fn get_mutability(&self) -> Mutability {
        let val = unsafe { wasmedge::WasmEdge_GlobalTypeGetMutability(self.ctx) };
        val.into()
    }
}
impl Drop for GlobalType {
    fn drop(&mut self) {
        if !self.registered && !self.ctx.is_null() {
            unsafe { wasmedge::WasmEdge_GlobalTypeDelete(self.ctx) }
        }
    }
}
