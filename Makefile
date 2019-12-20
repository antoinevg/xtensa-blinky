PROJECT_NAME := blinky

DEVICE=/dev/cu.SLAB_USBtoUART


all: build

build:
		ulimit -n 1024 && idf.py build

flash:
		ulimit -n 1024 && idf.py -p $(DEVICE) flash &&	\
		idf.py -p $(DEVICE) monitor

clean:
	rm -rf build/
	rm -rf components/rust/target/
	rm -rf components/rust/tmp/
	rm -rf components/rust/src/rust_build-build
	rm -rf components/rust/src/rust_build-stamp/

.PHONY: build
