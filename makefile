# makefile to insert optimization flags directly in rustc (avoid cargo)
build-v1:
	rustc -C opt-level=3 -C target-cpu=native -C codegen-units=1 ./src/nbody-v1.rs -o ./nbody.o

clean:
	rm -f ./nbody.o

run:
	./nbody.o 50000000