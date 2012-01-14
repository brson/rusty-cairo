use std;
use cairo;

#[test]
fn png_b() {
	let surface: cairo::surface = cairo::mk_image_surface_from_file("png-1.png");
	let context: cairo::context = cairo::mk_context(surface);
	
	context.set_source_rgb(0.0, 1.0, 1.0);
	context.rectangle(40.0, 10.0, 120.0, 80.0);
	context.fill();
	
	assert context.get_status() == cairo::STATUS_SUCCESS;
	assert surface.get_image_width() == 200u;
	assert surface.get_image_height() == 100u;
	assert surface.write_to_file("png-2.png") == cairo::STATUS_SUCCESS;
}

#[test]
fn png_a() {
	let surface: cairo::surface = cairo::mk_image_surface(cairo::FORMAT_RGB24, 200u, 100u);
	let context: cairo::context = cairo::mk_context(surface);
	
	context.set_source_rgb(0.0, 0.0, 0.0);
	context.paint();
	
	assert context.get_status() == cairo::STATUS_SUCCESS;
	assert surface.write_to_file("png-1.png") == cairo::STATUS_SUCCESS;
}
