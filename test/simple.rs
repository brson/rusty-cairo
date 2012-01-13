use std;
use cairo;
import core::f64::consts;

#[test]
fn test() {
	let surface = cairo::mk_image_surface(cairo::FORMAT_RGB24, 100u, 100u);
	let context = cairo::mk_context(surface);
	
	context.set_source_rgb(1.0, 0.0, 0.0);
	context.paint();
	context.set_source_rgb(1.0, 1.0, 1.0);
	context.arc(50.0, 50.0, 50.0, 0.0, core::f64::consts::pi * 2.0);
	context.fill();
	
	assert context.get_status() == cairo::STATUS_SUCCESS;
	assert surface.write_to_file("simple.png") == cairo::STATUS_SUCCESS;
}
