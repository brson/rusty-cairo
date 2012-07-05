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

	SVG_VERSION_1_1,
	SVG_VERSION_1_2,
	svg_version,

	PDF_VERSION_1_4,
	PDF_VERSION_1_5,
	pdf_version,

	PS_VERSION_2,
	PS_VERSION_3,
	ps_version,

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
	wrap_device,
	surface,
	wrap_surface,
	mk_surface_from_similar,
	mk_image_surface,
	mk_image_surface_from_file,
	mk_image_surface_from_data,
	mk_pdf_surface,
	mk_svg_surface,
	mk_ps_surface,

	pattern,
	wrap_pattern,
	mk_pattern_from_rgb,
	mk_pattern_from_rgba,
	mk_pattern_from_linear_gradient,
	mk_pattern_from_radial_gradient,
	mk_pattern_from_surface,

	matrix,
	wrap_matrix,
	mk_matrix,

	path,
	wrap_path,

	glyph,
	wrap_glyph,
	mk_glyph,

	text_cluster,
	wrap_text_cluster,
	mk_text_cluster,

	font_extents,
	wrap_font_extents,
	text_extents,
	wrap_text_extents,

	font_face,
	wrap_font_face,
	mk_font_face_from_toy_font,
	mk_font_face_from_file,

	scaled_font,
	wrap_scaled_font,

	font_options,
	wrap_font_options,
	mk_font_options,
	mk_font_options_from_copy,

	context,
	wrap_context,
	mk_context;

/*
 * FIXME all wrapped objects need to be cached rather than 
 * recreating new objects every time for each internal cairo pointer
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
extern mod c {
	fn fopen(path: *u8, mode: *u8) -> libc::intptr_t;
	fn fclose(file: libc::intptr_t);
}

#[link_name = "freetype"]
#[abi = "cdecl"]
extern mod cft {
	fn FT_Init_FreeType(library: *libc::intptr_t) -> libc::c_int;
	fn FT_New_Face(library: libc::intptr_t, path: *u8, offset: libc::c_long, face: *libc::intptr_t) -> libc::c_int;
	fn FT_Done_Face(face: libc::intptr_t);
	fn FT_Done_FreeType(face: libc::intptr_t);
}

#[link_name = "cairo"]
#[abi = "cdecl"]
extern mod ccairo {
	fn cairo_ft_font_face_create_for_ft_face(ft_face: libc::intptr_t, flags: libc::c_int) -> libc::intptr_t;
	fn cairo_font_face_set_user_data(face: libc::intptr_t, key: libc::intptr_t, ft_face: libc::intptr_t, cb: libc::intptr_t);	

	fn cairo_version_string() -> *libc::c_char;
	fn cairo_create(surface: libc::intptr_t) -> libc::intptr_t;
	fn cairo_reference(context: libc::intptr_t) -> libc::intptr_t;
	fn cairo_destroy(context: libc::intptr_t);
	fn cairo_status(context: libc::intptr_t) -> libc::c_int;
	fn cairo_save(context: libc::intptr_t);
	fn cairo_restore(context: libc::intptr_t);
	fn cairo_get_target(context: libc::intptr_t) -> libc::intptr_t;
	fn cairo_push_group(context: libc::intptr_t);
	fn cairo_push_group_with_content(context: libc::intptr_t, content: libc::c_int);
	fn cairo_pop_group(context: libc::intptr_t) -> libc::intptr_t;
	fn cairo_pop_group_to_source(context: libc::intptr_t);
	fn cairo_get_group_target(context: libc::intptr_t) -> libc::intptr_t;
	fn cairo_set_source_rgb(context: libc::intptr_t, red: float, green: float, blue: float);
	fn cairo_set_source_rgba(context: libc::intptr_t, red: float, green: float, blue: float, alpha: float);
	fn cairo_set_source(context: libc::intptr_t, pattern: libc::intptr_t);
	fn cairo_set_source_surface(context: libc::intptr_t, surface: libc::intptr_t, x: float, y: float);
	fn cairo_get_source(context: libc::intptr_t) -> libc::intptr_t;
	fn cairo_set_antialias(context: libc::intptr_t, antialias: libc::c_int);
	fn cairo_get_antialias(context: libc::intptr_t) -> libc::c_int;
	fn cairo_set_dash(context: libc::intptr_t, dashes: *float, num_dashes: libc::c_int, offset: float);
	fn cairo_get_dash_count(context: libc::intptr_t) -> libc::c_int;
	fn cairo_get_dash(context: libc::intptr_t, dashes: *float, offset: *float);
	fn cairo_set_fill_rule(context: libc::intptr_t, rule: libc::c_int);
	fn cairo_get_fill_rule(context: libc::intptr_t) -> libc::c_int;
	fn cairo_set_line_cap(context: libc::intptr_t, cap: libc::c_int);
	fn cairo_get_line_cap(context: libc::intptr_t) -> libc::c_int;
	fn cairo_set_line_join(context: libc::intptr_t, join: libc::c_int);
	fn cairo_get_line_join(context: libc::intptr_t) -> libc::c_int;
	fn cairo_set_line_width(context: libc::intptr_t, width: float);
	fn cairo_get_line_width(context: libc::intptr_t) -> float;
	fn cairo_set_miter_limit(context: libc::intptr_t, limit: float);
	fn cairo_get_miter_limit(context: libc::intptr_t) -> float;
	fn cairo_set_operator(context: libc::intptr_t, join: libc::c_int);
	fn cairo_get_operator(context: libc::intptr_t) -> libc::c_int;
	fn cairo_set_tolerance(context: libc::intptr_t, limit: float);
	fn cairo_get_tolerance(context: libc::intptr_t) -> float;
	fn cairo_clip(context: libc::intptr_t);
	fn cairo_clip_preserve(context: libc::intptr_t);
	fn cairo_clip_extents(context: libc::intptr_t, x1: *float, y1: *float, x2: *float, y2: *float);
	fn cairo_in_clip(context: libc::intptr_t, x: float, y: float) -> libc::c_int;
	fn cairo_reset_clip(context: libc::intptr_t);
	fn cairo_fill(context: libc::intptr_t);
	fn cairo_fill_preserve(context: libc::intptr_t);
	fn cairo_fill_extents(context: libc::intptr_t, x1: *float, y1: *float, x2: *float, y2: *float);
	fn cairo_in_fill(context: libc::intptr_t, x: float, y: float) -> libc::c_int;
	fn cairo_mask(context: libc::intptr_t, pattern: libc::intptr_t);
	fn cairo_mask_surface(context: libc::intptr_t, surface: libc::intptr_t, surface_x: float, surface_y: float);
	fn cairo_paint(context: libc::intptr_t);
	fn cairo_paint_with_alpha(context: libc::intptr_t, alpha: float);
	fn cairo_stroke(context: libc::intptr_t);
	fn cairo_stroke_preserve(context: libc::intptr_t);
	fn cairo_stroke_extents(context: libc::intptr_t, x1: *float, y1: *float, x2: *float, y2: *float);
	fn cairo_in_stroke(context: libc::intptr_t, x: float, y: float) -> libc::c_int;
	fn cairo_copy_page(context: libc::intptr_t);
	fn cairo_show_page(context: libc::intptr_t);
	fn cairo_get_reference_count(context: libc::intptr_t) -> libc::c_uint;
	
	fn cairo_translate(context: libc::intptr_t, x: float, y: float);
	fn cairo_scale(context: libc::intptr_t, x: float, y: float);
	fn cairo_rotate(context: libc::intptr_t, angle: float);
	fn cairo_transform(context: libc::intptr_t, matrix: libc::intptr_t);
	fn cairo_set_matrix(context: libc::intptr_t, matrix: libc::intptr_t);
	fn cairo_get_matrix(context: libc::intptr_t) -> libc::intptr_t;
	fn cairo_identity_matrix(context: libc::intptr_t);
	fn cairo_user_to_device(context: libc::intptr_t, x: *float, y: *float);
	fn cairo_user_to_device_distance(context: libc::intptr_t, x: *float, y: *float);
	fn cairo_device_to_user(context: libc::intptr_t, x: *float, y: *float);
	fn cairo_device_to_user_distance(context: libc::intptr_t, x: *float, y: *float);
	
	fn cairo_copy_path(context: libc::intptr_t) -> libc::intptr_t;
	fn cairo_copy_path_flat(context: libc::intptr_t) -> libc::intptr_t;
	fn cairo_path_destroy(path: libc::intptr_t);
	fn cairo_append_path(context: libc::intptr_t, path: libc::intptr_t);
	fn cairo_has_current_point(context: libc::intptr_t) -> libc::c_int;
	fn cairo_get_current_point(context: libc::intptr_t, x: *float, y: *float);
	fn cairo_new_path(context: libc::intptr_t);
	fn cairo_new_sub_path(context: libc::intptr_t);
	fn cairo_close_path(context: libc::intptr_t);
	fn cairo_arc(context: libc::intptr_t, x: float, y: float, radius: float, angle1: float, angle2: float);
	fn cairo_arc_negative(context: libc::intptr_t, x: float, y: float, radius: float, angle1: float, angle2: float);
	fn cairo_curve_to(context: libc::intptr_t, x1: float, y1: float, x2: float, y2: float, x3: float, y3: float);
	fn cairo_line_to(context: libc::intptr_t, x: float, y: float);
	fn cairo_move_to(context: libc::intptr_t, x: float, y: float);
	fn cairo_rectangle(context: libc::intptr_t, x: float, y: float, width: float, height: float);
	fn cairo_glyph_path(context: libc::intptr_t, glyph: libc::intptr_t, num_glyphs: libc::c_int);
	fn cairo_text_path(context: libc::intptr_t, text: *u8);
	fn cairo_rel_curve_to(context: libc::intptr_t, x1: float, y1: float, x2: float, y2: float, x3: float, y3: float);
	fn cairo_rel_line_to(context: libc::intptr_t, x: float, y: float);
	fn cairo_rel_move_to(context: libc::intptr_t, x: float, y: float);
	fn cairo_path_extents(context: libc::intptr_t, x1: *float, y1: *float, x2: *float, y2: *float);
	
	fn cairo_select_font_face(context: libc::intptr_t, face: *u8, slant: libc::c_int, weight: libc::c_int);
	fn cairo_set_font_size(context: libc::intptr_t, size: float);
	fn cairo_set_font_matrix(context: libc::intptr_t, matrix: libc::intptr_t);
	fn cairo_get_font_matrix(context: libc::intptr_t, matrix: *libc::intptr_t);
	fn cairo_set_font_options(context: libc::intptr_t, options: libc::intptr_t);
	fn cairo_get_font_options(context: libc::intptr_t, options: *libc::intptr_t);
	fn cairo_set_font_face(context: libc::intptr_t, face: libc::intptr_t);
	fn cairo_get_font_face(context: libc::intptr_t) -> libc::intptr_t;
	fn cairo_set_scaled_font(context: libc::intptr_t, font: libc::intptr_t);
	fn cairo_get_scaled_font(context: libc::intptr_t) -> libc::intptr_t;
	fn cairo_show_text(context: libc::intptr_t, text: *u8);
	fn cairo_show_glyphs(context: libc::intptr_t, glyphs: libc::intptr_t, num_glyphs: libc::c_int);
	fn cairo_show_text_glyphs(context: libc::intptr_t, text: *u8, text_len: libc::c_int, glyphs: libc::intptr_t, num_glyphs: libc::c_int, clusters: libc::intptr_t, num_clusters: libc::c_int, cluster_flags: libc::c_int);
	fn cairo_font_extents(context: libc::intptr_t, extents: libc::intptr_t);
	fn cairo_text_extents(context: libc::intptr_t, text: *u8, extents: libc::intptr_t);
	fn cairo_glyph_extents(context: libc::intptr_t, glyphs: libc::intptr_t, num_glyphs: libc::c_int, extents: libc::intptr_t);
	fn cairo_toy_font_face_create(family: *u8, slant: libc::c_int, weight: libc::c_int) -> libc::intptr_t;
	fn cairo_toy_font_face_get_family(face: libc::intptr_t) -> *libc::c_char;
	fn cairo_toy_font_face_get_slant(face: libc::intptr_t) -> libc::c_int;
	fn cairo_toy_font_face_get_weight(face: libc::intptr_t) -> libc::c_int;
	fn cairo_glyph_allocate(num_glyphs: libc::c_int) -> libc::intptr_t;
	fn cairo_glyph_free(glyphs: libc::intptr_t);
	fn cairo_text_cluster_allocate(num_clusters: libc::c_int) -> libc::intptr_t;
	fn cairo_text_cluster_free(clusters: libc::intptr_t);
	
	fn cairo_font_options_create() -> libc::intptr_t;
	fn cairo_font_options_copy(options: libc::intptr_t) -> libc::intptr_t;
	fn cairo_font_options_destroy(options: libc::intptr_t);
	fn cairo_font_options_status(options: libc::intptr_t) -> libc::c_int;
	fn cairo_font_options_merge(options: libc::intptr_t, other: libc::intptr_t);
	fn cairo_font_options_hash(options: libc::intptr_t) -> libc::c_ulong;
	fn cairo_font_options_equal(options: libc::intptr_t, other: libc::intptr_t) -> libc::c_int;
	fn cairo_font_options_set_antialias(options: libc::intptr_t, value: libc::c_int);
	fn cairo_font_options_get_antialias(options: libc::intptr_t) -> libc::c_int;
	fn cairo_font_options_set_subpixel_order(options: libc::intptr_t, value: libc::c_int);
	fn cairo_font_options_get_subpixel_order(options: libc::intptr_t) -> libc::c_int;
	fn cairo_font_options_set_hint_style(options: libc::intptr_t, value: libc::c_int);
	fn cairo_font_options_get_hint_style(options: libc::intptr_t) -> libc::c_int;
	fn cairo_font_options_set_hint_metrics(options: libc::intptr_t, value: libc::c_int);
	fn cairo_font_options_get_hint_metrics(options: libc::intptr_t) -> libc::c_int;
	
	fn cairo_font_face_reference(face: libc::intptr_t) -> libc::intptr_t;
	fn cairo_font_face_destroy(face: libc::intptr_t);
	fn cairo_font_face_status(face: libc::intptr_t) -> libc::c_int;
	fn cairo_font_face_get_type(face: libc::intptr_t) -> libc::c_int;
	fn cairo_font_face_get_reference_count(face: libc::intptr_t) -> libc::c_uint;
	
	fn cairo_scaled_font_destroy(font: libc::intptr_t);
	fn cairo_scaled_font_get_reference_count(font: libc::intptr_t) -> libc::c_uint;
	fn cairo_scaled_font_reference(font: libc::intptr_t) -> libc::intptr_t;
	fn cairo_scaled_font_get_type(font: libc::intptr_t) -> libc::c_int;
	fn cairo_scaled_font_get_scale_matrix(font: libc::intptr_t, matrix: libc::intptr_t);
	fn cairo_scaled_font_get_ctm(font: libc::intptr_t, matrix: libc::intptr_t);
	fn cairo_scaled_font_get_font_matrix(font: libc::intptr_t, matrix: libc::intptr_t);
	fn cairo_scaled_font_get_font_options(font: libc::intptr_t, options: libc::intptr_t);
	fn cairo_scaled_font_get_font_face(font: libc::intptr_t) -> libc::intptr_t;
	fn cairo_scaled_font_extents(font: libc::intptr_t, extents: libc::intptr_t);
	fn cairo_scaled_font_text_extents(font: libc::intptr_t, text: *u8, extents: libc::intptr_t);
	fn cairo_scaled_font_glyph_extents(font: libc::intptr_t, glyphs: libc::intptr_t, num_glyphs: libc::c_int, extents: libc::intptr_t);
	fn cairo_scaled_font_status(font: libc::intptr_t) -> libc::c_int;
	
	fn cairo_matrix_init(matrix: libc::intptr_t, xx: float, xy: float, yx: float, yy: float, x0: float, y0: float);
	fn cairo_matrix_init_identity(matrix: libc::intptr_t);
	fn cairo_matrix_init_translate(matrix: libc::intptr_t, x: float, y: float);
	fn cairo_matrix_init_scale(matrix: libc::intptr_t, x: float, y: float);
	fn cairo_matrix_init_rotate(matrix: libc::intptr_t, angle: float);
	fn cairo_matrix_translate(matrix: libc::intptr_t, x: float, y: float);
	fn cairo_matrix_scale(matrix: libc::intptr_t, x: float, y: float);
	fn cairo_matrix_rotate(matrix: libc::intptr_t, angle: float);
	fn cairo_matrix_invert(matrix: libc::intptr_t);
	fn cairo_matrix_multiply(matrix: libc::intptr_t, left: libc::intptr_t, right: libc::intptr_t);
	fn cairo_matrix_transform_distance(matrix: libc::intptr_t, x: *float, y: *float);
	fn cairo_matrix_transform_point(matrix: libc::intptr_t, x: *float, y: *float);
	
	fn cairo_pattern_destroy(pattern: libc::intptr_t);
	fn cairo_pattern_reference(pattern: libc::intptr_t) -> libc::intptr_t;
	fn cairo_pattern_create_rgb(red: float, blue: float, green: float) -> libc::intptr_t;
	fn cairo_pattern_create_rgba(red: float, blue: float, green: float, alpha: float) -> libc::intptr_t;
	fn cairo_pattern_create_linear(x0: float, y0: float, x1: float, y1: float) -> libc::intptr_t;
	fn cairo_pattern_create_radial(cx0: float, cy0: float, radius0: float, cx1: float, cy1: float, radius1: float) -> libc::intptr_t;
	fn cairo_pattern_create_for_surface(pattern: libc::intptr_t) -> libc::intptr_t;
	fn cairo_pattern_get_type(pattern: libc::intptr_t) -> libc::c_int;
	fn cairo_pattern_get_matrix(pattern: libc::intptr_t, matrix: libc::intptr_t);
	fn cairo_pattern_set_matrix(pattern: libc::intptr_t, matrix: libc::intptr_t);
	fn cairo_pattern_get_filter(pattern: libc::intptr_t) -> libc::c_int;
	fn cairo_pattern_set_filter(pattern: libc::intptr_t, filter: libc::c_int);
	fn cairo_pattern_get_extend(pattern: libc::intptr_t) -> libc::c_int;
	fn cairo_pattern_set_extend(pattern: libc::intptr_t, extend: libc::c_int);
	fn cairo_pattern_status(pattern: libc::intptr_t) -> libc::c_int;
	fn cairo_pattern_get_color_stop_count(pattern: libc::intptr_t, count: *libc::c_int);
	fn cairo_pattern_get_surface(pattern: libc::intptr_t, surface: *libc::intptr_t);
	fn cairo_pattern_add_color_stop_rgb(pattern: libc::intptr_t, offset: float, red: float, green: float, blue: float);
	fn cairo_pattern_add_color_stop_rgba(pattern: libc::intptr_t, offset: float, red: float, green: float, blue: float, alpha: float);
	fn cairo_pattern_get_color_stop_rgba(pattern: libc::intptr_t, index: libc::c_int, offset: *float, red: *float, green: *float, blue: *float, alpha: *float);
	fn cairo_pattern_get_rgba(pattern: libc::intptr_t, red: *float, green: *float, blue: *float, alpha: *float);
	fn cairo_pattern_get_linear_points(pattern: libc::intptr_t, x0: *float, y0: *float, x1: *float, y1: *float);
	fn cairo_pattern_get_radial_circles(pattern: libc::intptr_t, cx0: *float, cy0: *float, radius0: *float, cx1: *float, cy1: *float, radius1: *float);
	
	fn cairo_format_stride_for_width(format: libc::c_int, width: libc::c_int) -> libc::c_int;
	fn cairo_surface_create_similar(surface: libc::intptr_t, content: libc::c_int, width: libc::c_int, height: libc::c_int) -> libc::intptr_t;
	fn cairo_image_surface_create_from_png(data: *u8) -> libc::intptr_t;
	fn cairo_image_surface_create_for_data(data: *u8, format: libc::c_int, width: libc::c_int, height: libc::c_int, stride: libc::c_int) -> libc::intptr_t;
	fn cairo_surface_write_to_png(surface: libc::intptr_t,file: *u8) -> libc::c_int;
	fn cairo_surface_destroy(surface: libc::intptr_t);
	fn cairo_image_surface_create(format: libc::c_int, width: libc::c_int, height: libc::c_int) -> libc::intptr_t;
	fn cairo_surface_has_show_text_glyphs(surface: libc::intptr_t) -> libc::c_int;
	fn cairo_surface_show_page(surface: libc::intptr_t);
	fn cairo_surface_copy_page(surface: libc::intptr_t);
	fn cairo_surface_get_type(surface: libc::intptr_t) -> libc::c_int;
	fn cairo_surface_set_fallback_resolution(surface: libc::intptr_t, x: float, y: float);
	fn cairo_surface_get_fallback_resolution(surface: libc::intptr_t, x: *float, y: *float);
	fn cairo_surface_set_device_offset(surface: libc::intptr_t, x: float, y: float);
	fn cairo_surface_get_device_offset(surface: libc::intptr_t, x: *float, y: *float);
	fn cairo_surface_mark_dirty(surface: libc::intptr_t);
	fn cairo_surface_mark_dirty_rectangle(surface: libc::intptr_t, x: libc::c_int, y: libc::c_int, width: libc::c_int, height: libc::c_int);
	fn cairo_surface_get_content(surface: libc::intptr_t) -> libc::c_int;
	fn cairo_surface_get_font_options(surface: libc::intptr_t, options: libc::intptr_t);
	fn cairo_surface_get_device(surface: libc::intptr_t) -> libc::intptr_t;
	fn cairo_surface_status(surface: libc::intptr_t) -> libc::c_int;
	fn cairo_surface_flush(surface: libc::intptr_t);
	fn cairo_surface_reference(surface: libc::intptr_t) -> libc::intptr_t;
	fn cairo_image_surface_get_data(surface: libc::intptr_t) -> *u8;
	fn cairo_image_surface_get_format(surface: libc::intptr_t) -> libc::c_int;
	fn cairo_image_surface_get_height(surface: libc::intptr_t) -> libc::c_int;
	fn cairo_image_surface_get_width(surface: libc::intptr_t) -> libc::c_int;
	fn cairo_image_surface_get_stride(surface: libc::intptr_t) -> libc::c_int;
	fn cairo_pdf_surface_restrict_to_version(surface: libc::intptr_t, version: libc::c_int);
	fn cairo_svg_surface_restrict_to_version(surface: libc::intptr_t, version: libc::c_int);
	fn cairo_pdf_surface_set_size(surface: libc::intptr_t, width: float, height: float);
	fn cairo_pdf_surface_create(path: *u8, width: float, height: float) -> libc::intptr_t;
	fn cairo_svg_surface_create(path: *u8, width: float, height: float) -> libc::intptr_t;
	fn cairo_ps_surface_restrict_to_level(surface: libc::intptr_t, level: libc::c_int);
	fn cairo_ps_surface_set_size(surface: libc::intptr_t, width: float, height: float);
	fn cairo_ps_surface_set_eps(surface: libc::intptr_t, eps: libc::c_int);
	fn cairo_ps_surface_get_eps(surface: libc::intptr_t) -> libc::c_int;
	fn cairo_ps_surface_dsc_begin_setup(surface: libc::intptr_t);
	fn cairo_ps_surface_dsc_begin_page_setup(surface: libc::intptr_t);
	fn cairo_ps_surface_dsc_comment(surface: libc::intptr_t, text: *u8);
	fn cairo_ps_surface_create(path: *u8, width: float, height: float) -> libc::intptr_t;
	
	fn cairo_device_reference(device: libc::intptr_t) -> libc::intptr_t;
	fn cairo_device_destroy(device: libc::intptr_t);
	fn cairo_device_flush(device: libc::intptr_t);
	fn cairo_device_acquire(device: libc::intptr_t) -> libc::c_int;
	fn cairo_device_release(device: libc::intptr_t);
	fn cairo_device_get_type(device: libc::intptr_t) -> libc::c_int;
	
	fn cairo_status_to_string(status: libc::c_int) -> *libc::c_char;
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
	mut ascent: float,
	mut descent: float,
	mut height: float,
	mut max_x_advance: float,
	mut max_y_advance: float
};
type text_extents_record = {
	mut x_bearing: float,
	mut y_bearing: float,
	mut width: float,
	mut height: float,
	mut x_advance: float,
	mut y_advance: float
};
type matrix_record = {
	mut xx: float,
	mut yx: float,
	mut xy: float,
	mut yy: float,
	mut x0: float,
	mut y0: float
};
type glyph_record = {
	mut index: libc::c_ulong,
	mut x: float,
	mut y: float
};
type text_cluster_record = {
	mut num_bytes: libc::c_int,
	mut num_glyphs: libc::c_int
};

const SVG_VERSION_1_1: int = 0;
const SVG_VERSION_1_2: int = 1;

type svg_version = int;

const PDF_VERSION_1_4: int = 0;
const PDF_VERSION_1_5: int = 1;

type pdf_version = int;

const PS_VERSION_2: int = 0;
const PS_VERSION_3: int = 1;

type ps_version = int;

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
	fn get_internal() -> libc::intptr_t;
	
	fn flush();
	fn get_type() -> device_type;
	fn acquire() -> status;
	fn release();
}

class device_res {
    let internal: libc::intptr_t;

    new(internal: libc::intptr_t) {
        self.internal = internal;
    }

    drop {
        ccairo::cairo_device_destroy(self.internal);
    }

    fn val() -> libc::intptr_t { self.internal }
}

fn wrap_device(internal: libc::intptr_t) -> device {
	impl of device for @device_res {
		fn get_internal() -> libc::intptr_t {
			ret self.val() as libc::intptr_t;
		}
		
		fn flush() {
			ccairo::cairo_device_flush(self.val());
		}
		fn get_type() -> device_type {
			ret ccairo::cairo_device_get_type(self.val()) as device_type;
		}
		fn acquire() -> status {
			ret ccairo::cairo_device_acquire(self.val()) as status;
		}
		fn release() {
			ccairo::cairo_device_release(self.val());
		}
	}
	
	ret @device_res(internal) as device;
}

/*
 * Surface
 */

iface surface {
	fn get_internal() -> libc::intptr_t;

	fn pdf_restrict_to_version(version: pdf_version);
	fn pdf_set_size(width_in_points: float, height_in_points: float);

	fn svg_restrict_to_version(version: svg_version);

	fn ps_restrict_to_version(version: ps_version);
	fn ps_set_size(width_in_points: float, height_in_points: float);
	fn ps_set_encapsulated(eps: bool);
	fn ps_is_encapsulated() -> bool;
	fn ps_begin_setup_comments();
	fn ps_begin_page_setup_comments();
	fn ps_comment(text: str);

	fn image_get_data() -> [mut u8];
	fn image_get_format() -> format;
	fn image_get_width() -> uint;
	fn image_get_height() -> uint;
	fn image_get_stride() -> uint;
	fn image_get_size() -> (uint, uint);

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

class surface_res {
    let internal: libc::intptr_t;

    new(internal: libc::intptr_t) {
        self.internal = internal;
    }

    drop {
        ccairo::cairo_surface_destroy(self.internal);
    }

    fn val() -> libc::intptr_t { self.internal }
}

fn wrap_surface(internal: libc::intptr_t) -> surface {
	impl of surface for @surface_res {
		fn get_internal() -> libc::intptr_t {
			ret self.val() as libc::intptr_t;
		}
		
		fn pdf_restrict_to_version(version: pdf_version) {
			ccairo::cairo_pdf_surface_restrict_to_version(self.val(), version as libc::c_int);
		}
		fn pdf_set_size(width_in_points: float, height_in_points: float) {
			ccairo::cairo_pdf_surface_set_size(self.val(), width_in_points, height_in_points);
		}
		fn svg_restrict_to_version(version: svg_version) {
			ccairo::cairo_svg_surface_restrict_to_version(self.val(), version as libc::c_int);
		}
		fn ps_restrict_to_version(version: ps_version) {
			ccairo::cairo_ps_surface_restrict_to_level(self.val(), version as libc::c_int);
		}
		fn ps_set_size(width_in_points: float, height_in_points: float) {
			ccairo::cairo_ps_surface_set_size(self.val(), width_in_points, height_in_points);
		}
		fn ps_set_encapsulated(eps: bool) {
			ccairo::cairo_ps_surface_set_eps(self.val(), eps as libc::c_int);
		}
		fn ps_is_encapsulated() -> bool {
			ret ccairo::cairo_ps_surface_get_eps(self.val()) == (1 as libc::c_int);
		}
		fn ps_begin_setup_comments() {
			ccairo::cairo_ps_surface_dsc_begin_setup(self.val());
		}
		fn ps_begin_page_setup_comments() {
			ccairo::cairo_ps_surface_dsc_begin_page_setup(self.val());
		}
		fn ps_comment(text: str) unsafe {
			let bytes = str::bytes(text);
		
			ccairo::cairo_ps_surface_dsc_comment(self.val(), vec::unsafe::to_ptr(bytes));
		}
		fn image_get_data() -> [mut u8] unsafe { // TODO test if this is mut, as rust might not keep the same pointers but reallocate
			let data = ccairo::cairo_image_surface_get_data(self.val()); // FIXME boxed free
		
			ret vec::to_mut(vec::unsafe::from_buf(data, self.image_get_stride() * self.image_get_height()));
		}
		fn image_get_format() -> format {
			ret ccairo::cairo_image_surface_get_format(self.val()) as format;
		}
		fn image_get_width() -> uint {
			ret ccairo::cairo_image_surface_get_width(self.val()) as uint;
		}
		fn image_get_height() -> uint {
			ret ccairo::cairo_image_surface_get_height(self.val()) as uint;
		}
		fn image_get_stride() -> uint {
			ret ccairo::cairo_image_surface_get_stride(self.val()) as uint;
		}
		fn image_get_size() -> (uint, uint) {
			ret (self.image_get_width(), self.image_get_height());
		}

		fn write_to_file(file: str) -> status unsafe {
			let path = os::make_absolute(file);
			let split = path::splitext(path);
			let mut bytes = str::bytes(path);
		
			vec::push(bytes, 0 as u8);
		
			alt split {
				(base, ".png") {
					ret ccairo::cairo_surface_write_to_png(self.val(), vec::unsafe::to_ptr(bytes)) as status;
				}
				(base, _) {
					ret STATUS_WRITE_ERROR;
				}
			}
		}
		fn get_status() -> status {
			ret ccairo::cairo_surface_status(self.val()) as status;
		}
		fn flush() {
			ccairo::cairo_surface_flush(self.val());
		}
		fn get_device() -> device {
			ret wrap_device(ccairo::cairo_device_reference(ccairo::cairo_surface_get_device(self.val())));
		}
		fn get_font_options() -> font_options {
			let options = mk_font_options();
		
			ccairo::cairo_surface_get_font_options(self.val(), options.get_internal());
		
			ret options;
		}
		fn get_content() -> content {
			ret ccairo::cairo_surface_get_content(self.val()) as content;
		}
		fn mark_dirty() {
			ccairo::cairo_surface_mark_dirty(self.val());
		}
		fn mark_dirty_rectangle(x: uint, y: uint, width: uint, height: uint) {
			ccairo::cairo_surface_mark_dirty_rectangle(self.val(), x as libc::c_int, y as libc::c_int, width as libc::c_int, height as libc::c_int);
		}
		fn set_device_offset(x: float, y: float) {
			ccairo::cairo_surface_set_device_offset(self.val(), x, y);
		}
		fn get_device_offset() -> (float, float) {
			let x: float = 0.0;
			let y: float = 0.0;
		
			ccairo::cairo_surface_get_device_offset(self.val(), ptr::addr_of(x), ptr::addr_of(y));
		
			ret (x, y);
		}
		fn set_fallback_resolution(x_ppi: float, y_ppi: float) {
			ccairo::cairo_surface_set_fallback_resolution(self.val(), x_ppi, y_ppi);
		}
		fn get_fallback_resolution() -> (float, float) {
			let x_ppi: float = 0.0;
			let y_ppi: float = 0.0;
		
			ccairo::cairo_surface_get_fallback_resolution(self.val(), ptr::addr_of(x_ppi), ptr::addr_of(y_ppi));
		
			ret (x_ppi, y_ppi);
		}
		fn get_type() -> surface_type {
			ret ccairo::cairo_surface_get_type(self.val()) as surface_type;
		}
		fn copy_page() {
			ccairo::cairo_surface_copy_page(self.val());
		}
		fn show_page() {
			ccairo::cairo_surface_show_page(self.val());
		}
		fn has_show_text_glyphs() -> bool {
			ret ccairo::cairo_surface_has_show_text_glyphs(self.val()) == (1 as libc::c_int);
		}
	}
	
	ret @surface_res(internal) as surface;
}
fn mk_surface_from_similar(other: surface, content: content, width: uint, height: uint) -> surface unsafe {
	let result = wrap_surface(ccairo::cairo_surface_create_similar(other.get_internal(), content as libc::c_int, width as libc::c_int, height as libc::c_int));
	let status = result.get_status();
	
	if status != STATUS_SUCCESS {
		fail #fmt("Could not make a surface from a similar surface: %s", status_to_str(status));
	}
	
	ret result;
}
fn mk_pdf_surface(file: str, width_in_points: float, height_in_points: float) -> surface unsafe {
	let path = os::make_absolute(file);
	let mut bytes = str::bytes(path);
		
	vec::push(bytes, 0 as u8);
	
	let result = wrap_surface(ccairo::cairo_pdf_surface_create(vec::unsafe::to_ptr(bytes), width_in_points, height_in_points));
	let status = result.get_status();
	
	if status != STATUS_SUCCESS {
		fail #fmt("Could not make an PDF surface: %s", status_to_str(status));
	}
	
	ret result;
}
fn mk_svg_surface(file: str, width_in_points: float, height_in_points: float) -> surface unsafe {
	let path = os::make_absolute(file);
	let mut bytes = str::bytes(path);
		
	vec::push(bytes, 0 as u8);
	
	let result = wrap_surface(ccairo::cairo_svg_surface_create(vec::unsafe::to_ptr(bytes), width_in_points, height_in_points));
	let status = result.get_status();
	
	if status != STATUS_SUCCESS {
		fail #fmt("Could not make an SVG surface: %s", status_to_str(status));
	}
	
	ret result;
}
fn mk_ps_surface(file: str, width_in_points: float, height_in_points: float) -> surface unsafe {
	let path = os::make_absolute(file);
	let mut bytes = str::bytes(path);
	
	vec::push(bytes, 0 as u8);
	
	let result = wrap_surface(ccairo::cairo_ps_surface_create(vec::unsafe::to_ptr(bytes), width_in_points, height_in_points));
	let status = result.get_status();
	
	if status != STATUS_SUCCESS {
		fail #fmt("Could not make an PS surface: %s", status_to_str(status));
	}
	
	ret result;
}
fn mk_image_surface(format: format, width: uint, height: uint) -> surface {
	let result = wrap_surface(ccairo::cairo_image_surface_create(format as libc::c_int, width as libc::c_int, height as libc::c_int));
	let status = result.get_status();
	
	if status != STATUS_SUCCESS {
		fail #fmt("Could not make an image surface: %s", status_to_str(status));
	}
	
	ret result;
}
fn mk_image_surface_from_file(file: str) -> surface unsafe {
	// TODO .jpg (maybe .bmp)
	let path = os::make_absolute(file);
	let split = path::splitext(path);
	let mut bytes = str::bytes(path);
	let mut internal: libc::intptr_t;
	
	vec::push(bytes, 0 as u8);
	
	let path_cstr: *u8 = vec::unsafe::to_ptr(bytes);
		
	alt split {
		(base, ".png") {
			/*
			 * Let's leave cairo to do the actual PNG work seeing as it supports it
			 * by default, I just want to have a failure for the
			 * obvious non-existant file/no read error consistent with other formats when they are added
			 */
		
			let mode = "rb";
			let mode_bytes = str::bytes(mode);
			let mode_cstr = vec::unsafe::to_ptr(mode_bytes);
			let file: libc::intptr_t = c::fopen(path_cstr, mode_cstr);
			
			if file == (0 as libc::intptr_t) {
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
	let result = wrap_surface(ccairo::cairo_image_surface_create_for_data(vec::unsafe::to_ptr(data), format as libc::c_int, width as libc::c_int, height as libc::c_int, stride as libc::c_int));
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
	fn get_internal() -> libc::intptr_t;
	
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
	fn get_filter() -> filter;
	fn set_matrix(matrix: matrix);
	fn get_matrix() -> matrix;
	fn get_type() -> pattern_type;
}

class pattern_res {
    let internal: libc::intptr_t;

    new(internal: libc::intptr_t) {
        self.internal = internal;
    }

    drop {
        ccairo::cairo_pattern_destroy(self.internal);
    }

    fn val() -> libc::intptr_t { self.internal }
}

fn wrap_pattern(internal: libc::intptr_t) -> pattern {
	impl of pattern for @pattern_res {
		fn get_internal() -> libc::intptr_t {
			ret self.val() as libc::intptr_t;
		}
		
		fn add_color_stop_rgb(offset: float, red: float, green: float, blue: float) {
			ccairo::cairo_pattern_add_color_stop_rgb(self.val(), offset, red, green, blue);
		}
		fn add_color_stop_rgba(offset: float, red: float, green: float, blue: float, alpha: float) {
			ccairo::cairo_pattern_add_color_stop_rgba(self.val(), offset, red, green, blue, alpha);
		}
		fn get_color_stop_count() -> uint {
			let count: libc::c_int = 0 as libc::c_int;
		
			ccairo::cairo_pattern_get_color_stop_count(self.val(), ptr::addr_of(count));
		
			ret count as uint;
		}
		fn get_color_stop(index: uint) -> (float, float, float, float, float) {
			let offset: float = 0.0;
			let red: float = 0.0;
			let green: float = 0.0;
			let blue: float = 0.0;
			let alpha: float = 0.0;
		
			ccairo::cairo_pattern_get_color_stop_rgba(self.val(), index as libc::c_int, ptr::addr_of(offset), ptr::addr_of(red), ptr::addr_of(green), ptr::addr_of(blue), ptr::addr_of(alpha));
		
			ret (offset, red, green, blue, alpha);
		}
		fn get_rgba() -> (float, float, float, float) {
			let red: float = 0.0;
			let green: float = 0.0;
			let blue: float = 0.0;
			let alpha: float = 0.0;
		
			ccairo::cairo_pattern_get_rgba(self.val(), ptr::addr_of(red), ptr::addr_of(green), ptr::addr_of(blue), ptr::addr_of(alpha));
		
			ret (red, green, blue, alpha);
		}
		fn get_surface() -> surface {
			let internal: libc::intptr_t = 0 as libc::intptr_t;
		
			ccairo::cairo_pattern_get_surface(self.val(), ptr::addr_of(internal));
		
			ret wrap_surface(ccairo::cairo_surface_reference(internal));
		}
		fn get_linear_points() -> (float, float, float, float) {
			let x0: float = 0.0;
			let y0: float = 0.0;
			let x1: float = 0.0;
			let y1: float = 0.0;
		
			ccairo::cairo_pattern_get_linear_points(self.val(), ptr::addr_of(x0), ptr::addr_of(y0), ptr::addr_of(x1), ptr::addr_of(y1));
			
			ret (x0, y0, x1, y1);
		}
		fn get_radial_circles() -> (float, float, float, float, float, float) {
			let x0: float = 0.0;
			let y0: float = 0.0;
			let r0: float = 0.0;
			let x1: float = 0.0;
			let y1: float = 0.0;
			let r1: float = 0.0;
		
			ccairo::cairo_pattern_get_radial_circles(self.val(), ptr::addr_of(x0), ptr::addr_of(y0), ptr::addr_of(r0), ptr::addr_of(x1), ptr::addr_of(y1), ptr::addr_of(r1));
			
			ret (x0, y0, r0, x1, y1, r1);
		}
		fn get_status() -> status {
			ret ccairo::cairo_pattern_status(self.val()) as status;
		}
		fn set_extend(extend: extend) {
			ccairo::cairo_pattern_set_extend(self.val(), extend as libc::c_int);
		}
		fn get_extend() -> extend {
			ret ccairo::cairo_pattern_get_extend(self.val()) as extend;
		}
		fn set_filter(filter: filter) {
			ccairo::cairo_pattern_set_filter(self.val(), filter as libc::c_int);
		}
		fn get_filter() -> filter {
			ret ccairo::cairo_pattern_get_filter(self.val()) as filter;
		}
		fn set_matrix(matrix: matrix) {
			ccairo::cairo_pattern_set_matrix(self.val(), matrix.get_internal());
		}
		fn get_matrix() -> matrix {
			let matrix: matrix = mk_matrix([
				0.0, 0.0,
				0.0, 0.0,
				0.0, 0.0
			]);
		
			ccairo::cairo_pattern_get_matrix(self.val(), matrix.get_internal());
		
			ret matrix;
		}
		fn get_type() -> pattern_type {
			ret ccairo::cairo_pattern_get_type(self.val()) as pattern_type;
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
	fn get_internal() -> libc::intptr_t;
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
	fn multiply(other: matrix) -> matrix;
	fn transform_distance(x: float, y: float) -> (float, float);
	fn transform_point(x: float, y: float) -> (float, float);
}

class matrix_res {
    let internal: libc::intptr_t;

    new(internal: libc::intptr_t) {
        self.internal = internal;
    }

    drop {
        ptr::addr_of(self.internal);
    }

    fn val() -> libc::intptr_t { self.internal }
}

fn wrap_matrix(internal: libc::intptr_t) -> matrix {
	impl of matrix for @matrix_res {
		fn get_internal() -> libc::intptr_t {
			ret self.val() as libc::intptr_t;
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
			ccairo::cairo_matrix_init(self.val(), values[0], values[1], values[2], values[3], values[4], values[5]);
		}
		fn set_identity() {
			ccairo::cairo_matrix_init_identity(self.val());
		}
		fn set_translate(x: float, y: float) {
			ccairo::cairo_matrix_init_translate(self.val(), x, y);
		}
		fn set_scale(x: float, y: float) {
			ccairo::cairo_matrix_init_scale(self.val(), x, y);
		}
		fn set_rotate(angle: float) {
			ccairo::cairo_matrix_init_rotate(self.val(), angle);
		}
		fn translate(x: float, y: float) {
			ccairo::cairo_matrix_translate(self.val(), x, y);
		}
		fn scale(x: float, y: float) {
			ccairo::cairo_matrix_scale(self.val(), x, y);
		}
		fn rotate(angle: float) {
			ccairo::cairo_matrix_rotate(self.val(), angle);
		}
		fn invert() {
			ccairo::cairo_matrix_invert(self.val());
		}
		fn multiply(other: matrix) -> matrix {
			let result: matrix = mk_matrix([
				0.0, 0.0,
				0.0, 0.0,
				0.0, 0.0
			]);
			
			ccairo::cairo_matrix_multiply(result.get_internal(), self.val(), other.get_internal());
			
			ret result;
		}
		fn transform_distance(x: float, y: float) -> (float, float) {
			let xt: float = x;
			let yt: float = y;
		
			ccairo::cairo_matrix_transform_distance(self.val(), ptr::addr_of(xt), ptr::addr_of(yt));
		
			ret (xt, yt);
		}
		fn transform_point(x: float, y: float) -> (float, float) {
			let xt: float = x;
			let yt: float = y;
		
			ccairo::cairo_matrix_transform_point(self.val(), ptr::addr_of(xt), ptr::addr_of(yt));
		
			ret (xt, yt);
		}
	}
	
	ret @matrix_res(internal) as matrix;
}
fn mk_matrix(values: [float]) -> matrix {
	let record: matrix_record = {
		mut xx: 0.0,
		mut yx: 0.0,
		mut xy: 0.0,
		mut yy: 0.0,
		mut x0: 0.0,
		mut y0: 0.0
	};
	let result = wrap_matrix(ptr::addr_of(record) as libc::intptr_t);
	
	result.set_values(values);
	
	ret result;
}

/*
 * Path
 */

iface path {
	fn get_internal() -> libc::intptr_t;
}

class path_res {
    let internal: libc::intptr_t;

    new(internal: libc::intptr_t) {
        self.internal = internal;
    }

    drop {
	ccairo::cairo_path_destroy(self.internal);
    }

    fn val() -> libc::intptr_t { self.internal }
}

fn wrap_path(internal: libc::intptr_t) -> path {
	impl of path for @path_res {
		fn get_internal() -> libc::intptr_t {
			ret self.val() as libc::intptr_t;
		}
	}
	
	ret @path_res(internal) as path;
}

/*
 * Text
 */

iface glyph {
	fn get_internal() -> libc::intptr_t;
	fn get_record() -> glyph_record;

	fn set_index(index: uint);
	fn set_x(x: float);
	fn set_y(y: float);
	fn get_index() -> uint;
	fn get_x() -> float;
	fn get_y() -> float;
}

class glyph_res {
    let internal: libc::intptr_t;

    new(internal: libc::intptr_t) {
        self.internal = internal;
    }

    drop {
	ptr::addr_of(self.internal);
    }

    fn val() -> libc::intptr_t { self.internal }
}

fn wrap_glyph(internal: libc::intptr_t) -> glyph {
	impl of glyph for @glyph_res {
		fn get_internal() -> libc::intptr_t {
			ret self.val() as libc::intptr_t;
		}
		fn get_record() -> glyph_record unsafe {
			ret *(self.get_internal() as *glyph_record);
		}
		
		fn set_index(index: uint) {
			self.get_record().index = index as libc::c_ulong;
		}
		fn set_x(x: float) {
			self.get_record().x = x as float;
		}
		fn set_y(y: float) {
			self.get_record().y = y;
		}
		fn get_index() -> uint {
			ret self.get_record().index as uint;
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
		mut index: index as libc::c_ulong,
		mut x: x,
		mut y: y
	};
	
	ret wrap_glyph(ptr::addr_of(record) as libc::intptr_t);
}

iface text_cluster {
	fn get_internal() -> libc::intptr_t;
	fn get_record() -> text_cluster_record;
	
	fn set_num_bytes(num_bytes: uint);
	fn set_num_glyphs(num_glyphs: uint);
	fn get_num_bytes() -> uint;
	fn get_num_glyphs() -> uint;
}

class text_cluster_res {
    let internal: libc::intptr_t;

    new(internal: libc::intptr_t) {
        self.internal = internal;
    }

    drop {
        ptr::addr_of(self.internal);
    }

    fn val() -> libc::intptr_t { self.internal }
}

fn wrap_text_cluster(internal: libc::intptr_t) -> text_cluster {
	impl of text_cluster for @text_cluster_res {
		fn get_internal() -> libc::intptr_t {
			ret self.val() as libc::intptr_t;
		}
		fn get_record() -> text_cluster_record unsafe {
			ret *(self.get_internal() as *text_cluster_record);
		}
		
		fn set_num_bytes(num_bytes: uint) {
			self.get_record().num_bytes = num_bytes as libc::c_int;
		}
		fn set_num_glyphs(num_glyphs: uint) {
			self.get_record().num_glyphs = num_glyphs as libc::c_int;
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
		mut num_bytes: num_bytes as libc::c_int,
		mut num_glyphs: num_glyphs as libc::c_int
	};
	
	ret wrap_text_cluster(ptr::addr_of(record) as libc::intptr_t);
}

iface font_extents {
	fn get_internal() -> libc::intptr_t;
	fn get_record() -> font_extents_record;

	fn get_ascent() -> float;
	fn get_descent() -> float;
	fn get_height() -> float;
	fn get_max_x_advance() -> float;
	fn get_max_y_advance() -> float;
}

class font_extents_res {
    let internal: libc::intptr_t;

    new(internal: libc::intptr_t) {
        self.internal = internal;
    }

    drop {
        ptr::addr_of(self.internal);
    }

    fn val() -> libc::intptr_t { self.internal }
}

fn wrap_font_extents(internal: libc::intptr_t) -> font_extents {
	impl of font_extents for @font_extents_res {
		fn get_internal() -> libc::intptr_t {
			ret self.val() as libc::intptr_t;
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
	fn get_internal() -> libc::intptr_t;
	fn get_record() -> text_extents_record;

	fn get_x_bearing() -> float;
	fn get_y_bearing() -> float;
	fn get_width() -> float;
	fn get_height() -> float;
	fn get_size() -> (float, float);
	fn get_x_advance() -> float;
	fn get_y_advance() -> float;
}

class text_extents_res {
    let internal: libc::intptr_t;

    new(internal: libc::intptr_t) {
        self.internal = internal;
    }

    drop {
        ptr::addr_of(self.internal);
    }

    fn val() -> libc::intptr_t { self.internal }
}

fn wrap_text_extents(internal: libc::intptr_t) -> text_extents {
	impl of text_extents for @text_extents_res {
		fn get_internal() -> libc::intptr_t {
			ret self.val() as libc::intptr_t;
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
		fn get_y_advance() -> float {
			ret self.get_record().y_advance;
		}
	}
	
	ret @text_extents_res(internal) as text_extents;
}

type font_face_free_record = {
	library: libc::intptr_t,
	face: libc::intptr_t,
	internal: libc::intptr_t
};

iface font_face {
	fn get_internal() -> libc::intptr_t;

	fn get_status() -> status;
	fn get_type() -> font_type;
	fn get_toy_slant() -> font_slant;
	fn get_toy_weight() -> font_weight;
	fn get_toy_family() -> str;
}

class font_face_res {
    let record: @font_face_free_record;

    new(record: @font_face_free_record) {
        self.record = record;
    }

    drop {
	let face = self.record.face;
	let library = self.record.library;
	let internal = self.record.internal;
	
	ccairo::cairo_font_face_destroy(internal);
	
	if face != (0 as libc::intptr_t) {
		cft::FT_Done_Face(face);
	}
	if library != (0 as libc::intptr_t) {
		cft::FT_Done_FreeType(library);
	}
    }

    fn val() -> libc::intptr_t { self.record.internal }
}

fn wrap_font_face(record: @font_face_free_record) -> font_face {
	impl of font_face for @font_face_res {
		fn get_internal() -> libc::intptr_t {
			ret self.val();
		}
		
		fn get_status() -> status {
			ret ccairo::cairo_font_face_status(self.get_internal()) as status;
		}
		fn get_type() -> font_type {
			ret ccairo::cairo_font_face_get_type(self.get_internal()) as font_type;
		}
		fn get_toy_slant() -> font_slant {
			ret ccairo::cairo_toy_font_face_get_slant(self.get_internal()) as font_slant;
		}
		fn get_toy_weight() -> font_weight {
			ret ccairo::cairo_toy_font_face_get_weight(self.get_internal()) as font_weight;
		}
		fn get_toy_family() -> str unsafe {
			ret str::unsafe::from_c_str(ccairo::cairo_toy_font_face_get_family(self.get_internal()));
		}
	}
	
	ret @font_face_res(record) as font_face;
}
fn mk_font_face_from_toy_font(family: str, slant: font_slant, weight: font_weight) -> font_face unsafe {
	let mut bytes = str::bytes(family);
	
	vec::push(bytes, 0 as u8);
	
	let free_record: @font_face_free_record = @{
		library: 0 as libc::intptr_t,
		face: 0 as libc::intptr_t,
		internal: ccairo::cairo_toy_font_face_create(vec::unsafe::to_ptr(bytes), slant as libc::c_int, weight as libc::c_int)
	};
	let result = wrap_font_face(free_record);
	let status = result.get_status();
	
	if status != STATUS_SUCCESS {
		fail #fmt("Could not make a font face from a toy font: %s", status_to_str(status));
	}
	
	ret result;
}

fn mk_font_face_from_file(file: str) -> font_face unsafe {
	let path = os::make_absolute(file);
	let split = path::splitext(path);
	let mut bytes = str::bytes(path);
	let mut internal: libc::intptr_t;
	let face_internal: libc::intptr_t = 0 as libc::intptr_t;
		let library_internal: libc::intptr_t = 0 as libc::intptr_t;
	
	vec::push(bytes, 0 as u8);
		
	alt split {
		(base, ".ttf") {
			if cft::FT_Init_FreeType(ptr::addr_of(library_internal)) != (0 as libc::c_int) {
				fail "Could not make a font face from a font file: unable to initialize freetype";
			}

			if cft::FT_New_Face(library_internal, vec::unsafe::to_ptr(bytes), 0 as libc::c_long, ptr::addr_of(face_internal)) != (0 as libc::c_int) {
				fail "Could not make a font face from a font file: unable to load font";
			}
			
			internal = ccairo::cairo_ft_font_face_create_for_ft_face(face_internal, 0 as libc::c_int);
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
	fn get_internal() -> libc::intptr_t;
	
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

class scaled_font_res {
    let internal: libc::intptr_t;

    new(internal: libc::intptr_t) {
        self.internal = internal;
    }

    drop {
	ccairo::cairo_scaled_font_destroy(self.internal);
    }

    fn val() -> libc::intptr_t { self.internal }
}

fn wrap_scaled_font(internal: libc::intptr_t) -> scaled_font {
	impl of scaled_font for @scaled_font_res {
		fn get_internal() -> libc::intptr_t {
			ret self.val() as libc::intptr_t;
		}
	
		fn get_status() -> status {
			ret ccairo::cairo_scaled_font_status(self.val()) as status;
		}
		fn extents() -> font_extents {
			let record: font_extents_record = {
				mut ascent: 0.0,
				mut descent: 0.0,
				mut height: 0.0,
				mut max_x_advance: 0.0,
				mut max_y_advance: 0.0
			};
			let result = wrap_font_extents(ptr::addr_of(record) as libc::intptr_t);
		
			ccairo::cairo_scaled_font_extents(self.val(), result.get_internal());
		
			ret result;
		}
		fn text_extents(text: str) -> text_extents unsafe {
			let mut bytes = str::bytes(text);
			let record: text_extents_record = {
				mut x_bearing: 0.0,
				mut y_bearing: 0.0,
				mut width: 0.0,
				mut height: 0.0,
				mut x_advance: 0.0,
				mut y_advance: 0.0
			};
			let result = wrap_text_extents(ptr::addr_of(record) as libc::intptr_t);
		
			vec::push(bytes, 0 as u8);
			ccairo::cairo_scaled_font_text_extents(self.val(), vec::unsafe::to_ptr(bytes), result.get_internal());
		
			ret result;
		}
		fn glyph_extents(glyphs: [glyph]) -> text_extents unsafe {
			let record: text_extents_record = {
				mut x_bearing: 0.0,
				mut y_bearing: 0.0,
				mut width: 0.0,
				mut height: 0.0,
				mut x_advance: 0.0,
				mut y_advance: 0.0
			};
			let result = wrap_text_extents(ptr::addr_of(record) as libc::intptr_t);
			let mut cglyphs: [libc::intptr_t] = [];
		
		        for glyphs.each |glyph| {
				cglyphs += [glyph.get_internal()];
			}
		
			ccairo::cairo_scaled_font_glyph_extents(self.val(), vec::unsafe::to_ptr(cglyphs) as libc::intptr_t, vec::len(cglyphs) as libc::c_int, result.get_internal());
		
			ret result;
		}
		fn get_font_face() -> font_face { // FIXME
			let free_record: @font_face_free_record = @{
				library: 0 as libc::intptr_t,
				face: 0 as libc::intptr_t,
				internal: ccairo::cairo_font_face_reference(ccairo::cairo_scaled_font_get_font_face(self.val()))
			};
		
			ret wrap_font_face(free_record);
		}
		fn get_font_options() -> font_options {
			let options: font_options = mk_font_options();
		
			ccairo::cairo_scaled_font_get_font_options(self.val(), options.get_internal());
		
			ret options;
		}
		fn get_font_matrix() -> matrix {
			let matrix: matrix = mk_matrix([
				0.0, 0.0,
				0.0, 0.0,
				0.0, 0.0
			]);
		
			ccairo::cairo_scaled_font_get_font_matrix(self.val(), matrix.get_internal());
		
			ret matrix;
		}
		fn get_ctm() -> matrix {
			let matrix: matrix = mk_matrix([
				0.0, 0.0,
				0.0, 0.0,
				0.0, 0.0
			]);
		
			ccairo::cairo_scaled_font_get_ctm(self.val(), matrix.get_internal());
		
			ret matrix;
		}
		fn get_scale_matrix() -> matrix {
			let matrix: matrix = mk_matrix([
				0.0, 0.0,
				0.0, 0.0,
				0.0, 0.0
			]);
		
			ccairo::cairo_scaled_font_get_scale_matrix(self.val(), matrix.get_internal());
		
			ret matrix;
		}
		fn get_type() -> font_type {
			ret ccairo::cairo_scaled_font_get_type(self.val()) as font_type;
		}
	}
	
	ret @scaled_font_res(internal) as scaled_font;
}

iface font_options {
	fn get_internal() -> libc::intptr_t;	
	
	fn get_status() -> status;
	fn hash() -> uint;
	fn merge(other: font_options);
	fn equals(other: font_options) -> bool;
	fn set_antialias(antialias: antialias);
	fn get_antialias() -> antialias;
	fn set_subpixel_order(order: subpixel_order);
	fn get_subpixel_order() -> subpixel_order;
	fn set_hint_style(hint: hint_style);
	fn get_hint_style() -> hint_style;
	fn set_hint_metrics(hint: hint_metrics);
	fn get_hint_metrics() -> hint_metrics;
}

class font_options_res {
    let internal: libc::intptr_t;

    new(internal: libc::intptr_t) {
        self.internal = internal;
    }

    drop {
        ccairo::cairo_font_options_destroy(self.internal);
    }

    fn val() -> libc::intptr_t { self.internal }
}

fn wrap_font_options(internal: libc::intptr_t) -> font_options {
	impl of font_options for @font_options_res {
		fn get_internal() -> libc::intptr_t {
			ret self.val() as libc::intptr_t;
		}
		
		fn get_status() -> status {
			ret ccairo::cairo_font_options_status(self.val()) as status;
		}
		fn merge(other: font_options) {
			ccairo::cairo_font_options_merge(self.val(), other.get_internal());
		}
		fn hash() -> uint {
			ret ccairo::cairo_font_options_hash(self.val()) as uint;
		}
		fn equals(other: font_options) -> bool {
			ret ccairo::cairo_font_options_equal(self.val(), other.get_internal()) == (1 as libc::c_int);
		}
		fn set_antialias(antialias: antialias) {
			ccairo::cairo_font_options_set_antialias(self.val(), antialias as libc::c_int);
		}
		fn get_antialias() -> antialias {
			ret ccairo::cairo_font_options_get_antialias(self.val()) as antialias;
		}
		fn set_subpixel_order(order: subpixel_order) {
			ccairo::cairo_font_options_set_subpixel_order(self.val(), order as libc::c_int);
		}
		fn get_subpixel_order() -> subpixel_order {
			ret ccairo::cairo_font_options_get_subpixel_order(self.val()) as subpixel_order;
		}
		fn set_hint_style(hint: hint_style) {
			ccairo::cairo_font_options_set_hint_style(self.val(), hint as libc::c_int);
		}
		fn get_hint_style() -> hint_style {
			ret ccairo::cairo_font_options_get_hint_style(self.val()) as hint_style;
		}
		fn set_hint_metrics(hint: hint_metrics) {
			ccairo::cairo_font_options_set_hint_metrics(self.val(), hint as libc::c_int);
		}
		fn get_hint_metrics() -> hint_metrics {
			ret ccairo::cairo_font_options_get_hint_metrics(self.val()) as hint_metrics;
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
	fn get_internal() -> libc::intptr_t;

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

class context_res {
    let internal: libc::intptr_t;

    new(internal: libc::intptr_t) {
        self.internal = internal;
    }

    drop {
        ccairo::cairo_destroy(self.internal);
    }

    fn val() -> libc::intptr_t { self.internal }
}

fn wrap_context(internal: libc::intptr_t) -> context {
	impl of context for @context_res {
		fn get_internal() -> libc::intptr_t {
			ret self.val() as libc::intptr_t;
		}
	
		fn get_status() -> status {
			ret ccairo::cairo_status(self.val()) as status;
		}
		fn save() {
			ccairo::cairo_save(self.val());
		}
		fn restore() {
			ccairo::cairo_restore(self.val());
		}
		fn get_target() -> surface {
			ret wrap_surface(ccairo::cairo_get_target(self.val()));
		}
		fn push_group() {
			ccairo::cairo_push_group(self.val());
		}
		fn push_group_with_content(content: content) {
			ccairo::cairo_push_group_with_content(self.val(), content as libc::c_int);
		}
		fn pop_group() -> pattern {
			ret wrap_pattern(ccairo::cairo_pop_group(self.val()));
		}
		fn pop_group_to_source() {
			ccairo::cairo_pop_group_to_source(self.val());
		}
		fn get_group_target() -> surface {
			ret wrap_surface(ccairo::cairo_get_group_target(self.val()));
		}
		fn set_source_rgb(red: float, green: float, blue: float) {
			ccairo::cairo_set_source_rgb(self.val(), red, green, blue);
		}
		fn set_source_rgba(red: float, green: float, blue: float, alpha: float) {
			ccairo::cairo_set_source_rgba(self.val(), red, green, blue, alpha);
		}
		fn set_source(pattern: pattern) {
			ccairo::cairo_set_source(self.val(), pattern.get_internal());
		}
		fn set_source_surface(surface: surface, x: float, y: float) {
			ccairo::cairo_set_source_surface(self.val(), surface.get_internal(), x, y);
		}
		fn get_source() -> pattern {
			ret wrap_pattern(ccairo::cairo_get_source(self.val()));
		}
		fn set_antialias(antialias: antialias) {
			ccairo::cairo_set_antialias(self.val(), antialias as libc::c_int);
		}
		fn get_antialias() -> antialias {
			ret ccairo::cairo_get_antialias(self.val()) as antialias;
		}
		fn set_dash(dashes: [float], offset: float) unsafe {
			ccairo::cairo_set_dash(self.val(), vec::unsafe::to_ptr(dashes), vec::len(dashes) as libc::c_int, offset);
		}
		fn get_dash_count() -> uint {
			ret ccairo::cairo_get_dash_count(self.val()) as uint;
		}
		fn get_dash() -> [float] unsafe {
			let dashes: [float] = [];
		
			ccairo::cairo_get_dash(self.val(), vec::unsafe::to_ptr(dashes), ptr::null());
		
			ret dashes;
		}
		fn get_dash_offset() -> float {
			let offset: float = 0.0;
		
			ccairo::cairo_get_dash(self.val(), ptr::null(), ptr::addr_of(offset));
		
			ret offset;
		}
		fn set_fill_rule(rule: fill_rule) {
			ccairo::cairo_set_fill_rule(self.val(), rule as libc::c_int);
		}
		fn get_fill_rule() -> fill_rule {
			ret ccairo::cairo_get_fill_rule(self.val()) as fill_rule;
		}
		fn set_line_cap(cap: line_cap) {
			ccairo::cairo_set_line_cap(self.val(), cap as libc::c_int);
		}
		fn get_line_cap() -> line_cap {
			ret ccairo::cairo_get_line_cap(self.val()) as line_cap;
		}
		fn set_line_join(join: line_join) {
			ccairo::cairo_set_line_join(self.val(), join as libc::c_int);
		}
		fn get_line_join() -> line_join {
			ret ccairo::cairo_get_line_join(self.val()) as line_join;
		}
		fn set_line_width(width: float) {
			ccairo::cairo_set_line_width(self.val(), width);
		}
		fn get_line_width() -> float {
			ret ccairo::cairo_get_line_width(self.val());
		}
		fn set_miter_limit(limit: float) {
			ccairo::cairo_set_miter_limit(self.val(), limit);
		}
		fn get_miter_limit() -> float {
			ret ccairo::cairo_get_miter_limit(self.val());
		}
		fn set_operator(op: operator) {
			ccairo::cairo_set_operator(self.val(), op as libc::c_int);
		}
		fn get_operator() -> operator {
			ret ccairo::cairo_get_operator(self.val()) as operator;
		}
		fn set_tolerance(tol: float) {
			ccairo::cairo_set_tolerance(self.val(), tol);
		}
		fn get_tolerance() -> float {
			ret ccairo::cairo_get_tolerance(self.val());
		}
		fn clip() {
			ccairo::cairo_clip(self.val());
		}
		fn clip_preserve() {
			ccairo::cairo_clip_preserve(self.val());
		}
		fn clip_extents() -> (float, float, float, float) {
			let x1: float = 0.0;
			let y1: float = 0.0;
			let x2: float = 0.0;
			let y2: float = 0.0;
		
			ccairo::cairo_clip_extents(self.val(), ptr::addr_of(x1), ptr::addr_of(y1), ptr::addr_of(x2), ptr::addr_of(y2));
		
			ret (x1, y1, x2, y2);
		}
		fn in_clip(x: float, y: float) -> bool {
			ret ccairo::cairo_in_clip(self.val(), x, y) == (1 as libc::c_int);
		}
		fn reset_clip() {
			ccairo::cairo_reset_clip(self.val());
		}
		fn fill() {
			ccairo::cairo_fill(self.val());
		}
		fn fill_preserve() {
			ccairo::cairo_fill_preserve(self.val());
		}
		fn fill_extents() -> (float, float, float, float) {
			let x1: float = 0.0;
			let y1: float = 0.0;
			let x2: float = 0.0;
			let y2: float = 0.0;
		
			ccairo::cairo_fill_extents(self.val(), ptr::addr_of(x1), ptr::addr_of(y1), ptr::addr_of(x2), ptr::addr_of(y2));
		
			ret (x1, y1, x2, y2);
		}
		fn in_fill(x: float, y: float) -> bool {
			ret ccairo::cairo_in_fill(self.val(), x, y) == (1 as libc::c_int);
		}
		fn mask(pattern: pattern) {
			ccairo::cairo_mask(self.val(), pattern.get_internal());
		}
		fn mask_surface(surface: surface, x: float, y: float) {
			ccairo::cairo_mask_surface(self.val(), surface.get_internal(), x, y);
		}
		fn paint() {
			ccairo::cairo_paint(self.val());
		}
		fn paint_with_alpha(alpha: float) {
			ccairo::cairo_paint_with_alpha(self.val(), alpha);
		}
		fn stroke() {
			ccairo::cairo_stroke(self.val());
		}
		fn stroke_preserve() {
			ccairo::cairo_stroke_preserve(self.val());
		}
		fn stroke_extents() -> (float, float, float, float) {
			let x1: float = 0.0;
			let y1: float = 0.0;
			let x2: float = 0.0;
			let y2: float = 0.0;
		
			ccairo::cairo_stroke_extents(self.val(), ptr::addr_of(x1), ptr::addr_of(y1), ptr::addr_of(x2), ptr::addr_of(y2));
		
			ret (x1, y1, x2, y2);
		}
		fn in_stroke(x: float, y: float) -> bool {
			ret ccairo::cairo_in_stroke(self.val(), x, y) == (1 as libc::c_int);
		}
		fn copy_page() {
			ccairo::cairo_copy_page(self.val());
		}
		fn show_page() {
			ccairo::cairo_show_page(self.val());
		}
	
		fn copy_path() -> path {
			ret wrap_path(ccairo::cairo_copy_path(self.val()));
		}
		fn copy_path_flat() -> path {
			ret wrap_path(ccairo::cairo_copy_path_flat(self.val()));
		}
		fn append_path(path: path) {
			ccairo::cairo_append_path(self.val(), path.get_internal());
		}
		fn has_current_point() -> bool {
			ret ccairo::cairo_has_current_point(self.val()) == (1 as libc::c_int);
		}
		fn get_current_point() -> (float, float) {
			let x: float = 0.0;
			let y: float = 0.0;
		
			ccairo::cairo_get_current_point(self.val(), ptr::addr_of(x), ptr::addr_of(y));
		
			ret (x, y);
		}
		fn new_path() {
			ccairo::cairo_new_path(self.val());
		}
		fn new_sub_path() {
			ccairo::cairo_new_sub_path(self.val());
		}
		fn close_path() {
			ccairo::cairo_close_path(self.val());
		}
		fn arc(x: float, y: float, radius: float, angle1: float, angle2: float) {
			ccairo::cairo_arc(self.val(), x, y, radius, angle1, angle2);
		}
		fn arc_negative(x: float, y: float, radius: float, angle1: float, angle2: float) {
			ccairo::cairo_arc_negative(self.val(), x, y, radius, angle1, angle2);
		}
		fn curve_to(x1: float, y1: float, x2: float, y2: float, x3: float, y3: float) {
			ccairo::cairo_curve_to(self.val(), x1, y1, x2, y2, x3, y3);
		}
		fn line_to(x: float, y: float) {
			ccairo::cairo_line_to(self.val(), x, y);
		}
		fn move_to(x: float, y: float) {
			ccairo::cairo_move_to(self.val(), x, y);
		}
		fn rectangle(x: float, y: float, width: float, height: float) {
			ccairo::cairo_rectangle(self.val(), x, y, width, height);
		}
		fn glyph_path(glyphs: [glyph]) unsafe {
			let mut cglyphs: [libc::intptr_t] = [];
		
		        for glyphs.each |glyph| {
				cglyphs += [glyph.get_internal()];
			}
		
			ccairo::cairo_glyph_path(self.val(), vec::unsafe::to_ptr(cglyphs) as libc::intptr_t, vec::len(cglyphs) as libc::c_int);
		}
		fn text_path(text: str) unsafe {
			let mut bytes = str::bytes(text);
		
			vec::push(bytes, 0 as u8);
		
			ccairo::cairo_text_path(self.val(), vec::unsafe::to_ptr(bytes));
		}
		fn rel_curve_to(x1: float, y1: float, x2: float, y2: float, x3: float, y3: float) {
			ccairo::cairo_rel_curve_to(self.val(), x1, y1, x2, y2, x3, y3);
		}
		fn rel_line_to(x: float, y: float) {
			ccairo::cairo_rel_line_to(self.val(), x, y);
		}
		fn rel_move_to(x: float, y: float) {
			ccairo::cairo_rel_move_to(self.val(), x, y);
		}
		fn path_extents() -> (float, float, float, float) {
			let x1: float = 0.0;
			let y1: float = 0.0;
			let x2: float = 0.0;
			let y2: float = 0.0;
		
			ccairo::cairo_path_extents(self.val(), ptr::addr_of(x1), ptr::addr_of(y1), ptr::addr_of(x2), ptr::addr_of(y2));
		
			ret (x1, y1, x2, y2);
		}
	
		fn translate(x: float, y: float) {
			ccairo::cairo_translate(self.val(), x, y);
		}
		fn scale(x: float, y: float) {
			ccairo::cairo_scale(self.val(), x, y);
		}
		fn rotate(angle: float) {
			ccairo::cairo_rotate(self.val(), angle);
		}
		fn transform(matrix: matrix) {
			ccairo::cairo_transform(self.val(), matrix.get_internal());
		}
		fn set_matrix(matrix: matrix) {
			ccairo::cairo_set_matrix(self.val(), matrix.get_internal());
		}
		fn get_matrix() -> matrix {
			ret wrap_matrix(ccairo::cairo_get_matrix(self.val()));
		}
		fn identity_matrix() {
			ccairo::cairo_identity_matrix(self.val())
		}
		fn user_to_device(x: float, y: float) -> (float, float) {
			let xt: float = x;
			let yt: float = y;
		
			ccairo::cairo_user_to_device(self.val(), ptr::addr_of(x), ptr::addr_of(y));
		
			ret (xt, yt);
		}
		fn user_to_device_distance(x: float, y: float) -> (float, float) {
			let xt: float = x;
			let yt: float = y;
		
			ccairo::cairo_user_to_device_distance(self.val(), ptr::addr_of(x), ptr::addr_of(y));
		
			ret (xt, yt);
		}
		fn device_to_user(x: float, y: float) -> (float, float) {
			let xt: float = x;
			let yt: float = y;
		
			ccairo::cairo_device_to_user(self.val(), ptr::addr_of(x), ptr::addr_of(y));
		
			ret (xt, yt);
		}
		fn device_to_user_distance(x: float, y: float) -> (float, float) {
			let xt: float = x;
			let yt: float = y;
		
			ccairo::cairo_device_to_user_distance(self.val(), ptr::addr_of(x), ptr::addr_of(y));
		
			ret (xt, yt);
		}
	
		fn select_font_face(face: str, slant: font_slant, weight: font_weight) unsafe {
			let mut bytes = str::bytes(face);
		
			vec::push(bytes, 0 as u8);
		
			ccairo::cairo_select_font_face(self.val(), vec::unsafe::to_ptr(bytes), slant as libc::c_int, weight as libc::c_int);
		}
		fn set_font_size(size: float) {
			ccairo::cairo_set_font_size(self.val(), size);
		}
		fn set_font_matrix(matrix: matrix) {
			ccairo::cairo_set_font_matrix(self.val(), matrix.get_internal());
		}
		fn get_font_matrix() -> matrix {
			let internal: libc::intptr_t = 0 as libc::intptr_t;
		
			ccairo::cairo_get_font_matrix(self.val(), ptr::addr_of(internal));
		
			ret wrap_matrix(internal);
		}
		fn set_font_options(options: font_options) {
			ccairo::cairo_set_font_options(self.val(), options.get_internal());
		}
		fn get_font_options() -> font_options {
			let internal: libc::intptr_t = 0 as libc::intptr_t;
		
			ccairo::cairo_get_font_options(self.val(), ptr::addr_of(internal));
		
			ret wrap_font_options(internal);
		}
		fn set_font_face(face: font_face) {
			ccairo::cairo_set_font_face(self.val(), face.get_internal());
		}
		fn get_font_face() -> font_face {
			let free_record: @font_face_free_record = @{
				library: 0 as libc::intptr_t,
				face: 0 as libc::intptr_t,
				internal: ccairo::cairo_font_face_reference(ccairo::cairo_get_font_face(self.val()))
			};
			
			ret wrap_font_face(free_record);
		}
		fn set_scaled_font(font: scaled_font) {
			ccairo::cairo_set_scaled_font(self.val(), font.get_internal());
		}
		fn get_scaled_font() -> scaled_font {
			ret wrap_scaled_font(ccairo::cairo_scaled_font_reference(ccairo::cairo_get_scaled_font(self.val())));
		}
		fn show_text(text: str) unsafe {
			let mut bytes = str::bytes(text);
		
			vec::push(bytes, 0 as u8);
		
			ccairo::cairo_show_text(self.val(), vec::unsafe::to_ptr(bytes));
		}
		fn show_glyphs(glyphs: [glyph]) unsafe {
			let mut cglyphs: [libc::intptr_t] = [];
		
		        for glyphs.each |glyph| {
				cglyphs += [glyph.get_internal()];
			}
		
			ccairo::cairo_show_glyphs(self.val(), vec::unsafe::to_ptr(cglyphs) as libc::intptr_t, vec::len(cglyphs) as libc::c_int);
		}
		fn show_text_glyphs(text: str, glyphs: [glyph], clusters: [text_cluster], cluster_flags: text_cluster_flags) unsafe {
			let mut cglyphs: [libc::intptr_t] = [];
			let mut cclusters: [libc::intptr_t] = [];
			let mut bytes = str::bytes(text);
		
			vec::push(bytes, 0 as u8);
		
		        for glyphs.each |glyph| {
				cglyphs += [glyph.get_internal()];
			}
		        for clusters.each |cluster| {
				cclusters += [cluster.get_internal()];
			}
		
			ccairo::cairo_show_text_glyphs(self.val(), vec::unsafe::to_ptr(bytes), vec::len(bytes) as libc::c_int, vec::unsafe::to_ptr(cglyphs) as libc::intptr_t, vec::len(cglyphs) as libc::c_int, vec::unsafe::to_ptr(cclusters) as libc::intptr_t, vec::len(cclusters) as libc::c_int, cluster_flags as libc::c_int);
		}
		fn font_extents() -> font_extents {
			let record: font_extents_record = {
				mut ascent: 0.0,
				mut descent: 0.0,
				mut height: 0.0,
				mut max_x_advance: 0.0,
				mut max_y_advance: 0.0
			};
			let result = wrap_font_extents(ptr::addr_of(record) as libc::intptr_t);
		
			ccairo::cairo_font_extents(self.val(), result.get_internal());
		
			ret result;
		}
		fn text_extents(text: str) -> text_extents unsafe {
			let mut bytes = str::bytes(text);
			let record = @{
				mut x_bearing: 0.0,
				mut y_bearing: 0.0,
				mut width: 0.0,
				mut height: 0.0,
				mut x_advance: 0.0,
				mut y_advance: 0.0
			};
			let result = wrap_text_extents(ptr::addr_of(record) as libc::intptr_t);
		
			vec::push(bytes, 0 as u8);
			ccairo::cairo_text_extents(self.val(), vec::unsafe::to_ptr(bytes), result.get_internal());
		
			ret result;
		}
		fn glyph_extents(glyphs: [glyph]) -> text_extents unsafe {
			let record = @{
				mut x_bearing: 0.0,
				mut y_bearing: 0.0,
				mut width: 0.0,
				mut height: 0.0,
				mut x_advance: 0.0,
				mut y_advance: 0.0
			};
			let result = wrap_text_extents(ptr::addr_of(record) as libc::intptr_t);
			let mut cglyphs: [libc::intptr_t] = [];
		
		        for glyphs.each |glyph| {
				cglyphs += [glyph.get_internal()];
			}
		
			ccairo::cairo_glyph_extents(self.val(), vec::unsafe::to_ptr(cglyphs) as libc::intptr_t, vec::len(cglyphs) as libc::c_int, result.get_internal());
		
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
	ret ccairo::cairo_format_stride_for_width(format as libc::c_int, width as libc::c_int) as uint;
}
fn status_to_str(status: status) -> str unsafe {
	ret str::unsafe::from_c_str(ccairo::cairo_status_to_string(status as libc::c_int));
}
fn get_version() -> str {
	ret "v0.2.1pre";
}
fn get_cairo_version() -> str unsafe {
	ret str::unsafe::from_c_str(ccairo::cairo_version_string());
}
