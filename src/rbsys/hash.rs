use crate::rbsys::types::{CallbackMutPtr, CallbackPtr, Value};

pub use rb_sys::{
    // VALUE
    // rb_hash_aref(VALUE hash, VALUE key)
    rb_hash_aref,
    // VALUE
    // rb_hash_aset(VALUE hash, VALUE key, VALUE val)
    rb_hash_aset,
    // VALUE
    // rb_hash_clear(VALUE hash)
    rb_hash_clear,
    // VALUE
    // rb_hash_delete(VALUE hash, VALUE key)
    rb_hash_delete,
    // VALUE
    // rb_hash_dup(VALUE hash)
    rb_hash_dup,
    // void
    // rb_hash_foreach(VALUE hash, int (*func)(ANYARGS), VALUE farg)
    rb_hash_foreach,
    // VALUE
    // rb_hash_new(void)
    rb_hash_new,
    // VALUE
    // rb_hash_size(VALUE hash)
    rb_hash_size,
};
