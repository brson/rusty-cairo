## v0.2

* Added matrix.multiply(other), font_options.merge(other), font_options.equals(other).
* Fixed copy_path_flat.
* Upgraded to implementation and interfaces in Rust, rather than objs (objs are now deprecated).
* Fixed freetype fonts being cleaned up correctly (valgrind now reports 0 errors).
* Added get_version (gets bindings version) and get_cairo_version (gets internal cairo version).
* Fixed strict type requirements on different architectures.
* Fixed incorrect values on some constants.
* Added messages for all failures. (using status_to_str)
* Added mk_ps_surface. Same deal as SVG and PDF surfaces, they will be changed in the future so that you use write_to_file rather than providing the file name in the constructor.

## v0.1.2

* Added freetype support (mk_font_face_from_file("../some/font.ttf")) (will add more backends later, quartz, win32, etc.)
* Added mk_pdf_surface and mk_svg_surface. Currently it requires the output file name as the first argument for the constructors. In the future it will be changed so you can use write_to_file and if it was a specially made svg or pdf surface and it has a .svg/.pdf extension, it would save it.

## v0.1.1

* Added export statement so internal functions don't get exported.
* Fixed accidental naming of and added some more cairo constants.
* Renamed crate.rs to cairo.rs.
* Ensured that required core libraries were imported via import statements.
* Made all record types mutable.
* Added glyph.set_\* and text_cluster.set_\*.
* Added surface.restrict_\*_to_version for special surfaces that have versioning. Considering special surfaces don't work yet, these don't work.
* Added surface.set_pdf_size(width, height)
* Added text_extents.get_size() -> (width, height)
* Added surface.get_image_size(), surface.get_image_width(), surface.get_image_height(), surface.get_image_data() for image surfaces.

## v0.1

* Initial version of the bindings, most functionality of cairo is supplied.
