PROJECT_NAME := blinky

all: build

build:
		ulimit -n 1024 && idf.py build

flash:
		ulimit -n 1024 && idf.py -p /dev/cu.SLAB_USBtoUART flash &&	\
		idf.py -p /dev/cu.SLAB_USBtoUART monitor

clean:
	rm -rf build/
	rm -rf components/rust/target/
	rm -rf components/rust/tmp/
	rm -rf components/rust/src/rust_build-build
	rm -rf components/rust/src/rust_build-stamp/

.PHONY: build
