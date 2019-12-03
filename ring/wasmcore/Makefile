
# DOCKER LLVM

DOCKER_IMAGE_VERSION := 0.3.0
DOCKER_IMAGE := wehlutyk/wasm-compiler:$(DOCKER_IMAGE_VERSION)
export BAR_LIB_DIR := libwasmcore
DOCKER_RUN = sudo docker run \
  --user $(shell id -u):$(shell id -g) \
  --volume $(CURDIR):/c:ro \
  --volume $(CURDIR)/$(BAR_LIB_DIR):/c/$(BAR_LIB_DIR) \
  --workdir /c \
  --interactive \
  --tty \
  --rm \
  $(DOCKER_IMAGE)
#--volume $(CURDIR)/target:/c/target \
#--volume $HOME/.cargo:/cargo \
#--volume (rustc +nightly --print sysroot):/rust:ro \
all: $(OUTPATHS_BROWSER) $(OUTPATHS_NODEJS)

LLVM_HOST := native

ifeq ($(LLVM_HOST), docker)
	LLVM_AR := $(DOCKER_RUN) /clang/bin/llvm-ar
	LLVM_CLANG := $(DOCKER_RUN) /clang/bin/clang
else
	LLVM_AR :=  llvm-ar-6.0
	LLVM_CLANG := clang
endif


target/wasm32-unknown-unknown/debug/foo.wasm: src/* $(BAR_LIB_DIR)/libbar.a
	cargo build --target=wasm32-unknown-unknown

$(BAR_LIB_DIR)/libwasmcore.a: $(BAR_LIB_DIR)/wasmcore.o
	$(LLVM_AR) rcsv $(BAR_LIB_DIR)/libwasmcore.a $(BAR_LIB_DIR)/wasmcore.o

$(BAR_LIB_DIR)/wasmcore.o: $(BAR_LIB_DIR)/lib.c
	$(LLVM_CLANG) -c --target=wasm32-unknown-unknown-wasm -emit-llvm -o $(BAR_LIB_DIR)/wasmcore.o $(BAR_LIB_DIR)/lib.c

doit: $(BAR_LIB_DIR)/libwasmcore.a

clean:
	rm -f $(BAR_LIB_DIR)/*.o $(BAR_LIB_DIR)/*.a

.PHONY: all clean env test doit