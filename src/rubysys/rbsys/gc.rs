use super::types::{c_int, size_t, ssize_t, CallbackPtr, Value};

pub use rb_sys::{
    // void
    // rb_gc_adjust_memory_usage(ssize_t diff)
    rb_gc_adjust_memory_usage,
    // size_t
    // rb_gc_count(void)
    rb_gc_count,
    // VALUE
    // rb_gc_disable(void)
    rb_gc_disable,
    // VALUE
    // rb_gc_enable(void)
    rb_gc_enable,
    // void
    // rb_gc_force_recycle(VALUE obj)
    rb_gc_force_recycle,
    // void
    // rb_gc_mark(VALUE ptr)
    rb_gc_mark,
    // void
    // rb_gc_mark_maybe(VALUE obj)
    rb_gc_mark_maybe,
    // void
    // rb_gc_register_address(VALUE *addr)
    rb_gc_register_address,
    // void
    // rb_gc_register_mark_object(VALUE obj)
    rb_gc_register_mark_object,
    // VALUE
    // rb_gc_start(void)
    rb_gc_start,
    // size_t
    // rb_gc_stat(VALUE key)
    rb_gc_stat,
    // void
    // rb_gc_unregister_address(VALUE *addr)
    rb_gc_unregister_address,
};

extern "C" {

    // int
    // rb_objspace_marked_object_p(VALUE obj)
    pub fn rb_objspace_marked_object_p(obj: Value) -> c_int;
}
