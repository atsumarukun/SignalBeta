x86_64-unknown-uefi:
	cd ./uefi && cargo build

x86_64-unknown-elf:
	cd ./kernel && cargo build

cp: x86_64-unknown-uefi x86_64-unknown-elf
	mkdir -p ./esp/efi/boot
	cp ./target/x86_64-unknown-uefi/debug/uefi.efi ./esp/efi/boot/bootx64.efi
	cp ./target/x86_64-unknown-elf/debug/kernel.elf ./esp/kernel.elf

.PHONY: boot
boot: cp
	qemu-system-x86_64 -bios OVMF.fd -drive format=raw,file=fat:rw:esp -monitor stdio