// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
#[cfg(any(feature = "v3_6", feature = "dox"))]
use LevelBarMode;
use Orientable;
use Widget;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v3_6", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_6", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v3_6", feature = "dox"))]
use libc;
#[cfg(any(feature = "v3_6", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v3_6", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct LevelBar(Object<ffi::GtkLevelBar, ffi::GtkLevelBarClass>): Widget, Buildable, Orientable;

    match fn {
        get_type => || ffi::gtk_level_bar_get_type(),
    }
}

impl LevelBar {
    #[cfg(any(feature = "v3_6", feature = "dox"))]
    pub fn new() -> LevelBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_level_bar_new()).downcast_unchecked()
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    pub fn new_for_interval(min_value: f64, max_value: f64) -> LevelBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_level_bar_new_for_interval(min_value, max_value)).downcast_unchecked()
        }
    }
}

#[cfg(any(feature = "v3_6", feature = "dox"))]
impl Default for LevelBar {
    fn default() -> Self {
        Self::new()
    }
}

pub trait LevelBarExt {
    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn add_offset_value(&self, name: &str, value: f64);

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn get_inverted(&self) -> bool;

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_max_value(&self) -> f64;

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_min_value(&self) -> f64;

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_mode(&self) -> LevelBarMode;

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_offset_value<'a, P: Into<Option<&'a str>>>(&self, name: P) -> Option<f64>;

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_value(&self) -> f64;

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn remove_offset_value<'a, P: Into<Option<&'a str>>>(&self, name: P);

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn set_inverted(&self, inverted: bool);

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn set_max_value(&self, value: f64);

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn set_min_value(&self, value: f64);

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn set_mode(&self, mode: LevelBarMode);

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn set_value(&self, value: f64);

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn connect_offset_changed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn connect_property_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn connect_property_max_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn connect_property_min_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn connect_property_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<LevelBar> + IsA<glib::object::Object>> LevelBarExt for O {
    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn add_offset_value(&self, name: &str, value: f64) {
        unsafe {
            ffi::gtk_level_bar_add_offset_value(self.to_glib_none().0, name.to_glib_none().0, value);
        }
    }

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn get_inverted(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_level_bar_get_inverted(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_max_value(&self) -> f64 {
        unsafe {
            ffi::gtk_level_bar_get_max_value(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_min_value(&self) -> f64 {
        unsafe {
            ffi::gtk_level_bar_get_min_value(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_mode(&self) -> LevelBarMode {
        unsafe {
            from_glib(ffi::gtk_level_bar_get_mode(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_offset_value<'a, P: Into<Option<&'a str>>>(&self, name: P) -> Option<f64> {
        let name = name.into();
        let name = name.to_glib_none();
        unsafe {
            let mut value = mem::uninitialized();
            let ret = from_glib(ffi::gtk_level_bar_get_offset_value(self.to_glib_none().0, name.0, &mut value));
            if ret { Some(value) } else { None }
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_value(&self) -> f64 {
        unsafe {
            ffi::gtk_level_bar_get_value(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn remove_offset_value<'a, P: Into<Option<&'a str>>>(&self, name: P) {
        let name = name.into();
        let name = name.to_glib_none();
        unsafe {
            ffi::gtk_level_bar_remove_offset_value(self.to_glib_none().0, name.0);
        }
    }

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn set_inverted(&self, inverted: bool) {
        unsafe {
            ffi::gtk_level_bar_set_inverted(self.to_glib_none().0, inverted.to_glib());
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn set_max_value(&self, value: f64) {
        unsafe {
            ffi::gtk_level_bar_set_max_value(self.to_glib_none().0, value);
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn set_min_value(&self, value: f64) {
        unsafe {
            ffi::gtk_level_bar_set_min_value(self.to_glib_none().0, value);
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn set_mode(&self, mode: LevelBarMode) {
        unsafe {
            ffi::gtk_level_bar_set_mode(self.to_glib_none().0, mode.to_glib());
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn set_value(&self, value: f64) {
        unsafe {
            ffi::gtk_level_bar_set_value(self.to_glib_none().0, value);
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn connect_offset_changed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "offset-changed",
                transmute(offset_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn connect_property_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::inverted",
                transmute(notify_inverted_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn connect_property_max_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::max-value",
                transmute(notify_max_value_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn connect_property_min_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::min-value",
                transmute(notify_min_value_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn connect_property_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::mode",
                transmute(notify_mode_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::value",
                transmute(notify_value_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_6", feature = "dox"))]
unsafe extern "C" fn offset_changed_trampoline<P>(this: *mut ffi::GtkLevelBar, name: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<LevelBar> {
    callback_guard!();
    let f: &&(Fn(&P, &str) + 'static) = transmute(f);
    f(&LevelBar::from_glib_borrow(this).downcast_unchecked(), &String::from_glib_none(name))
}

#[cfg(any(feature = "v3_8", feature = "dox"))]
unsafe extern "C" fn notify_inverted_trampoline<P>(this: *mut ffi::GtkLevelBar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<LevelBar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&LevelBar::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_6", feature = "dox"))]
unsafe extern "C" fn notify_max_value_trampoline<P>(this: *mut ffi::GtkLevelBar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<LevelBar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&LevelBar::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_6", feature = "dox"))]
unsafe extern "C" fn notify_min_value_trampoline<P>(this: *mut ffi::GtkLevelBar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<LevelBar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&LevelBar::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_6", feature = "dox"))]
unsafe extern "C" fn notify_mode_trampoline<P>(this: *mut ffi::GtkLevelBar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<LevelBar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&LevelBar::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_6", feature = "dox"))]
unsafe extern "C" fn notify_value_trampoline<P>(this: *mut ffi::GtkLevelBar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<LevelBar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&LevelBar::from_glib_borrow(this).downcast_unchecked())
}
