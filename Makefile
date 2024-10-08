# Specify the target to cargo so that the dependencies specified in `Cargo.toml` are fetched and built for that target (e.g. the thumbv7 ARM target with a floating point unit: `thumbv7em-none-eabihf `).

# `--` passes the subsequent flags on to `rustc`.
# `-o` specifies the name of the output to be the name of this make target (shorthand is `$@`).
# `-C` are 'codegen' options, documented here: https://doc.rust-lang.org/rustc/codegen-options/index.html
# `linker=arm-none-eabi-gcc` to use the older default linker `arm-none-eabi-gcc`.
# `-nostartfiles` "Do not use the standard system startup files when linking. The standard system libraries are used normally, unless -nostdlib, -nolibc, or -nodefaultlibs is used."
# `-Tlink` to specify the linker script (with `-T`) as `link.x`.
hello.hex : hello.bin
	arm-none-eabi-objcopy -O ihex $< $@

hello.bin :
	cargo rustc \
	--target thumbv7em-none-eabihf \
	--bin compiling \
	-- \
	-o $@ \
	--emit obj=hello.o,asm=hello.s,dep-info=hello.d \
	-C linker=arm-none-eabi-gcc \
	-C link-arg=-nostartfiles \
	-C link-arg=-Tjames.link.x \

clean :
	rm -r *.bin *.d *.hex *.o *.s
