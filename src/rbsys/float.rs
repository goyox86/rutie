use crate::rbsys::types::{c_double, Value};

pub use rb_sys::{
    // VALUE
    // rb_float_new(double d)
    rb_float_new,
    // double
    // rb_num2dbl(VALUE val)
    rb_num2dbl,
    // VALUE
    // rb_to_float(VALUE val)
    rb_to_float,
};
