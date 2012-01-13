all:
	rustc --lib ./src/cairo.rs --out-dir=./lib
test: all
	rustc --test -L ./lib ./test/simple.rs -o ./test/simple.elf
	rustc --test -L ./lib ./test/a1-bug.rs -o ./test/a1-bug.elf
	rustc --test -L ./lib ./test/rounded-rectangle-fill.rs -o ./test/rounded-rectangle-fill.elf
	rustc --test -L ./lib ./test/rounded-rectangle-stroke.rs -o ./test/rounded-rectangle-stroke.elf
	rustc --test -L ./lib ./test/toy-font-face.rs -o ./test/toy-font-face.elf
	rustc --test -L ./lib ./test/shape-sierpinski.rs -o ./test/shape-sierpinski.elf
	rustc --test -L ./lib ./test/ft-font-face.rs -o ./test/ft-font-face.elf
	rustc --test -L ./lib ./test/svg.rs -o ./test/svg.elf
	rustc --test -L ./lib ./test/pdf.rs -o ./test/pdf.elf
	rustc --test -L ./lib ./test/ps.rs -o ./test/ps.elf
	cd test \
	&& ./simple.elf \
	&& ./a1-bug.elf \
	&& ./rounded-rectangle-fill.elf \
	&& ./rounded-rectangle-stroke.elf \
	&& ./toy-font-face.elf \
	&& ./shape-sierpinski.elf \
	&& ./ft-font-face.elf \
	&& ./svg.elf \
	&& ./pdf.elf \
	&& ./ps.elf \
	&& cd ..
clean:
	rm -R -f ./lib/*
	rm -R -f ./test/*.elf
	rm -R -f ./test/*.png
	rm -R -f ./test/*.svg
	rm -R -f ./test/*.pdf
	rm -R -f ./test/*.ps
