use super::{
    constant::{FL_USER_8, FL_USER_9},
    types::{
        c_char, c_int, size_t, CallbackPtr, EncodingIndex, EncodingType, InternalValue, RBasic,
        Value,
    },
};
use std::mem;

pub const ENC_DUMMY_FLAG: isize = 1 << 24;
pub const ENC_INDEX_MASK: isize = !(!0 << 24);
pub const ENC_CODERANGE_UNKNOWN: isize = 0;
pub const ENC_CODERANGE_7BIT: isize = FL_USER_8;
pub const ENC_CODERANGE_VALID: isize = FL_USER_9;
pub const ENC_CODERANGE_BROKEN: isize = FL_USER_8 | FL_USER_9;
pub const ENC_CODERANGE_MASK: isize =
    ENC_CODERANGE_7BIT | ENC_CODERANGE_VALID | ENC_CODERANGE_BROKEN;

pub use rb_sys::{
    // int
    // rb_econv_prepare_opts(VALUE opthash, VALUE *opts)
    rb_econv_prepare_opts,
    // VALUE
    // rb_enc_associate(VALUE obj, rb_encoding *enc)
    rb_enc_associate,
    // VALUE
    // rb_enc_associate_index(VALUE obj, int idx)
    rb_enc_associate_index,
    // unsigned int
    // rb_enc_codepoint_len(const char *p, const char *e, int *len_p, rb_encoding *enc)
    rb_enc_codepoint_len,
    // rb_encoding*
    // rb_enc_compatible(VALUE str1, VALUE str2)
    rb_enc_compatible,
    // VALUE
    // rb_enc_default_external(void)
    rb_enc_default_external,
    // VALUE
    // rb_enc_default_internal(void)
    rb_enc_default_internal,
    // int
    // rb_enc_find_index(const char *name)
    rb_enc_find_index,
    // ------------------------------------------------------
    // LINKER CANNOT FIND
    // // static VALUE
    // // rb_enc_from_encoding_index(int idx)
    // pub fn rb_enc_from_encoding_index(idx: c_int) -> Value;
    // ------------------------------------------------------
    // VALUE
    // rb_enc_from_encoding(rb_encoding *encoding)
    rb_enc_from_encoding,
    // rb_encoding *
    // rb_enc_from_index(int index)
    rb_enc_from_index,
    // int
    // rb_enc_get_index(VALUE obj)
    rb_enc_get_index,
    // void
    // rb_enc_set_default_external(VALUE encoding)
    rb_enc_set_default_external,
    // void
    // rb_enc_set_default_internal(VALUE encoding)
    rb_enc_set_default_internal,
    // void
    // rb_enc_set_index(VALUE obj, int idx)
    rb_enc_set_index,
    // int
    // rb_filesystem_encindex(void)
    rb_filesystem_encindex,
    // int
    // rb_locale_encindex(void)
    rb_locale_encindex,
    // VALUE
    // rb_obj_encoding(VALUE obj)
    rb_obj_encoding,
    // VALUE
    // rb_str_encode(VALUE str, VALUE to, int ecflags, VALUE ecopts)
    rb_str_encode,
    // VALUE
    // rb_str_export_to_enc(VALUE str, rb_encoding *enc)
    rb_str_export_to_enc,
    // rb_encoding *
    // rb_to_encoding(VALUE enc)
    rb_to_encoding,
    // int
    // rb_to_encoding_index(VALUE enc)
    rb_to_encoding_index,
    // int
    // rb_usascii_encindex(void)
    rb_usascii_encindex,
    // int
    // rb_utf8_encindex(void)
    rb_utf8_encindex,
};

pub unsafe fn coderange_set(obj: Value, code_range: InternalValue) {
    let basic: *mut RBasic = mem::transmute(obj.value);
    (*basic).flags = ((*basic).flags & !(ENC_CODERANGE_MASK as InternalValue)) | code_range
}

pub unsafe fn coderange_clear(obj: Value) {
    coderange_set(obj, 0)
}
