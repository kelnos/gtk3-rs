// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ObjectFactory;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "AtkNoOpObjectFactory")]
    pub struct NoOpObjectFactory(Object<ffi::AtkNoOpObjectFactory, ffi::AtkNoOpObjectFactoryClass>) @extends ObjectFactory;

    match fn {
        type_ => || ffi::atk_no_op_object_factory_get_type(),
    }
}

impl NoOpObjectFactory {
    pub const NONE: Option<&'static NoOpObjectFactory> = None;

    #[doc(alias = "atk_no_op_object_factory_new")]
    pub fn new() -> NoOpObjectFactory {
        assert_initialized_main_thread!();
        unsafe { ObjectFactory::from_glib_full(ffi::atk_no_op_object_factory_new()).unsafe_cast() }
    }
}

impl Default for NoOpObjectFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for NoOpObjectFactory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("NoOpObjectFactory")
    }
}
