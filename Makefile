all:
	mkdir -p ./lib
	rustc --lib ./cairo.rc --out-dir ./lib
test: all
	rustc --test -L ./lib ./test/simple.rs --out-dir ./test
	rustc --test -L ./lib ./test/a1-bug.rs --out-dir ./test
	rustc --test -L ./lib ./test/rounded-rectangle-fill.rs --out-dir ./test
	rustc --test -L ./lib ./test/rounded-rectangle-stroke.rs --out-dir ./test
	rustc --test -L ./lib ./test/toy-font-face.rs --out-dir ./test
	rustc --test -L ./lib ./test/shape-sierpinski.rs --out-dir ./test
	rustc --test -L ./lib ./test/ft-font-face.rs --out-dir ./test
	rustc --test -L ./lib ./test/svg.rs --out-dir ./test
	rustc --test -L ./lib ./test/pdf.rs --out-dir ./test
	rustc --test -L ./lib ./test/ps.rs --out-dir ./test
	rustc --test -L ./lib ./test/png.rs --out-dir ./test
	cd test \
	&& ./simple \
	&& ./a1-bug \
	&& ./rounded-rectangle-fill \
	&& ./rounded-rectangle-stroke \
	&& ./toy-font-face \
	&& ./shape-sierpinski \
	&& ./ft-font-face \
	&& ./svg \
	&& ./pdf \
	&& ./ps \
	&& ./png \
	&& cd ..
clean:
	rm -f ./lib/*
	find ./test -type f -not -iname "*.rs" -exec rm \{\} \;
