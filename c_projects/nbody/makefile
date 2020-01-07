build-gcc: nbody.c
	/usr/bin/gcc -pipe -Wall -O3 -fomit-frame-pointer -march=native -funroll-loops -static ./nbody.c -o ./nbody.o -lm

build-clang: nbody.c
	/usr/bin/clang -O3 -fomit-frame-pointer -march=native -funroll-loops -static ./nbody.c -o ./nbody.o -lm

clean:
	rm -f nbody.o

run:
	./nbody.o 50000000