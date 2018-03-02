// This file was generated by gir (https://github.com/gtk-rs/gir @ fbb95f4)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Container;
use ToolItem;
use Widget;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct ToolButton(Object<ffi::GtkToolButton, ffi::GtkToolButtonClass>): ToolItem, Bin, Container, Widget, Buildable, Actionable;

    match fn {
        get_type => || ffi::gtk_tool_button_get_type(),
    }
}

impl ToolButton {
    pub fn new<'a, 'b, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b str>>>(icon_widget: Q, label: R) -> ToolButton {
        assert_initialized_main_thread!();
        let icon_widget = icon_widget.into();
        let icon_widget = icon_widget.to_glib_none();
        let label = label.into();
        let label = label.to_glib_none();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_tool_button_new(icon_widget.0, label.0)).downcast_unchecked()
        }
    }

    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn new_from_stock(stock_id: &str) -> ToolButton {
        assert_initialized_main_thread!();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_tool_button_new_from_stock(stock_id.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait ToolButtonExt {
    fn get_icon_name(&self) -> Option<String>;

    fn get_icon_widget(&self) -> Option<Widget>;

    fn get_label(&self) -> Option<String>;

    fn get_label_widget(&self) -> Option<Widget>;

    #[cfg_attr(feature = "v3_10", deprecated)]
    fn get_stock_id(&self) -> Option<String>;

    fn get_use_underline(&self) -> bool;

    fn set_icon_name<'a, P: Into<Option<&'a str>>>(&self, icon_name: P);

    fn set_icon_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, icon_widget: Q);

    fn set_label<'a, P: Into<Option<&'a str>>>(&self, label: P);

    fn set_label_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, label_widget: Q);

    #[cfg_attr(feature = "v3_10", deprecated)]
    fn set_stock_id<'a, P: Into<Option<&'a str>>>(&self, stock_id: P);

    fn set_use_underline(&self, use_underline: bool);

    fn connect_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_clicked(&self);

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icon_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v3_10", deprecated)]
    fn connect_property_stock_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_underline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ToolButton> + IsA<glib::object::Object> + glib::object::ObjectExt> ToolButtonExt for O {
    fn get_icon_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_tool_button_get_icon_name(self.to_glib_none().0))
        }
    }

    fn get_icon_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tool_button_get_icon_widget(self.to_glib_none().0))
        }
    }

    fn get_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_tool_button_get_label(self.to_glib_none().0))
        }
    }

    fn get_label_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tool_button_get_label_widget(self.to_glib_none().0))
        }
    }

    fn get_stock_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_tool_button_get_stock_id(self.to_glib_none().0))
        }
    }

    fn get_use_underline(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_button_get_use_underline(self.to_glib_none().0))
        }
    }

    fn set_icon_name<'a, P: Into<Option<&'a str>>>(&self, icon_name: P) {
        let icon_name = icon_name.into();
        let icon_name = icon_name.to_glib_none();
        unsafe {
            ffi::gtk_tool_button_set_icon_name(self.to_glib_none().0, icon_name.0);
        }
    }

    fn set_icon_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, icon_widget: Q) {
        let icon_widget = icon_widget.into();
        let icon_widget = icon_widget.to_glib_none();
        unsafe {
            ffi::gtk_tool_button_set_icon_widget(self.to_glib_none().0, icon_widget.0);
        }
    }

    fn set_label<'a, P: Into<Option<&'a str>>>(&self, label: P) {
        let label = label.into();
        let label = label.to_glib_none();
        unsafe {
            ffi::gtk_tool_button_set_label(self.to_glib_none().0, label.0);
        }
    }

    fn set_label_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, label_widget: Q) {
        let label_widget = label_widget.into();
        let label_widget = label_widget.to_glib_none();
        unsafe {
            ffi::gtk_tool_button_set_label_widget(self.to_glib_none().0, label_widget.0);
        }
    }

    fn set_stock_id<'a, P: Into<Option<&'a str>>>(&self, stock_id: P) {
        let stock_id = stock_id.into();
        let stock_id = stock_id.to_glib_none();
        unsafe {
            ffi::gtk_tool_button_set_stock_id(self.to_glib_none().0, stock_id.0);
        }
    }

    fn set_use_underline(&self, use_underline: bool) {
        unsafe {
            ffi::gtk_tool_button_set_use_underline(self.to_glib_none().0, use_underline.to_glib());
        }
    }

    fn connect_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "clicked",
                transmute(clicked_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_clicked(&self) {
        let _ = self.emit("clicked", &[]).unwrap();
    }

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::icon-name",
                transmute(notify_icon_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_icon_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::icon-widget",
                transmute(notify_icon_widget_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::label",
                transmute(notify_label_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_label_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::label-widget",
                transmute(notify_label_widget_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_stock_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::stock-id",
                transmute(notify_stock_id_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_use_underline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::use-underline",
                transmute(notify_use_underline_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn clicked_trampoline<P>(this: *mut ffi::GtkToolButton, f: glib_ffi::gpointer)
where P: IsA<ToolButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ToolButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_icon_name_trampoline<P>(this: *mut ffi::GtkToolButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ToolButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ToolButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_icon_widget_trampoline<P>(this: *mut ffi::GtkToolButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ToolButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ToolButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_label_trampoline<P>(this: *mut ffi::GtkToolButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ToolButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ToolButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_label_widget_trampoline<P>(this: *mut ffi::GtkToolButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ToolButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ToolButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_stock_id_trampoline<P>(this: *mut ffi::GtkToolButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ToolButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ToolButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_use_underline_trampoline<P>(this: *mut ffi::GtkToolButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ToolButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ToolButton::from_glib_borrow(this).downcast_unchecked())
}
