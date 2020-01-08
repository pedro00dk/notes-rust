# makefile to insert optimization flags directly in rustc (avoid cargo)
build:
	rustc -C opt-level=3 -C target-cpu=native -C codegen-units=1 ./src/nbody_idiom.rs -o ./nbody.o

build_v1:
	rustc -C opt-level=3 -C target-cpu=native -C codegen-units=1 ./src/nbody_v1.rs -o ./nbody.o

build_v2:
	rustc -C opt-level=3 -C target-cpu=native -C codegen-units=1 ./src/nbody_v2.rs -o ./nbody.o

build_v3:
	rustc -C opt-level=3 -C target-cpu=native -C codegen-units=1 ./src/nbody_v3.rs -o ./nbody.o

build_v4:
	rustc -C opt-level=3 -C target-cpu=native -C codegen-units=1 ./src/nbody_v4.rs -o ./nbody.o

build_v5:
	rustc -C opt-level=3 -C target-cpu=native -C codegen-units=1 ./src/nbody_v5.rs -o ./nbody.o

clean:
	rm -f ./nbody.o

run:
	./nbody.o 50000000