// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use CellRenderer;
use TreePath;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct CellRendererToggle(Object<ffi::GtkCellRendererToggle, ffi::GtkCellRendererToggleClass>): CellRenderer;

    match fn {
        get_type => || ffi::gtk_cell_renderer_toggle_get_type(),
    }
}

impl CellRendererToggle {
    pub fn new() -> CellRendererToggle {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(ffi::gtk_cell_renderer_toggle_new()).downcast_unchecked()
        }
    }
}

impl Default for CellRendererToggle {
    fn default() -> Self {
        Self::new()
    }
}

pub trait CellRendererToggleExt {
    fn get_activatable(&self) -> bool;

    fn get_active(&self) -> bool;

    fn get_radio(&self) -> bool;

    fn set_activatable(&self, setting: bool);

    fn set_active(&self, setting: bool);

    fn set_radio(&self, radio: bool);

    fn get_property_inconsistent(&self) -> bool;

    fn set_property_inconsistent(&self, inconsistent: bool);

    fn get_property_indicator_size(&self) -> i32;

    fn set_property_indicator_size(&self, indicator_size: i32);

    fn connect_toggled<F: Fn(&Self, TreePath) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_activatable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_inconsistent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_indicator_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_radio_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CellRendererToggle> + IsA<glib::object::Object>> CellRendererToggleExt for O {
    fn get_activatable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_renderer_toggle_get_activatable(self.to_glib_none().0))
        }
    }

    fn get_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_renderer_toggle_get_active(self.to_glib_none().0))
        }
    }

    fn get_radio(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_renderer_toggle_get_radio(self.to_glib_none().0))
        }
    }

    fn set_activatable(&self, setting: bool) {
        unsafe {
            ffi::gtk_cell_renderer_toggle_set_activatable(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_active(&self, setting: bool) {
        unsafe {
            ffi::gtk_cell_renderer_toggle_set_active(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_radio(&self, radio: bool) {
        unsafe {
            ffi::gtk_cell_renderer_toggle_set_radio(self.to_glib_none().0, radio.to_glib());
        }
    }

    fn get_property_inconsistent(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "inconsistent".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_inconsistent(&self, inconsistent: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "inconsistent".to_glib_none().0, Value::from(&inconsistent).to_glib_none().0);
        }
    }

    fn get_property_indicator_size(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "indicator-size".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_indicator_size(&self, indicator_size: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "indicator-size".to_glib_none().0, Value::from(&indicator_size).to_glib_none().0);
        }
    }

    fn connect_toggled<F: Fn(&Self, TreePath) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, TreePath) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "toggled",
                transmute(toggled_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_activatable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::activatable",
                transmute(notify_activatable_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::active",
                transmute(notify_active_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_inconsistent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::inconsistent",
                transmute(notify_inconsistent_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_indicator_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::indicator-size",
                transmute(notify_indicator_size_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_radio_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::radio",
                transmute(notify_radio_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn toggled_trampoline<P>(this: *mut ffi::GtkCellRendererToggle, path: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<CellRendererToggle> {
    callback_guard!();
    let f: &&(Fn(&P, TreePath) + 'static) = transmute(f);
    let path = from_glib_full(ffi::gtk_tree_path_new_from_string(path));
    f(&CellRendererToggle::from_glib_borrow(this).downcast_unchecked(), path)
}

unsafe extern "C" fn notify_activatable_trampoline<P>(this: *mut ffi::GtkCellRendererToggle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererToggle> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellRendererToggle::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_active_trampoline<P>(this: *mut ffi::GtkCellRendererToggle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererToggle> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellRendererToggle::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_inconsistent_trampoline<P>(this: *mut ffi::GtkCellRendererToggle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererToggle> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellRendererToggle::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_indicator_size_trampoline<P>(this: *mut ffi::GtkCellRendererToggle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererToggle> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellRendererToggle::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_radio_trampoline<P>(this: *mut ffi::GtkCellRendererToggle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererToggle> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellRendererToggle::from_glib_borrow(this).downcast_unchecked())
}
