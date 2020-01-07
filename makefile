# makefile to insert optimization flags directly in rustc (avoid cargo)
build_v1:
	rustc -C opt-level=3 -C target-cpu=native -C codegen-units=1 ./src/nbody_v1.rs -o ./nbody.o

build_v2:
	rustc -C opt-level=3 -C target-cpu=native -C codegen-units=1 ./src/nbody_v2.rs -o ./nbody.o

clean:
	rm -f ./nbody.o

run:
	./nbody.o 50000000