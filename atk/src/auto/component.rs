// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::CoordType;
use crate::Layer;
use crate::Object;
use crate::Rectangle;
#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
use crate::ScrollType;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "AtkComponent")]
    pub struct Component(Interface<ffi::AtkComponent, ffi::AtkComponentIface>);

    match fn {
        type_ => || ffi::atk_component_get_type(),
    }
}

impl Component {
    pub const NONE: Option<&'static Component> = None;
}

pub trait ComponentExt: 'static {
    #[doc(alias = "atk_component_contains")]
    fn contains(&self, x: i32, y: i32, coord_type: CoordType) -> bool;

    #[doc(alias = "atk_component_get_alpha")]
    #[doc(alias = "get_alpha")]
    fn alpha(&self) -> f64;

    #[doc(alias = "atk_component_get_extents")]
    #[doc(alias = "get_extents")]
    fn extents(&self, coord_type: CoordType) -> (i32, i32, i32, i32);

    #[doc(alias = "atk_component_get_layer")]
    #[doc(alias = "get_layer")]
    fn layer(&self) -> Layer;

    #[doc(alias = "atk_component_get_mdi_zorder")]
    #[doc(alias = "get_mdi_zorder")]
    fn mdi_zorder(&self) -> i32;

    #[doc(alias = "atk_component_get_position")]
    #[doc(alias = "get_position")]
    fn position(&self, coord_type: CoordType) -> (i32, i32);

    #[doc(alias = "atk_component_get_size")]
    #[doc(alias = "get_size")]
    fn size(&self) -> (i32, i32);

    #[doc(alias = "atk_component_grab_focus")]
    fn grab_focus(&self) -> bool;

    #[doc(alias = "atk_component_ref_accessible_at_point")]
    fn ref_accessible_at_point(&self, x: i32, y: i32, coord_type: CoordType) -> Option<Object>;

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    #[doc(alias = "atk_component_scroll_to")]
    fn scroll_to(&self, type_: ScrollType) -> bool;

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    #[doc(alias = "atk_component_scroll_to_point")]
    fn scroll_to_point(&self, coords: CoordType, x: i32, y: i32) -> bool;

    #[doc(alias = "atk_component_set_extents")]
    fn set_extents(&self, x: i32, y: i32, width: i32, height: i32, coord_type: CoordType) -> bool;

    #[doc(alias = "atk_component_set_position")]
    fn set_position(&self, x: i32, y: i32, coord_type: CoordType) -> bool;

    #[doc(alias = "atk_component_set_size")]
    fn set_size(&self, width: i32, height: i32) -> bool;

    #[doc(alias = "bounds-changed")]
    fn connect_bounds_changed<F: Fn(&Self, &Rectangle) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Component>> ComponentExt for O {
    fn contains(&self, x: i32, y: i32, coord_type: CoordType) -> bool {
        unsafe {
            from_glib(ffi::atk_component_contains(
                self.as_ref().to_glib_none().0,
                x,
                y,
                coord_type.into_glib(),
            ))
        }
    }

    fn alpha(&self) -> f64 {
        unsafe { ffi::atk_component_get_alpha(self.as_ref().to_glib_none().0) }
    }

    fn extents(&self, coord_type: CoordType) -> (i32, i32, i32, i32) {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            ffi::atk_component_get_extents(
                self.as_ref().to_glib_none().0,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
                width.as_mut_ptr(),
                height.as_mut_ptr(),
                coord_type.into_glib(),
            );
            (
                x.assume_init(),
                y.assume_init(),
                width.assume_init(),
                height.assume_init(),
            )
        }
    }

    fn layer(&self) -> Layer {
        unsafe { from_glib(ffi::atk_component_get_layer(self.as_ref().to_glib_none().0)) }
    }

    fn mdi_zorder(&self) -> i32 {
        unsafe { ffi::atk_component_get_mdi_zorder(self.as_ref().to_glib_none().0) }
    }

    fn position(&self, coord_type: CoordType) -> (i32, i32) {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            ffi::atk_component_get_position(
                self.as_ref().to_glib_none().0,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
                coord_type.into_glib(),
            );
            (x.assume_init(), y.assume_init())
        }
    }

    fn size(&self) -> (i32, i32) {
        unsafe {
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            ffi::atk_component_get_size(
                self.as_ref().to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            (width.assume_init(), height.assume_init())
        }
    }

    fn grab_focus(&self) -> bool {
        unsafe {
            from_glib(ffi::atk_component_grab_focus(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn ref_accessible_at_point(&self, x: i32, y: i32, coord_type: CoordType) -> Option<Object> {
        unsafe {
            from_glib_full(ffi::atk_component_ref_accessible_at_point(
                self.as_ref().to_glib_none().0,
                x,
                y,
                coord_type.into_glib(),
            ))
        }
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn scroll_to(&self, type_: ScrollType) -> bool {
        unsafe {
            from_glib(ffi::atk_component_scroll_to(
                self.as_ref().to_glib_none().0,
                type_.into_glib(),
            ))
        }
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn scroll_to_point(&self, coords: CoordType, x: i32, y: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_component_scroll_to_point(
                self.as_ref().to_glib_none().0,
                coords.into_glib(),
                x,
                y,
            ))
        }
    }

    fn set_extents(&self, x: i32, y: i32, width: i32, height: i32, coord_type: CoordType) -> bool {
        unsafe {
            from_glib(ffi::atk_component_set_extents(
                self.as_ref().to_glib_none().0,
                x,
                y,
                width,
                height,
                coord_type.into_glib(),
            ))
        }
    }

    fn set_position(&self, x: i32, y: i32, coord_type: CoordType) -> bool {
        unsafe {
            from_glib(ffi::atk_component_set_position(
                self.as_ref().to_glib_none().0,
                x,
                y,
                coord_type.into_glib(),
            ))
        }
    }

    fn set_size(&self, width: i32, height: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_component_set_size(
                self.as_ref().to_glib_none().0,
                width,
                height,
            ))
        }
    }

    fn connect_bounds_changed<F: Fn(&Self, &Rectangle) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn bounds_changed_trampoline<
            P: IsA<Component>,
            F: Fn(&P, &Rectangle) + 'static,
        >(
            this: *mut ffi::AtkComponent,
            arg1: *mut ffi::AtkRectangle,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Component::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(arg1),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"bounds-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    bounds_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Component")
    }
}
