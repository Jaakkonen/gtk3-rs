// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(clippy::approx_constant, clippy::type_complexity, clippy::unreadable_literal)]

extern crate libc;
extern crate glib_sys as glib;
extern crate gobject_sys as gobject;
extern crate gio_sys as gio;

#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, intptr_t, uintptr_t, time_t, FILE};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type GdkColorspace = c_int;
pub const GDK_COLORSPACE_RGB: GdkColorspace = 0;

pub type GdkInterpType = c_int;
pub const GDK_INTERP_NEAREST: GdkInterpType = 0;
pub const GDK_INTERP_TILES: GdkInterpType = 1;
pub const GDK_INTERP_BILINEAR: GdkInterpType = 2;
pub const GDK_INTERP_HYPER: GdkInterpType = 3;

pub type GdkPixbufAlphaMode = c_int;
pub const GDK_PIXBUF_ALPHA_BILEVEL: GdkPixbufAlphaMode = 0;
pub const GDK_PIXBUF_ALPHA_FULL: GdkPixbufAlphaMode = 1;

pub type GdkPixbufError = c_int;
pub const GDK_PIXBUF_ERROR_CORRUPT_IMAGE: GdkPixbufError = 0;
pub const GDK_PIXBUF_ERROR_INSUFFICIENT_MEMORY: GdkPixbufError = 1;
pub const GDK_PIXBUF_ERROR_BAD_OPTION: GdkPixbufError = 2;
pub const GDK_PIXBUF_ERROR_UNKNOWN_TYPE: GdkPixbufError = 3;
pub const GDK_PIXBUF_ERROR_UNSUPPORTED_OPERATION: GdkPixbufError = 4;
pub const GDK_PIXBUF_ERROR_FAILED: GdkPixbufError = 5;
pub const GDK_PIXBUF_ERROR_INCOMPLETE_ANIMATION: GdkPixbufError = 6;

pub type GdkPixbufRotation = c_int;
pub const GDK_PIXBUF_ROTATE_NONE: GdkPixbufRotation = 0;
pub const GDK_PIXBUF_ROTATE_COUNTERCLOCKWISE: GdkPixbufRotation = 90;
pub const GDK_PIXBUF_ROTATE_UPSIDEDOWN: GdkPixbufRotation = 180;
pub const GDK_PIXBUF_ROTATE_CLOCKWISE: GdkPixbufRotation = 270;

// Constants

// Callbacks
pub type GdkPixbufDestroyNotify = Option<unsafe extern "C" fn(*mut u8, gpointer)>;
pub type GdkPixbufSaveFunc = Option<unsafe extern "C" fn(*const u8, size_t, *mut *mut glib::GError, gpointer) -> gboolean>;

// Records
#[repr(C)]
pub struct GdkPixbufFormat(c_void);

impl ::std::fmt::Debug for GdkPixbufFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkPixbufFormat @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GdkPixbufLoaderClass {
    pub parent_class: gobject::GObjectClass,
    pub size_prepared: Option<unsafe extern "C" fn(*mut GdkPixbufLoader, c_int, c_int)>,
    pub area_prepared: Option<unsafe extern "C" fn(*mut GdkPixbufLoader)>,
    pub area_updated: Option<unsafe extern "C" fn(*mut GdkPixbufLoader, c_int, c_int, c_int, c_int)>,
    pub closed: Option<unsafe extern "C" fn(*mut GdkPixbufLoader)>,
}

impl ::std::fmt::Debug for GdkPixbufLoaderClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkPixbufLoaderClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .field("size_prepared", &self.size_prepared)
         .field("area_prepared", &self.area_prepared)
         .field("area_updated", &self.area_updated)
         .field("closed", &self.closed)
         .finish()
    }
}

#[repr(C)]
pub struct GdkPixbufSimpleAnimClass(c_void);

impl ::std::fmt::Debug for GdkPixbufSimpleAnimClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkPixbufSimpleAnimClass @ {:?}", self as *const _))
         .finish()
    }
}

// Classes
#[repr(C)]
pub struct GdkPixbuf(c_void);

impl ::std::fmt::Debug for GdkPixbuf {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkPixbuf @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
pub struct GdkPixbufAnimation(c_void);

impl ::std::fmt::Debug for GdkPixbufAnimation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkPixbufAnimation @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
pub struct GdkPixbufAnimationIter(c_void);

impl ::std::fmt::Debug for GdkPixbufAnimationIter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkPixbufAnimationIter @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GdkPixbufLoader {
    pub parent_instance: gobject::GObject,
    pub priv_: gpointer,
}

impl ::std::fmt::Debug for GdkPixbufLoader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkPixbufLoader @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[repr(C)]
pub struct GdkPixbufSimpleAnim(c_void);

impl ::std::fmt::Debug for GdkPixbufSimpleAnim {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkPixbufSimpleAnim @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
pub struct GdkPixbufSimpleAnimIter(c_void);

impl ::std::fmt::Debug for GdkPixbufSimpleAnimIter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkPixbufSimpleAnimIter @ {:?}", self as *const _))
         .finish()
    }
}

extern "C" {

    //=========================================================================
    // GdkColorspace
    //=========================================================================
    pub fn gdk_colorspace_get_type() -> GType;

    //=========================================================================
    // GdkInterpType
    //=========================================================================
    pub fn gdk_interp_type_get_type() -> GType;

    //=========================================================================
    // GdkPixbufAlphaMode
    //=========================================================================
    pub fn gdk_pixbuf_alpha_mode_get_type() -> GType;

    //=========================================================================
    // GdkPixbufError
    //=========================================================================
    pub fn gdk_pixbuf_error_get_type() -> GType;
    pub fn gdk_pixbuf_error_quark() -> glib::GQuark;

    //=========================================================================
    // GdkPixbufRotation
    //=========================================================================
    pub fn gdk_pixbuf_rotation_get_type() -> GType;

    //=========================================================================
    // GdkPixbufFormat
    //=========================================================================
    pub fn gdk_pixbuf_format_get_type() -> GType;
    pub fn gdk_pixbuf_format_copy(format: *const GdkPixbufFormat) -> *mut GdkPixbufFormat;
    pub fn gdk_pixbuf_format_free(format: *mut GdkPixbufFormat);
    pub fn gdk_pixbuf_format_get_description(format: *mut GdkPixbufFormat) -> *mut c_char;
    pub fn gdk_pixbuf_format_get_extensions(format: *mut GdkPixbufFormat) -> *mut *mut c_char;
    pub fn gdk_pixbuf_format_get_license(format: *mut GdkPixbufFormat) -> *mut c_char;
    pub fn gdk_pixbuf_format_get_mime_types(format: *mut GdkPixbufFormat) -> *mut *mut c_char;
    pub fn gdk_pixbuf_format_get_name(format: *mut GdkPixbufFormat) -> *mut c_char;
    pub fn gdk_pixbuf_format_is_disabled(format: *mut GdkPixbufFormat) -> gboolean;
    #[cfg(any(feature = "v2_36", feature = "dox"))]
    pub fn gdk_pixbuf_format_is_save_option_supported(format: *mut GdkPixbufFormat, option_key: *const c_char) -> gboolean;
    pub fn gdk_pixbuf_format_is_scalable(format: *mut GdkPixbufFormat) -> gboolean;
    pub fn gdk_pixbuf_format_is_writable(format: *mut GdkPixbufFormat) -> gboolean;
    pub fn gdk_pixbuf_format_set_disabled(format: *mut GdkPixbufFormat, disabled: gboolean);

    //=========================================================================
    // GdkPixbuf
    //=========================================================================
    pub fn gdk_pixbuf_get_type() -> GType;
    pub fn gdk_pixbuf_new(colorspace: GdkColorspace, has_alpha: gboolean, bits_per_sample: c_int, width: c_int, height: c_int) -> *mut GdkPixbuf;
    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn gdk_pixbuf_new_from_bytes(data: *mut glib::GBytes, colorspace: GdkColorspace, has_alpha: gboolean, bits_per_sample: c_int, width: c_int, height: c_int, rowstride: c_int) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_data(data: *const u8, colorspace: GdkColorspace, has_alpha: gboolean, bits_per_sample: c_int, width: c_int, height: c_int, rowstride: c_int, destroy_fn: GdkPixbufDestroyNotify, destroy_fn_data: gpointer) -> *mut GdkPixbuf;
    #[cfg(any(windows, feature = "dox"))]
    pub fn gdk_pixbuf_new_from_file_utf8(filename: *const c_char, error: *mut *mut glib::GError) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_file(filename: *const c_char, error: *mut *mut glib::GError) -> *mut GdkPixbuf;
    #[cfg(any(windows, feature = "dox"))]
    pub fn gdk_pixbuf_new_from_file_at_scale_utf8(filename: *const c_char, width: c_int, height: c_int, preserve_aspect_ratio: gboolean, error: *mut *mut glib::GError) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_file_at_scale(filename: *const c_char, width: c_int, height: c_int, preserve_aspect_ratio: gboolean, error: *mut *mut glib::GError) -> *mut GdkPixbuf;
    #[cfg(any(windows, feature = "dox"))]
    pub fn gdk_pixbuf_new_from_file_at_size_utf8(filename: *const c_char, width: c_int, height: c_int, error: *mut *mut glib::GError) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_file_at_size(filename: *const c_char, width: c_int, height: c_int, error: *mut *mut glib::GError) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_inline(data_length: c_int, data: *const u8, copy_pixels: gboolean, error: *mut *mut glib::GError) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_resource(resource_path: *const c_char, error: *mut *mut glib::GError) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_resource_at_scale(resource_path: *const c_char, width: c_int, height: c_int, preserve_aspect_ratio: gboolean, error: *mut *mut glib::GError) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_stream(stream: *mut gio::GInputStream, cancellable: *mut gio::GCancellable, error: *mut *mut glib::GError) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_stream_at_scale(stream: *mut gio::GInputStream, width: c_int, height: c_int, preserve_aspect_ratio: gboolean, cancellable: *mut gio::GCancellable, error: *mut *mut glib::GError) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_stream_finish(async_result: *mut gio::GAsyncResult, error: *mut *mut glib::GError) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_xpm_data(data: *mut *const c_char) -> *mut GdkPixbuf;
    #[cfg(any(feature = "v2_36_8", feature = "dox"))]
    pub fn gdk_pixbuf_calculate_rowstride(colorspace: GdkColorspace, has_alpha: gboolean, bits_per_sample: c_int, width: c_int, height: c_int) -> c_int;
    pub fn gdk_pixbuf_get_file_info(filename: *const c_char, width: *mut c_int, height: *mut c_int) -> *mut GdkPixbufFormat;
    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn gdk_pixbuf_get_file_info_async(filename: *const c_char, cancellable: *mut gio::GCancellable, callback: gio::GAsyncReadyCallback, user_data: gpointer);
    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn gdk_pixbuf_get_file_info_finish(async_result: *mut gio::GAsyncResult, width: *mut c_int, height: *mut c_int, error: *mut *mut glib::GError) -> *mut GdkPixbufFormat;
    pub fn gdk_pixbuf_get_formats() -> *mut glib::GSList;
    pub fn gdk_pixbuf_new_from_stream_async(stream: *mut gio::GInputStream, cancellable: *mut gio::GCancellable, callback: gio::GAsyncReadyCallback, user_data: gpointer);
    pub fn gdk_pixbuf_new_from_stream_at_scale_async(stream: *mut gio::GInputStream, width: c_int, height: c_int, preserve_aspect_ratio: gboolean, cancellable: *mut gio::GCancellable, callback: gio::GAsyncReadyCallback, user_data: gpointer);
    pub fn gdk_pixbuf_save_to_stream_finish(async_result: *mut gio::GAsyncResult, error: *mut *mut glib::GError) -> gboolean;
    pub fn gdk_pixbuf_add_alpha(pixbuf: *const GdkPixbuf, substitute_color: gboolean, r: c_uchar, g: c_uchar, b: c_uchar) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_apply_embedded_orientation(src: *mut GdkPixbuf) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_composite(src: *const GdkPixbuf, dest: *mut GdkPixbuf, dest_x: c_int, dest_y: c_int, dest_width: c_int, dest_height: c_int, offset_x: c_double, offset_y: c_double, scale_x: c_double, scale_y: c_double, interp_type: GdkInterpType, overall_alpha: c_int);
    pub fn gdk_pixbuf_composite_color(src: *const GdkPixbuf, dest: *mut GdkPixbuf, dest_x: c_int, dest_y: c_int, dest_width: c_int, dest_height: c_int, offset_x: c_double, offset_y: c_double, scale_x: c_double, scale_y: c_double, interp_type: GdkInterpType, overall_alpha: c_int, check_x: c_int, check_y: c_int, check_size: c_int, color1: u32, color2: u32);
    pub fn gdk_pixbuf_composite_color_simple(src: *const GdkPixbuf, dest_width: c_int, dest_height: c_int, interp_type: GdkInterpType, overall_alpha: c_int, check_size: c_int, color1: u32, color2: u32) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_copy(pixbuf: *const GdkPixbuf) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_copy_area(src_pixbuf: *const GdkPixbuf, src_x: c_int, src_y: c_int, width: c_int, height: c_int, dest_pixbuf: *mut GdkPixbuf, dest_x: c_int, dest_y: c_int);
    #[cfg(any(feature = "v2_36", feature = "dox"))]
    pub fn gdk_pixbuf_copy_options(src_pixbuf: *mut GdkPixbuf, dest_pixbuf: *mut GdkPixbuf) -> gboolean;
    pub fn gdk_pixbuf_fill(pixbuf: *mut GdkPixbuf, pixel: u32);
    pub fn gdk_pixbuf_flip(src: *const GdkPixbuf, horizontal: gboolean) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_get_bits_per_sample(pixbuf: *const GdkPixbuf) -> c_int;
    pub fn gdk_pixbuf_get_byte_length(pixbuf: *const GdkPixbuf) -> size_t;
    pub fn gdk_pixbuf_get_colorspace(pixbuf: *const GdkPixbuf) -> GdkColorspace;
    pub fn gdk_pixbuf_get_has_alpha(pixbuf: *const GdkPixbuf) -> gboolean;
    pub fn gdk_pixbuf_get_height(pixbuf: *const GdkPixbuf) -> c_int;
    pub fn gdk_pixbuf_get_n_channels(pixbuf: *const GdkPixbuf) -> c_int;
    pub fn gdk_pixbuf_get_option(pixbuf: *mut GdkPixbuf, key: *const c_char) -> *const c_char;
    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn gdk_pixbuf_get_options(pixbuf: *mut GdkPixbuf) -> *mut glib::GHashTable;
    pub fn gdk_pixbuf_get_pixels(pixbuf: *const GdkPixbuf) -> *mut u8;
    pub fn gdk_pixbuf_get_pixels_with_length(pixbuf: *const GdkPixbuf, length: *mut c_uint) -> *mut u8;
    pub fn gdk_pixbuf_get_rowstride(pixbuf: *const GdkPixbuf) -> c_int;
    pub fn gdk_pixbuf_get_width(pixbuf: *const GdkPixbuf) -> c_int;
    pub fn gdk_pixbuf_new_subpixbuf(src_pixbuf: *mut GdkPixbuf, src_x: c_int, src_y: c_int, width: c_int, height: c_int) -> *mut GdkPixbuf;
    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn gdk_pixbuf_read_pixel_bytes(pixbuf: *const GdkPixbuf) -> *mut glib::GBytes;
    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn gdk_pixbuf_read_pixels(pixbuf: *const GdkPixbuf) -> *const u8;
    pub fn gdk_pixbuf_ref(pixbuf: *mut GdkPixbuf) -> *mut GdkPixbuf;
    #[cfg(any(feature = "v2_36", feature = "dox"))]
    pub fn gdk_pixbuf_remove_option(pixbuf: *mut GdkPixbuf, key: *const c_char) -> gboolean;
    pub fn gdk_pixbuf_rotate_simple(src: *const GdkPixbuf, angle: GdkPixbufRotation) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_saturate_and_pixelate(src: *const GdkPixbuf, dest: *mut GdkPixbuf, saturation: c_float, pixelate: gboolean);
    #[cfg(any(windows, feature = "dox"))]
    pub fn gdk_pixbuf_save_utf8(pixbuf: *mut GdkPixbuf, filename: *const c_char, type_: *const c_char, error: *mut *mut glib::GError, ...) -> gboolean;
    pub fn gdk_pixbuf_save(pixbuf: *mut GdkPixbuf, filename: *const c_char, type_: *const c_char, error: *mut *mut glib::GError, ...) -> gboolean;
    pub fn gdk_pixbuf_save_to_buffer(pixbuf: *mut GdkPixbuf, buffer: *mut *mut u8, buffer_size: *mut size_t, type_: *const c_char, error: *mut *mut glib::GError, ...) -> gboolean;
    pub fn gdk_pixbuf_save_to_bufferv(pixbuf: *mut GdkPixbuf, buffer: *mut *mut u8, buffer_size: *mut size_t, type_: *const c_char, option_keys: *mut *mut c_char, option_values: *mut *mut c_char, error: *mut *mut glib::GError) -> gboolean;
    pub fn gdk_pixbuf_save_to_callback(pixbuf: *mut GdkPixbuf, save_func: GdkPixbufSaveFunc, user_data: gpointer, type_: *const c_char, error: *mut *mut glib::GError, ...) -> gboolean;
    pub fn gdk_pixbuf_save_to_callbackv(pixbuf: *mut GdkPixbuf, save_func: GdkPixbufSaveFunc, user_data: gpointer, type_: *const c_char, option_keys: *mut *mut c_char, option_values: *mut *mut c_char, error: *mut *mut glib::GError) -> gboolean;
    pub fn gdk_pixbuf_save_to_stream(pixbuf: *mut GdkPixbuf, stream: *mut gio::GOutputStream, type_: *const c_char, cancellable: *mut gio::GCancellable, error: *mut *mut glib::GError, ...) -> gboolean;
    pub fn gdk_pixbuf_save_to_stream_async(pixbuf: *mut GdkPixbuf, stream: *mut gio::GOutputStream, type_: *const c_char, cancellable: *mut gio::GCancellable, callback: gio::GAsyncReadyCallback, user_data: gpointer, ...);
    #[cfg(any(feature = "v2_36", feature = "dox"))]
    pub fn gdk_pixbuf_save_to_streamv(pixbuf: *mut GdkPixbuf, stream: *mut gio::GOutputStream, type_: *const c_char, option_keys: *mut *mut c_char, option_values: *mut *mut c_char, cancellable: *mut gio::GCancellable, error: *mut *mut glib::GError) -> gboolean;
    #[cfg(any(feature = "v2_36", feature = "dox"))]
    pub fn gdk_pixbuf_save_to_streamv_async(pixbuf: *mut GdkPixbuf, stream: *mut gio::GOutputStream, type_: *const c_char, option_keys: *mut *mut c_char, option_values: *mut *mut c_char, cancellable: *mut gio::GCancellable, callback: gio::GAsyncReadyCallback, user_data: gpointer);
    #[cfg(any(windows, feature = "dox"))]
    pub fn gdk_pixbuf_savev_utf8(pixbuf: *mut GdkPixbuf, filename: *const c_char, type_: *const c_char, option_keys: *mut *mut c_char, option_values: *mut *mut c_char, error: *mut *mut glib::GError) -> gboolean;
    pub fn gdk_pixbuf_savev(pixbuf: *mut GdkPixbuf, filename: *const c_char, type_: *const c_char, option_keys: *mut *mut c_char, option_values: *mut *mut c_char, error: *mut *mut glib::GError) -> gboolean;
    pub fn gdk_pixbuf_scale(src: *const GdkPixbuf, dest: *mut GdkPixbuf, dest_x: c_int, dest_y: c_int, dest_width: c_int, dest_height: c_int, offset_x: c_double, offset_y: c_double, scale_x: c_double, scale_y: c_double, interp_type: GdkInterpType);
    pub fn gdk_pixbuf_scale_simple(src: *const GdkPixbuf, dest_width: c_int, dest_height: c_int, interp_type: GdkInterpType) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_set_option(pixbuf: *mut GdkPixbuf, key: *const c_char, value: *const c_char) -> gboolean;
    pub fn gdk_pixbuf_unref(pixbuf: *mut GdkPixbuf);

    //=========================================================================
    // GdkPixbufAnimation
    //=========================================================================
    pub fn gdk_pixbuf_animation_get_type() -> GType;
    #[cfg(any(windows, feature = "dox"))]
    pub fn gdk_pixbuf_animation_new_from_file_utf8(filename: *const c_char, error: *mut *mut glib::GError) -> *mut GdkPixbufAnimation;
    pub fn gdk_pixbuf_animation_new_from_file(filename: *const c_char, error: *mut *mut glib::GError) -> *mut GdkPixbufAnimation;
    pub fn gdk_pixbuf_animation_new_from_resource(resource_path: *const c_char, error: *mut *mut glib::GError) -> *mut GdkPixbufAnimation;
    pub fn gdk_pixbuf_animation_new_from_stream(stream: *mut gio::GInputStream, cancellable: *mut gio::GCancellable, error: *mut *mut glib::GError) -> *mut GdkPixbufAnimation;
    pub fn gdk_pixbuf_animation_new_from_stream_finish(async_result: *mut gio::GAsyncResult, error: *mut *mut glib::GError) -> *mut GdkPixbufAnimation;
    pub fn gdk_pixbuf_animation_new_from_stream_async(stream: *mut gio::GInputStream, cancellable: *mut gio::GCancellable, callback: gio::GAsyncReadyCallback, user_data: gpointer);
    pub fn gdk_pixbuf_animation_get_height(animation: *mut GdkPixbufAnimation) -> c_int;
    pub fn gdk_pixbuf_animation_get_iter(animation: *mut GdkPixbufAnimation, start_time: *const glib::GTimeVal) -> *mut GdkPixbufAnimationIter;
    pub fn gdk_pixbuf_animation_get_static_image(animation: *mut GdkPixbufAnimation) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_animation_get_width(animation: *mut GdkPixbufAnimation) -> c_int;
    pub fn gdk_pixbuf_animation_is_static_image(animation: *mut GdkPixbufAnimation) -> gboolean;
    pub fn gdk_pixbuf_animation_ref(animation: *mut GdkPixbufAnimation) -> *mut GdkPixbufAnimation;
    pub fn gdk_pixbuf_animation_unref(animation: *mut GdkPixbufAnimation);

    //=========================================================================
    // GdkPixbufAnimationIter
    //=========================================================================
    pub fn gdk_pixbuf_animation_iter_get_type() -> GType;
    pub fn gdk_pixbuf_animation_iter_advance(iter: *mut GdkPixbufAnimationIter, current_time: *const glib::GTimeVal) -> gboolean;
    pub fn gdk_pixbuf_animation_iter_get_delay_time(iter: *mut GdkPixbufAnimationIter) -> c_int;
    pub fn gdk_pixbuf_animation_iter_get_pixbuf(iter: *mut GdkPixbufAnimationIter) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_animation_iter_on_currently_loading_frame(iter: *mut GdkPixbufAnimationIter) -> gboolean;

    //=========================================================================
    // GdkPixbufLoader
    //=========================================================================
    pub fn gdk_pixbuf_loader_get_type() -> GType;
    pub fn gdk_pixbuf_loader_new() -> *mut GdkPixbufLoader;
    pub fn gdk_pixbuf_loader_new_with_mime_type(mime_type: *const c_char, error: *mut *mut glib::GError) -> *mut GdkPixbufLoader;
    pub fn gdk_pixbuf_loader_new_with_type(image_type: *const c_char, error: *mut *mut glib::GError) -> *mut GdkPixbufLoader;
    pub fn gdk_pixbuf_loader_close(loader: *mut GdkPixbufLoader, error: *mut *mut glib::GError) -> gboolean;
    pub fn gdk_pixbuf_loader_get_animation(loader: *mut GdkPixbufLoader) -> *mut GdkPixbufAnimation;
    pub fn gdk_pixbuf_loader_get_format(loader: *mut GdkPixbufLoader) -> *mut GdkPixbufFormat;
    pub fn gdk_pixbuf_loader_get_pixbuf(loader: *mut GdkPixbufLoader) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_loader_set_size(loader: *mut GdkPixbufLoader, width: c_int, height: c_int);
    pub fn gdk_pixbuf_loader_write(loader: *mut GdkPixbufLoader, buf: *const u8, count: size_t, error: *mut *mut glib::GError) -> gboolean;
    pub fn gdk_pixbuf_loader_write_bytes(loader: *mut GdkPixbufLoader, buffer: *mut glib::GBytes, error: *mut *mut glib::GError) -> gboolean;

    //=========================================================================
    // GdkPixbufSimpleAnim
    //=========================================================================
    pub fn gdk_pixbuf_simple_anim_get_type() -> GType;
    pub fn gdk_pixbuf_simple_anim_new(width: c_int, height: c_int, rate: c_float) -> *mut GdkPixbufSimpleAnim;
    pub fn gdk_pixbuf_simple_anim_add_frame(animation: *mut GdkPixbufSimpleAnim, pixbuf: *mut GdkPixbuf);
    pub fn gdk_pixbuf_simple_anim_get_loop(animation: *mut GdkPixbufSimpleAnim) -> gboolean;
    pub fn gdk_pixbuf_simple_anim_set_loop(animation: *mut GdkPixbufSimpleAnim, loop_: gboolean);

    //=========================================================================
    // GdkPixbufSimpleAnimIter
    //=========================================================================
    pub fn gdk_pixbuf_simple_anim_iter_get_type() -> GType;

}
