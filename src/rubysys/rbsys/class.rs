use super::types::{c_char, c_int, Argc, CallbackPtr, Id, Value};

pub use rb_sys::{
    // VALUE
    // rb_class_new_instance(int argc, const VALUE *argv, VALUE klass)
    rb_class_new_instance,
    // VALUE
    // rb_class_superclass(VALUE klass)
    rb_class_superclass,
    // VALUE
    // rb_const_get(VALUE obj, ID id)
    rb_const_get,
    // void
    // rb_define_attr(VALUE klass, const char *name, int read, int write)
    rb_define_attr,
    // VALUE
    // rb_define_class(const char *name, VALUE super)
    rb_define_class,
    // VALUE
    // rb_define_class_under(VALUE outer, const char *name, VALUE super)
    rb_define_class_under,
    // void
    // rb_define_const(VALUE klass, const char *name, VALUE val)
    rb_define_const,
    // void
    // rb_define_method(VALUE klass, const char *name, VALUE (*func)(ANYARGS), int argc)
    rb_define_method,
    // VALUE
    // rb_define_module(const char *name)
    rb_define_module,
    // void
    // rb_define_module_function(VALUE module, const char *name, VALUE (*func)(ANYARGS), int argc)
    rb_define_module_function,
    // VALUE
    // rb_define_module_under(VALUE outer, const char *name)
    rb_define_module_under,
    // void
    // rb_define_private_method(VALUE klass, const char *name, VALUE (*func)(ANYARGS), int argc)
    rb_define_private_method,
    // void
    // rb_define_singleton_method(VALUE obj, const char *name, VALUE (*func)(ANYARGS), int argc)
    rb_define_singleton_method, // VALUE
    // rb_eql(VALUE obj1, VALUE obj2)
    rb_eql,
    // VALUE
    // rb_equal(VALUE obj1, VALUE obj2)
    rb_equal,
    // void
    // rb_extend_object(VALUE object, VALUE module)
    rb_extend_object,
    // void
    // rb_include_module(VALUE klass, VALUE module)
    rb_include_module,
    // VALUE
    // rb_ivar_get(VALUE obj, ID id)
    rb_ivar_get,
    // VALUE
    // rb_ivar_set(VALUE obj, ID id, VALUE val)
    rb_ivar_set,
    // VALUE
    // rb_mod_ancestors(VALUE mod)
    rb_mod_ancestors,
    // VALUE
    // rb_obj_class(VALUE obj)
    rb_obj_class,
    // VALUE
    // rb_obj_freeze(VALUE obj)
    rb_obj_freeze,
    // VALUE
    // rb_obj_frozen_p(VALUE obj)
    rb_obj_frozen_p,
    // void
    // rb_prepend_module(VALUE klass, VALUE module)
    rb_prepend_module,
    // int
    // rb_respond_to(VALUE obj, ID id)
    rb_respond_to,
    // int
    // rb_scan_args(int argc, const VALUE *argv, const char *fmt, ...)
    rb_scan_args,
    // VALUE
    // rb_singleton_class(VALUE obj)
    rb_singleton_class,
};
