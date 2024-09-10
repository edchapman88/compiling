# By default `make` starts with the first target
hello : hello.o
	# -o "Write output to <file>".
	gcc -o hello hello.o

hello.o :
	# -c "Only run preprocess, compile, and assemble steps" which produces `.o` file only.
	gcc -c -o hello.o simple.c

clean :
	rm hello.o hello
