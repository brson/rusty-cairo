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
