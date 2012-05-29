use std;
use cairo;

#[test]
fn ft_font_face() {
	let surface: cairo::surface = cairo::mk_image_surface(cairo::FORMAT_RGB24, 0u, 0u);
	let context: cairo::context = cairo::mk_context(surface);
	let mut font: cairo::font_face = cairo::mk_font_face_from_file("pt-sans.ttf");
	
	assert font.get_type() == cairo::FONT_TYPE_FT;
	assert font.get_status() == cairo::STATUS_SUCCESS;
	
	context.set_font_face(font);
	
	font = context.get_font_face();
	
	assert font.get_type() == cairo::FONT_TYPE_FT;
	assert font.get_status() == cairo::STATUS_SUCCESS;
	assert context.get_status() == cairo::STATUS_SUCCESS;
}
