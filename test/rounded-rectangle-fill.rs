use std;
use cairo;
import core::f64::consts;

const WIDTH: uint = 80u;
const HEIGHT: uint = 80u;

fn rounded_rectangle(context: cairo::context, x: float, y: float, w: float, h: float, r: float)
{	
	context.new_sub_path();
	context.arc(x + r, y + r, r, core::f64::consts::pi, 3.0 * core::f64::consts::pi / 2.0);
	context.arc(x + w - r, y + r, r, 3.0 * core::f64::consts::pi / 2.0, 2.0 * core::f64::consts::pi);
	context.arc(x + w - r, y + h - r, r, 0.0, core::f64::consts::pi / 2.0);
	context.arc(x + r, y + h - r, r, core::f64::consts::pi / 2.0, core::f64::consts::pi);
	context.close_path();
}

#[test]
fn rounded_rectangle_fill() {
	let surface: cairo::surface = cairo::mk_image_surface(cairo::FORMAT_RGB24, WIDTH, HEIGHT);
	let context: cairo::context = cairo::mk_context(surface);
	let widthd: float = WIDTH as float;
	let heightd: float = HEIGHT as float;
	
	context.set_source_rgb(1.0, 1.0, 1.0);
	context.paint();
	context.set_source_rgb(0.0, 0.0, 0.0);
	context.set_fill_rule(cairo::FILL_RULE_EVEN_ODD);
	rounded_rectangle(context, 5.0, 5.0, widthd - 10.0, heightd - 10.0, 15.0);
	rounded_rectangle(context, 15.0, 15.0, widthd - 30.0, heightd - 30.0, 5.0);
	context.fill();
	
	assert context.get_status() == cairo::STATUS_SUCCESS;
	assert surface.write_to_file("rounded-rectangle-fill.png") == cairo::STATUS_SUCCESS;
}
