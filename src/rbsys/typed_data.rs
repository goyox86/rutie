use crate::types::{c_char, c_int, c_void, size_t, Value};

pub use rb_sys::{
    // void *
    // rb_check_typeddata(VALUE obj, const rb_data_type_t *data_type)
    rb_check_typeddata,
    // VALUE
    // rb_data_typed_object_wrap(VALUE klass, void *datap, const rb_data_type_t *type)
    rb_data_typed_object_wrap,
    // int
    // rb_typeddata_inherited_p(const rb_data_type_t *child, const rb_data_type_t *parent)
    rb_typeddata_inherited_p,
    // int
    // rb_typeddata_is_kind_of(VALUE obj, const rb_data_type_t *data_type)
    rb_typeddata_is_kind_of,
};

#[repr(C)]
pub struct RbDataTypeFunction {
    pub dmark: Option<extern "C" fn(*mut c_void)>,
    pub dfree: Option<extern "C" fn(*mut c_void)>,
    pub dsize: Option<extern "C" fn(*const c_void) -> size_t>,
    pub reserved: [*mut c_void; 2],
}

unsafe impl Send for RbDataTypeFunction {}
unsafe impl Sync for RbDataTypeFunction {}

#[repr(C)]
pub struct RbDataType {
    pub wrap_struct_name: *const c_char,
    pub function: RbDataTypeFunction,
    pub parent: *const RbDataType,
    pub data: *mut c_void,
    pub flags: Value,
}

unsafe impl Send for RbDataType {}
unsafe impl Sync for RbDataType {}
