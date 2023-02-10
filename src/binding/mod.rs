#[cfg(not(feature = "rb-sys"))]
mod legacy;

#[cfg(not(feature = "rb-sys"))]
pub use legacy::{
    array, class, encoding, fixnum, float, gc, global, hash, module, rproc, string, symbol, thread,
    vm,
};

#[cfg(feature = "rb-sys")]
mod rbsys;

// #[cfg(feature = "rb-sys")]
// pub use rbsys::{
//     array, class, encoding, fixnum, float, gc, global, hash, module, rproc, string, symbol, thread,
//     vm,
// };

#[cfg(feature = "rb-sys")]
pub use rbsys::{
    array, class, encoding, fixnum, float, gc, global, hash, module, rproc, string, symbol, vm,
};
