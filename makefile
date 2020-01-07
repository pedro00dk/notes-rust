# makefile to insert optimization flags directly in rustc (avoid cargo)
build:
	rustc -C opt-level=3 -C target-cpu=native -C codegen-units=1 ./src/main.rs -o ./main.o

clean:
	rm -f ./main.o

run:
	./main.o 50000000