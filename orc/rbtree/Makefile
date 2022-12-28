main:  \
./rbtree.o \
./test.o
	gcc -o main  \
./rbtree.o \
./test.o

.c.o: 
	gcc -c $<

.cpp.o: 
	g++ -c $<

clean::
	rm -rf Makefile main c2rust crusts compile_commands.json txl10.8b.linux64 Cargo.lock target