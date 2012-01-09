use std;
use cairo;

#[test]
fn test() {
	let surface: cairo::surface = cairo::mk_pdf_surface("svg.pdf", 100.0, 100.0);
	let context: cairo::context = cairo::mk_context(surface);
	
	context.set_source_rgb(0.0, 0.0, 0.0);
	context.paint();
	context.set_source_rgb(1.0, 0.0, 0.0);
	context.rectangle(10.0, 10.0, 80.0, 80.0);
	context.fill();
	
	assert context.get_status() == cairo::STATUS_SUCCESS;
}
