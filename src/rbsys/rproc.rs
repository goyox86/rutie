use crate::{
    rbsys::{
        constant::UNLIMITED_ARGUMENTS,
        types::{c_int, Argc, Value},
    },
    AnyException, Exception,
};

pub use rb_sys::{
    // VALUE
    // rb_binding_new(void)
    rb_binding_new,
    rb_obj_is_method,
    rb_obj_is_proc,
    // VALUE
    // rb_proc_call_with_block(VALUE self, int argc, const VALUE *argv, VALUE passed_procval)
    rb_proc_call_with_block,
};

pub fn check_arity(argc: c_int, min: c_int, max: c_int) -> Result<c_int, AnyException> {
    if argc < min || (max != UNLIMITED_ARGUMENTS as c_int && argc > max) {
        let err_mess = if min == max {
            format!(
                "wrong number of arguments (given {}, expected {})",
                argc, min
            )
        } else if max == UNLIMITED_ARGUMENTS as c_int {
            format!(
                "wrong number of arguments (given {}, expected {}+)",
                argc, min
            )
        } else {
            format!(
                "wrong number of arguments (given {}, expected {}..{})",
                argc, min, max
            )
        };

        return Err(AnyException::new("ArgumentError", Some(&err_mess)));
    }

    Ok(argc)
}
