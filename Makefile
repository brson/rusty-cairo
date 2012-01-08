TEST_FILES := $(wildcard test/*.rs)

all:
	rustc --lib ./src/crate.rs --out-dir=./lib
test: all
	rustc --test -L ./lib ./test/a1-bug.rs -o ./test/a1-bug.elf
	rustc --test -L ./lib ./test/rounded-rectangle-fill.rs -o ./test/rounded-rectangle-fill.elf
	rustc --test -L ./lib ./test/rounded-rectangle-stroke.rs -o ./test/rounded-rectangle-stroke.elf
	rustc --test -L ./lib ./test/toy-font-face.rs -o ./test/toy-font-face.elf
	rustc --test -L ./lib ./test/shape-sierpinski.rs -o ./test/shape-sierpinski.elf
	cd test \
	&& ./a1-bug.elf \
	&& ./rounded-rectangle-fill.elf \
	&& ./rounded-rectangle-stroke.elf \
	&& ./toy-font-face.elf \
	&& ./shape-sierpinski.elf \
	&& cd ..
clean:
	rm -R -f ./lib/*
	rm -R -f ./test/*.elf
	rm -R -f ./test/*.png
