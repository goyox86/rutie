use crate::rbsys::types::{c_char, c_int, c_void, Argc, CallbackPtr, Id, Value, VmPointer};

pub use ::rb_sys::{
    // VALUE
    // rb_block_call(VALUE obj, ID mid, int argc, const VALUE * argv,
    //               VALUE (*bl_proc) (ANYARGS), VALUE data2)
    rb_block_call,
    // int
    // rb_block_given_p(void)
    rb_block_given_p,
    // VALUE
    // rb_block_proc(void)
    rb_block_proc,
    // VALUE
    // rb_call_super(int argc, const VALUE *argv)
    rb_call_super,
    // VALUE
    // rb_errinfo(void)
    rb_errinfo,
    // VALUE
    // rb_eval_string(const char *str)
    rb_eval_string,
    // VALUE
    // rb_eval_string_protect(const char *str, int *pstate)
    rb_eval_string_protect,
    // //////////////// UNAVAILABLE METHOD ////////////////
    // // VALUE
    // // rb_f_eval(int argc, const VALUE *argv, VALUE self)
    // pub fn rb_f_eval(argc: c_int, argv: *const Value, self_: Value) -> Value;
    // ///////////////// ///////////////// ///////////////
    // void
    // rb_exc_raise(VALUE mesg)
    rb_exc_raise,
    // void
    // rb_exit(int status)
    rb_exit,
    // VALUE
    // rb_f_abort(int argc, const VALUE *argv)
    rb_f_abort,
    // VALUE
    // rb_funcallv(VALUE recv, ID mid, int argc, const VALUE *argv)
    rb_funcallv,
    // VALUE
    // rb_funcallv_public(VALUE recv, ID mid, int argc, const VALUE *argv)
    rb_funcallv_public,
    // VALUE
    // rb_protect(VALUE (* proc) (VALUE), VALUE data, int *pstate)
    rb_protect,
    // void
    // rb_raise(VALUE exc, const char *fmt, ...)
    rb_raise,
    // VALUE
    // rb_require(const char *fname)
    rb_require,
    // void
    // rb_set_errinfo(VALUE err)
    rb_set_errinfo,
    // VALUE
    // rb_yield(VALUE val)
    rb_yield,
    // VALUE
    // rb_yield_splat(VALUE values)
    rb_yield_splat,
    // void
    // ruby_init(void)
    ruby_init,
    // void
    // ruby_init_loadpath(void)
    ruby_init_loadpath,
    // void
    // ruby_vm_at_exit(void(*func)(ruby_vm_t *))
    ruby_vm_at_exit,
};
