use crate::rbsys::{
    constant::{FL_USER_1, FL_USER_3, FL_USER_4, FL_USHIFT},
    types::{c_long, InternalValue, RBasic, Value},
};

use libc::size_t;
use std::mem;

pub use rb_sys::{
    // VALUE
    // rb_ary_concat(VALUE x, VALUE y)
    rb_ary_concat,
    // VALUE
    // rb_ary_dup(VALUE ary)
    rb_ary_dup,
    // rb_ary_entry(VALUE ary, long offset)
    rb_ary_entry,
    // VALUE
    // rb_ary_join(VALUE ary, VALUE sep)
    rb_ary_join,
    // VALUE
    // rb_ary_new(void)
    rb_ary_new,
    // VALUE
    // rb_ary_new_capa(long capa)
    rb_ary_new_capa,
    // VALUE
    // rb_ary_new_from_values(long n, const VALUE *elts)
    rb_ary_new_from_values,
    // VALUE
    // rb_ary_pop(VALUE ary)
    rb_ary_pop,
    // VALUE
    // rb_ary_push(VALUE ary, VALUE item)
    rb_ary_push,
    // VALUE
    // rb_ary_reverse(VALUE ary)
    rb_ary_reverse,
    // VALUE
    // rb_ary_shift(VALUE ary)
    rb_ary_shift,
    // VALUE
    // rb_ary_sort(VALUE ary)
    rb_ary_sort,
    // VALUE
    // rb_ary_sort_bang(VALUE ary)
    rb_ary_sort_bang,
    // void
    // rb_ary_store(VALUE ary, long idx, VALUE val)
    rb_ary_store,
    // VALUE
    // rb_ary_to_s(VALUE ary)
    rb_ary_to_s,
    // VALUE
    // rb_ary_unshift(VALUE ary, VALUE item)
    rb_ary_unshift,
};

// #[link_name = "ruby_rarray_flags"]
#[derive(Debug, PartialEq)]
#[repr(C)]
enum RArrayEmbed {
    LenMax = 3,
    Flag = rb_sys::ruby_rarray_flags::RARRAY_EMBED_FLAG as isize,
    LenMask = rb_sys::ruby_rarray_flags::RARRAY_EMBED_LEN_MASK as isize,
    LenShift = rb_sys::ruby_rarray_consts::RARRAY_EMBED_LEN_SHIFT as isize,
}

#[repr(C)]
struct RArrayAs {
    heap: RArrayHeap,
}

#[repr(C)]
struct RArrayHeap {
    len: c_long,
    // Really, this is a union but value is the largest item.
    value: InternalValue,
    ptr: InternalValue,
}

use rb_sys::RArray;

pub unsafe fn rb_ary_len(value: Value) -> c_long {
    let rarray: *const RArray = mem::transmute(value.value);
    let flags = (*rarray).basic.flags;

    if flags & (RArrayEmbed::Flag as u64) == 0 {
        (*rarray).as_.heap.len
    } else {
        ((flags as i64 >> RArrayEmbed::LenShift as i64)
            & (RArrayEmbed::LenMask as i64 >> RArrayEmbed::LenShift as i64)) as c_long
    }
}
