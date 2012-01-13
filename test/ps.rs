use std;
use cairo;

#[test]
fn test() {
	let surface: cairo::surface = cairo::mk_ps_surface("ps.ps", 100.0, 100.0);
	let context: cairo::context = cairo::mk_context(surface);
	
	context.set_source_rgb(0.0, 0.0, 0.0);
	context.paint();
	context.set_source_rgb(1.0, 0.0, 0.0);
	context.rectangle(10.0, 10.0, 80.0, 80.0);
	context.fill();
	surface.flush();
	
	assert surface.get_status() == cairo::STATUS_SUCCESS;
	assert context.get_status() == cairo::STATUS_SUCCESS;
}
