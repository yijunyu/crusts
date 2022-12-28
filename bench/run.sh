#!/bin/bash
init() {
	git clone https://github.com/clibs/buffer
	git clone https://github.com/haampie/libtree
	git clone https://github.com/jakogut/tinyvm
	git clone https://github.com/c-util/c-rbtree
}
#init
for f in *; do 
   if [ -d $f -a -f $f/Makefile ]; then
	echo evaluating on $f ... 
	cd $f > /dev/null
	make clean
	rm -rf compile_commands.json src/*.rs build.rs lib.rs Cargo.toml rust-toolchain rust-toolchain.toml Cargo.lock target
	crusts >& crusts.log
	cargo build >& cargo-build.log
	cd - > /dev/null
   fi
done
