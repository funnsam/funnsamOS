i686-elf-as codes/boot.s -o boot.o
i686-elf-gccgo -static -Werror -nostdlib -nostartfiles -nodefaultlibs -c codes/terminal.go -o terminal.go.o
i686-elf-objcopy -j .go_export terminal.go.o terminal.gox
i686-elf-gccgo -static -Werror -nostdlib -nostartfiles -nodefaultlibs -c codes/kernel.go -o kernel.go.o
i686-elf-gcc -T codes/linker.ld -o myos.bin -ffreestanding -O2 -nostdlib boot.o terminal.go.o kernel.go.o -lgcc