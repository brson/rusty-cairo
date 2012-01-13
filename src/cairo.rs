#[link(name = "cairo", vers = "0.2", author = "Zack0Wack0")];
#[crate_type = "lib"];

use std;
import std::{fs, tempfile};
import core::{ptr, vec, str, ctypes};

export 
	get_version,
	get_cairo_version,
	
	STATUS_SUCCESS,
	STATUS_NO_MEMORY,
	STATUS_INVALID_RESTORE,
	STATUS_INVALID_POP_GROUP,
	STATUS_NO_CURRENT_POINT,
	STATUS_INVALID_MATRIX,
	STATUS_INVALID_STATUS,
	STATUS_NULL_POINTER,
	STATUS_INVALID_STRING,
	STATUS_INVALID_PATH_DATA,
	STATUS_READ_ERROR,
	STATUS_WRITE_ERROR,
	STATUS_SURFACE_FINISHED,
	STATUS_SURFACE_TYPE_MISMATCH,
	STATUS_PATTERN_TYPE_MISMATCH,
	STATUS_INVALID_CONTENT,
	STATUS_INVALID_FORMAT,
	STATUS_INVALID_VISUAL,
	STATUS_FILE_NOT_FOUND,
	STATUS_INVALID_DASH,
	STATUS_INVALID_DSC_COMMENT,
	STATUS_INVALID_INDEX,
	STATUS_CLIP_NOT_REPRESENTABLE,
	STATUS_TEMP_FILE_ERROR,
	STATUS_INVALID_STRIDE,
	STATUS_FONT_TYPE_MISMATCH,
	STATUS_USER_FONT_IMMUTABLE,
	STATUS_USER_FONT_ERROR,
	STATUS_NEGATIVE_COUNT,
	STATUS_INVALID_CLUSTERS,
	STATUS_INVALID_SLANT,
	STATUS_INVALID_WEIGHT,
	STATUS_INVALID_SIZE,
	STATUS_USER_FONT_NOT_IMPLEMENTED,
	STATUS_DEVICE_TYPE_MISMATCH,
	STATUS_DEVICE_ERROR,
	STATUS_LAST_STATUS,
	status,
	status_to_str,

	FORMAT_INVALID,
	FORMAT_ARGB32,
	FORMAT_RGB24,
	FORMAT_A8,
	FORMAT_A1,
	FORMAT_RGB16_565,
	format,
	format_stride_for_width,

	FONT_SLANT_NORMAL,
	FONT_SLANT_ITALIC,
	FONT_SLANT_OBLIQUE,
	font_slant,

	FONT_WEIGHT_NORMAL,
	FONT_WEIGHT_BOLD,
	font_weight,

	ANTIALIAS_DEFAULT,
	ANTIALIAS_NONE,
	ANTIALIAS_GRAY,
	ANTIALIAS_SUBPIXEL,
	antialias,

	FILL_RULE_WINDING,
	FILL_RULE_EVEN_ODD,
	fill_rule,

	LINE_CAP_BUTT,
	LINE_CAP_ROUND,
	LINE_CAP_SQUARE,
	line_cap,

	LINE_JOIN_MITER,
	LINE_JOIN_ROUND,
	LINE_JOIN_BEVEL,
	line_join,

	OPERATOR_CLEAR,
	OPERATOR_SOURCE,
	OPERATOR_OVER,
	OPERATOR_IN,
	OPERATOR_OUT,
	OPERATOR_ATOP,
	OPERATOR_DEST,
	OPERATOR_DEST_OVER,
	OPERATOR_DEST_IN,
	OPERATOR_DEST_OUT,
	OPERATOR_DEST_ATOP,
	OPERATOR_XOR,
	OPERATOR_ADD,
	OPERATOR_SATURATE,
	OPERATOR_MULTIPLY,
	OPERATOR_SCREEN,
	OPERATOR_OVERLAY,
	OPERATOR_DARKEN,
	OPERATOR_LIGHTEN,
	OPERATOR_COLOR_DODGE,
	OPERATOR_COLOR_BURN,
	OPERATOR_HARD_LIGHT,
	OPERATOR_SOFT_LIGHT,
	OPERATOR_DIFFERENCE,
	OPERATOR_EXCLUSION,
	OPERATOR_HSL_HUE,
	OPERATOR_HSL_SATURATION,
	OPERATOR_HSL_COLOR,
	OPERATOR_HSL_LUMINOSITY,
	operator,

	CONTENT_COLOR,
	CONTENT_ALPHA,
	CONTENT_COLOR_ALPHA,
	content,

	SURFACE_TYPE_IMAGE,
	SURFACE_TYPE_PDF,
	SURFACE_TYPE_PS,
	SURFACE_TYPE_XLIB,
	SURFACE_TYPE_XCB,
	SURFACE_TYPE_GLITZ,
	SURFACE_TYPE_QUARTZ,
	SURFACE_TYPE_WIN32,
	SURFACE_TYPE_BEOS,
	SURFACE_TYPE_DIRECTFB,
	SURFACE_TYPE_SVG,
	SURFACE_TYPE_OS2,
	SURFACE_TYPE_WIN32_PRINTING,
	SURFACE_TYPE_QUARTZ_IMAGE,
	SURFACE_TYPE_SCRIPT,
	SURFACE_TYPE_QT,
	SURFACE_TYPE_RECORDING,
	SURFACE_TYPE_VG,
	SURFACE_TYPE_GL,
	SURFACE_TYPE_DRM,
	SURFACE_TYPE_TEE,
	SURFACE_TYPE_XML,
	SURFACE_TYPE_SKIA,
	SURFACE_TYPE_SUBSURFACE,
	surface_type,

	TEXT_CLUSTER_FLAG_BACKWARD,
	text_cluster_flags,

	SUBPIXEL_ORDER_DEFAULT,
	SUBPIXEL_ORDER_RGB,
	SUBPIXEL_ORDER_BGR,
	SUBPIXEL_ORDER_VRGB,
	SUBPIXEL_ORDER_VBGR,
	subpixel_order,

	HINT_STYLE_DEFAULT,
	HINT_STYLE_NONE,
	HINT_STYLE_SLIGHT,
	HINT_STYLE_MEDIUM,
	HINT_STYLE_FULL,
	hint_style,

	HINT_METRICS_DEFAULT,
	HINT_METRICS_OFF,
	HINT_METRICS_ON,

	hint_metrics,

	FONT_TYPE_TOY,
	FONT_TYPE_FT,
	FONT_TYPE_WIN32,
	FONT_TYPE_QUARTZ,
	FONT_TYPE_USER,
	font_type,

	font_extents_record,
	text_extents_record,
	matrix_record,
	glyph_record,
	text_cluster_record,

	DEVICE_TYPE_DRM,
	DEVICE_TYPE_GL,
	DEVICE_TYPE_SCRIPT,
	DEVICE_TYPE_XCB,
	DEVICE_TYPE_XLIB,
	DEVICE_TYPE_XML,
	device_type,

	EXTEND_NONE,
	EXTEND_REPEAT,
	EXTEND_REFLECT,
	EXTEND_PAD,
	extend,

	FILTER_FAST,
	FILTER_GOOD,
	FILTER_BEST,
	FILTER_NEAREST,
	FILTER_BILINEAR,
	FILTER_GAUSSIAN,
	filter,

	PATTERN_TYPE_SOLID,
	PATTERN_TYPE_SURFACE,
	PATTERN_TYPE_LINEAR,
	PATTERN_TYPE_RADIAL,
	pattern_type,

	device,
	surface,
	mk_surface_from_similar,
	mk_image_surface,
	mk_image_surface_from_file,
	mk_image_surface_from_data,
	mk_pdf_surface,
	mk_svg_surface,
	mk_ps_surface,

	pattern,
	mk_pattern_from_rgb,
	mk_pattern_from_rgba,
	mk_pattern_from_linear_gradient,
	mk_pattern_from_radial_gradient,
	mk_pattern_from_surface,

	matrix,
	mk_matrix,

	path,

	glyph,
	mk_glyph,

	text_cluster,
	mk_text_cluster,

	font_extents,
	text_extents,

	font_face,
	mk_font_face_from_toy_font,
	mk_font_face_from_file,

	scaled_font,

	font_options,
	mk_font_options,
	mk_font_options_from_copy,

	context,
	mk_context;

/*
 * FIXME all wrapped objects need to be cached rather than 
 * recreating new objects every time for each **self cairo pointer
 *
 * TODO more image formats
 
 * FIXME make special surfaces output by write_to_file
 * rather than providing output file in constructor
 * (make a temp file and then copy file when outputting? only other way is to use cairo streams)
 
 * TODO possibly make mk_svg_surface_from_file, using libsvg-cairo (I think it supports this)
 
 * FIXME ensure all memory referencing is correct
 */

#[nolink]
#[abi = "cdecl"]
native mod c {
	fn fopen(path: *u8, mode: *u8) -> core::ctypes::intptr_t;
	fn fclose(file: core::ctypes::intptr_t);
}

#[link_name = "freetype"]
#[abi = "cdecl"]
native mod cft {
	fn FT_Init_FreeType(library: *core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn FT_New_Face(library: core::ctypes::intptr_t, path: *u8, offset: core::ctypes::long, face: *core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn FT_Done_Face(face: core::ctypes::intptr_t);
	fn FT_Done_FreeType(face: core::ctypes::intptr_t);
}

#[link_name = "cairo"]
#[abi = "cdecl"]
native mod ccairo {
	fn cairo_ft_font_face_create_for_ft_face(ft_face: core::ctypes::intptr_t, flags: core::ctypes::c_int) -> core::ctypes::intptr_t;
	fn cairo_font_face_set_user_data(face: core::ctypes::intptr_t, key: core::ctypes::intptr_t, ft_face: core::ctypes::intptr_t, cb: core::ctypes::intptr_t);	

	fn cairo_version_string() -> *u8;
	fn cairo_create(surface: core::ctypes::intptr_t) -> core::ctypes::intptr_t;
	fn cairo_reference(context: core::ctypes::intptr_t) -> core::ctypes::intptr_t;
	fn cairo_destroy(context: core::ctypes::intptr_t);
	fn cairo_status(context: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_save(context: core::ctypes::intptr_t);
	fn cairo_restore(context: core::ctypes::intptr_t);
	fn cairo_get_target(context: core::ctypes::intptr_t) -> core::ctypes::intptr_t;
	fn cairo_push_group(context: core::ctypes::intptr_t);
	fn cairo_push_group_with_content(context: core::ctypes::intptr_t, content: core::ctypes::c_int);
	fn cairo_pop_group(context: core::ctypes::intptr_t) -> core::ctypes::intptr_t;
	fn cairo_pop_group_to_source(context: core::ctypes::intptr_t);
	fn cairo_get_group_target(context: core::ctypes::intptr_t) -> core::ctypes::intptr_t;
	fn cairo_set_source_rgb(context: core::ctypes::intptr_t, red: f64, green: f64, blue: f64);
	fn cairo_set_source_rgba(context: core::ctypes::intptr_t, red: f64, green: f64, blue: f64, alpha: f64);
	fn cairo_set_source(context: core::ctypes::intptr_t, pattern: core::ctypes::intptr_t);
	fn cairo_set_source_surface(context: core::ctypes::intptr_t, surface: core::ctypes::intptr_t, x: f64, y: f64);
	fn cairo_get_source(context: core::ctypes::intptr_t) -> core::ctypes::intptr_t;
	fn cairo_set_antialias(context: core::ctypes::intptr_t, antialias: core::ctypes::c_int);
	fn cairo_get_antialias(context: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_set_dash(context: core::ctypes::intptr_t, dashes: *f64, num_dashes: core::ctypes::c_int, offset: f64);
	fn cairo_get_dash_count(context: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_get_dash(context: core::ctypes::intptr_t, dashes: *f64, offset: *f64);
	fn cairo_set_fill_rule(context: core::ctypes::intptr_t, rule: core::ctypes::c_int);
	fn cairo_get_fill_rule(context: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_set_line_cap(context: core::ctypes::intptr_t, cap: core::ctypes::c_int);
	fn cairo_get_line_cap(context: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_set_line_join(context: core::ctypes::intptr_t, join: core::ctypes::c_int);
	fn cairo_get_line_join(context: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_set_line_width(context: core::ctypes::intptr_t, width: f64);
	fn cairo_get_line_width(context: core::ctypes::intptr_t) -> f64;
	fn cairo_set_miter_limit(context: core::ctypes::intptr_t, limit: f64);
	fn cairo_get_miter_limit(context: core::ctypes::intptr_t) -> f64;
	fn cairo_set_operator(context: core::ctypes::intptr_t, join: core::ctypes::c_int);
	fn cairo_get_operator(context: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_set_tolerance(context: core::ctypes::intptr_t, limit: f64);
	fn cairo_get_tolerance(context: core::ctypes::intptr_t) -> f64;
	fn cairo_clip(context: core::ctypes::intptr_t);
	fn cairo_clip_preserve(context: core::ctypes::intptr_t);
	fn cairo_clip_extents(context: core::ctypes::intptr_t, x1: *f64, y1: *f64, x2: *f64, y2: *f64);
	fn cairo_in_clip(context: core::ctypes::intptr_t, x: f64, y: f64) -> core::ctypes::c_int;
	fn cairo_reset_clip(context: core::ctypes::intptr_t);
	fn cairo_fill(context: core::ctypes::intptr_t);
	fn cairo_fill_preserve(context: core::ctypes::intptr_t);
	fn cairo_fill_extents(context: core::ctypes::intptr_t, x1: *f64, y1: *f64, x2: *f64, y2: *f64);
	fn cairo_in_fill(context: core::ctypes::intptr_t, x: f64, y: f64) -> core::ctypes::c_int;
	fn cairo_mask(context: core::ctypes::intptr_t, pattern: core::ctypes::intptr_t);
	fn cairo_mask_surface(context: core::ctypes::intptr_t, surface: core::ctypes::intptr_t, surface_x: f64, surface_y: f64);
	fn cairo_paint(context: core::ctypes::intptr_t);
	fn cairo_paint_with_alpha(context: core::ctypes::intptr_t, alpha: f64);
	fn cairo_stroke(context: core::ctypes::intptr_t);
	fn cairo_stroke_preserve(context: core::ctypes::intptr_t);
	fn cairo_stroke_extents(context: core::ctypes::intptr_t, x1: *f64, y1: *f64, x2: *f64, y2: *f64);
	fn cairo_in_stroke(context: core::ctypes::intptr_t, x: f64, y: f64) -> core::ctypes::c_int;
	fn cairo_copy_page(context: core::ctypes::intptr_t);
	fn cairo_show_page(context: core::ctypes::intptr_t);
	fn cairo_get_reference_count(context: core::ctypes::intptr_t) -> core::ctypes::c_uint;
	
	fn cairo_translate(context: core::ctypes::intptr_t, x: f64, y: f64);
	fn cairo_scale(context: core::ctypes::intptr_t, x: f64, y: f64);
	fn cairo_rotate(context: core::ctypes::intptr_t, angle: f64);
	fn cairo_transform(context: core::ctypes::intptr_t, matrix: core::ctypes::intptr_t);
	fn cairo_set_matrix(context: core::ctypes::intptr_t, matrix: core::ctypes::intptr_t);
	fn cairo_get_matrix(context: core::ctypes::intptr_t) -> core::ctypes::intptr_t;
	fn cairo_identity_matrix(context: core::ctypes::intptr_t);
	fn cairo_user_to_device(context: core::ctypes::intptr_t, x: *f64, y: *f64);
	fn cairo_user_to_device_distance(context: core::ctypes::intptr_t, x: *f64, y: *f64);
	fn cairo_device_to_user(context: core::ctypes::intptr_t, x: *f64, y: *f64);
	fn cairo_device_to_user_distance(context: core::ctypes::intptr_t, x: *f64, y: *f64);
	
	fn cairo_copy_path(context: core::ctypes::intptr_t) -> core::ctypes::intptr_t;
	fn cairo_copy_path_flat(context: core::ctypes::intptr_t) -> core::ctypes::intptr_t;
	fn cairo_path_destroy(path: core::ctypes::intptr_t);
	fn cairo_append_path(context: core::ctypes::intptr_t, path: core::ctypes::intptr_t);
	fn cairo_has_current_point(context: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_get_current_point(context: core::ctypes::intptr_t, x: *f64, y: *f64);
	fn cairo_new_path(context: core::ctypes::intptr_t);
	fn cairo_new_sub_path(context: core::ctypes::intptr_t);
	fn cairo_close_path(context: core::ctypes::intptr_t);
	fn cairo_arc(context: core::ctypes::intptr_t, x: f64, y: f64, radius: f64, angle1: f64, angle2: f64);
	fn cairo_arc_negative(context: core::ctypes::intptr_t, x: f64, y: f64, radius: f64, angle1: f64, angle2: f64);
	fn cairo_curve_to(context: core::ctypes::intptr_t, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64);
	fn cairo_line_to(context: core::ctypes::intptr_t, x: f64, y: f64);
	fn cairo_move_to(context: core::ctypes::intptr_t, x: f64, y: f64);
	fn cairo_rectangle(context: core::ctypes::intptr_t, x: f64, y: f64, width: f64, height: f64);
	fn cairo_glyph_path(context: core::ctypes::intptr_t, glyph: core::ctypes::intptr_t, num_glyphs: core::ctypes::c_int);
	fn cairo_text_path(context: core::ctypes::intptr_t, text: *u8);
	fn cairo_rel_curve_to(context: core::ctypes::intptr_t, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64);
	fn cairo_rel_line_to(context: core::ctypes::intptr_t, x: f64, y: f64);
	fn cairo_rel_move_to(context: core::ctypes::intptr_t, x: f64, y: f64);
	fn cairo_path_extents(context: core::ctypes::intptr_t, x1: *f64, y1: *f64, x2: *f64, y2: *f64);
	
	fn cairo_select_font_face(context: core::ctypes::intptr_t, face: *u8, slant: core::ctypes::c_int, weight: core::ctypes::c_int);
	fn cairo_set_font_size(context: core::ctypes::intptr_t, size: f64);
	fn cairo_set_font_matrix(context: core::ctypes::intptr_t, matrix: core::ctypes::intptr_t);
	fn cairo_get_font_matrix(context: core::ctypes::intptr_t, matrix: *core::ctypes::intptr_t);
	fn cairo_set_font_options(context: core::ctypes::intptr_t, options: core::ctypes::intptr_t);
	fn cairo_get_font_options(context: core::ctypes::intptr_t, options: *core::ctypes::intptr_t);
	fn cairo_set_font_face(context: core::ctypes::intptr_t, face: core::ctypes::intptr_t);
	fn cairo_get_font_face(context: core::ctypes::intptr_t) -> core::ctypes::intptr_t;
	fn cairo_set_scaled_font(context: core::ctypes::intptr_t, font: core::ctypes::intptr_t);
	fn cairo_get_scaled_font(context: core::ctypes::intptr_t) -> core::ctypes::intptr_t;
	fn cairo_show_text(context: core::ctypes::intptr_t, text: *u8);
	fn cairo_show_glyphs(context: core::ctypes::intptr_t, glyphs: core::ctypes::intptr_t, num_glyphs: core::ctypes::c_int);
	fn cairo_show_text_glyphs(context: core::ctypes::intptr_t, text: *u8, text_len: core::ctypes::c_int, glyphs: core::ctypes::intptr_t, num_glyphs: core::ctypes::c_int, clusters: core::ctypes::intptr_t, num_clusters: core::ctypes::c_int, cluster_flags: core::ctypes::c_int);
	fn cairo_font_extents(context: core::ctypes::intptr_t, extents: core::ctypes::intptr_t);
	fn cairo_text_extents(context: core::ctypes::intptr_t, text: *u8, extents: core::ctypes::intptr_t);
	fn cairo_glyph_extents(context: core::ctypes::intptr_t, glyphs: core::ctypes::intptr_t, num_glyphs: core::ctypes::c_int, extents: core::ctypes::intptr_t);
	fn cairo_toy_font_face_create(family: *u8, slant: core::ctypes::c_int, weight: core::ctypes::c_int) -> core::ctypes::intptr_t;
	fn cairo_toy_font_face_get_family(face: core::ctypes::intptr_t) -> *u8;
	fn cairo_toy_font_face_get_slant(face: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_toy_font_face_get_weight(face: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_glyph_allocate(num_glyphs: core::ctypes::c_int) -> core::ctypes::intptr_t;
	fn cairo_glyph_free(glyphs: core::ctypes::intptr_t);
	fn cairo_text_cluster_allocate(num_clusters: core::ctypes::c_int) -> core::ctypes::intptr_t;
	fn cairo_text_cluster_free(clusters: core::ctypes::intptr_t);
	
	fn cairo_font_options_create() -> core::ctypes::intptr_t;
	fn cairo_font_options_copy(options: core::ctypes::intptr_t) -> core::ctypes::intptr_t;
	fn cairo_font_options_destroy(options: core::ctypes::intptr_t);
	fn cairo_font_options_status(options: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_font_options_merge(options: core::ctypes::intptr_t, other: core::ctypes::intptr_t);
	fn cairo_font_options_hash(options: core::ctypes::intptr_t) -> core::ctypes::ulong;
	fn cairo_font_options_equal(options: core::ctypes::intptr_t, other: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_font_options_set_antialias(options: core::ctypes::intptr_t, value: core::ctypes::c_int);
	fn cairo_font_options_get_antialias(options: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_font_options_set_subpixel_order(options: core::ctypes::intptr_t, value: core::ctypes::c_int);
	fn cairo_font_options_get_subpixel_order(options: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_font_options_set_hint_style(options: core::ctypes::intptr_t, value: core::ctypes::c_int);
	fn cairo_font_options_get_hint_style(options: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_font_options_set_hint_metrics(options: core::ctypes::intptr_t, value: core::ctypes::c_int);
	fn cairo_font_options_get_hint_metrics(options: core::ctypes::intptr_t) -> core::ctypes::c_int;
	
	fn cairo_font_face_reference(face: core::ctypes::intptr_t) -> core::ctypes::intptr_t;
	fn cairo_font_face_destroy(face: core::ctypes::intptr_t);
	fn cairo_font_face_status(face: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_font_face_get_type(face: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_font_face_get_reference_count(face: core::ctypes::intptr_t) -> core::ctypes::c_uint;
	
	fn cairo_scaled_font_destroy(font: core::ctypes::intptr_t);
	fn cairo_scaled_font_get_reference_count(font: core::ctypes::intptr_t) -> core::ctypes::c_uint;
	fn cairo_scaled_font_reference(font: core::ctypes::intptr_t) -> core::ctypes::intptr_t;
	fn cairo_scaled_font_get_type(font: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_scaled_font_get_scale_matrix(font: core::ctypes::intptr_t, matrix: core::ctypes::intptr_t);
	fn cairo_scaled_font_get_ctm(font: core::ctypes::intptr_t, matrix: core::ctypes::intptr_t);
	fn cairo_scaled_font_get_font_matrix(font: core::ctypes::intptr_t, matrix: core::ctypes::intptr_t);
	fn cairo_scaled_font_get_font_options(font: core::ctypes::intptr_t, options: core::ctypes::intptr_t);
	fn cairo_scaled_font_get_font_face(font: core::ctypes::intptr_t) -> core::ctypes::intptr_t;
	fn cairo_scaled_font_extents(font: core::ctypes::intptr_t, extents: core::ctypes::intptr_t);
	fn cairo_scaled_font_text_extents(font: core::ctypes::intptr_t, text: *u8, extents: core::ctypes::intptr_t);
	fn cairo_scaled_font_glyph_extents(font: core::ctypes::intptr_t, glyphs: core::ctypes::intptr_t, num_glyphs: core::ctypes::c_int, extents: core::ctypes::intptr_t);
	fn cairo_scaled_font_status(font: core::ctypes::intptr_t) -> core::ctypes::c_int;
	
	fn cairo_matrix_init(matrix: core::ctypes::intptr_t, xx: f64, xy: f64, yx: f64, yy: f64, x0: f64, y0: f64);
	fn cairo_matrix_init_identity(matrix: core::ctypes::intptr_t);
	fn cairo_matrix_init_translate(matrix: core::ctypes::intptr_t, x: f64, y: f64);
	fn cairo_matrix_init_scale(matrix: core::ctypes::intptr_t, x: f64, y: f64);
	fn cairo_matrix_init_rotate(matrix: core::ctypes::intptr_t, angle: f64);
	fn cairo_matrix_translate(matrix: core::ctypes::intptr_t, x: f64, y: f64);
	fn cairo_matrix_scale(matrix: core::ctypes::intptr_t, x: f64, y: f64);
	fn cairo_matrix_rotate(matrix: core::ctypes::intptr_t, angle: f64);
	fn cairo_matrix_invert(matrix: core::ctypes::intptr_t);
	fn cairo_matrix_multiply(matrix: core::ctypes::intptr_t, left: core::ctypes::intptr_t, right: core::ctypes::intptr_t);
	fn cairo_matrix_transform_distance(matrix: core::ctypes::intptr_t, x: *f64, y: *f64);
	fn cairo_matrix_transform_point(matrix: core::ctypes::intptr_t, x: *f64, y: *f64);
	
	fn cairo_pattern_destroy(pattern: core::ctypes::intptr_t);
	fn cairo_pattern_reference(pattern: core::ctypes::intptr_t) -> core::ctypes::intptr_t;
	fn cairo_pattern_create_rgb(red: f64, blue: f64, green: f64) -> core::ctypes::intptr_t;
	fn cairo_pattern_create_rgba(red: f64, blue: f64, green: f64, alpha: f64) -> core::ctypes::intptr_t;
	fn cairo_pattern_create_linear(x0: f64, y0: f64, x1: f64, y1: f64) -> core::ctypes::intptr_t;
	fn cairo_pattern_create_radial(cx0: f64, cy0: f64, radius0: f64, cx1: f64, cy1: f64, radius1: f64) -> core::ctypes::intptr_t;
	fn cairo_pattern_create_for_surface(pattern: core::ctypes::intptr_t) -> core::ctypes::intptr_t;
	fn cairo_pattern_get_type(pattern: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_pattern_get_matrix(pattern: core::ctypes::intptr_t, matrix: core::ctypes::intptr_t);
	fn cairo_pattern_set_matrix(pattern: core::ctypes::intptr_t, matrix: core::ctypes::intptr_t);
	fn cairo_pattern_get_filter(pattern: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_pattern_set_filter(pattern: core::ctypes::intptr_t, filter: core::ctypes::c_int);
	fn cairo_pattern_get_extend(pattern: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_pattern_set_extend(pattern: core::ctypes::intptr_t, extend: core::ctypes::c_int);
	fn cairo_pattern_status(pattern: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_pattern_get_color_stop_count(pattern: core::ctypes::intptr_t, count: *core::ctypes::c_int);
	fn cairo_pattern_get_surface(pattern: core::ctypes::intptr_t, surface: *core::ctypes::intptr_t);
	fn cairo_pattern_add_color_stop_rgb(pattern: core::ctypes::intptr_t, offset: f64, red: f64, green: f64, blue: f64);
	fn cairo_pattern_add_color_stop_rgba(pattern: core::ctypes::intptr_t, offset: f64, red: f64, green: f64, blue: f64, alpha: f64);
	fn cairo_pattern_get_color_stop_rgba(pattern: core::ctypes::intptr_t, index: core::ctypes::c_int, offset: *f64, red: *f64, green: *f64, blue: *f64, alpha: *f64);
	fn cairo_pattern_get_rgba(pattern: core::ctypes::intptr_t, red: *f64, green: *f64, blue: *f64, alpha: *f64);
	fn cairo_pattern_get_linear_points(pattern: core::ctypes::intptr_t, x0: *f64, y0: *f64, x1: *f64, y1: *f64);
	fn cairo_pattern_get_radial_circles(pattern: core::ctypes::intptr_t, cx0: *f64, cy0: *f64, radius0: *f64, cx1: *f64, cy1: *f64, radius1: *f64);
	
	fn cairo_format_stride_for_width(format: core::ctypes::c_int, width: core::ctypes::c_int) -> core::ctypes::c_int;
	fn cairo_surface_create_similar(surface: core::ctypes::intptr_t, content: core::ctypes::c_int, width: core::ctypes::c_int, height: core::ctypes::c_int) -> core::ctypes::intptr_t;
	fn cairo_image_surface_create_from_png(data: *u8) -> core::ctypes::intptr_t;
	fn cairo_image_surface_create_for_data(data: *u8, format: core::ctypes::c_int, width: core::ctypes::c_int, height: core::ctypes::c_int, stride: core::ctypes::c_int) -> core::ctypes::intptr_t;
	fn cairo_surface_write_to_png(surface: core::ctypes::intptr_t,file: *u8) -> core::ctypes::c_int;
	fn cairo_surface_destroy(surface: core::ctypes::intptr_t);
	fn cairo_image_surface_create(format: core::ctypes::c_int, width: core::ctypes::c_int, height: core::ctypes::c_int) -> core::ctypes::intptr_t;
	fn cairo_surface_has_show_text_glyphs(surface: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_surface_show_page(surface: core::ctypes::intptr_t);
	fn cairo_surface_copy_page(surface: core::ctypes::intptr_t);
	fn cairo_surface_get_type(surface: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_surface_set_fallback_resolution(surface: core::ctypes::intptr_t, x: f64, y: f64);
	fn cairo_surface_get_fallback_resolution(surface: core::ctypes::intptr_t, x: *f64, y: *f64);
	fn cairo_surface_set_device_offset(surface: core::ctypes::intptr_t, x: f64, y: f64);
	fn cairo_surface_get_device_offset(surface: core::ctypes::intptr_t, x: *f64, y: *f64);
	fn cairo_surface_mark_dirty(surface: core::ctypes::intptr_t);
	fn cairo_surface_mark_dirty_rectangle(surface: core::ctypes::intptr_t, x: core::ctypes::c_int, y: core::ctypes::c_int, width: core::ctypes::c_int, height: core::ctypes::c_int);
	fn cairo_surface_get_content(surface: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_surface_get_font_options(surface: core::ctypes::intptr_t, options: core::ctypes::intptr_t);
	fn cairo_surface_get_device(surface: core::ctypes::intptr_t) -> core::ctypes::intptr_t;
	fn cairo_surface_status(surface: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_surface_flush(surface: core::ctypes::intptr_t);
	fn cairo_surface_reference(surface: core::ctypes::intptr_t) -> core::ctypes::intptr_t;
	fn cairo_image_surface_get_data(surface: core::ctypes::intptr_t) -> *u8;
	fn cairo_image_surface_get_format(surface: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_image_surface_get_height(surface: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_image_surface_get_width(surface: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_image_surface_get_stride(surface: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_pdf_surface_restrict_to_version(surface: core::ctypes::intptr_t, version: core::ctypes::c_int);
	fn cairo_svg_surface_restrict_to_version(surface: core::ctypes::intptr_t, version: core::ctypes::c_int);
	fn cairo_pdf_surface_set_size(surface: core::ctypes::intptr_t, width: f64, height: f64);
	fn cairo_pdf_surface_create(path: *u8, width: f64, height: f64) -> core::ctypes::intptr_t;
	fn cairo_svg_surface_create(path: *u8, width: f64, height: f64) -> core::ctypes::intptr_t;
	fn cairo_ps_surface_restrict_to_level(surface: core::ctypes::intptr_t, level: core::ctypes::c_int);
	fn cairo_ps_surface_set_size(surface: core::ctypes::intptr_t, width: f64, height: f64);
	fn cairo_ps_surface_set_eps(surface: core::ctypes::intptr_t, eps: core::ctypes::c_int);
	fn cairo_ps_surface_get_eps(surface: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_ps_surface_dsc_begin_setup(surface: core::ctypes::intptr_t);
	fn cairo_ps_surface_dsc_begin_page_setup(surface: core::ctypes::intptr_t);
	fn cairo_ps_surface_dsc_comment(surface: core::ctypes::intptr_t, text: *u8);
	fn cairo_ps_surface_create(path: *u8, width: f64, height: f64) -> core::ctypes::intptr_t;
	
	fn cairo_device_reference(device: core::ctypes::intptr_t) -> core::ctypes::intptr_t;
	fn cairo_device_destroy(device: core::ctypes::intptr_t);
	fn cairo_device_flush(device: core::ctypes::intptr_t);
	fn cairo_device_acquire(device: core::ctypes::intptr_t) -> core::ctypes::c_int;
	fn cairo_device_release(device: core::ctypes::intptr_t);
	fn cairo_device_get_type(device: core::ctypes::intptr_t) -> core::ctypes::c_int;
	
	fn cairo_status_to_string(status: core::ctypes::c_int) -> *u8;
}

const STATUS_SUCCESS: int = 0;
const STATUS_NO_MEMORY: int = 1;
const STATUS_INVALID_RESTORE: int = 2;
const STATUS_INVALID_POP_GROUP: int = 3;
const STATUS_NO_CURRENT_POINT: int = 4;
const STATUS_INVALID_MATRIX: int = 5;
const STATUS_INVALID_STATUS: int = 6;
const STATUS_NULL_POINTER: int = 7;
const STATUS_INVALID_STRING: int = 8;
const STATUS_INVALID_PATH_DATA: int = 9;
const STATUS_READ_ERROR: int = 10;
const STATUS_WRITE_ERROR: int = 11;
const STATUS_SURFACE_FINISHED: int = 12;
const STATUS_SURFACE_TYPE_MISMATCH: int = 13;
const STATUS_PATTERN_TYPE_MISMATCH: int = 14;
const STATUS_INVALID_CONTENT: int = 15;
const STATUS_INVALID_FORMAT: int = 16;
const STATUS_INVALID_VISUAL: int = 17;
const STATUS_FILE_NOT_FOUND: int = 18;
const STATUS_INVALID_DASH: int = 19;
const STATUS_INVALID_DSC_COMMENT: int = 20;
const STATUS_INVALID_INDEX: int = 21;
const STATUS_CLIP_NOT_REPRESENTABLE: int = 22;
const STATUS_TEMP_FILE_ERROR: int = 23;
const STATUS_INVALID_STRIDE: int = 24;
const STATUS_FONT_TYPE_MISMATCH: int = 25;
const STATUS_USER_FONT_IMMUTABLE: int = 26;
const STATUS_USER_FONT_ERROR: int = 27;
const STATUS_NEGATIVE_COUNT: int = 28;
const STATUS_INVALID_CLUSTERS: int = 29;
const STATUS_INVALID_SLANT: int = 30;
const STATUS_INVALID_WEIGHT: int = 31;
const STATUS_INVALID_SIZE: int = 32;
const STATUS_USER_FONT_NOT_IMPLEMENTED: int = 33;
const STATUS_DEVICE_TYPE_MISMATCH: int = 34;
const STATUS_DEVICE_ERROR: int = 35;
const STATUS_LAST_STATUS: int = 36;

type status = int;

const FORMAT_INVALID: int = -1;
const FORMAT_ARGB32: int = 0;
const FORMAT_RGB24: int = 1;
const FORMAT_A8: int = 2;
const FORMAT_A1: int = 3;
const FORMAT_RGB16_565: int = 4;

type format = int;

const FONT_SLANT_NORMAL: int = 0;
const FONT_SLANT_ITALIC: int = 1;
const FONT_SLANT_OBLIQUE: int = 2;

type font_slant = int;

const FONT_WEIGHT_NORMAL: int = 0;
const FONT_WEIGHT_BOLD: int = 1;

type font_weight = int;

const ANTIALIAS_DEFAULT: int = 0;
const ANTIALIAS_NONE: int = 1;
const ANTIALIAS_GRAY: int = 2;
const ANTIALIAS_SUBPIXEL: int = 3;

type antialias = int;

const FILL_RULE_WINDING: int = 0;
const FILL_RULE_EVEN_ODD: int = 1;

type fill_rule = int;

const LINE_CAP_BUTT: int = 0;
const LINE_CAP_ROUND: int = 1;
const LINE_CAP_SQUARE: int = 2;

type line_cap = int;

const LINE_JOIN_MITER: int = 0;
const LINE_JOIN_ROUND: int = 1;
const LINE_JOIN_BEVEL: int = 2;

type line_join = int;

const OPERATOR_CLEAR: int = 0;
const OPERATOR_SOURCE: int = 1;
const OPERATOR_OVER: int = 2;
const OPERATOR_IN: int = 3;
const OPERATOR_OUT: int = 4;
const OPERATOR_ATOP: int = 5;
const OPERATOR_DEST: int = 6;
const OPERATOR_DEST_OVER: int = 7;
const OPERATOR_DEST_IN: int = 8;
const OPERATOR_DEST_OUT: int = 9;
const OPERATOR_DEST_ATOP: int = 10;
const OPERATOR_XOR: int = 11;
const OPERATOR_ADD: int = 12;
const OPERATOR_SATURATE: int = 13;
const OPERATOR_MULTIPLY: int = 14;
const OPERATOR_SCREEN: int = 15;
const OPERATOR_OVERLAY: int = 16;
const OPERATOR_DARKEN: int = 17;
const OPERATOR_LIGHTEN: int = 18;
const OPERATOR_COLOR_DODGE: int = 19;
const OPERATOR_COLOR_BURN: int = 20;
const OPERATOR_HARD_LIGHT: int = 21;
const OPERATOR_SOFT_LIGHT: int = 22;
const OPERATOR_DIFFERENCE: int = 23;
const OPERATOR_EXCLUSION: int = 24;
const OPERATOR_HSL_HUE: int = 25;
const OPERATOR_HSL_SATURATION: int = 26;
const OPERATOR_HSL_COLOR: int = 27;
const OPERATOR_HSL_LUMINOSITY: int = 28;

type operator = int;

const CONTENT_COLOR: int = 0x1000;
const CONTENT_ALPHA: int = 0x2000;
const CONTENT_COLOR_ALPHA: int = 0x3000;

type content = int;

const SURFACE_TYPE_IMAGE: int = 0;
const SURFACE_TYPE_PDF: int = 1;
const SURFACE_TYPE_PS: int = 2;
const SURFACE_TYPE_XLIB: int = 3;
const SURFACE_TYPE_XCB: int = 4;
const SURFACE_TYPE_GLITZ: int = 5;
const SURFACE_TYPE_QUARTZ: int = 6;
const SURFACE_TYPE_WIN32: int = 7;
const SURFACE_TYPE_BEOS: int = 8;
const SURFACE_TYPE_DIRECTFB: int = 9;
const SURFACE_TYPE_SVG: int = 10;
const SURFACE_TYPE_OS2: int = 11;
const SURFACE_TYPE_WIN32_PRINTING: int = 12;
const SURFACE_TYPE_QUARTZ_IMAGE: int = 13;
const SURFACE_TYPE_SCRIPT: int = 14;
const SURFACE_TYPE_QT: int = 15;
const SURFACE_TYPE_RECORDING: int = 16;
const SURFACE_TYPE_VG: int = 17;
const SURFACE_TYPE_GL: int = 18;
const SURFACE_TYPE_DRM: int = 19;
const SURFACE_TYPE_TEE: int = 20;
const SURFACE_TYPE_XML: int = 21;
const SURFACE_TYPE_SKIA: int = 22;
const SURFACE_TYPE_SUBSURFACE: int = 23;

type surface_type = int;

const TEXT_CLUSTER_FLAG_BACKWARD: int = 0x00000001;

type text_cluster_flags = int;

const SUBPIXEL_ORDER_DEFAULT: int = 0;
const SUBPIXEL_ORDER_RGB: int = 1;
const SUBPIXEL_ORDER_BGR: int = 2;
const SUBPIXEL_ORDER_VRGB: int = 3;
const SUBPIXEL_ORDER_VBGR: int = 4;
    
type subpixel_order = int;

const HINT_STYLE_DEFAULT: int = 0;
const HINT_STYLE_NONE: int = 1;
const HINT_STYLE_SLIGHT: int = 2;
const HINT_STYLE_MEDIUM: int = 3;
const HINT_STYLE_FULL: int = 4;
    
type hint_style = int;

const HINT_METRICS_DEFAULT: int = 0;
const HINT_METRICS_OFF: int = 1;
const HINT_METRICS_ON: int = 2;

type hint_metrics = int;

const FONT_TYPE_TOY: int = 0;
const FONT_TYPE_FT: int = 1;
const FONT_TYPE_WIN32: int = 2;
const FONT_TYPE_QUARTZ: int = 3;
const FONT_TYPE_USER: int = 4;

type font_type = int;

type font_extents_record = {
	mutable ascent: f64,
	mutable descent: f64,
	mutable height: f64,
	mutable max_x_advance: f64,
	mutable max_y_advance: f64
};
type text_extents_record = {
	mutable x_bearing: f64,
	mutable y_bearing: f64,
	mutable width: f64,
	mutable height: f64,
	mutable x_advance: f64,
	mutable y_advance: f64
};
type matrix_record = {
	mutable xx: f64,
	mutable yx: f64,
	mutable xy: f64,
	mutable yy: f64,
	mutable x0: f64,
	mutable y0: f64
};
type glyph_record = {
	mutable index: core::ctypes::ulong,
	mutable x: f64,
	mutable y: f64
};
type text_cluster_record = {
	mutable num_bytes: core::ctypes::c_int,
	mutable num_glyphs: core::ctypes::c_int
};

const DEVICE_TYPE_DRM: int = 0;
const DEVICE_TYPE_GL: int = 1;
const DEVICE_TYPE_SCRIPT: int = 2;
const DEVICE_TYPE_XCB: int = 3;
const DEVICE_TYPE_XLIB: int = 4;
const DEVICE_TYPE_XML: int = 5;

type device_type = int;

const EXTEND_NONE: int = 0;
const EXTEND_REPEAT: int = 1;
const EXTEND_REFLECT: int = 2;
const EXTEND_PAD: int = 3;

type extend = int;

const FILTER_FAST: int = 0;
const FILTER_GOOD: int = 1;
const FILTER_BEST: int = 2;
const FILTER_NEAREST: int = 3;
const FILTER_BILINEAR: int = 4;
const FILTER_GAUSSIAN: int = 5;

type filter = int;

const PATTERN_TYPE_SOLID: int = 0;
const PATTERN_TYPE_SURFACE: int = 1;
const PATTERN_TYPE_LINEAR: int = 2;
const PATTERN_TYPE_RADIAL: int = 3;

type pattern_type = int;

/*
 * Device
 */

iface device {
	fn get_internal() -> core::ctypes::intptr_t;
	
	fn flush();
	fn get_type() -> device_type;
	fn acquire() -> status;
	fn release();
}
resource device_res(internal: core::ctypes::intptr_t) {
	ccairo::cairo_device_destroy(internal);
}

fn wrap_device(internal: core::ctypes::intptr_t) -> device {
	impl of device for @device_res {
		fn get_internal() -> core::ctypes::intptr_t {
			ret **self as core::ctypes::intptr_t;
		}
		
		fn flush() {
			ccairo::cairo_device_flush(**self);
		}
		fn get_type() -> device_type {
			ret ccairo::cairo_device_get_type(**self) as device_type;
		}
		fn acquire() -> status {
			ret ccairo::cairo_device_acquire(**self) as status;
		}
		fn release() {
			ccairo::cairo_device_release(**self);
		}
	}
	
	ret @device_res(internal) as device;
}

/*
 * Surface
 */

iface surface {
	fn get_internal() -> core::ctypes::intptr_t;
	
	fn restrict_to_pdf_version(version: str);
	fn set_pdf_size(width_in_points: float, height_in_points: float);
	
	fn restrict_to_svg_version(version: str);
	
	fn restrict_to_ps_version(version: str);
	fn set_ps_size(width_in_points: float, height_in_points: float);
	fn set_ps_encapsulated(eps: bool);
	fn is_ps_encapsulated() -> bool;
	fn ps_begin_setup_comments();
	fn ps_begin_page_setup_comments();
	fn ps_comment(text: str);
	
	fn get_image_data() -> [mutable u8];
	fn get_image_format() -> format;
	fn get_image_width() -> uint;
	fn get_image_height() -> uint;
	fn get_image_stride() -> uint;
	fn get_image_size() -> (uint, uint);
	
	fn write_to_file(file: str) -> status;
	fn get_status() -> status;
	fn flush();
	fn get_device() -> device;
	fn get_font_options() -> font_options;
	fn get_content() -> content;
	fn mark_dirty();
	fn mark_dirty_rectangle(x: uint, y: uint, width: uint, height: uint);
	fn set_device_offset(x: float, y: float);
	fn get_device_offset() -> (float, float);
	fn set_fallback_resolution(x_ppi: float, y_ppi: float);
	fn get_fallback_resolution() -> (float, float);
	fn get_type() -> surface_type;
	fn copy_page();
	fn has_show_text_glyphs() -> bool;
}
resource surface_res(internal: core::ctypes::intptr_t) {
	ccairo::cairo_surface_destroy(internal);
}

fn wrap_surface(internal: core::ctypes::intptr_t) -> surface {
	impl of surface for @surface_res {
		fn get_internal() -> core::ctypes::intptr_t {
			ret **self as core::ctypes::intptr_t;
		}
		
		fn restrict_to_pdf_version(version: str) {
			alt version {
				"1.4" {
					ccairo::cairo_pdf_surface_restrict_to_version(**self, 0 as core::ctypes::c_int);
				}
				"1.5" {
					ccairo::cairo_pdf_surface_restrict_to_version(**self, 1 as core::ctypes::c_int);
				}
			}
		}
		fn set_pdf_size(width_in_points: float, height_in_points: float) {
			ccairo::cairo_pdf_surface_set_size(**self, width_in_points, height_in_points);
		}
		fn restrict_to_svg_version(version: str) {
			alt version {
				"1.1" {
					ccairo::cairo_svg_surface_restrict_to_version(**self, 0 as core::ctypes::c_int);
				}
				"1.2" {
					ccairo::cairo_svg_surface_restrict_to_version(**self, 1 as core::ctypes::c_int);
				}
			}
		}
		fn restrict_to_ps_version(version: str) {
			alt version {
				"2" {
					ccairo::cairo_ps_surface_restrict_to_level(**self, 0 as core::ctypes::c_int);
				}
				"3" {
					ccairo::cairo_ps_surface_restrict_to_level(**self, 1 as core::ctypes::c_int);
				}
			}
		}
		fn set_ps_size(width_in_points: float, height_in_points: float) {
			ccairo::cairo_ps_surface_set_size(**self, width_in_points, height_in_points);
		}
		fn set_ps_encapsulated(eps: bool) {
			ccairo::cairo_ps_surface_set_eps(**self, (eps ? 1 : 0) as core::ctypes::c_int);
		}
		fn is_ps_encapsulated() -> bool {
			ret ccairo::cairo_ps_surface_get_eps(**self) == (1 as core::ctypes::c_int);
		}
		fn ps_begin_setup_comments() {
			ccairo::cairo_ps_surface_dsc_begin_setup(**self);
		}
		fn ps_begin_page_setup_comments() {
			ccairo::cairo_ps_surface_dsc_begin_page_setup(**self);
		}
		fn ps_comment(text: str) unsafe {
			let bytes = core::str::bytes(text);
		
			ccairo::cairo_ps_surface_dsc_comment(**self, core::vec::unsafe::to_ptr(bytes));
		}
		fn get_image_data() -> [mutable u8] unsafe { // TODO test if this is mutable, as rust might not keep the same pointers but reallocate
			let data = ccairo::cairo_image_surface_get_data(**self); // FIXME boxed free
		
			ret core::vec::to_mut(core::vec::unsafe::from_buf(data, self.get_image_stride() * self.get_image_height()));
		}
		fn get_image_format() -> format {
			ret ccairo::cairo_image_surface_get_format(**self) as format;
		}
		fn get_image_width() -> uint {
			ret ccairo::cairo_image_surface_get_width(**self) as uint;
		}
		fn get_image_height() -> uint {
			ret ccairo::cairo_image_surface_get_height(**self) as uint;
		}
		fn get_image_stride() -> uint {
			ret ccairo::cairo_image_surface_get_stride(**self) as uint;
		}
		fn get_image_size() -> (uint, uint) {
			ret (self.get_image_width(), self.get_image_height());
		}

		fn write_to_file(file: str) -> status unsafe {
			let path = std::fs::make_absolute(file);
			let split = std::fs::splitext(path);
			let bytes = core::str::bytes(path);
		
			core::vec::push(bytes, 0 as u8);
		
			alt split {
				(base, ".png") {
					ret ccairo::cairo_surface_write_to_png(**self, core::vec::unsafe::to_ptr(bytes)) as status;
				}
				(base, _) {
					ret STATUS_WRITE_ERROR;
				}
			}
		}
		fn get_status() -> status {
			ret ccairo::cairo_surface_status(**self) as status;
		}
		fn flush() {
			ccairo::cairo_surface_flush(**self);
		}
		fn get_device() -> device {
			ret wrap_device(ccairo::cairo_device_reference(ccairo::cairo_surface_get_device(**self)));
		}
		fn get_font_options() -> font_options {
			let options = mk_font_options();
		
			ccairo::cairo_surface_get_font_options(**self, options.get_internal());
		
			ret options;
		}
		fn get_content() -> content {
			ret ccairo::cairo_surface_get_content(**self) as content;
		}
		fn mark_dirty() {
			ccairo::cairo_surface_mark_dirty(**self);
		}
		fn mark_dirty_rectangle(x: uint, y: uint, width: uint, height: uint) {
			ccairo::cairo_surface_mark_dirty_rectangle(**self, x as core::ctypes::c_int, y as core::ctypes::c_int, width as core::ctypes::c_int, height as core::ctypes::c_int);
		}
		fn set_device_offset(x: float, y: float) {
			ccairo::cairo_surface_set_device_offset(**self, x, y);
		}
		fn get_device_offset() -> (float, float) {
			let x: f64 = 0.0;
			let y: f64 = 0.0;
		
			ccairo::cairo_surface_get_device_offset(**self, core::ptr::addr_of(x), core::ptr::addr_of(y));
		
			ret (x, y);
		}
		fn set_fallback_resolution(x_ppi: float, y_ppi: float) {
			ccairo::cairo_surface_set_fallback_resolution(**self, x_ppi, y_ppi);
		}
		fn get_fallback_resolution() -> (float, float) {
			let x_ppi: f64 = 0.0;
			let y_ppi: f64 = 0.0;
		
			ccairo::cairo_surface_get_fallback_resolution(**self, core::ptr::addr_of(x_ppi), core::ptr::addr_of(y_ppi));
		
			ret (x_ppi, y_ppi);
		}
		fn get_type() -> surface_type {
			ret ccairo::cairo_surface_get_type(**self) as surface_type;
		}
		fn copy_page() {
			ccairo::cairo_surface_copy_page(**self);
		}
		fn show_page() {
			ccairo::cairo_surface_show_page(**self);
		}
		fn has_show_text_glyphs() -> bool {
			ret ccairo::cairo_surface_has_show_text_glyphs(**self) == (1 as core::ctypes::c_int);
		}
	}
	
	ret @surface_res(internal) as surface;
}
fn mk_surface_from_similar(other: surface, content: content, width: uint, height: uint) -> surface unsafe {
	let result = wrap_surface(ccairo::cairo_surface_create_similar(other.get_internal(), content as core::ctypes::c_int, width as core::ctypes::c_int, height as core::ctypes::c_int));
	let status = result.get_status();
	
	if status != STATUS_SUCCESS {
		fail #fmt("Could not make a surface from a similar surface: %s", status_to_str(status));
	}
	
	ret result;
}
fn mk_pdf_surface(file: str, width_in_points: float, height_in_points: float) -> surface unsafe {
	let path = std::fs::make_absolute(file);
	let bytes = core::str::bytes(path);
		
	core::vec::push(bytes, 0 as u8);
	
	let result = wrap_surface(ccairo::cairo_pdf_surface_create(core::vec::unsafe::to_ptr(bytes), width_in_points, height_in_points));
	let status = result.get_status();
	
	if status != STATUS_SUCCESS {
		fail #fmt("Could not make an PDF surface: %s", status_to_str(status));
	}
	
	ret result;
}
fn mk_svg_surface(file: str, width_in_points: float, height_in_points: float) -> surface unsafe {
	let path = std::fs::make_absolute(file);
	let bytes = core::str::bytes(path);
		
	core::vec::push(bytes, 0 as u8);
	
	let result = wrap_surface(ccairo::cairo_svg_surface_create(core::vec::unsafe::to_ptr(bytes), width_in_points, height_in_points));
	let status = result.get_status();
	
	if status != STATUS_SUCCESS {
		fail #fmt("Could not make an SVG surface: %s", status_to_str(status));
	}
	
	ret result;
}
fn mk_ps_surface(file: str, width_in_points: float, height_in_points: float) -> surface unsafe {
	let path = std::fs::make_absolute(file);
	let bytes = core::str::bytes(path);
	
	core::vec::push(bytes, 0 as u8);
	
	let result = wrap_surface(ccairo::cairo_ps_surface_create(core::vec::unsafe::to_ptr(bytes), width_in_points, height_in_points));
	let status = result.get_status();
	
	if status != STATUS_SUCCESS {
		fail #fmt("Could not make an PS surface: %s", status_to_str(status));
	}
	
	ret result;
}
fn mk_image_surface(format: format, width: uint, height: uint) -> surface {
	let result = wrap_surface(ccairo::cairo_image_surface_create(format as core::ctypes::c_int, width as core::ctypes::c_int, height as core::ctypes::c_int));
	let status = result.get_status();
	
	if status != STATUS_SUCCESS {
		fail #fmt("Could not make an image surface: %s", status_to_str(status));
	}
	
	ret result;
}
fn mk_image_surface_from_file(file: str) -> surface unsafe {
	// TODO .jpg (maybe .bmp)
	let path = std::fs::make_absolute(file);
	let split = std::fs::splitext(path);
	let bytes = core::str::bytes(path);
	let internal: core::ctypes::intptr_t;
	
	core::vec::push(bytes, 0 as u8);
	
	let path_cstr: *u8 = core::vec::unsafe::to_ptr(bytes);
		
	alt split {
		(base, ".png") {
			/*
			 * Let's leave cairo to do the actual PNG work seeing as it supports it
			 * by default, I just want to have a failure for the
			 * obvious non-existant file/no read error consistent with other formats when they are added
			 */
		
			let mode = "rb";
			let mode_bytes = core::str::bytes(mode);
			let mode_cstr = core::vec::unsafe::to_ptr(mode_bytes);
			let file: core::ctypes::intptr_t = c::fopen(path_cstr, mode_cstr);
			
			if file == (0 as core::ctypes::intptr_t) {
				fail "Could not make an image surface from a file: unable to load image";
			}
			
			c::fclose(file);
			
			internal = ccairo::cairo_image_surface_create_from_png(path_cstr);
		}
		(base, _) {
			fail "Could not make an image surface from a file: unsupported image format";
		}
	}
	
	let result = wrap_surface(internal);
	let status = result.get_status();
	
	if status != STATUS_SUCCESS {
		fail #fmt("Could not make an image surface from a file: %s", status_to_str(status));
	}
	
	ret result;
}
fn mk_image_surface_from_data(data: [u8], format: format, width: uint, height: uint, stride: uint) -> surface unsafe {
	let result = wrap_surface(ccairo::cairo_image_surface_create_for_data(core::vec::unsafe::to_ptr(data), format as core::ctypes::c_int, width as core::ctypes::c_int, height as core::ctypes::c_int, stride as core::ctypes::c_int));
	let status = result.get_status();
	
	if status != STATUS_SUCCESS {
		fail #fmt("Could not make an image surface from data: %s", status_to_str(status));
	}
	
	ret result;
}

/*
 * Pattern
 */

iface pattern {
	fn get_internal() -> core::ctypes::intptr_t;
	
	fn add_color_stop_rgb(offset: float, red: float, green: float, blue: float);
	fn add_color_stop_rgba(offset: float, red: float, green: float, blue: float, alpha: float);
	fn get_color_stop_count() -> uint;
	fn get_color_stop(index: uint) -> (float, float, float, float, float);
	fn get_rgba() -> (float, float, float, float);
	fn get_surface() -> surface;
	fn get_linear_points() -> (float, float, float, float);
	fn get_radial_circles() -> (float, float, float, float, float, float);
	fn get_status() -> status;
	fn set_extend(extend: extend);
	fn get_extend() -> extend;
	fn set_filter(filter: filter);
	fn get_extend() -> extend;
	fn set_matrix(matrix: matrix);
	fn get_matrix() -> matrix;
	fn get_type() -> pattern_type;
}
resource pattern_res(internal: core::ctypes::intptr_t) {
	ccairo::cairo_pattern_destroy(internal);
}

fn wrap_pattern(internal: core::ctypes::intptr_t) -> pattern {
	impl of pattern for @pattern_res {
		fn get_internal() -> core::ctypes::intptr_t {
			ret **self as core::ctypes::intptr_t;
		}
		
		fn add_color_stop_rgb(offset: float, red: float, green: float, blue: float) {
			ccairo::cairo_pattern_add_color_stop_rgb(**self, offset, red, green, blue);
		}
		fn add_color_stop_rgba(offset: float, red: float, green: float, blue: float, alpha: float) {
			ccairo::cairo_pattern_add_color_stop_rgba(**self, offset, red, green, blue, alpha);
		}
		fn get_color_stop_count() -> uint {
			let count: core::ctypes::c_int = 0 as core::ctypes::c_int;
		
			ccairo::cairo_pattern_get_color_stop_count(**self, core::ptr::addr_of(count));
		
			ret count as uint;
		}
		fn get_color_stop(index: uint) -> (float, float, float, float, float) {
			let offset: f64 = 0.0;
			let red: f64 = 0.0;
			let green: f64 = 0.0;
			let blue: f64 = 0.0;
			let alpha: f64 = 0.0;
		
			ccairo::cairo_pattern_get_color_stop_rgba(**self, index as core::ctypes::c_int, core::ptr::addr_of(offset), core::ptr::addr_of(red), core::ptr::addr_of(green), core::ptr::addr_of(blue), core::ptr::addr_of(alpha));
		
			ret (offset, red, green, blue, alpha);
		}
		fn get_rgba() -> (float, float, float, float) {
			let red: f64 = 0.0;
			let green: f64 = 0.0;
			let blue: f64 = 0.0;
			let alpha: f64 = 0.0;
		
			ccairo::cairo_pattern_get_rgba(**self, core::ptr::addr_of(red), core::ptr::addr_of(green), core::ptr::addr_of(blue), core::ptr::addr_of(alpha));
		
			ret (red, green, blue, alpha);
		}
		fn get_surface() -> surface {
			let internal: core::ctypes::intptr_t = 0 as core::ctypes::intptr_t;
		
			ccairo::cairo_pattern_get_surface(**self, core::ptr::addr_of(internal));
		
			ret wrap_surface(ccairo::cairo_surface_reference(internal));
		}
		fn get_linear_points() -> (float, float, float, float) {
			let x0: f64 = 0.0;
			let y0: f64 = 0.0;
			let x1: f64 = 0.0;
			let y1: f64 = 0.0;
		
			ccairo::cairo_pattern_get_linear_points(**self, core::ptr::addr_of(x0), core::ptr::addr_of(y0), core::ptr::addr_of(x1), core::ptr::addr_of(y1));
			
			ret (x0, y0, x1, y1);
		}
		fn get_radial_circles() -> (float, float, float, float, float, float) {
			let x0: f64 = 0.0;
			let y0: f64 = 0.0;
			let r0: f64 = 0.0;
			let x1: f64 = 0.0;
			let y1: f64 = 0.0;
			let r1: f64 = 0.0;
		
			ccairo::cairo_pattern_get_radial_circles(**self, core::ptr::addr_of(x0), core::ptr::addr_of(y0), core::ptr::addr_of(r0), core::ptr::addr_of(x1), core::ptr::addr_of(y1), core::ptr::addr_of(r1));
			
			ret (x0, y0, r0, x1, y1, r1);
		}
		fn get_status() -> status {
			ret ccairo::cairo_pattern_status(**self) as status;
		}
		fn set_extend(extend: extend) {
			ccairo::cairo_pattern_set_extend(**self, extend as core::ctypes::c_int);
		}
		fn get_extend() -> extend {
			ret ccairo::cairo_pattern_get_extend(**self) as extend;
		}
		fn set_filter(filter: filter) {
			ccairo::cairo_pattern_set_filter(**self, filter as core::ctypes::c_int);
		}
		fn get_extend() -> extend {
			ret ccairo::cairo_pattern_get_filter(**self) as filter;
		}
		fn set_matrix(matrix: matrix) {
			ccairo::cairo_pattern_set_matrix(**self, matrix.get_internal());
		}
		fn get_matrix() -> matrix {
			let matrix: matrix = mk_matrix([
				0.0, 0.0,
				0.0, 0.0,
				0.0, 0.0
			]);
		
			ccairo::cairo_pattern_get_matrix(**self, matrix.get_internal());
		
			ret matrix;
		}
		fn get_type() -> pattern_type {
			ret ccairo::cairo_pattern_get_type(**self) as pattern_type;
		}
	}
	
	ret @pattern_res(internal) as pattern;
}
fn mk_pattern_from_rgb(red: float, green: float, blue: float) -> pattern {
	let result = wrap_pattern(ccairo::cairo_pattern_create_rgb(red, green, blue));
	let status = result.get_status();
	
	if status != STATUS_SUCCESS {
		fail #fmt("Could not make a pattern from an RGB value: %s", status_to_str(status));
	}
	
	ret result;
}
fn mk_pattern_from_rgba(red: float, green: float, blue: float, alpha: float) -> pattern {
	let result = wrap_pattern(ccairo::cairo_pattern_create_rgba(red, green, blue, alpha));
	let status = result.get_status();
	
	if status != STATUS_SUCCESS {
		fail #fmt("Could not make a pattern from an RGBA value: %s", status_to_str(status));
	}
	
	ret result;
}
fn mk_pattern_from_linear_gradient(x0: float, y0: float, x1: float, y1: float) -> pattern {
	let result = wrap_pattern(ccairo::cairo_pattern_create_linear(x0, y0, x1, y1));
	let status = result.get_status();
	
	if status != STATUS_SUCCESS {
		fail #fmt("Could not make a pattern from a linear gradient: %s", status_to_str(status));
	}
	
	ret result;
}
fn mk_pattern_from_radial_gradient(cx0: float, cy0: float, radius0: float, cx1: float, cy1: float, radius1: float) -> pattern {
	let result = wrap_pattern(ccairo::cairo_pattern_create_radial(cx0, cy0, radius0, cx1, cy1, radius1));
	let status = result.get_status();
	
	if status != STATUS_SUCCESS {
		fail #fmt("Could not make a pattern from a radial gradient: %s", status_to_str(status));
	}
	
	ret result;
}
fn mk_pattern_from_surface(surface: surface) -> pattern {
	let result = wrap_pattern(ccairo::cairo_pattern_create_for_surface(surface.get_internal()));
	let status = result.get_status();
	
	if status != STATUS_SUCCESS {
		fail #fmt("Could not make a pattern from a surface: %s", status_to_str(status));
	}
	
	ret result;
}

/*
 * Matrix
 */

iface matrix {
	fn get_internal() -> core::ctypes::intptr_t;
	fn get_record() -> matrix_record;
	
	fn get_values() -> [float];
	fn set_values(values: [float]);
	fn set_identity();
	fn set_translate(x: float, y: float);
	fn set_scale(x: float, y: float);
	fn set_rotate(angle: float);
	fn translate(x: float, y: float);
	fn scale(x: float, y: float);
	fn rotate(angle: float);
	fn invert();
	fn transform_distance(x: float, y: float) -> (float, float);
	fn transform_point(x: float, y: float) -> (float, float);
}
resource matrix_res(internal: core::ctypes::intptr_t) {
	internal;
}

fn wrap_matrix(internal: core::ctypes::intptr_t) -> matrix {
	impl of matrix for @matrix_res {
		fn get_internal() -> core::ctypes::intptr_t {
			ret **self as core::ctypes::intptr_t;
		}
		fn get_record() -> matrix_record unsafe {
			ret *(self.get_internal() as *matrix_record);
		}
		
		fn get_values() -> [float] {
			let record: matrix_record = self.get_record();
		
			ret [
				record.xx, record.yx,
				record.xy, record.yy,
				record.x0, record.y0
			];
		}
		fn set_values(values: [float]) {
			ccairo::cairo_matrix_init(**self, values[0], values[1], values[2], values[3], values[4], values[5]);
		}
		fn set_identity() {
			ccairo::cairo_matrix_init_identity(**self);
		}
		fn set_translate(x: float, y: float) {
			ccairo::cairo_matrix_init_translate(**self, x, y);
		}
		fn set_scale(x: float, y: float) {
			ccairo::cairo_matrix_init_scale(**self, x, y);
		}
		fn set_rotate(angle: float) {
			ccairo::cairo_matrix_init_rotate(**self, angle);
		}
		fn translate(x: float, y: float) {
			ccairo::cairo_matrix_translate(**self, x, y);
		}
		fn scale(x: float, y: float) {
			ccairo::cairo_matrix_scale(**self, x, y);
		}
		fn rotate(angle: float) {
			ccairo::cairo_matrix_rotate(**self, angle);
		}
		fn invert() {
			ccairo::cairo_matrix_invert(**self);
		}
		/*FIXME fn multiply(other: matrix) {
			ccairo::cairo_matrix_multiply(**self, **self, **other);
		}*/
		fn transform_distance(x: float, y: float) -> (float, float) {
			let xt: f64 = x;
			let yt: f64 = y;
		
			ccairo::cairo_matrix_transform_distance(**self, core::ptr::addr_of(xt), core::ptr::addr_of(yt));
		
			ret (xt, yt);
		}
		fn transform_point(x: float, y: float) -> (float, float) {
			let xt: f64 = x;
			let yt: f64 = y;
		
			ccairo::cairo_matrix_transform_point(**self, core::ptr::addr_of(xt), core::ptr::addr_of(yt));
		
			ret (xt, yt);
		}
	}
	
	ret @matrix_res(internal) as matrix;
}
fn mk_matrix(values: [float]) -> matrix {
	let record: matrix_record = {
		mutable xx: 0.0,
		mutable yx: 0.0,
		mutable xy: 0.0,
		mutable yy: 0.0,
		mutable x0: 0.0,
		mutable y0: 0.0
	};
	let result = wrap_matrix(core::ptr::addr_of(record) as core::ctypes::intptr_t);
	
	result.set_values(values);
	
	ret result;
}

/*
 * Path
 */

iface path {
	fn get_internal() -> core::ctypes::intptr_t;
}
resource path_res(internal: core::ctypes::intptr_t) {
	ccairo::cairo_path_destroy(internal);
}

fn wrap_path(internal: core::ctypes::intptr_t) -> path {
	impl of path for @path_res {
		fn get_internal() -> core::ctypes::intptr_t {
			ret **self as core::ctypes::intptr_t;
		}
	}
	
	ret @path_res(internal) as path;
}

/*
 * Text
 */

iface glyph {
	fn get_internal() -> core::ctypes::intptr_t;
	fn get_record() -> glyph_record;

	fn set_index(index: uint);
	fn set_x(x: float);
	fn set_y(y: float);
	fn get_index() -> uint;
	fn get_x() -> float;
	fn get_y() -> float;
}
resource glyph_res(internal: core::ctypes::intptr_t) {
	internal;
}

fn wrap_glyph(internal: core::ctypes::intptr_t) -> glyph {
	impl of glyph for @glyph_res {
		fn get_internal() -> core::ctypes::intptr_t {
			ret **self as core::ctypes::intptr_t;
		}
		fn get_record() -> glyph_record unsafe {
			ret *(self.get_internal() as *glyph_record);
		}
		
		fn set_index(index: uint) {
			self.get_record().index = index as core::ctypes::ulong;
		}
		fn set_x(x: float) {
			self.get_record().x = x as float;
		}
		fn set_y(y: float) {
			self.get_record().y = y;
		}
		fn get_index() -> uint {
			ret self.get_record().index;
		}
		fn get_x() -> float {
			ret self.get_record().x;
		}
		fn get_y() -> float {
			ret self.get_record().y;
		}
	}
	
	ret @glyph_res(internal) as glyph;
}
fn mk_glyph(index: uint, x: float, y: float) -> glyph {
	let record: glyph_record = { // FIXME
		mutable index: index,
		mutable x: x,
		mutable y: y
	};
	
	ret wrap_glyph(core::ptr::addr_of(record) as core::ctypes::intptr_t);
}

iface text_cluster {
	fn get_internal() -> core::ctypes::intptr_t;
	fn get_record() -> text_cluster_record;
	
	fn set_num_bytes(num_bytes: uint);
	fn set_num_glyphs(num_glyphs: uint);
	fn get_num_bytes() -> uint;
	fn get_num_glyphs() -> uint;
}
resource text_cluster_res(internal: core::ctypes::intptr_t) {
	internal;
}

fn wrap_text_cluster(internal: core::ctypes::intptr_t) -> text_cluster {
	impl of text_cluster for @text_cluster_res {
		fn get_internal() -> core::ctypes::intptr_t {
			ret **self as core::ctypes::intptr_t;
		}
		fn get_record() -> text_cluster_record unsafe {
			ret *(self.get_internal() as *text_cluster_record);
		}
		
		fn set_num_bytes(num_bytes: uint) {
			self.get_record().num_bytes = num_bytes as core::ctypes::c_int;
		}
		fn set_num_glyphs(num_glyphs: uint) {
			self.get_record().num_glyphs = num_glyphs as core::ctypes::c_int;
		}
		fn get_num_bytes() -> uint {
			ret self.get_record().num_bytes as uint;
		}
		fn get_num_glyphs() -> uint {
			ret self.get_record().num_glyphs as uint;
		}
	}
	
	ret @text_cluster_res(internal) as text_cluster;
}
fn mk_text_cluster(num_bytes: uint, num_glyphs: uint) -> text_cluster {
	let record: text_cluster_record = {
		mutable num_bytes: num_bytes as core::ctypes::c_int,
		mutable num_glyphs: num_glyphs as core::ctypes::c_int
	};
	
	ret wrap_text_cluster(core::ptr::addr_of(record) as core::ctypes::intptr_t);
}

iface font_extents {
	fn get_internal() -> core::ctypes::intptr_t;
	fn get_record() -> font_extents_record;

	fn get_ascent() -> float;
	fn get_descent() -> float;
	fn get_height() -> float;
	fn get_max_x_advance() -> float;
	fn get_max_y_advance() -> float;
}
resource font_extents_res(internal: core::ctypes::intptr_t) {
	internal;
}

fn wrap_font_extents(internal: core::ctypes::intptr_t) -> font_extents {
	impl of font_extents for @font_extents_res {
		fn get_internal() -> core::ctypes::intptr_t {
			ret **self as core::ctypes::intptr_t;
		}
		fn get_record() -> font_extents_record unsafe {
			ret *(self.get_internal() as *font_extents_record);
		}
		
		fn get_ascent() -> float {
			ret self.get_record().ascent;
		}
		fn get_descent() -> float {
			ret self.get_record().descent;
		}
		fn get_height() -> float {
			ret self.get_record().height;
		}
		fn get_max_x_advance() -> float {
			ret self.get_record().max_x_advance;
		}
		fn get_max_y_advance() -> float {
			ret self.get_record().max_y_advance;
		}
	}
	
	ret @font_extents_res(internal) as font_extents;
}

iface text_extents {
	fn get_internal() -> core::ctypes::intptr_t;
	fn get_record() -> text_extents_record;

	fn get_x_bearing() -> float;
	fn get_y_bearing() -> float;
	fn get_width() -> float;
	fn get_height() -> float;
	fn get_size() -> (float, float);
	fn get_x_advance() -> float;
	fn get_y_bearing() -> float;
}
resource text_extents_res(internal: core::ctypes::intptr_t) {
	internal;
}

fn wrap_text_extents(internal: core::ctypes::intptr_t) -> text_extents {
	impl of text_extents for @text_extents_res {
		fn get_internal() -> core::ctypes::intptr_t {
			ret **self as core::ctypes::intptr_t;
		}
		fn get_record() -> text_extents_record unsafe {
			ret *(self.get_internal() as *text_extents_record);
		}
		
		fn get_x_bearing() -> float {
			ret self.get_record().x_bearing;
		}
		fn get_y_bearing() -> float {
			ret self.get_record().y_bearing;
		}
		fn get_width() -> float {
			ret self.get_record().width;
		}
		fn get_height() -> float {
			ret self.get_record().height;
		}
		fn get_size() -> (float, float) {
			ret (self.get_width(), self.get_height());
		}
		fn get_x_advance() -> float {
			ret self.get_record().x_advance;
		}
		fn get_y_bearing() -> float {
			ret self.get_record().y_bearing;
		}
	}
	
	ret @text_extents_res(internal) as text_extents;
}

type font_face_free_record = {
	library: core::ctypes::intptr_t,
	face: core::ctypes::intptr_t,
	internal: core::ctypes::intptr_t
};

iface font_face {
	fn get_internal() -> core::ctypes::intptr_t;

	fn get_status() -> status;
	fn get_type() -> font_type;
	fn get_toy_slant() -> font_slant;
	fn get_toy_weight() -> font_weight;
	fn get_toy_family() -> str;
}
resource font_face_res(record: @font_face_free_record) {
	let face = record.face;
	let library = record.library;
	let internal = record.internal;
	
	ccairo::cairo_font_face_destroy(internal);
	
	if face != (0 as core::ctypes::intptr_t) {
		cft::FT_Done_Face(face);
	}
	if library != (0 as core::ctypes::intptr_t) {
		cft::FT_Done_FreeType(library);
	}
}

fn wrap_font_face(record: @font_face_free_record) -> font_face {
	impl of font_face for @font_face_res {
		fn get_internal() -> core::ctypes::intptr_t {
			ret self.internal as core::ctypes::intptr_t;
		}
		
		fn get_status() -> status {
			ret ccairo::cairo_font_face_status(self.internal) as status;
		}
		fn get_type() -> font_type {
			ret ccairo::cairo_font_face_get_type(self.internal) as font_type;
		}
		fn get_toy_slant() -> font_slant {
			ret ccairo::cairo_toy_font_face_get_slant(self.internal);
		}
		fn get_toy_weight() -> font_weight {
			ret ccairo::cairo_toy_font_face_get_weight(self.internal);
		}
		fn get_toy_family() -> str unsafe {
			ret core::str::from_cstr(ccairo::cairo_toy_font_face_get_family(self.internal));
		}
	}
	
	ret @font_face_res(record) as font_face;
}
fn mk_font_face_from_toy_font(family: str, slant: font_slant, weight: font_weight) -> font_face unsafe {
	let bytes = core::str::bytes(family);
	
	core::vec::push(bytes, 0 as u8);
	
	let free_record: @font_face_free_record = @{
		library: 0 as ctypes::intptr_t,
		face: 0 as ctypes::intptr_t,
		internal: ccairo::cairo_toy_font_face_create(core::vec::unsafe::to_ptr(bytes), slant as core::ctypes::c_int, weight as core::ctypes::c_int)
	};
	let result = wrap_font_face(free_record);
	let status = result.get_status();
	
	if status != STATUS_SUCCESS {
		fail #fmt("Could not make a font face from a toy font: %s", status_to_str(status));
	}
	
	ret result;
}

fn mk_font_face_from_file(file: str) -> font_face unsafe {
	let path = std::fs::make_absolute(file);
	let split = std::fs::splitext(path);
	let bytes = core::str::bytes(path);
	let internal: core::ctypes::intptr_t;
	let face_internal: core::ctypes::intptr_t = 0 as core::ctypes::intptr_t;
		let library_internal: core::ctypes::intptr_t = 0 as core::ctypes::intptr_t;
	
	core::vec::push(bytes, 0 as u8);
		
	alt split {
		(base, ".ttf") {
			if cft::FT_Init_FreeType(core::ptr::addr_of(library_internal)) != (0 as core::ctypes::c_int) {
				fail "Could not make a font face from a font file: unable to initialize freetype";
			}

			if cft::FT_New_Face(library_internal, core::vec::unsafe::to_ptr(bytes), 0 as core::ctypes::long, core::ptr::addr_of(face_internal)) != (0 as core::ctypes::c_int) {
				fail "Could not make a font face from a font file: unable to load font";
			}
			
			internal = ccairo::cairo_ft_font_face_create_for_ft_face(face_internal, 0 as core::ctypes::c_int);
		}
		(base, _) {
			fail "Could not make a font face from a font file: unsupported font extension";
		}
	}
	
	let free_record: @font_face_free_record = @{
		library: library_internal,
		face: face_internal,
		internal: internal
	};
	let result = wrap_font_face(free_record);
	let status = result.get_status();
	
	if status != STATUS_SUCCESS {
		fail #fmt("Could not make a font face from a file: %s", status_to_str(status));
	}
	
	ret result;
}

iface scaled_font {
	fn get_internal() -> core::ctypes::intptr_t;
	
	fn get_status() -> status;
	fn extents() -> font_extents;
	fn text_extents(text: str) -> text_extents;
	fn glyph_extents(glyphs: [glyph]) -> text_extents;
	fn get_font_face() -> font_face;
	fn get_font_options() -> font_options;
	fn get_font_matrix() -> matrix;
	fn get_ctm() -> matrix;
	fn get_scale_matrix() -> matrix;
	fn get_type() -> font_type;
}
resource scaled_font_res(internal: core::ctypes::intptr_t) {
	ccairo::cairo_scaled_font_destroy(internal);
}

fn wrap_scaled_font(internal: core::ctypes::intptr_t) -> scaled_font {
	impl of scaled_font for @scaled_font_res {
		fn get_internal() -> core::ctypes::intptr_t {
			ret **self as core::ctypes::intptr_t;
		}
	
		fn get_status() -> status {
			ret ccairo::cairo_scaled_font_status(**self) as status;
		}
		fn extents() -> font_extents {
			let record: font_extents_record = {
				mutable ascent: 0.0,
				mutable descent: 0.0,
				mutable height: 0.0,
				mutable max_x_advance: 0.0,
				mutable max_y_advance: 0.0
			};
			let result = wrap_font_extents(core::ptr::addr_of(record) as core::ctypes::intptr_t);
		
			ccairo::cairo_scaled_font_extents(**self, result.get_internal());
		
			ret result;
		}
		fn text_extents(text: str) -> text_extents unsafe {
			let bytes = core::str::bytes(text);
			let record: text_extents_record = {
				mutable x_bearing: 0.0,
				mutable y_bearing: 0.0,
				mutable width: 0.0,
				mutable height: 0.0,
				mutable x_advance: 0.0,
				mutable y_advance: 0.0
			};
			let result = wrap_text_extents(core::ptr::addr_of(record) as core::ctypes::intptr_t);
		
			core::vec::push(bytes, 0 as u8);
			ccairo::cairo_scaled_font_text_extents(**self, core::vec::unsafe::to_ptr(bytes), result.get_internal());
		
			ret result;
		}
		fn glyph_extents(glyphs: [glyph]) -> text_extents unsafe {
			let record: text_extents_record = {
				mutable x_bearing: 0.0,
				mutable y_bearing: 0.0,
				mutable width: 0.0,
				mutable height: 0.0,
				mutable x_advance: 0.0,
				mutable y_advance: 0.0
			};
			let result = wrap_text_extents(core::ptr::addr_of(record) as core::ctypes::intptr_t);
			let cglyphs: [core::ctypes::intptr_t] = [];
		
			for glyph in glyphs {
				cglyphs += [glyph.get_internal()];
			}
		
			ccairo::cairo_scaled_font_glyph_extents(**self, core::vec::unsafe::to_ptr(cglyphs) as core::ctypes::intptr_t, core::vec::len(cglyphs) as core::ctypes::c_int, result.get_internal());
		
			ret result;
		}
		fn get_font_face() -> font_face { // FIXME
			let free_record: @font_face_free_record = @{
				library: 0 as ctypes::intptr_t,
				face: 0 as ctypes::intptr_t,
				internal: ccairo::cairo_font_face_reference(ccairo::cairo_scaled_font_get_font_face(**self))
			};
		
			ret wrap_font_face(free_record);
		}
		fn get_font_options() -> font_options {
			let options: font_options = mk_font_options();
		
			ccairo::cairo_scaled_font_get_font_options(**self, options.get_internal());
		
			ret options;
		}
		fn get_font_matrix() -> matrix {
			let matrix: matrix = mk_matrix([
				0.0, 0.0,
				0.0, 0.0,
				0.0, 0.0
			]);
		
			ccairo::cairo_scaled_font_get_font_matrix(**self, matrix.get_internal());
		
			ret matrix;
		}
		fn get_ctm() -> matrix {
			let matrix: matrix = mk_matrix([
				0.0, 0.0,
				0.0, 0.0,
				0.0, 0.0
			]);
		
			ccairo::cairo_scaled_font_get_ctm(**self, matrix.get_internal());
		
			ret matrix;
		}
		fn get_scale_matrix() -> matrix {
			let matrix: matrix = mk_matrix([
				0.0, 0.0,
				0.0, 0.0,
				0.0, 0.0
			]);
		
			ccairo::cairo_scaled_font_get_scale_matrix(**self, matrix.get_internal());
		
			ret matrix;
		}
		fn get_type() -> font_type {
			ret ccairo::cairo_scaled_font_get_type(**self) as font_type;
		}
	}
	
	ret @scaled_font_res(internal) as scaled_font;
}

iface font_options {
	fn get_internal() -> core::ctypes::intptr_t;	
	
	fn get_status() -> status;
	fn hash() -> uint;
	fn set_antialias(antialias: antialias);
	fn get_antialias() -> antialias;
	fn set_subpixel_order(order: subpixel_order);
	fn get_subpixel_order() -> subpixel_order;
	fn set_hint_style(hint: hint_style);
	fn get_hint_style() -> hint_style;
	fn set_hint_metrics(hint: hint_metrics);
	fn get_hint_metrics() -> hint_metrics;
}
resource font_options_res(internal: core::ctypes::intptr_t) {
	ccairo::cairo_font_options_destroy(internal);
}

fn wrap_font_options(internal: core::ctypes::intptr_t) -> font_options {
	impl of font_options for @font_options_res {
		fn get_internal() -> core::ctypes::intptr_t {
			ret **self as core::ctypes::intptr_t;
		}
		
		fn get_status() -> status {
			ret ccairo::cairo_font_options_status(**self) as status;
		}
		/* FIXME fn merge(other: font_options) {
		}*/
		fn hash() -> uint {
			ret ccairo::cairo_font_options_hash(**self) as uint;
		}
		/* FIXME fn equals(other: font_options) -> bool {
		}*/
		fn set_antialias(antialias: antialias) {
			ccairo::cairo_font_options_set_antialias(**self, antialias as core::ctypes::c_int);
		}
		fn get_antialias() -> antialias {
			ret ccairo::cairo_font_options_get_antialias(**self) as antialias;
		}
		fn set_subpixel_order(order: subpixel_order) {
			ccairo::cairo_font_options_set_subpixel_order(**self, order as core::ctypes::c_int);
		}
		fn get_subpixel_order() -> subpixel_order {
			ret ccairo::cairo_font_options_get_subpixel_order(**self) as subpixel_order;
		}
		fn set_hint_style(hint: hint_style) {
			ccairo::cairo_font_options_set_hint_style(**self, hint as core::ctypes::c_int);
		}
		fn get_hint_style() -> hint_style {
			ret ccairo::cairo_font_options_get_hint_style(**self) as hint_style;
		}
		fn set_hint_metrics(hint: hint_metrics) {
			ccairo::cairo_font_options_set_hint_metrics(**self, hint as core::ctypes::c_int);
		}
		fn get_hint_metrics() -> hint_metrics {
			ret ccairo::cairo_font_options_get_hint_metrics(**self) as hint_metrics;
		}
	}
	
	ret @font_options_res(internal) as font_options;
}
fn mk_font_options() -> font_options {
	ret wrap_font_options(ccairo::cairo_font_options_create());
}
fn mk_font_options_from_copy(other: font_options) -> font_options {
	ret wrap_font_options(ccairo::cairo_font_options_copy(other.get_internal()));
}

/*
 * Context
 */

iface context {
	fn get_internal() -> core::ctypes::intptr_t;

	fn get_status() -> status;
	fn save();
	fn restore();
	fn get_target() -> surface;
	fn push_group();
	fn push_group_with_content(content: content);
	fn pop_group() -> pattern;
	fn pop_group_to_source();
	fn get_group_target() -> surface;
	fn set_source_rgb(red: float, green: float, blue: float);
	fn set_source_rgba(red: float, green: float, blue: float, alpha: float);
	fn set_source(pattern: pattern);
	fn set_source_surface(surface: surface, x: float, y: float);
	fn get_source() -> pattern;
	fn set_antialias(antialias: antialias);
	fn get_antialias() -> antialias;
	fn set_dash(dashes: [float], offset: float);
	fn get_dash_count() -> uint;
	fn get_dash() -> [float];
	fn get_dash_offset() -> float;
	fn set_fill_rule(rule: fill_rule);
	fn get_fill_rule() -> fill_rule;
	fn set_line_cap(cap: line_cap);
	fn get_line_cap() -> line_cap;
	fn set_line_join(join: line_join);
	fn get_line_join() -> line_join;
	fn set_line_width(width: float);
	fn get_line_width() -> float;
	fn set_miter_limit(limit: float);
	fn get_miter_limit() -> float;
	fn set_operator(op: operator);
	fn get_operator() -> operator;
	fn set_tolerance(tol: float);
	fn get_tolerance() -> float;
	fn clip();
	fn clip_preserve();
	fn clip_extents() -> (float, float, float, float);
	fn in_clip(x: float, y: float) -> bool;
	fn reset_clip();
	fn fill();
	fn fill_preserve();
	fn fill_extents() -> (float, float, float, float);
	fn in_fill(x: float, y: float) -> bool;
	fn mask(pattern: pattern);
	fn mask_surface(surface: surface, x: float, y: float);
	fn paint();
	fn paint_with_alpha(alpha: float);
	fn stroke();
	fn stroke_preserve();
	fn stroke_extents() -> (float, float, float, float);
	fn in_stroke(x: float, y: float) -> bool;
	fn copy_page();
	fn show_page();
	
	fn copy_path() -> path;
	fn copy_path_flat() -> path;
	fn append_path(path: path);
	fn has_current_point() -> bool;
	fn get_current_point() -> (float, float);
	fn new_path();
	fn new_sub_path();
	fn close_path();
	fn arc(x: float, y: float, radius: float, angle1: float, angle2: float);
	fn arc_negative(x: float, y: float, radius: float, angle1: float, angle2: float);
	fn curve_to(x1: float, y1: float, x2: float, y2: float, x3: float, y3: float);
	fn line_to(x: float, y: float);
	fn move_to(x: float, y: float);
	fn rectangle(x: float, y: float, width: float, height: float);
	fn glyph_path(glyphs: [glyph]);
	fn text_path(text: str);
	fn rel_curve_to(x1: float, y1: float, x2: float, y2: float, x3: float, y3: float);
	fn rel_line_to(x: float, y: float);
	fn rel_move_to(x: float, y: float);
	fn path_extents() -> (float, float, float, float);
	
	fn translate(x: float, y: float);
	fn scale(x: float, y: float);
	fn rotate(angle: float);
	fn transform(matrix: matrix);
	fn set_matrix(matrix: matrix);
	fn get_matrix() -> matrix;
	fn identity_matrix();
	fn user_to_device(x: float, y: float) -> (float, float);
	fn user_to_device_distance(x: float, y: float) -> (float, float);
	fn device_to_user(x: float, y: float) -> (float, float);
	fn device_to_user_distance(x: float, y: float) -> (float, float);
	
	fn select_font_face(face: str, slant: font_slant, weight: font_weight);
	fn set_font_size(size: float);
	fn set_font_matrix(matrix: matrix);
	fn get_font_matrix() -> matrix;
	fn set_font_options(options: font_options);
	fn get_font_options() -> font_options;
	fn set_font_face(face: font_face);
	fn get_font_face() -> font_face;
	fn set_scaled_font(font: scaled_font);
	fn get_scaled_font() -> scaled_font;
	fn show_text(text: str);
	fn show_glyphs (glyphs: [glyph]);
	fn show_text_glyphs(text: str, glyphs: [glyph], clusters: [text_cluster], cluster_flags: text_cluster_flags);
	fn font_extents() -> font_extents;
	fn text_extents(text: str) -> text_extents;
	fn glyph_extents(glyphs: [glyph]) -> text_extents;
}
resource context_res(internal: core::ctypes::intptr_t) {
	ccairo::cairo_destroy(internal);
}


fn wrap_context(internal: core::ctypes::intptr_t) -> context {
	impl of context for @context_res {
		fn get_internal() -> core::ctypes::intptr_t {
			ret **self as core::ctypes::intptr_t;
		}
	
		fn get_status() -> status {
			ret ccairo::cairo_status(**self) as status;
		}
		fn save() {
			ccairo::cairo_save(**self);
		}
		fn restore() {
			ccairo::cairo_restore(**self);
		}
		fn get_target() -> surface {
			ret wrap_surface(ccairo::cairo_get_target(**self));
		}
		fn push_group() {
			ccairo::cairo_push_group(**self);
		}
		fn push_group_with_content(content: content) {
			ccairo::cairo_push_group_with_content(**self, content as core::ctypes::c_int);
		}
		fn pop_group() -> pattern {
			ret wrap_pattern(ccairo::cairo_pop_group(**self));
		}
		fn pop_group_to_source() {
			ccairo::cairo_pop_group_to_source(**self);
		}
		fn get_group_target() -> surface {
			ret wrap_surface(ccairo::cairo_get_group_target(**self));
		}
		fn set_source_rgb(red: float, green: float, blue: float) {
			ccairo::cairo_set_source_rgb(**self, red, green, blue);
		}
		fn set_source_rgba(red: float, green: float, blue: float, alpha: float) {
			ccairo::cairo_set_source_rgba(**self, red, green, blue, alpha);
		}
		fn set_source(pattern: pattern) {
			ccairo::cairo_set_source(**self, pattern.get_internal());
		}
		fn set_source_surface(surface: surface, x: float, y: float) {
			ccairo::cairo_set_source_surface(**self, surface.get_internal(), x, y);
		}
		fn get_source() -> pattern {
			ret wrap_pattern(ccairo::cairo_get_source(**self));
		}
		fn set_antialias(antialias: antialias) {
			ccairo::cairo_set_antialias(**self, antialias as core::ctypes::c_int);
		}
		fn get_antialias() -> antialias {
			ret ccairo::cairo_get_antialias(**self) as antialias;
		}
		fn set_dash(dashes: [float], offset: float) unsafe {
			ccairo::cairo_set_dash(**self, core::vec::unsafe::to_ptr(dashes), core::vec::len(dashes) as core::ctypes::c_int, offset);
		}
		fn get_dash_count() -> uint {
			ret ccairo::cairo_get_dash_count(**self) as uint;
		}
		fn get_dash() -> [float] unsafe {
			let dashes: [f64] = [];
		
			ccairo::cairo_get_dash(**self, core::vec::unsafe::to_ptr(dashes), core::ptr::null());
		
			ret dashes;
		}
		fn get_dash_offset() -> float {
			let offset: f64 = 0.0;
		
			ccairo::cairo_get_dash(**self, core::ptr::null(), core::ptr::addr_of(offset));
		
			ret offset;
		}
		fn set_fill_rule(rule: fill_rule) {
			ccairo::cairo_set_fill_rule(**self, rule as core::ctypes::c_int);
		}
		fn get_fill_rule() -> fill_rule {
			ret ccairo::cairo_get_fill_rule(**self) as fill_rule;
		}
		fn set_line_cap(cap: line_cap) {
			ccairo::cairo_set_line_cap(**self, cap as core::ctypes::c_int);
		}
		fn get_line_cap() -> line_cap {
			ret ccairo::cairo_get_line_cap(**self) as line_cap;
		}
		fn set_line_join(join: line_join) {
			ccairo::cairo_set_line_join(**self, join as core::ctypes::c_int);
		}
		fn get_line_join() -> line_join {
			ret ccairo::cairo_get_line_join(**self) as line_join;
		}
		fn set_line_width(width: float) {
			ccairo::cairo_set_line_width(**self, width);
		}
		fn get_line_width() -> float {
			ret ccairo::cairo_get_line_width(**self);
		}
		fn set_miter_limit(limit: float) {
			ccairo::cairo_set_miter_limit(**self, limit);
		}
		fn get_miter_limit() -> float {
			ret ccairo::cairo_get_miter_limit(**self);
		}
		fn set_operator(op: operator) {
			ccairo::cairo_set_operator(**self, op as core::ctypes::c_int);
		}
		fn get_operator() -> operator {
			ret ccairo::cairo_get_operator(**self) as operator;
		}
		fn set_tolerance(tol: float) {
			ccairo::cairo_set_tolerance(**self, tol);
		}
		fn get_tolerance() -> float {
			ret ccairo::cairo_get_tolerance(**self);
		}
		fn clip() {
			ccairo::cairo_clip(**self);
		}
		fn clip_preserve() {
			ccairo::cairo_clip_preserve(**self);
		}
		fn clip_extents() -> (float, float, float, float) {
			let x1: f64 = 0.0;
			let y1: f64 = 0.0;
			let x2: f64 = 0.0;
			let y2: f64 = 0.0;
		
			ccairo::cairo_clip_extents(**self, core::ptr::addr_of(x1), core::ptr::addr_of(y1), core::ptr::addr_of(x2), core::ptr::addr_of(y2));
		
			ret (x1, y1, x2, y2);
		}
		fn in_clip(x: float, y: float) -> bool {
			ret ccairo::cairo_in_clip(**self, x, y) == (1 as core::ctypes::c_int);
		}
		fn reset_clip() {
			ccairo::cairo_reset_clip(**self);
		}
		fn fill() {
			ccairo::cairo_fill(**self);
		}
		fn fill_preserve() {
			ccairo::cairo_fill_preserve(**self);
		}
		fn fill_extents() -> (float, float, float, float) {
			let x1: f64 = 0.0;
			let y1: f64 = 0.0;
			let x2: f64 = 0.0;
			let y2: f64 = 0.0;
		
			ccairo::cairo_fill_extents(**self, core::ptr::addr_of(x1), core::ptr::addr_of(y1), core::ptr::addr_of(x2), core::ptr::addr_of(y2));
		
			ret (x1, y1, x2, y2);
		}
		fn in_fill(x: float, y: float) -> bool {
			ret ccairo::cairo_in_fill(**self, x, y) == (1 as core::ctypes::c_int);
		}
		fn mask(pattern: pattern) {
			ccairo::cairo_mask(**self, pattern.get_internal());
		}
		fn mask_surface(surface: surface, x: float, y: float) {
			ccairo::cairo_mask_surface(**self, surface.get_internal(), x, y);
		}
		fn paint() {
			ccairo::cairo_paint(**self);
		}
		fn paint_with_alpha(alpha: float) {
			ccairo::cairo_paint_with_alpha(**self, alpha);
		}
		fn stroke() {
			ccairo::cairo_stroke(**self);
		}
		fn stroke_preserve() {
			ccairo::cairo_stroke_preserve(**self);
		}
		fn stroke_extents() -> (float, float, float, float) {
			let x1: f64 = 0.0;
			let y1: f64 = 0.0;
			let x2: f64 = 0.0;
			let y2: f64 = 0.0;
		
			ccairo::cairo_stroke_extents(**self, core::ptr::addr_of(x1), core::ptr::addr_of(y1), core::ptr::addr_of(x2), core::ptr::addr_of(y2));
		
			ret (x1, y1, x2, y2);
		}
		fn in_stroke(x: float, y: float) -> bool {
			ret ccairo::cairo_in_stroke(**self, x, y) == (1 as core::ctypes::c_int);
		}
		fn copy_page() {
			ccairo::cairo_copy_page(**self);
		}
		fn show_page() {
			ccairo::cairo_show_page(**self);
		}
	
		fn copy_path() -> path {
			ret wrap_path(ccairo::cairo_copy_path(**self));
		}
		fn copy_path_flat() -> path {
			ret wrap_path(ccairo::cairo_copy_path_flat(**self));
		}
		fn append_path(path: path) {
			ccairo::cairo_append_path(**self, path.get_internal());
		}
		fn has_current_point() -> bool {
			ret ccairo::cairo_has_current_point(**self) == (1 as core::ctypes::c_int);
		}
		fn get_current_point() -> (float, float) {
			let x: f64 = 0.0;
			let y: f64 = 0.0;
		
			ccairo::cairo_get_current_point(**self, core::ptr::addr_of(x), core::ptr::addr_of(y));
		
			ret (x, y);
		}
		fn new_path() {
			ccairo::cairo_new_path(**self);
		}
		fn new_sub_path() {
			ccairo::cairo_new_sub_path(**self);
		}
		fn close_path() {
			ccairo::cairo_close_path(**self);
		}
		fn arc(x: float, y: float, radius: float, angle1: float, angle2: float) {
			ccairo::cairo_arc(**self, x, y, radius, angle1, angle2);
		}
		fn arc_negative(x: float, y: float, radius: float, angle1: float, angle2: float) {
			ccairo::cairo_arc_negative(**self, x, y, radius, angle1, angle2);
		}
		fn curve_to(x1: float, y1: float, x2: float, y2: float, x3: float, y3: float) {
			ccairo::cairo_curve_to(**self, x1, y1, x2, y2, x3, y3);
		}
		fn line_to(x: float, y: float) {
			ccairo::cairo_line_to(**self, x, y);
		}
		fn move_to(x: float, y: float) {
			ccairo::cairo_move_to(**self, x, y);
		}
		fn rectangle(x: float, y: float, width: float, height: float) {
			ccairo::cairo_rectangle(**self, x, y, width, height);
		}
		fn glyph_path(glyphs: [glyph]) unsafe {
			let cglyphs: [core::ctypes::intptr_t] = [];
		
			for glyph in glyphs {
				cglyphs += [glyph.get_internal()];
			}
		
			ccairo::cairo_glyph_path(**self, core::vec::unsafe::to_ptr(cglyphs) as core::ctypes::intptr_t, core::vec::len(cglyphs) as core::ctypes::c_int);
		}
		fn text_path(text: str) unsafe {
			let bytes = core::str::bytes(text);
		
			core::vec::push(bytes, 0 as u8);
		
			ccairo::cairo_text_path(**self, core::vec::unsafe::to_ptr(bytes));
		}
		fn rel_curve_to(x1: float, y1: float, x2: float, y2: float, x3: float, y3: float) {
			ccairo::cairo_rel_curve_to(**self, x1, y1, x2, y2, x3, y3);
		}
		fn rel_line_to(x: float, y: float) {
			ccairo::cairo_rel_line_to(**self, x, y);
		}
		fn rel_move_to(x: float, y: float) {
			ccairo::cairo_rel_move_to(**self, x, y);
		}
		fn path_extents() -> (float, float, float, float) {
			let x1: f64 = 0.0;
			let y1: f64 = 0.0;
			let x2: f64 = 0.0;
			let y2: f64 = 0.0;
		
			ccairo::cairo_path_extents(**self, core::ptr::addr_of(x1), core::ptr::addr_of(y1), core::ptr::addr_of(x2), core::ptr::addr_of(y2));
		
			ret (x1, y1, x2, y2);
		}
	
		fn translate(x: float, y: float) {
			ccairo::cairo_translate(**self, x, y);
		}
		fn scale(x: float, y: float) {
			ccairo::cairo_scale(**self, x, y);
		}
		fn rotate(angle: float) {
			ccairo::cairo_rotate(**self, angle);
		}
		fn transform(matrix: matrix) {
			ccairo::cairo_transform(**self, matrix.get_internal());
		}
		fn set_matrix(matrix: matrix) {
			ccairo::cairo_set_matrix(**self, matrix.get_internal());
		}
		fn get_matrix() -> matrix {
			ret wrap_matrix(ccairo::cairo_get_matrix(**self));
		}
		fn identity_matrix() {
			ccairo::cairo_identity_matrix(**self)
		}
		fn user_to_device(x: float, y: float) -> (float, float) {
			let xt: f64 = x;
			let yt: f64 = y;
		
			ccairo::cairo_user_to_device(**self, core::ptr::addr_of(x), core::ptr::addr_of(y));
		
			ret (xt, yt);
		}
		fn user_to_device_distance(x: float, y: float) -> (float, float) {
			let xt: f64 = x;
			let yt: f64 = y;
		
			ccairo::cairo_user_to_device_distance(**self, core::ptr::addr_of(x), core::ptr::addr_of(y));
		
			ret (xt, yt);
		}
		fn device_to_user(x: float, y: float) -> (float, float) {
			let xt: f64 = x;
			let yt: f64 = y;
		
			ccairo::cairo_device_to_user(**self, core::ptr::addr_of(x), core::ptr::addr_of(y));
		
			ret (xt, yt);
		}
		fn device_to_user_distance(x: float, y: float) -> (float, float) {
			let xt: f64 = x;
			let yt: f64 = y;
		
			ccairo::cairo_device_to_user_distance(**self, core::ptr::addr_of(x), core::ptr::addr_of(y));
		
			ret (xt, yt);
		}
	
		fn select_font_face(face: str, slant: font_slant, weight: font_weight) unsafe {
			let bytes = core::str::bytes(face);
		
			core::vec::push(bytes, 0 as u8);
		
			ccairo::cairo_select_font_face(**self, core::vec::unsafe::to_ptr(bytes), slant as core::ctypes::c_int, weight as core::ctypes::c_int);
		}
		fn set_font_size(size: float) {
			ccairo::cairo_set_font_size(**self, size);
		}
		fn set_font_matrix(matrix: matrix) {
			ccairo::cairo_set_font_matrix(**self, matrix.get_internal());
		}
		fn get_font_matrix() -> matrix {
			let internal: core::ctypes::intptr_t = 0 as core::ctypes::intptr_t;
		
			ccairo::cairo_get_font_matrix(**self, core::ptr::addr_of(internal));
		
			ret wrap_matrix(internal);
		}
		fn set_font_options(options: font_options) {
			ccairo::cairo_set_font_options(**self, options.get_internal());
		}
		fn get_font_options() -> font_options {
			let internal: core::ctypes::intptr_t = 0 as core::ctypes::intptr_t;
		
			ccairo::cairo_get_font_options(**self, core::ptr::addr_of(internal));
		
			ret wrap_font_options(internal);
		}
		fn set_font_face(face: font_face) {
			ccairo::cairo_set_font_face(**self, face.get_internal());
		}
		fn get_font_face() -> font_face {
			let free_record: @font_face_free_record = @{
				library: 0 as ctypes::intptr_t,
				face: 0 as ctypes::intptr_t,
				internal: ccairo::cairo_font_face_reference(ccairo::cairo_get_font_face(**self))
			};
			
			ret wrap_font_face(free_record);
		}
		fn set_scaled_font(font: scaled_font) {
			ccairo::cairo_set_scaled_font(**self, font.get_internal());
		}
		fn get_scaled_font() -> scaled_font {
			ret wrap_scaled_font(ccairo::cairo_scaled_font_reference(ccairo::cairo_get_scaled_font(**self)));
		}
		fn show_text(text: str) unsafe {
			let bytes = core::str::bytes(text);
		
			core::vec::push(bytes, 0 as u8);
		
			ccairo::cairo_show_text(**self, core::vec::unsafe::to_ptr(bytes));
		}
		fn show_glyphs(glyphs: [glyph]) unsafe {
			let cglyphs: [core::ctypes::intptr_t] = [];
		
			for glyph in glyphs {
				cglyphs += [glyph.get_internal()];
			}
		
			ccairo::cairo_show_glyphs(**self, core::vec::unsafe::to_ptr(cglyphs) as core::ctypes::intptr_t, core::vec::len(cglyphs) as core::ctypes::c_int);
		}
		fn show_text_glyphs(text: str, glyphs: [glyph], clusters: [text_cluster], cluster_flags: text_cluster_flags) unsafe {
			let cglyphs: [core::ctypes::intptr_t] = [];
			let cclusters: [core::ctypes::intptr_t] = [];
			let bytes = core::str::bytes(text);
		
			core::vec::push(bytes, 0 as u8);
		
			for glyph in glyphs {
				cglyphs += [glyph.get_internal()];
			}
			for cluster in clusters {
				cclusters += [cluster.get_internal()];
			}
		
			ccairo::cairo_show_text_glyphs(**self, core::vec::unsafe::to_ptr(bytes), core::vec::len(bytes) as core::ctypes::c_int, core::vec::unsafe::to_ptr(cglyphs) as core::ctypes::intptr_t, core::vec::len(cglyphs) as core::ctypes::c_int, core::vec::unsafe::to_ptr(cclusters) as core::ctypes::intptr_t, core::vec::len(cclusters) as core::ctypes::c_int, cluster_flags as core::ctypes::c_int);
		}
		fn font_extents() -> font_extents {
			let record: font_extents_record = {
				mutable ascent: 0.0,
				mutable descent: 0.0,
				mutable height: 0.0,
				mutable max_x_advance: 0.0,
				mutable max_y_advance: 0.0
			};
			let result = wrap_font_extents(core::ptr::addr_of(record) as core::ctypes::intptr_t);
		
			ccairo::cairo_font_extents(**self, result.get_internal());
		
			ret result;
		}
		fn text_extents(text: str) -> text_extents unsafe {
			let bytes = core::str::bytes(text);
			let record = @{
				mutable x_bearing: 0.0,
				mutable y_bearing: 0.0,
				mutable width: 0.0,
				mutable height: 0.0,
				mutable x_advance: 0.0,
				mutable y_advance: 0.0
			};
			let result = wrap_text_extents(core::ptr::addr_of(record) as core::ctypes::intptr_t);
		
			core::vec::push(bytes, 0 as u8);
			ccairo::cairo_text_extents(**self, core::vec::unsafe::to_ptr(bytes), result.get_internal());
		
			ret result;
		}
		fn glyph_extents(glyphs: [glyph]) -> text_extents unsafe {
			let record = @{
				mutable x_bearing: 0.0,
				mutable y_bearing: 0.0,
				mutable width: 0.0,
				mutable height: 0.0,
				mutable x_advance: 0.0,
				mutable y_advance: 0.0
			};
			let result = wrap_text_extents(core::ptr::addr_of(record) as core::ctypes::intptr_t);
			let cglyphs: [core::ctypes::intptr_t] = [];
		
			for glyph in glyphs {
				cglyphs += [glyph.get_internal()];
			}
		
			ccairo::cairo_glyph_extents(**self, core::vec::unsafe::to_ptr(cglyphs) as core::ctypes::intptr_t, core::vec::len(cglyphs) as core::ctypes::c_int, result.get_internal());
		
			ret result;
		}
	}
	
	ret @context_res(internal) as context;
}
fn mk_context(surface: surface) -> context {
	let result = wrap_context(ccairo::cairo_create(surface.get_internal()));
	let status = result.get_status();
	
	if status != STATUS_SUCCESS {
		fail #fmt("Could not make a context: %s", status_to_str(status));
	}
	
	ret result;
}

/*
 * Utilities
 */
fn format_stride_for_width(format: format, width: uint) -> uint {
	ret ccairo::cairo_format_stride_for_width(format as core::ctypes::c_int, width as core::ctypes::c_int) as uint;
}
fn status_to_str(status: status) -> str unsafe {
	ret core::str::from_cstr(ccairo::cairo_status_to_string(status as core::ctypes::c_int));
}
fn get_version() -> str {
	ret "v0.2";
}
fn get_cairo_version() -> str unsafe {
	ret core::str::from_cstr(ccairo::cairo_version_string());
}
