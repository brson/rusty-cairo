use std;
use cairo;

const WIDTH: int = 80;
const HEIGHT: int = 80;
const PI: float = 3.141592653589793;

fn rounded_rectangle(context: cairo::context, x: float, y: float, w: float, h: float, r: float)
{
	context.new_sub_path();
	context.arc(x + r, y + r, r, PI, 3.0 * PI / 2.0);
	context.arc(x + w - r, y + r, r, 3.0 * PI / 2.0, 2.0 * PI);
	context.arc(x + w - r, y + h - r, r, 0.0, PI / 2.0);
	context.arc(x + r, y + h - r, r, PI / 2.0, PI);
	context.close_path();
}

#[test]
fn test() {
	let surface: cairo::surface = cairo::mk_image_surface(cairo::FORMAT_RGB24, WIDTH, HEIGHT);
	let context: cairo::context = cairo::mk_context(surface);
	let widthd: float = WIDTH as float;
	let heightd: float = HEIGHT as float;
	
	context.set_source_rgb(1.0, 1.0, 1.0);
	context.paint();
	context.set_source_rgb(0.0, 0.0, 0.0);
	context.set_line_width(10.0);
	rounded_rectangle(context, 10.0, 10.0, widthd - 20.0, heightd - 20.0, 10.0);
	context.stroke();
	
	assert context.get_status() == cairo::STATUS_SUCCESS;
	assert surface.write_to_file("rounded-rectangle-stroke.png") == cairo::STATUS_SUCCESS;
}
