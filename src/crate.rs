#[link(name = "cairo", vers = "0.1", author = "Zack0Wack0")];

use std;
import std::{fs};

/* FIXME all wrapped objects need to be cached rather than 
recreating new objects every time for each internal cairo pointer
*/

#[link_name = "cairo"]
native mod ccairo {
	fn cairo_create(surface: ctypes::intptr_t) -> ctypes::intptr_t;
	fn cairo_reference(context: ctypes::intptr_t) -> ctypes::intptr_t;
	fn cairo_destroy(context: ctypes::intptr_t);
	fn cairo_status(context: ctypes::intptr_t) -> ctypes::c_int;
	fn cairo_save(context: ctypes::intptr_t);
	fn cairo_restore(context: ctypes::intptr_t);
	fn cairo_get_target(context: ctypes::intptr_t) -> ctypes::intptr_t;
	fn cairo_push_group(context: ctypes::intptr_t);
	fn cairo_push_group_with_content(context: ctypes::intptr_t, content: ctypes::c_int);
	fn cairo_pop_group(context: ctypes::intptr_t) -> ctypes::intptr_t;
	fn cairo_pop_group_to_source(context: ctypes::intptr_t);
	fn cairo_get_group_target(context: ctypes::intptr_t) -> ctypes::intptr_t;
	fn cairo_set_source_rgb(context: ctypes::intptr_t, red: f64, green: f64, blue: f64);
	fn cairo_set_source_rgba(context: ctypes::intptr_t, red: f64, green: f64, blue: f64, alpha: f64);
	fn cairo_set_source(context: ctypes::intptr_t, pattern: ctypes::intptr_t);
	fn cairo_set_source_surface(context: ctypes::intptr_t, surface: ctypes::intptr_t, x: f64, y: f64);
	fn cairo_get_source(context: ctypes::intptr_t) -> ctypes::intptr_t;
	fn cairo_set_antialias(context: ctypes::intptr_t, antialias: ctypes::c_int);
	fn cairo_get_antialias(context: ctypes::intptr_t) -> ctypes::c_int;
	fn cairo_set_dash(context: ctypes::intptr_t, dashes: *f64, num_dashes: ctypes::c_int, offset: f64);
	fn cairo_get_dash_count(context: ctypes::intptr_t) -> ctypes::c_int;
	fn cairo_get_dash(context: ctypes::intptr_t, dashes: *f64, offset: *f64);
	fn cairo_set_fill_rule(context: ctypes::intptr_t, rule: ctypes::c_int);
	fn cairo_get_fill_rule(context: ctypes::intptr_t) -> ctypes::c_int;
	fn cairo_set_line_cap(context: ctypes::intptr_t, cap: ctypes::c_int);
	fn cairo_get_line_cap(context: ctypes::intptr_t) -> ctypes::c_int;
	fn cairo_set_line_join(context: ctypes::intptr_t, join: ctypes::c_int);
	fn cairo_get_line_join(context: ctypes::intptr_t) -> ctypes::c_int;
	fn cairo_set_line_width(context: ctypes::intptr_t, width: f64);
	fn cairo_get_line_width(context: ctypes::intptr_t) -> f64;
	fn cairo_set_miter_limit(context: ctypes::intptr_t, limit: f64);
	fn cairo_get_miter_limit(context: ctypes::intptr_t) -> f64;
	fn cairo_set_operator(context: ctypes::intptr_t, join: ctypes::c_int);
	fn cairo_get_operator(context: ctypes::intptr_t) -> ctypes::c_int;
	fn cairo_set_tolerance(context: ctypes::intptr_t, limit: f64);
	fn cairo_get_tolerance(context: ctypes::intptr_t) -> f64;
	fn cairo_clip(context: ctypes::intptr_t);
	fn cairo_clip_preserve(context: ctypes::intptr_t);
	fn cairo_clip_extents(context: ctypes::intptr_t, x1: *f64, y1: *f64, x2: *f64, y2: *f64);
	fn cairo_in_clip(context: ctypes::intptr_t, x: f64, y: f64) -> ctypes::c_int;
	fn cairo_reset_clip(context: ctypes::intptr_t);
	fn cairo_fill(context: ctypes::intptr_t);
	fn cairo_fill_preserve(context: ctypes::intptr_t);
	fn cairo_fill_extents(context: ctypes::intptr_t, x1: *f64, y1: *f64, x2: *f64, y2: *f64);
	fn cairo_in_fill(context: ctypes::intptr_t, x: f64, y: f64) -> ctypes::c_int;
	fn cairo_mask(context: ctypes::intptr_t, pattern: ctypes::intptr_t);
	fn cairo_mask_surface(context: ctypes::intptr_t, surface: ctypes::intptr_t, surface_x: f64, surface_y: f64);
	fn cairo_paint(context: ctypes::intptr_t);
	fn cairo_paint_with_alpha(context: ctypes::intptr_t, alpha: f64);
	fn cairo_stroke(context: ctypes::intptr_t);
	fn cairo_stroke_preserve(context: ctypes::intptr_t);
	fn cairo_stroke_extents(context: ctypes::intptr_t, x1: *f64, y1: *f64, x2: *f64, y2: *f64);
	fn cairo_in_stroke(context: ctypes::intptr_t, x: f64, y: f64) -> ctypes::c_int;
	fn cairo_copy_page(context: ctypes::intptr_t);
	fn cairo_show_page(context: ctypes::intptr_t);
	fn cairo_get_reference_count(context: ctypes::intptr_t) -> ctypes::c_uint;
	
	fn cairo_translate(context: ctypes::intptr_t, x: f64, y: f64);
	fn cairo_scale(context: ctypes::intptr_t, x: f64, y: f64);
	fn cairo_rotate(context: ctypes::intptr_t, angle: f64);
	fn cairo_transform(context: ctypes::intptr_t, matrix: ctypes::intptr_t);
	fn cairo_set_matrix(context: ctypes::intptr_t, matrix: ctypes::intptr_t);
	fn cairo_get_matrix(context: ctypes::intptr_t) -> ctypes::intptr_t;
	fn cairo_identity_matrix(context: ctypes::intptr_t);
	fn cairo_user_to_device(context: ctypes::intptr_t, x: *f64, y: *f64);
	fn cairo_user_to_device_distance(context: ctypes::intptr_t, x: *f64, y: *f64);
	fn cairo_device_to_user(context: ctypes::intptr_t, x: *f64, y: *f64);
	fn cairo_device_to_user_distance(context: ctypes::intptr_t, x: *f64, y: *f64);
	
	fn cairo_copy_path(context: ctypes::intptr_t) -> ctypes::intptr_t;
	fn cairo_copy_path_flat(context: ctypes::intptr_t) -> ctypes::intptr_t;
	fn cairo_path_destroy(path: ctypes::intptr_t);
	fn cairo_append_path(context: ctypes::intptr_t, path: ctypes::intptr_t);
	fn cairo_has_current_point(context: ctypes::intptr_t) -> ctypes::c_int;
	fn cairo_get_current_point(context: ctypes::intptr_t, x: *f64, y: *f64);
	fn cairo_new_path(context: ctypes::intptr_t);
	fn cairo_new_sub_path(context: ctypes::intptr_t);
	fn cairo_close_path(context: ctypes::intptr_t);
	fn cairo_arc(context: ctypes::intptr_t, x: f64, y: f64, radius: f64, angle1: f64, angle2: f64);
	fn cairo_arc_negative(context: ctypes::intptr_t, x: f64, y: f64, radius: f64, angle1: f64, angle2: f64);
	fn cairo_curve_to(context: ctypes::intptr_t, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64);
	fn cairo_line_to(context: ctypes::intptr_t, x: f64, y: f64);
	fn cairo_move_to(context: ctypes::intptr_t, x: f64, y: f64);
	fn cairo_rectangle(context: ctypes::intptr_t, x: f64, y: f64, width: f64, height: f64);
	fn cairo_glyph_path(context: ctypes::intptr_t, glyph: ctypes::intptr_t, num_glyphs: ctypes::c_int);
	fn cairo_text_path(context: ctypes::intptr_t, text: *u8);
	fn cairo_rel_curve_to(context: ctypes::intptr_t, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64);
	fn cairo_rel_line_to(context: ctypes::intptr_t, x: f64, y: f64);
	fn cairo_rel_move_to(context: ctypes::intptr_t, x: f64, y: f64);
	fn cairo_path_extents(context: ctypes::intptr_t, x1: *f64, y1: *f64, x2: *f64, y2: *f64);
	
	fn cairo_select_font_face(context: ctypes::intptr_t, face: *u8, slant: ctypes::c_int, weight: ctypes::c_int);
	fn cairo_set_font_size(context: ctypes::intptr_t, size: f64);
	fn cairo_set_font_matrix(context: ctypes::intptr_t, matrix: ctypes::intptr_t);
	fn cairo_get_font_matrix(context: ctypes::intptr_t, matrix: ctypes::intptr_t);
	fn cairo_set_font_options(context: ctypes::intptr_t, options: ctypes::intptr_t);
	fn cairo_get_font_options(context: ctypes::intptr_t, options: ctypes::intptr_t);
	fn cairo_set_font_face(context: ctypes::intptr_t, face: ctypes::intptr_t);
	fn cairo_get_font_face(context: ctypes::intptr_t) -> ctypes::intptr_t;
	fn cairo_set_scaled_font(context: ctypes::intptr_t, font: ctypes::intptr_t);
	fn cairo_get_scaled_font(context: ctypes::intptr_t) -> ctypes::intptr_t;
	fn cairo_show_text(context: ctypes::intptr_t, text: *u8);
	fn cairo_show_glyphs(context: ctypes::intptr_t, glyphs: ctypes::intptr_t, num_glyphs: ctypes::c_int);
	fn cairo_show_text_glyphs(context: ctypes::intptr_t, text: *u8, text_len: ctypes::c_int, glyphs: ctypes::intptr_t, num_glyphs: ctypes::c_int, clusters: ctypes::intptr_t, num_clusters: ctypes::c_int, cluster_flags: ctypes::c_int);
	fn cairo_font_extents(context: ctypes::intptr_t, extents: ctypes::intptr_t);
	fn cairo_text_extents(context: ctypes::intptr_t, text: *u8, extents: ctypes::intptr_t);
	fn cairo_glyph_extents(context: ctypes::intptr_t, glyphs: ctypes::intptr_t, num_glyphs: ctypes::c_int, extents: ctypes::intptr_t);
	fn cairo_toy_font_face_create(family: *u8, slant: ctypes::c_int, weight: ctypes::c_int) -> ctypes::intptr_t;
	fn cairo_toy_font_face_get_family(face: ctypes::intptr_t) -> *u8;
	fn cairo_toy_font_face_get_slant(face: ctypes::intptr_t) -> ctypes::c_int;
	fn cairo_toy_font_face_get_weight(face: ctypes::intptr_t) -> ctypes::c_int;
	fn cairo_glyph_allocate(num_glyphs: ctypes::c_int) -> ctypes::intptr_t;
	fn cairo_glyph_free(glyphs: ctypes::intptr_t);
	fn cairo_text_cluster_allocate(num_clusters: ctypes::c_int) -> ctypes::intptr_t;
	fn cairo_text_cluster_free(clusters: ctypes::intptr_t);
	
	fn cairo_font_options_create() -> ctypes::intptr_t;
	fn cairo_font_options_copy(options: ctypes::intptr_t) -> ctypes::intptr_t;
	fn cairo_font_options_destroy(options: ctypes::intptr_t);
	fn cairo_font_options_status(options: ctypes::intptr_t) -> ctypes::c_int;
	fn cairo_font_options_merge(options: ctypes::intptr_t, other: ctypes::intptr_t);
	fn cairo_font_options_hash(options: ctypes::intptr_t) -> ctypes::ulong;
	fn cairo_font_options_equal(options: ctypes::intptr_t, other: ctypes::intptr_t) -> ctypes::c_int;
	fn cairo_font_options_set_antialias(options: ctypes::intptr_t, value: ctypes::c_int);
	fn cairo_font_options_get_antialias(options: ctypes::intptr_t) -> ctypes::c_int;
	fn cairo_font_options_set_subpixel_order(options: ctypes::intptr_t, value: ctypes::c_int);
	fn cairo_font_options_get_subpixel_order(options: ctypes::intptr_t) -> ctypes::c_int;
	fn cairo_font_options_set_hint_style(options: ctypes::intptr_t, value: ctypes::c_int);
	fn cairo_font_options_get_hint_style(options: ctypes::intptr_t) -> ctypes::c_int;
	fn cairo_font_options_set_hint_metrics(options: ctypes::intptr_t, value: ctypes::c_int);
	fn cairo_font_options_get_hint_metrics(options: ctypes::intptr_t) -> ctypes::c_int;
	
	fn cairo_font_face_reference(face: ctypes::intptr_t) -> ctypes::intptr_t;
	fn cairo_font_face_destroy(face: ctypes::intptr_t);
	fn cairo_font_face_status(face: ctypes::intptr_t) -> ctypes::c_int;
	fn cairo_font_face_get_type(face: ctypes::intptr_t) -> ctypes::c_int;
	fn cairo_font_face_get_reference_count(face: ctypes::intptr_t) -> ctypes::c_uint;
	
	fn cairo_scaled_font_destroy(font: ctypes::intptr_t);
	fn cairo_scaled_font_get_reference_count(font: ctypes::intptr_t) -> ctypes::c_uint;
	fn cairo_scaled_font_reference(font: ctypes::intptr_t) -> ctypes::intptr_t;
	fn cairo_scaled_font_get_type(font: ctypes::intptr_t) -> ctypes::c_int;
	fn cairo_scaled_font_get_scale_matrix(font: ctypes::intptr_t, matrix: ctypes::intptr_t);
	fn cairo_scaled_font_get_ctm(font: ctypes::intptr_t, matrix: ctypes::intptr_t);
	fn cairo_scaled_font_get_font_matrix(font: ctypes::intptr_t, matrix: ctypes::intptr_t);
	fn cairo_scaled_font_get_font_options(font: ctypes::intptr_t, options: ctypes::intptr_t);
	fn cairo_scaled_font_get_font_face(font: ctypes::intptr_t) -> ctypes::intptr_t;
	fn cairo_scaled_font_extents(font: ctypes::intptr_t, extents: ctypes::intptr_t);
	fn cairo_scaled_font_text_extents(font: ctypes::intptr_t, text: *u8, extents: ctypes::intptr_t);
	fn cairo_scaled_font_glyph_extents(font: ctypes::intptr_t, glyphs: ctypes::intptr_t, num_glyphs: ctypes::c_int, extents: ctypes::intptr_t);
	fn cairo_scaled_font_status(font: ctypes::intptr_t) -> ctypes::c_int;
	
	fn cairo_matrix_init(matrix: ctypes::intptr_t, xx: f64, xy: f64, yx: f64, yy: f64, x0: f64, y0: f64);
	fn cairo_matrix_init_identity(matrix: ctypes::intptr_t);
	fn cairo_matrix_init_translate(matrix: ctypes::intptr_t, x: f64, y: f64);
	fn cairo_matrix_init_scale(matrix: ctypes::intptr_t, x: f64, y: f64);
	fn cairo_matrix_init_rotate(matrix: ctypes::intptr_t, angle: f64);
	fn cairo_matrix_translate(matrix: ctypes::intptr_t, x: f64, y: f64);
	fn cairo_matrix_scale(matrix: ctypes::intptr_t, x: f64, y: f64);
	fn cairo_matrix_rotate(matrix: ctypes::intptr_t, angle: f64);
	fn cairo_matrix_invert(matrix: ctypes::intptr_t);
	fn cairo_matrix_multiply(matrix: ctypes::intptr_t, left: ctypes::intptr_t, right: ctypes::intptr_t);
	fn cairo_matrix_transform_distance(matrix: ctypes::intptr_t, x: *f64, y: *f64);
	fn cairo_matrix_transform_point(matrix: ctypes::intptr_t, x: *f64, y: *f64);
	
	fn cairo_pattern_destroy(pattern: ctypes::intptr_t);
	fn cairo_pattern_reference(pattern: ctypes::intptr_t) -> ctypes::intptr_t;
	fn cairo_pattern_create_rgb(red: f64, blue: f64, green: f64) -> ctypes::intptr_t;
	fn cairo_pattern_create_rgba(red: f64, blue: f64, green: f64, alpha: f64) -> ctypes::intptr_t;
	fn cairo_pattern_create_linear(x0: f64, y0: f64, x1: f64, y1: f64) -> ctypes::intptr_t;
	fn cairo_pattern_create_radial(cx0: f64, cy0: f64, radius0: f64, cx1: f64, cy1: f64, radius1: f64) -> ctypes::intptr_t;
	fn cairo_pattern_create_for_surface(pattern: ctypes::intptr_t) -> ctypes::intptr_t;
	fn cairo_pattern_get_type(pattern: ctypes::intptr_t) -> ctypes::c_int;
	fn cairo_pattern_get_matrix(pattern: ctypes::intptr_t, matrix: ctypes::intptr_t);
	fn cairo_pattern_set_matrix(pattern: ctypes::intptr_t, matrix: ctypes::intptr_t);
	fn cairo_pattern_get_filter(pattern: ctypes::intptr_t) -> ctypes::c_int;
	fn cairo_pattern_set_filter(pattern: ctypes::intptr_t, filter: ctypes::c_int);
	fn cairo_pattern_get_extend(pattern: ctypes::intptr_t) -> ctypes::c_int;
	fn cairo_pattern_set_extend(pattern: ctypes::intptr_t, extend: ctypes::c_int);
	fn cairo_pattern_status(pattern: ctypes::intptr_t) -> ctypes::c_int;
	fn cairo_pattern_get_color_stop_count(pattern: ctypes::intptr_t, count: *ctypes::c_int);
	fn cairo_pattern_get_surface(pattern: ctypes::intptr_t, surface: *ctypes::intptr_t);
	fn cairo_pattern_add_color_stop_rgb(pattern: ctypes::intptr_t, offset: f64, red: f64, green: f64, blue: f64);
	fn cairo_pattern_add_color_stop_rgba(pattern: ctypes::intptr_t, offset: f64, red: f64, green: f64, blue: f64, alpha: f64);
	fn cairo_pattern_get_color_stop_rgba(pattern: ctypes::intptr_t, index: ctypes::c_int, offset: *f64, red: *f64, green: *f64, blue: *f64, alpha: *f64);
	fn cairo_pattern_get_rgba(pattern: ctypes::intptr_t, red: *f64, green: *f64, blue: *f64, alpha: *f64);
	fn cairo_pattern_get_linear_points(pattern: ctypes::intptr_t, x0: *f64, y0: *f64, x1: *f64, y1: *f64);
	fn cairo_pattern_get_radial_circles(pattern: ctypes::intptr_t, cx0: *f64, cy0: *f64, radius0: *f64, cx1: *f64, cy1: *f64, radius1: *f64);
	
	fn cairo_format_stride_for_width(format: ctypes::c_int, width: ctypes::c_int) -> ctypes::c_int;
	fn cairo_surface_create_similar(surface: ctypes::intptr_t, content: ctypes::c_int, width: ctypes::c_int, height: ctypes::c_int) -> ctypes::intptr_t;
	fn cairo_image_surface_create_from_png(data: *u8) -> ctypes::intptr_t;
	fn cairo_image_surface_create_for_data(data: *u8, format: ctypes::c_int, width: ctypes::c_int, height: ctypes::c_int, stride: ctypes::c_int) -> ctypes::intptr_t;
	fn cairo_surface_write_to_png(surface: ctypes::intptr_t,file: *u8) -> ctypes::c_int;
	fn cairo_surface_destroy(surface: ctypes::intptr_t);
	fn cairo_image_surface_create(format: ctypes::c_int, width: ctypes::c_int, height: ctypes::c_int) -> ctypes::intptr_t;
	fn cairo_surface_has_show_text_glyphs(surface: ctypes::intptr_t) -> ctypes::c_int;
	fn cairo_surface_show_page(surface: ctypes::intptr_t);
	fn cairo_surface_copy_page(surface: ctypes::intptr_t);
	fn cairo_surface_get_type(surface: ctypes::intptr_t) -> ctypes::c_int;
	fn cairo_surface_set_fallback_resolution(surface: ctypes::intptr_t, x: f64, y: f64);
	fn cairo_surface_get_fallback_resolution(surface: ctypes::intptr_t, x: *f64, y: *f64);
	fn cairo_surface_set_device_offset(surface: ctypes::intptr_t, x: f64, y: f64);
	fn cairo_surface_get_device_offset(surface: ctypes::intptr_t, x: *f64, y: *f64);
	fn cairo_surface_mark_dirty(surface: ctypes::intptr_t);
	fn cairo_surface_mark_dirty_rectangle(surface: ctypes::intptr_t, x: ctypes::c_int, y: ctypes::c_int, width: ctypes::c_int, height: ctypes::c_int);
	fn cairo_surface_get_content(surface: ctypes::intptr_t) -> ctypes::c_int;
	fn cairo_surface_get_font_options(surface: ctypes::intptr_t, options: ctypes::intptr_t);
	fn cairo_surface_get_device(surface: ctypes::intptr_t) -> ctypes::intptr_t;
	fn cairo_surface_status(surface: ctypes::intptr_t) -> ctypes::c_int;
	fn cairo_surface_flush(surface: ctypes::intptr_t);
	fn cairo_surface_reference(surface: ctypes::intptr_t) -> ctypes::intptr_t;
	
	fn cairo_device_reference(device: ctypes::intptr_t) -> ctypes::intptr_t;
	fn cairo_device_destroy(device: ctypes::intptr_t);
	fn cairo_device_flush(device: ctypes::intptr_t);
	fn cairo_device_acquire(device: ctypes::intptr_t) -> ctypes::c_int;
	fn cairo_device_release(device: ctypes::intptr_t);
	fn cairo_device_get_type(device: ctypes::intptr_t) -> ctypes::c_int;
	
	fn cairo_status_to_string(status: ctypes::c_int) -> *u8;
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

fn status_to_string(status: status) -> str unsafe {
	ret str::str_from_cstr(ccairo::cairo_status_to_string(status));
}

const FORMAT_INVALID: int = -1;
const FORMAT_ARGB32: int = 0;
const FORMAT_RGB24: int = 1;
const FORMAT_A8: int = 2;
const FORMAT_A1: int = 3;
const FORMAT_RGB16_565: int = 4;

type format = int;

fn stride_for_width(format: format, width: int) -> int {
	ret ccairo::cairo_format_stride_for_width(format, width);
}

const FONT_SLANT_NORMAL: int = 0;
const FONT_SLANT_ITALIC: int = 1;
const FONT_SLANT_OBLIQUE: int = 0;

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
	ascent: f64,
	descent: f64,
	height: f64,
	max_x_advance: f64,
	max_y_advance: f64
};
type text_extents_record = {
	x_bearing: f64,
	y_bearing: f64,
	width: f64,
	height: f64,
	x_advance: f64,
	y_advance: f64
};
type matrix_record = {
	xx: f64,
	yx: f64,
	xy: f64,
	yy: f64,
	x0: f64,
	y0: f64
};
type glyph_record = {
	index: ctypes::ulong,
	x: f64,
	y: f64
};
type text_cluster_record = {
	num_bytes: ctypes::c_int,
	num_glyphs: ctypes::c_int
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

const CAIRO_FILTER_FAST: int = 0;
const CAIRO_FILTER_GOOD: int = 1;
const CAIRO_FILTER_BEST: int = 2;
const CAIRO_FILTER_NEAREST: int = 3;
const CAIRO_FILTER_BILINEAR: int = 4;
const CAIRO_FILTER_GAUSSIAN: int = 5;

type filter = int;

const CAIRO_PATTERN_TYPE_SOLID: int = 0;
const CAIRO_PATTERN_TYPE_SURFACE: int = 1;
const CAIRO_PATTERN_TYPE_LINEAR: int = 2;
const CAIRO_PATTERN_TYPE_RADIAL: int = 3;

type pattern_type = int;

/*
 * Device
 */

obj device(internal: ctypes::intptr_t, res: @device_res) {
	// General
	
	fn flush() {
		ccairo::cairo_device_flush(internal);
	}
	fn get_type() -> device_type {
		ret ccairo::cairo_device_get_type(internal) as device_type;
	}
	fn acquire() -> status {
		ret ccairo::cairo_device_acquire(internal);
	}
	fn release() {
		ccairo::cairo_device_release(internal);
	}

	// Misc
	
	fn get_internal() -> ctypes::intptr_t {
		ret internal;
	}
}

resource device_res(internal: ctypes::intptr_t) {
	ccairo::cairo_device_destroy(internal);
}


/*
 * Surface
 */

obj surface(internal: ctypes::intptr_t, res: @surface_res) {
	// General

	fn write_to_file(file: str) -> status unsafe { // TODO more image formats
		let path = std::fs::make_absolute(file);
		let split = std::fs::splitext(path);
		let bytes = str::bytes(path);
		
		vec::push(bytes, 0 as u8);
		
		alt split {
			(base, ".png") {
				ret ccairo::cairo_surface_write_to_png(internal, vec::unsafe::to_ptr(bytes)) as status;
			}
			(base, _) {
				ret STATUS_WRITE_ERROR;
			}
		}
	}
	fn get_status() -> status {
		ret ccairo::cairo_surface_status(internal) as status;
	}
	fn flush() {
		ccairo::cairo_surface_flush(internal);
	}
	fn get_device() -> device {
		let other_internal: ctypes::intptr_t = ccairo::cairo_device_reference(ccairo::cairo_surface_get_device(internal));
		let other_res = @device_res(other_internal);
	
		ret device(other_internal, other_res);
	}
	fn get_font_options() -> font_options {
		let options: font_options = mk_font_options();
		
		ccairo::cairo_surface_get_font_options(internal, options.get_internal());
		
		ret options;
	}
	fn get_content() -> content {
		ret ccairo::cairo_surface_get_content(internal) as content;
	}
	fn mark_dirty() {
		ccairo::cairo_surface_mark_dirty(internal);
	}
	fn mark_dirty_rectangle(x: int, y: int, width: int, height: int) {
		ccairo::cairo_surface_mark_dirty_rectangle(internal, x, y, width, height);
	}
	fn set_device_offset(x: float, y: float) {
		ccairo::cairo_surface_set_device_offset(internal, x, y);
	}
	fn get_device_offset() -> (float, float) {
		let x: f64 = 0.0;
		let y: f64 = 0.0;
		
		ccairo::cairo_surface_get_device_offset(internal, ptr::addr_of(x), ptr::addr_of(y));
		
		ret (x, y);
	}
	fn set_fallback_resolution(x_ppi: float, y_ppi: float) {
		ccairo::cairo_surface_set_fallback_resolution(internal, x_ppi, y_ppi);
	}
	fn get_fallback_resolution() -> (float, float) {
		let x_ppi: f64 = 0.0;
		let y_ppi: f64 = 0.0;
		
		ccairo::cairo_surface_get_fallback_resolution(internal, ptr::addr_of(x_ppi), ptr::addr_of(y_ppi));
		
		ret (x_ppi, y_ppi);
	}
	fn get_type() -> surface_type {
		ret ccairo::cairo_surface_get_type(internal) as surface_type;
	}
	fn copy_page() {
		ccairo::cairo_surface_copy_page(internal);
	}
	fn show_page() {
		ccairo::cairo_surface_show_page(internal);
	}
	fn has_show_text_glyphs() -> bool {
		ret ccairo::cairo_surface_has_show_text_glyphs(internal) == 1;
	}
	
	// Misc
	
	fn get_internal() -> ctypes::intptr_t {
		ret internal;
	}
}

resource surface_res(internal: ctypes::intptr_t) {
	ccairo::cairo_surface_destroy(internal);
}

fn mk_surface_from_similar(other: surface, content: content, width: int, height: int) -> surface unsafe {
	let internal: ctypes::intptr_t = ccairo::cairo_surface_create_similar(other.get_internal(), content, width, height);
	let res = @surface_res(internal);
	
	ret surface(internal, res);
}
fn mk_image_surface(format: format, width: int, height: int) -> surface {
	let internal: ctypes::intptr_t = ccairo::cairo_image_surface_create(format, width, height);
	let res = @surface_res(internal);
	
	ret surface(internal, res);
}
fn mk_image_surface_from_file(file: str) -> surface unsafe {
	// Only PNG TODO add libjpeg and others later on (because that would be a cool extended feature)
	let path = std::fs::make_absolute(file);
	let split = std::fs::splitext(path);
	let bytes = str::bytes(path);
	let internal: ctypes::intptr_t;
	
	vec::push(bytes, 0 as u8);
		
	alt split {
		(base, ".png") {
			internal = ccairo::cairo_image_surface_create_from_png(vec::unsafe::to_ptr(bytes));
		}
		(base, _) {
			fail;
		}
	}

	let res = @surface_res(internal);
	
	ret surface(internal, res);
}
fn mk_image_surface_from_data(data: [u8], format: format, width: int, height: int, stride: int) -> surface unsafe {
	let internal: ctypes::intptr_t = ccairo::cairo_image_surface_create_for_data(vec::unsafe::to_ptr(data), format, width, height, stride);
	let res = @surface_res(internal);
	
	ret surface(internal, res);
}

/*
 * Pattern
 */

obj pattern(internal: ctypes::intptr_t, res: @pattern_res) {
	// General TODO
	
	fn add_color_stop_rgb(offset: float, red: float, green: float, blue: float) {
		ccairo::cairo_pattern_add_color_stop_rgb(internal, offset, red, green, blue);
	}
	fn add_color_stop_rgba(offset: float, red: float, green: float, blue: float, alpha: float) {
		ccairo::cairo_pattern_add_color_stop_rgba(internal, offset, red, green, blue, alpha);
	}
	fn get_color_stop_count() -> int {
		let count: ctypes::c_int = 0;
		
		ccairo::cairo_pattern_get_color_stop_count(internal, ptr::addr_of(count));
		
		ret count;
	}
	fn get_color_stop(index: int) -> (float, float, float, float, float) {
		let offset: f64 = 0.0;
		let red: f64 = 0.0;
		let green: f64 = 0.0;
		let blue: f64 = 0.0;
		let alpha: f64 = 0.0;
		
		ccairo::cairo_pattern_get_color_stop_rgba(internal, index, ptr::addr_of(offset), ptr::addr_of(red), ptr::addr_of(green), ptr::addr_of(blue), ptr::addr_of(alpha));
		
		ret (offset, red, green, blue, alpha);
	}
	fn get_rgba() -> (float, float, float, float) {
		let red: f64 = 0.0;
		let green: f64 = 0.0;
		let blue: f64 = 0.0;
		let alpha: f64 = 0.0;
		
		ccairo::cairo_pattern_get_rgba(internal, ptr::addr_of(red), ptr::addr_of(green), ptr::addr_of(blue), ptr::addr_of(alpha));
		
		ret (red, green, blue, alpha);
	}
	fn get_surface() -> surface {
		let other_internal: ctypes::intptr_t = 0 as ctypes::intptr_t;
		
		ccairo::cairo_pattern_get_surface(internal, ptr::addr_of(other_internal));
		
		let other_res = @surface_res(other_internal);
	
		ret surface(other_internal, other_res);
	}
	fn get_linear_points() -> (float, float, float, float) {
		let x0: f64 = 0.0;
		let y0: f64 = 0.0;
		let x1: f64 = 0.0;
		let y1: f64 = 0.0;
		
		ccairo::cairo_pattern_get_linear_points(internal, ptr::addr_of(x0), ptr::addr_of(y0), ptr::addr_of(x1), ptr::addr_of(y1));
			
		ret (x0, y0, x1, y1);
	}
	fn get_radial_circles() -> (float, float, float, float, float, float) {
		let x0: f64 = 0.0;
		let y0: f64 = 0.0;
		let r0: f64 = 0.0;
		let x1: f64 = 0.0;
		let y1: f64 = 0.0;
		let r1: f64 = 0.0;
		
		ccairo::cairo_pattern_get_radial_circles(internal, ptr::addr_of(x0), ptr::addr_of(y0), ptr::addr_of(r0), ptr::addr_of(x1), ptr::addr_of(y1), ptr::addr_of(r1));
			
		ret (x0, y0, r0, x1, y1, r1);
	}
	fn get_status() -> status {
		ret ccairo::cairo_pattern_status(internal) as status;
	}
	fn set_extend(extend: extend) {
		ccairo::cairo_pattern_set_extend(internal, extend);
	}
	fn get_extend() -> extend {
		ret ccairo::cairo_pattern_get_extend(internal) as extend;
	}
	fn set_filter(filter: filter) {
		ccairo::cairo_pattern_set_filter(internal, filter);
	}
	fn get_extend() -> extend {
		ret ccairo::cairo_pattern_get_filter(internal) as filter;
	}
	fn set_matrix(matrix: matrix) {
		ccairo::cairo_pattern_set_matrix(internal, matrix.get_internal());
	}
	fn get_matrix() -> matrix {
		let matrix: matrix = mk_matrix([
			0.0, 0.0,
			0.0, 0.0,
			0.0, 0.0
		]);
		
		ccairo::cairo_pattern_get_matrix(internal, matrix.get_internal());
		
		ret matrix;
	}
	fn get_type() -> pattern_type {
		ret ccairo::cairo_pattern_get_type(internal) as pattern_type;
	}

	// Misc
	
	fn get_internal() -> ctypes::intptr_t {
		ret internal;
	}
}

resource pattern_res(internal: ctypes::intptr_t) {
	ccairo::cairo_pattern_destroy(internal);
}

fn mk_pattern_from_rgb(red: float, green: float, blue: float) -> pattern {
	let internal: ctypes::intptr_t = ccairo::cairo_pattern_create_rgb(red, green, blue);
	let res = @pattern_res(internal);
	
	ret pattern(internal, res);
}
fn mk_pattern_from_rgba(red: float, green: float, blue: float, alpha: float) -> pattern {
	let internal: ctypes::intptr_t = ccairo::cairo_pattern_create_rgba(red, green, blue, alpha);
	let res = @pattern_res(internal);
	
	ret pattern(internal, res);
}
fn mk_pattern_from_linear_gradient(x0: float, y0: float, x1: float, y1: float) -> pattern {
	let internal: ctypes::intptr_t = ccairo::cairo_pattern_create_linear(x0, y0, x1, y1);
	let res = @pattern_res(internal);
	
	ret pattern(internal, res);
}
fn mk_pattern_from_radial_gradient(cx0: float, cy0: float, radius0: float, cx1: float, cy1: float, radius1: float) -> pattern {
	let internal: ctypes::intptr_t = ccairo::cairo_pattern_create_radial(cx0, cy0, radius0, cx1, cy1, radius1);
	let res = @pattern_res(internal);
	
	ret pattern(internal, res);
}
fn mk_pattern_from_surface(surface: surface) -> pattern {
	let internal: ctypes::intptr_t = ccairo::cairo_pattern_create_for_surface(surface.get_internal());
	let res = @pattern_res(internal);
	
	ret pattern(internal, res);
}

/*
 * Matrix
 */

obj matrix(internal: ctypes::intptr_t, res: @matrix_res) {
	// Misc	

	fn get_internal() -> ctypes::intptr_t {
		ret internal;
	}
	fn get_record() -> matrix_record unsafe {
		ret *(internal as *matrix_record);
	}
	
	// General
	
	fn get_values() -> [float] {
		let record: matrix_record = self.get_record();
		
		ret [
			record.xx, record.yx,
			record.xy, record.yy,
			record.x0, record.y0
		];
	}
	fn set_values(values: [float]) {
		ccairo::cairo_matrix_init(internal, values[0], values[1], values[2], values[3], values[4], values[5]);
	}
	fn set_identity() {
		ccairo::cairo_matrix_init_identity(internal);
	}
	fn set_translate(x: float, y: float) {
		ccairo::cairo_matrix_init_translate(internal, x, y);
	}
	fn set_scale(x: float, y: float) {
		ccairo::cairo_matrix_init_scale(internal, x, y);
	}
	fn set_rotate(angle: float) {
		ccairo::cairo_matrix_init_rotate(internal, angle);
	}
	fn translate(x: float, y: float) {
		ccairo::cairo_matrix_translate(internal, x, y);
	}
	fn scale(x: float, y: float) {
		ccairo::cairo_matrix_scale(internal, x, y);
	}
	fn rotate(angle: float) {
		ccairo::cairo_matrix_rotate(internal, angle);
	}
	fn invert() {
		ccairo::cairo_matrix_invert(internal);
	}
	/*FIXME fn multiply(other: matrix) {
		ccairo::cairo_matrix_multiply(internal, internal, other.get_internal());
	}*/
	fn transform_distance(x: float, y: float) -> (float, float) {
		let xt: f64 = x;
		let yt: f64 = y;
		
		ccairo::cairo_matrix_transform_distance(internal, ptr::addr_of(xt), ptr::addr_of(yt));
		
		ret (xt, yt);
	}
	fn transform_point(x: float, y: float) -> (float, float) {
		let xt: f64 = x;
		let yt: f64 = y;
		
		ccairo::cairo_matrix_transform_point(internal, ptr::addr_of(xt), ptr::addr_of(yt));
		
		ret (xt, yt);
	}
}

resource matrix_res(internal: ctypes::intptr_t) {
	internal;
}

fn mk_matrix(values: [float]) -> matrix {
	let record: matrix_record = {
		xx: 0.0,
		yx: 0.0,
		xy: 0.0,
		yy: 0.0,
		x0: 0.0,
		y0: 0.0
	};
	let internal: ctypes::intptr_t = ptr::addr_of(record) as ctypes::intptr_t;
	let res = @matrix_res(internal);
	let result: matrix = matrix(internal, res);
	
	result.set_values(values);
	
	ret result;
}

/*
 * Path
 */

obj path(internal: ctypes::intptr_t, res: @path_res) {
	// Misc

	fn get_internal() -> ctypes::intptr_t {
		ret internal;
	}
}

resource path_res(internal: ctypes::intptr_t) {
	ccairo::cairo_path_destroy(internal);
}

/*
 * Text
 */

obj glyph(internal: ctypes::intptr_t, res: @glyph_res) {
	// General
	
	fn get_index() -> uint {
		ret self.get_record().index;
	}
	fn get_x() -> float {
		ret self.get_record().x;
	}
	fn get_y() -> float {
		ret self.get_record().y;
	}

	// Misc

	fn get_record() -> glyph_record unsafe {
		ret *(internal as *glyph_record);
	}
	fn get_internal() -> ctypes::intptr_t {
		ret internal;
	}
}

resource glyph_res(internal: ctypes::intptr_t) {
	internal;
}

fn mk_glyph(index: uint, x: float, y: float) -> glyph {
	let record: glyph_record = {
		index: index,
		x: x,
		y: y
	};
	let internal: ctypes::intptr_t = ptr::addr_of(record) as ctypes::intptr_t;
	let res = @glyph_res(internal);
	
	ret glyph(internal, res);
}

obj text_cluster(internal: ctypes::intptr_t, res: @text_cluster_res) {
	// General
	
	fn get_num_bytes() -> int {
		ret self.get_record().num_bytes;
	}
	fn get_num_glyphs() -> int {
		ret self.get_record().num_glyphs;
	}

	// Misc

	fn get_record() -> text_cluster_record unsafe {
		ret *(internal as *text_cluster_record);
	}
	fn get_internal() -> ctypes::intptr_t {
		ret internal;
	}
}

resource text_cluster_res(internal: ctypes::intptr_t) {
	internal;
}

fn mk_text_cluster(num_bytes: int, num_glyphs: int) -> text_cluster {
	let record: text_cluster_record = {
		num_bytes: num_bytes,
		num_glyphs: num_glyphs
	};
	let internal: ctypes::intptr_t = ptr::addr_of(record) as ctypes::intptr_t;
	let res = @text_cluster_res(internal);
	
	ret text_cluster(internal, res);
}

obj font_extents(internal: ctypes::intptr_t, res: @font_extents_res) {	
	// Misc

	fn get_internal() -> ctypes::intptr_t {
		ret internal;
	}
	fn get_record() -> font_extents_record unsafe {
		ret *(internal as *font_extents_record);
	}
	
	// General
	
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

resource font_extents_res(internal: ctypes::intptr_t) {
	internal;
}

obj text_extents(internal: ctypes::intptr_t, res: @text_extents_res) {
	// Misc

	fn get_internal() -> ctypes::intptr_t {
		ret internal;
	}
	fn get_record() -> text_extents_record unsafe {
		ret *(internal as *text_extents_record);
	}
	
	// General
	
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
	fn get_x_advance() -> float {
		ret self.get_record().x_advance;
	}
	fn get_y_bearing() -> float {
		ret self.get_record().y_bearing;
	}
}

resource text_extents_res(internal: ctypes::intptr_t) {
	internal;
}

obj font_face(internal: ctypes::intptr_t, res: @font_face_res) {
	// General
	
	fn get_status() -> status {
		ret ccairo::cairo_font_face_status(internal) as status;
	}
	fn get_type() -> font_type {
		ret ccairo::cairo_font_face_get_type(internal) as font_type;
	}
	fn get_toy_slant() -> font_slant {
		ret ccairo::cairo_toy_font_face_get_slant(internal) as font_slant;
	}
	fn get_toy_weight() -> font_weight {
		ret ccairo::cairo_toy_font_face_get_weight(internal) as font_weight;
	}
	fn get_toy_family() -> str unsafe {
		ret str::str_from_cstr(ccairo::cairo_toy_font_face_get_family(internal));
	}
	
	// Misc

	fn get_internal() -> ctypes::intptr_t {
		ret internal;
	}
}

resource font_face_res(internal: ctypes::intptr_t) {
	ccairo::cairo_font_face_destroy(internal);
}

fn mk_font_face_from_toy_font(family: str, slant: font_slant, weight: font_weight) -> font_face unsafe {
	let bytes = str::bytes(family);
	
	vec::push(bytes, 0 as u8);
	
	let internal: ctypes::intptr_t = ccairo::cairo_toy_font_face_create(vec::unsafe::to_ptr(bytes), slant, weight);
	let res = @font_face_res(internal);
	
	ret font_face(internal, res);
}

// TODO mk_font_face_from_file

obj scaled_font(internal: ctypes::intptr_t, res: @scaled_font_res) {
	// General
	
	fn get_status() -> status {
		ret ccairo::cairo_scaled_font_status(internal) as status;
	}
	fn extents() -> font_extents {
		let record: font_extents_record = {
			ascent: 0.0,
			descent: 0.0,
			height: 0.0,
			max_x_advance: 0.0,
			max_y_advance: 0.0
		};
		let other_internal: ctypes::intptr_t = ptr::addr_of(record) as ctypes::intptr_t;
		let other_res = @font_extents_res(other_internal);
		
		ccairo::cairo_scaled_font_extents(internal, other_internal);
		
		ret font_extents(other_internal, other_res);
	}
	fn text_extents(text: str) -> text_extents unsafe {
		let bytes = str::bytes(text);
		let record: text_extents_record = {
			x_bearing: 0.0,
			y_bearing: 0.0,
			width: 0.0,
			height: 0.0,
			x_advance: 0.0,
			y_advance: 0.0
		};
		let other_internal: ctypes::intptr_t = ptr::addr_of(record) as ctypes::intptr_t;
		let other_res = @text_extents_res(other_internal);
		
		vec::push(bytes, 0 as u8);
		
		ccairo::cairo_scaled_font_text_extents(internal, vec::unsafe::to_ptr(bytes), other_internal);
		
		ret text_extents(other_internal, other_res);
	}
	fn glyph_extents(glyphs: [glyph]) -> text_extents unsafe {
		let record: text_extents_record = {
			x_bearing: 0.0,
			y_bearing: 0.0,
			width: 0.0,
			height: 0.0,
			x_advance: 0.0,
			y_advance: 0.0
		};
		let other_internal: ctypes::intptr_t = ptr::addr_of(record) as ctypes::intptr_t;
		let other_res = @text_extents_res(other_internal);
		let cglyphs: [ctypes::intptr_t] = [];
		
		for glyph in glyphs {
			cglyphs += [glyph.get_internal()];
		}
		
		ccairo::cairo_scaled_font_glyph_extents(internal, vec::unsafe::to_ptr(cglyphs) as ctypes::intptr_t, vec::len(cglyphs) as ctypes::c_int, other_internal);
		
		ret text_extents(other_internal, other_res);
	}
	fn get_font_face() -> font_face {
		let other_internal: ctypes::intptr_t = ccairo::cairo_font_face_reference(ccairo::cairo_scaled_font_get_font_face(internal));
		let other_res = @font_face_res(other_internal);
		
		ret font_face(other_internal, other_res);
	}
	fn get_font_options() -> font_options {
		let options: font_options = mk_font_options();
		
		ccairo::cairo_scaled_font_get_font_options(internal, options.get_internal());
		
		ret options;
	}
	fn get_font_matrix() -> matrix {
		let matrix: matrix = mk_matrix([
			0.0, 0.0,
			0.0, 0.0,
			0.0, 0.0
		]);
		
		ccairo::cairo_scaled_font_get_font_matrix(internal, matrix.get_internal());
		
		ret matrix;
	}
	fn get_ctm() -> matrix {
		let matrix: matrix = mk_matrix([
			0.0, 0.0,
			0.0, 0.0,
			0.0, 0.0
		]);
		
		ccairo::cairo_scaled_font_get_ctm(internal, matrix.get_internal());
		
		ret matrix;
	}
	fn get_scale_matrix() -> matrix {
		let matrix: matrix = mk_matrix([
			0.0, 0.0,
			0.0, 0.0,
			0.0, 0.0
		]);
		
		ccairo::cairo_scaled_font_get_scale_matrix(internal, matrix.get_internal());
		
		ret matrix;
	}
	fn get_type() -> font_type {
		ret ccairo::cairo_scaled_font_get_type(internal) as font_type;
	}
	
	// Misc

	fn get_internal() -> ctypes::intptr_t {
		ret internal;
	}
}

resource scaled_font_res(internal: ctypes::intptr_t) {
	ccairo::cairo_scaled_font_destroy(internal);
}

obj font_options(internal: ctypes::intptr_t, res: @font_options_res) {
	// General
	
	fn get_status() -> status {
		ret ccairo::cairo_font_options_status(internal);
	}
	/* FIXME fn merge(other: font_options) {
	}*/
	fn hash() -> uint {
		ret ccairo::cairo_font_options_hash(internal) as uint;
	}
	/* FIXME fn equals(other: font_options) -> bool {
	}*/
	fn set_antialias(antialias: antialias) {
		ccairo::cairo_font_options_set_antialias(internal, antialias);
	}
	fn get_antialias() -> antialias {
		ret ccairo::cairo_font_options_get_antialias(internal) as antialias;
	}
	fn set_subpixel_order(order: subpixel_order) {
		ccairo::cairo_font_options_set_subpixel_order(internal, order);
	}
	fn get_subpixel_order() -> subpixel_order {
		ret ccairo::cairo_font_options_get_subpixel_order(internal) as subpixel_order;
	}
	fn set_hint_style(hint: hint_style) {
		ccairo::cairo_font_options_set_hint_style(internal, hint);
	}
	fn get_hint_style() -> hint_style {
		ret ccairo::cairo_font_options_get_hint_style(internal) as hint_style;
	}
	fn set_hint_metrics(hint: hint_metrics) {
		ccairo::cairo_font_options_set_hint_metrics(internal, hint);
	}
	fn get_hint_metrics() -> hint_metrics {
		ret ccairo::cairo_font_options_get_hint_metrics(internal) as hint_metrics;
	}
	
	// Misc

	fn get_internal() -> ctypes::intptr_t {
		ret internal;
	}
}

resource font_options_res(internal: ctypes::intptr_t) {
	ccairo::cairo_font_options_destroy(internal);
}

fn mk_font_options() -> font_options {
	let internal: ctypes::intptr_t = ccairo::cairo_font_options_create();
	let res = @font_options_res(internal);
	
	ret font_options(internal, res);
}
fn mk_font_options_from_copy(other: font_options) -> font_options {
	let internal: ctypes::intptr_t = ccairo::cairo_font_options_copy(other.get_internal());
	let res = @font_options_res(internal);
	
	ret font_options(internal, res);
}

/*
 * Context
 */

obj context(internal: ctypes::intptr_t, res: @context_res) {
	// General
	
	fn get_status() -> status {
		ret ccairo::cairo_status(internal) as status;
	}
	fn save() {
		ccairo::cairo_save(internal);
	}
	fn restore() {
		ccairo::cairo_restore(internal);
	}
	fn get_target() -> surface {
		let other_internal: ctypes::intptr_t = ccairo::cairo_surface_reference(ccairo::cairo_get_target(internal));
		let other_res = @surface_res(other_internal);
		
		ret surface(other_internal, other_res);
	}
	fn push_group() {
		ccairo::cairo_push_group(internal);
	}
	fn push_group_with_content(content: content) {
		ccairo::cairo_push_group_with_content(internal, content);
	}
	fn pop_group() -> pattern {
		let other_internal: ctypes::intptr_t = ccairo::cairo_pop_group(internal);
		let other_res = @pattern_res(other_internal);
		
		ret pattern(other_internal, other_res);
	}
	fn pop_group_to_source() {
		ccairo::cairo_pop_group_to_source(internal);
	}
	fn get_group_target() -> surface {
		let other_internal: ctypes::intptr_t = ccairo::cairo_surface_reference(ccairo::cairo_get_group_target(internal));
		let other_res = @surface_res(other_internal);
		
		ret surface(other_internal, other_res);
	}
	fn set_source_rgb(red: float, green: float, blue: float) {
		ccairo::cairo_set_source_rgb(internal, red, green, blue);
	}
	fn set_source_rgba(red: float, green: float, blue: float, alpha: float) {
		ccairo::cairo_set_source_rgba(internal, red, green, blue, alpha);
	}
	fn set_source(pattern: pattern) {
		ccairo::cairo_set_source(internal, pattern.get_internal());
	}
	fn set_source_surface(surface: surface, x: float, y: float) {
		ccairo::cairo_set_source_surface(internal, surface.get_internal(), x, y);
	}
	fn get_source() -> pattern {
		let other_internal: ctypes::intptr_t = ccairo::cairo_pattern_reference(ccairo::cairo_get_source(internal));
		let other_res = @pattern_res(other_internal);
		
		ret pattern(other_internal, other_res);
	}
	fn set_antialias(antialias: antialias) {
		ccairo::cairo_set_antialias(internal, antialias);
	}
	fn get_antialias() -> antialias {
		ret ccairo::cairo_get_antialias(internal) as antialias;
	}
	fn set_dash(dashes: [float], offset: float) unsafe {
		ccairo::cairo_set_dash(internal, vec::unsafe::to_ptr(dashes), vec::len(dashes) as ctypes::c_int, offset);
	}
	fn get_dash_count() -> int {
		ret ccairo::cairo_get_dash_count(internal);
	}
	fn get_dash() -> [float] unsafe {
		let dashes: [f64] = [];
		
		ccairo::cairo_get_dash(internal, vec::unsafe::to_ptr(dashes), ptr::null());
		
		ret dashes;
	}
	fn get_dash_offset() -> float {
		let offset: f64 = 0.0;
		
		ccairo::cairo_get_dash(internal, ptr::null(), ptr::addr_of(offset));
		
		ret offset;
	}
	fn set_fill_rule(rule: fill_rule) {
		ccairo::cairo_set_fill_rule(internal, rule);
	}
	fn get_fill_rule() -> fill_rule {
		ret ccairo::cairo_get_fill_rule(internal) as fill_rule;
	}
	fn set_line_cap(cap: line_cap) {
		ccairo::cairo_set_line_cap(internal, cap);
	}
	fn get_line_cap() -> line_cap {
		ret ccairo::cairo_get_line_cap(internal) as line_cap;
	}
	fn set_line_join(join: line_join) {
		ccairo::cairo_set_line_join(internal, join);
	}
	fn get_line_join() -> line_join {
		ret ccairo::cairo_get_line_join(internal) as line_join;
	}
	fn set_line_width(width: float) {
		ccairo::cairo_set_line_width(internal, width);
	}
	fn get_line_width() -> float {
		ret ccairo::cairo_get_line_width(internal);
	}
	fn set_miter_limit(limit: float) {
		ccairo::cairo_set_miter_limit(internal, limit);
	}
	fn get_miter_limit() -> float {
		ret ccairo::cairo_get_miter_limit(internal);
	}
	fn set_operator(op: operator) {
		ccairo::cairo_set_operator(internal, op);
	}
	fn get_operator() -> operator {
		ret ccairo::cairo_get_operator(internal) as operator;
	}
	fn set_tolerance(tol: float) {
		ccairo::cairo_set_tolerance(internal, tol);
	}
	fn get_tolerance() -> float {
		ret ccairo::cairo_get_tolerance(internal);
	}
	fn clip() {
		ccairo::cairo_clip(internal);
	}
	fn clip_preserve() {
		ccairo::cairo_clip_preserve(internal);
	}
	fn clip_extents() -> (float, float, float, float) {
		let x1: f64 = 0.0;
		let y1: f64 = 0.0;
		let x2: f64 = 0.0;
		let y2: f64 = 0.0;
		
		ccairo::cairo_clip_extents(internal, ptr::addr_of(x1), ptr::addr_of(y1), ptr::addr_of(x2), ptr::addr_of(y2));
		
		ret (x1, y1, x2, y2);
	}
	fn in_clip(x: float, y: float) -> bool {
		ret ccairo::cairo_in_clip(internal,x,y) == 1;
	}
	fn reset_clip() {
		ccairo::cairo_reset_clip(internal);
	}
	fn fill() {
		ccairo::cairo_fill(internal);
	}
	fn fill_preserve() {
		ccairo::cairo_fill_preserve(internal);
	}
	fn fill_extents() -> (float, float, float, float) {
		let x1: f64 = 0.0;
		let y1: f64 = 0.0;
		let x2: f64 = 0.0;
		let y2: f64 = 0.0;
		
		ccairo::cairo_fill_extents(internal, ptr::addr_of(x1), ptr::addr_of(y1), ptr::addr_of(x2), ptr::addr_of(y2));
		
		ret (x1, y1, x2, y2);
	}
	fn in_fill(x: float, y: float) -> bool {
		ret ccairo::cairo_in_fill(internal, x, y) == 1;
	}
	fn mask(pattern: pattern) {
		ccairo::cairo_mask(internal, pattern.get_internal());
	}
	fn mask_surface(surface: surface, x: float, y: float) {
		ccairo::cairo_mask_surface(internal, surface.get_internal(), x, y);
	}
	fn paint() {
		ccairo::cairo_paint(internal);
	}
	fn paint_with_alpha(alpha: float) {
		ccairo::cairo_paint_with_alpha(internal, alpha);
	}
	fn stroke() {
		ccairo::cairo_stroke(internal);
	}
	fn stroke_preserve() {
		ccairo::cairo_stroke_preserve(internal);
	}
	fn stroke_extents() -> (float, float, float, float) {
		let x1: f64 = 0.0;
		let y1: f64 = 0.0;
		let x2: f64 = 0.0;
		let y2: f64 = 0.0;
		
		ccairo::cairo_stroke_extents(internal, ptr::addr_of(x1), ptr::addr_of(y1), ptr::addr_of(x2), ptr::addr_of(y2));
		
		ret (x1, y1, x2, y2);
	}
	fn in_stroke(x: float, y: float) -> bool {
		ret ccairo::cairo_in_stroke(internal, x, y) == 1;
	}
	fn copy_page() {
		ccairo::cairo_copy_page(internal);
	}
	fn show_page() {
		ccairo::cairo_show_page(internal);
	}
	
	// Paths
	
	fn copy_path() -> path {
		let other_internal: ctypes::intptr_t = ccairo::cairo_copy_path(internal);
		let other_res = @path_res(other_internal);
		
		ret path(other_internal, other_res);
	}
	fn copy_path_flat() -> path {
		let other_internal: ctypes::intptr_t = ccairo::cairo_copy_path(internal);
		let other_res = @path_res(other_internal);
		
		ret path(other_internal, other_res);
	}
	fn append_path(path: path) {
		ccairo::cairo_append_path(internal, path.get_internal());
	}
	fn has_current_point() -> bool {
		ret ccairo::cairo_has_current_point(internal) == 1;
	}
	fn get_current_point() -> (float, float) {
		let x: f64 = 0.0;
		let y: f64 = 0.0;
		
		ccairo::cairo_get_current_point(internal, ptr::addr_of(x), ptr::addr_of(y));
		
		ret (x, y);
	}
	fn new_path() {
		ccairo::cairo_new_path(internal);
	}
	fn new_sub_path() {
		ccairo::cairo_new_sub_path(internal);
	}
	fn close_path() {
		ccairo::cairo_close_path(internal);
	}
	fn arc(x: float, y: float, radius: float, angle1: float, angle2: float) {
		ccairo::cairo_arc(internal, x, y, radius, angle1, angle2);
	}
	fn arc_negative(x: float, y: float, radius: float, angle1: float, angle2: float) {
		ccairo::cairo_arc_negative(internal, x, y, radius, angle1, angle2);
	}
	fn curve_to(x1: float, y1: float, x2: float, y2: float, x3: float, y3: float) {
		ccairo::cairo_curve_to(internal, x1, y1, x2, y2, x3, y3);
	}
	fn line_to(x: float, y: float) {
		ccairo::cairo_line_to(internal, x, y);
	}
	fn move_to(x: float, y: float) {
		ccairo::cairo_move_to(internal, x, y);
	}
	fn rectangle(x: float, y: float, width: float, height: float) {
		ccairo::cairo_rectangle(internal, x, y, width, height);
	}
	fn glyph_path(glyphs: [glyph]) unsafe {
		let cglyphs: [ctypes::intptr_t] = [];
		
		for glyph in glyphs {
			cglyphs += [glyph.get_internal()];
		}
		
		ccairo::cairo_glyph_path(internal, vec::unsafe::to_ptr(cglyphs) as ctypes::intptr_t, vec::len(cglyphs) as ctypes::c_int);
	}
	fn text_path(text: str) unsafe {
		let bytes = str::bytes(text);
		
		vec::push(bytes, 0 as u8);
		
		ccairo::cairo_text_path(internal, vec::unsafe::to_ptr(bytes));
	}
	fn rel_curve_to(x1: float, y1: float, x2: float, y2: float, x3: float, y3: float) {
		ccairo::cairo_rel_curve_to(internal, x1, y1, x2, y2, x3, y3);
	}
	fn rel_line_to(x: float, y: float) {
		ccairo::cairo_rel_line_to(internal, x, y);
	}
	fn rel_move_to(x: float, y: float) {
		ccairo::cairo_rel_move_to(internal, x, y);
	}
	fn path_extents() -> (float, float, float, float) {
		let x1: f64 = 0.0;
		let y1: f64 = 0.0;
		let x2: f64 = 0.0;
		let y2: f64 = 0.0;
		
		ccairo::cairo_path_extents(internal, ptr::addr_of(x1), ptr::addr_of(y1), ptr::addr_of(x2), ptr::addr_of(y2));
		
		ret (x1, y1, x2, y2);
	}
	
	// Transformations
	
	fn translate(x: float, y: float) {
		ccairo::cairo_translate(internal, x, y);
	}
	fn scale(x: float, y: float) {
		ccairo::cairo_scale(internal, x, y);
	}
	fn rotate(angle: float) {
		ccairo::cairo_rotate(internal, angle);
	}
	fn transform(matrix: matrix) {
		ccairo::cairo_transform(internal, matrix.get_internal());
	}
	fn set_matrix(matrix: matrix) {
		ccairo::cairo_set_matrix(internal, matrix.get_internal());
	}
	fn get_matrix() -> matrix {
		let other_internal: ctypes::intptr_t = ccairo::cairo_get_matrix(internal);
		let other_res = @matrix_res(other_internal);
		
		ret matrix(other_internal, other_res);
	}
	fn identity_matrix() {
		ccairo::cairo_identity_matrix(internal)
	}
	fn user_to_device(x: float, y: float) -> (float, float) {
		let xt: f64 = x;
		let yt: f64 = y;
		
		ccairo::cairo_user_to_device(internal, ptr::addr_of(x), ptr::addr_of(y));
		
		ret (xt, yt);
	}
	fn user_to_device_distance(x: float, y: float) -> (float, float) {
		let xt: f64 = x;
		let yt: f64 = y;
		
		ccairo::cairo_user_to_device_distance(internal, ptr::addr_of(x), ptr::addr_of(y));
		
		ret (xt, yt);
	}
	fn device_to_user(x: float, y: float) -> (float, float) {
		let xt: f64 = x;
		let yt: f64 = y;
		
		ccairo::cairo_device_to_user(internal, ptr::addr_of(x), ptr::addr_of(y));
		
		ret (xt, yt);
	}
	fn device_to_user_distance(x: float, y: float) -> (float, float) {
		let xt: f64 = x;
		let yt: f64 = y;
		
		ccairo::cairo_device_to_user_distance(internal, ptr::addr_of(x), ptr::addr_of(y));
		
		ret (xt, yt);
	}
	
	// Text
	
	fn select_font_face(face: str, slant: font_slant, weight: font_weight) unsafe {
		let bytes = str::bytes(face);
		
		vec::push(bytes, 0 as u8);
		
		ccairo::cairo_select_font_face(internal, vec::unsafe::to_ptr(bytes), slant, weight);
	}
	fn set_font_size(size: float) {
		ccairo::cairo_set_font_size(internal, size);
	}
	fn set_font_matrix(matrix: matrix) {
		ccairo::cairo_set_font_matrix(internal, matrix.get_internal());
	}
	fn get_font_matrix() -> matrix {
		let record: matrix_record = {
			xx: 0.0,
			yx: 0.0,
			xy: 0.0,
			yy: 0.0,
			x0: 0.0,
			y0: 0.0
		};
		let other_internal: ctypes::intptr_t = ptr::addr_of(record) as ctypes::intptr_t;
		let other_res = @matrix_res(other_internal);
		
		ccairo::cairo_get_font_matrix(internal, other_internal);
		
		ret matrix(other_internal, other_res);
	}
	fn set_font_options(options: font_options) {
		ccairo::cairo_set_font_options(internal, options.get_internal());
	}
	fn get_font_options() -> font_options {
		let other_internal: ctypes::intptr_t = ccairo::cairo_font_options_create();
		let other_res = @font_options_res(other_internal);
		
		ccairo::cairo_get_font_options(internal, other_internal);
	
		ret font_options(other_internal, other_res);
	}
	fn set_font_face(face: font_face) {
		ccairo::cairo_set_font_face(internal, face.get_internal());
	}
	fn get_font_face() -> font_face {
		let other_internal: ctypes::intptr_t = ccairo::cairo_font_face_reference(ccairo::cairo_get_font_face(internal));
		let other_res = @font_face_res(other_internal);
	
		ret font_face(other_internal, other_res);
	}
	fn set_scaled_font(font: scaled_font) {
		ccairo::cairo_set_scaled_font(internal, font.get_internal());
	}
	fn get_scaled_font() -> scaled_font {
		let other_internal: ctypes::intptr_t = ccairo::cairo_scaled_font_reference(ccairo::cairo_get_scaled_font(internal));
		let other_res = @scaled_font_res(other_internal);
	
		ret scaled_font(other_internal, other_res);
	}
	fn show_text(text: str) unsafe {
		let bytes = str::bytes(text);
		
		vec::push(bytes, 0 as u8);
		
		ccairo::cairo_show_text(internal, vec::unsafe::to_ptr(bytes));
	}
	fn show_glyphs (glyphs: [glyph]) unsafe {
		let cglyphs: [ctypes::intptr_t] = [];
		
		for glyph in glyphs {
			cglyphs += [glyph.get_internal()];
		}
		
		ccairo::cairo_show_glyphs(internal, vec::unsafe::to_ptr(cglyphs) as ctypes::intptr_t, vec::len(cglyphs) as ctypes::c_int);
	}
	fn show_text_glyphs(text: str, glyphs: [glyph], clusters: [text_cluster], cluster_flags: text_cluster_flags) unsafe {
		let cglyphs: [ctypes::intptr_t] = [];
		let cclusters: [ctypes::intptr_t] = [];
		let bytes = str::bytes(text);
		
		vec::push(bytes, 0 as u8);
		
		for glyph in glyphs {
			cglyphs += [glyph.get_internal()];
		}
		for cluster in clusters {
			cclusters += [cluster.get_internal()];
		}
		
		ccairo::cairo_show_text_glyphs(internal, vec::unsafe::to_ptr(bytes), vec::len(bytes) as ctypes::c_int, vec::unsafe::to_ptr(cglyphs) as ctypes::intptr_t, vec::len(cglyphs) as ctypes::c_int, vec::unsafe::to_ptr(cclusters) as ctypes::intptr_t, vec::len(cclusters) as ctypes::c_int, cluster_flags);
	}
	fn font_extents() -> font_extents {
		let record: font_extents_record = {
			ascent: 0.0,
			descent: 0.0,
			height: 0.0,
			max_x_advance: 0.0,
			max_y_advance: 0.0
		};
		let other_internal: ctypes::intptr_t = ptr::addr_of(record) as ctypes::intptr_t;
		let other_res = @font_extents_res(other_internal);
		
		ccairo::cairo_font_extents(internal, other_internal);
		
		ret font_extents(other_internal, other_res);
	}
	fn text_extents(text: str) -> text_extents unsafe {
		let bytes = str::bytes(text);
		let record: text_extents_record = {
			x_bearing: 0.0,
			y_bearing: 0.0,
			width: 0.0,
			height: 0.0,
			x_advance: 0.0,
			y_advance: 0.0
		};
		let other_internal: ctypes::intptr_t = ptr::addr_of(record) as ctypes::intptr_t;
		let other_res = @text_extents_res(other_internal);
		
		vec::push(bytes, 0 as u8);
		ccairo::cairo_text_extents(internal, vec::unsafe::to_ptr(bytes), other_internal);
		
		ret text_extents(other_internal, other_res);
	}
	fn glyph_extents(glyphs: [glyph]) -> text_extents unsafe {
		let record: text_extents_record = {
			x_bearing: 0.0,
			y_bearing: 0.0,
			width: 0.0,
			height: 0.0,
			x_advance: 0.0,
			y_advance: 0.0
		};
		let other_internal: ctypes::intptr_t = ptr::addr_of(record) as ctypes::intptr_t;
		let other_res = @text_extents_res(other_internal);
		let cglyphs: [ctypes::intptr_t] = [];
		
		for glyph in glyphs {
			cglyphs += [glyph.get_internal()];
		}
		
		ccairo::cairo_glyph_extents(internal, vec::unsafe::to_ptr(cglyphs) as ctypes::intptr_t, vec::len(cglyphs) as ctypes::c_int, other_internal);
		
		ret text_extents(other_internal, other_res);
	}
	
	// Misc
	
	fn get_internal() -> ctypes::intptr_t {
		ret internal;
	}
}

resource context_res(internal: ctypes::intptr_t) {
	ccairo::cairo_destroy(internal);
}

fn mk_context(surface: surface) -> context {
	let internal: ctypes::intptr_t = ccairo::cairo_create(surface.get_internal());
	let res = @context_res(internal);
	
	ret context(internal, res);
}
