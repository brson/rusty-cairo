use std;
use cairo;

const WIDTH: uint = 1000u;
const HEIGHT: uint = 800u;

#[test]
fn test() {
	let surface: cairo::surface = cairo::mk_image_surface(cairo::FORMAT_RGB24, WIDTH, HEIGHT);
	let context: cairo::context = cairo::mk_context(surface);
	let points: [[float]] = [
		[627.016212, 221.749777],
		[756.120787, 221.749777],
		[756.120787, 557.602766],
		[626.952721, 557.602766],
		[626.548456, 493.315729]
	];
	
	context.set_source_rgb(0.0, 0.0, 0.0);
	context.paint();
	
	for point in points {
		context.line_to(point[0], point[1]);
	}
	
	context.set_source_rgb(1.0, 0.0, 0.0);
	context.fill_preserve();
	context.set_antialias(cairo::ANTIALIAS_NONE);
	context.set_source_rgb(0.0, 1.0, 0.0);
	context.fill();
	
	assert context.get_status() == cairo::STATUS_SUCCESS;
	assert surface.write_to_file("a1-bug.png") == cairo::STATUS_SUCCESS;
}
