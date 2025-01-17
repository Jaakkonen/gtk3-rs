// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
    pub struct Rectangle(BoxedInline<ffi::GdkRectangle>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::gdk_rectangle_get_type(), ptr as *mut _) as *mut ffi::GdkRectangle,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::gdk_rectangle_get_type(), ptr as *mut _),
        type_ => || ffi::gdk_rectangle_get_type(),
    }
}

impl Rectangle {
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    #[doc(alias = "gdk_rectangle_equal")]
    fn equal(&self, rect2: &Rectangle) -> bool {
        unsafe {
            from_glib(ffi::gdk_rectangle_equal(
                self.to_glib_none().0,
                rect2.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_rectangle_intersect")]
    pub fn intersect(&self, src2: &Rectangle) -> Option<Rectangle> {
        unsafe {
            let mut dest = Rectangle::uninitialized();
            let ret = from_glib(ffi::gdk_rectangle_intersect(
                self.to_glib_none().0,
                src2.to_glib_none().0,
                dest.to_glib_none_mut().0,
            ));
            if ret {
                Some(dest)
            } else {
                None
            }
        }
    }

    #[doc(alias = "gdk_rectangle_union")]
    pub fn union(&self, src2: &Rectangle) -> Rectangle {
        unsafe {
            let mut dest = Rectangle::uninitialized();
            ffi::gdk_rectangle_union(
                self.to_glib_none().0,
                src2.to_glib_none().0,
                dest.to_glib_none_mut().0,
            );
            dest
        }
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
impl PartialEq for Rectangle {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Rectangle {}
