use std;
use cairo;

#[test]
fn test() {
	let surface: cairo::surface = cairo::mk_image_surface(cairo::FORMAT_RGB24, 0u, 0u);
	let context: cairo::context = cairo::mk_context(surface);
	let font: cairo::font_face = context.get_font_face();
	
	assert font.get_type() == cairo::FONT_TYPE_TOY;
	assert font.get_toy_slant() == cairo::FONT_SLANT_NORMAL;
	assert font.get_toy_weight() == cairo::FONT_WEIGHT_NORMAL;
	assert font.get_status() == cairo::STATUS_SUCCESS;
	
	context.select_font_face("bizarre", cairo::FONT_SLANT_OBLIQUE, cairo::FONT_WEIGHT_BOLD);
	
	font = context.get_font_face();
	
	assert font.get_type() == cairo::FONT_TYPE_TOY;
	assert font.get_toy_family() == "bizarre";
	assert font.get_toy_slant() == cairo::FONT_SLANT_OBLIQUE;
	assert font.get_toy_weight() == cairo::FONT_WEIGHT_BOLD;
	assert font.get_status() == cairo::STATUS_SUCCESS;
	
	context.set_font_face(font);
	
	assert context.get_status() == cairo::STATUS_SUCCESS;
}
