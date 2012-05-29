use std;
use cairo;

#[test]
fn png() {
	let mut surface: cairo::surface = cairo::mk_image_surface(cairo::FORMAT_RGB24, 200u, 100u);
	let mut context: cairo::context = cairo::mk_context(surface);
	
	context.set_source_rgb(0.0, 0.0, 0.0);
	context.paint();
	
	assert context.get_status() == cairo::STATUS_SUCCESS;
	assert surface.write_to_file("png-1.png") == cairo::STATUS_SUCCESS;
	
	surface.flush();
	
	surface = cairo::mk_image_surface_from_file("png-1.png");
	context = cairo::mk_context(surface);
	
	context.set_source_rgb(0.0, 1.0, 1.0);
	context.rectangle(40.0, 10.0, 120.0, 80.0);
	context.fill();
	
	assert context.get_status() == cairo::STATUS_SUCCESS;
	assert surface.image_get_width() == 200u;
	assert surface.image_get_height() == 100u;
	assert surface.write_to_file("png-2.png") == cairo::STATUS_SUCCESS;
	
	surface.flush();
}
