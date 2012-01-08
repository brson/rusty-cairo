use std;
use cairo;

const WIDTH: uint = 1024u;
const HEIGHT: uint = 600u;

const FRAC_1_SQRT_3: float = 0.577359269;

fn triangle(context: cairo::context, size: float)
{
	context.move_to(0.0, 0.0);
	context.line_to(size, 0.0);
	context.line_to(size / 2.0, size * FRAC_1_SQRT_3);

	let nsize = size / 2.0;
	
	if size >= 4.0 {
		triangle(context, nsize);
		
		context.save();
		context.translate(nsize, 0.0);
		triangle(context, nsize);
		context.restore();
		
		context.save();
		context.translate(nsize / 2.0, nsize * FRAC_1_SQRT_3);
		triangle(context, nsize);
		context.restore();
	}
}

#[test]
fn test()
{
	let surface: cairo::surface = cairo::mk_image_surface(cairo::FORMAT_RGB24, WIDTH, 2u * HEIGHT);
	let context: cairo::context = cairo::mk_context(surface);
	
	context.set_source_rgb(1.0, 1.0, 1.0);
	context.paint();
	context.translate(0.0, 8.0);
	context.set_source_rgb(0.0, 0.0, 0.0);
	context.set_line_width(1.0);
	
	triangle(context, WIDTH as float);
	
	context.translate(0.0, 2.0 * (HEIGHT as float) - 16.0);
	context.scale(1.0, -1.0);
	
	triangle(context, WIDTH as float);
	
	context.stroke();
	
	assert context.get_status() == cairo::STATUS_SUCCESS;
	assert surface.write_to_file("shape-sierpinski.png") == cairo::STATUS_SUCCESS;
}
