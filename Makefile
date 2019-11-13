build:
	cargo build
run:
	cargo run
install:
	cargo install --path . -f
upload:
	cargo publish

# x86_64-unknown-linux-gnu
# x86_64-apple-darwin
# x86_64-pc-windows-gnu

CRATE_NAME:=goku
MISC:= README.md LICENSE
DIRNAME:=${CRATE_NAME}_${TARGET}

release_all:
	make release TARGET=x86_64-pc-windows-gnu    BIN_NAME=goku.exe
	make release TARGET=x86_64-apple-darwin      BIN_NAME=goku
	make release TARGET=x86_64-unknown-linux-gnu BIN_NAME=goku

.PHONY: release
release:
	cross build --target ${TARGET} --release
	mkdir -p ${DIRNAME}
	\
	cp ./target/${TARGET}/release/${BIN_NAME} ${DIRNAME}
	cp ${MISC} ${DIRNAME}
	\
	mkdir -p dist
	tar czf dist/${DIRNAME}.tar.gz ${DIRNAME}
	rm -rf ${DIRNAME}
