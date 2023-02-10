use super::types::{c_char, c_long, Id, Value};

pub use rb_sys::{
    // const char *
    // rb_id2name(ID id)
    rb_id2name,
    // VALUE
    // rb_id2sym(ID x)
    rb_id2sym,
    // ID
    // rb_intern(const char *name)
    rb_intern,
    // ID
    // rb_intern2(const char *name, long len)
    rb_intern2,
    // ID
    // rb_sym2id(VALUE sym)
    rb_sym2id,
};
