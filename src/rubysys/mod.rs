extern crate libc;

#[cfg(not(feature = "rb-sys"))]
mod legacy;

#[cfg(not(feature = "rb-sys"))]
pub use legacy::{
    array, class, constant, encoding, fixnum, float, gc, hash, rproc, string, symbol, thread,
    typed_data, types, value, vm,
};

#[cfg(not(feature = "rb-sys"))]
use legacy::types::Value;

#[cfg(not(feature = "rb-sys"))]
extern "C" {
    pub static rb_cObject: Value;
}

#[cfg(feature = "rb-sys")]
mod rbsys;

#[cfg(feature = "rb-sys")]
pub use rbsys::{
    array, class, constant, encoding, fixnum, float, gc, hash, rproc, string, symbol, thread,
    typed_data, types, value, vm,
};

#[cfg(feature = "rb-sys")]
pub use rbsys::types::Value;

#[cfg(feature = "rb-sys")]
pub use rb_sys::rb_cObject;
