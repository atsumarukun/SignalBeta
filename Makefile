x86_64-unknown-uefi:
	cd ./uefi && cargo build

cp-efi: x86_64-unknown-uefi
	mkdir -p ./esp/efi/boot
	cp ./target/x86_64-unknown-uefi/debug/uefi.efi ./esp/efi/boot/bootx64.efi

.PHONY: boot
boot: cp-efi 
	qemu-system-x86_64 -bios OVMF.fd -drive format=raw,file=fat:rw:esp -monitor stdio