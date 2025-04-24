build-capi-linux-x64:
	mkdir -p libs/linux_x64 && cd mrml-capi && cargo build --release --target=x86_64-unknown-linux-gnu && cp target/x86_64-unknown-linux-gnu/release/libmrml_capi.a ../libs/linux_x64

build-capi-linux-x86:
	mkdir -p libs/linux_x86 && cd mrml-capi && cargo build --release --target=i686-unknown-linux-gnu && cp target/i686-unknown-linux-gnu/release/libmrml_capi.a ../libs/linux_x86

build-capi-linux-arm64:
	mkdir -p libs/linux_arm64 && cd mrml-capi && cargo build --release --target=aarch64-unknown-linux-gnu && cp target/aarch64-unknown-linux-gnu/release/libmrml_capi.a ../libs/linux_arm64

build-capi-windows-x64:
	mkdir -p libs/windows_x64 && cd mrml-capi && cargo build --release --target=x86_64-pc-windows-gnu && cp target/x86_64-pc-windows-gnu/release/libmrml_capi.a ../libs/windows_x64

build-capi-windows-x86:
	mkdir -p libs/windows_x86 && cd mrml-capi && cargo build --release --target=i686-pc-windows-gnu && cp target/i686-pc-windows-gnu/release/libmrml_capi.a ../libs/windows_x86

build-capi-darwin-arm64:
	mkdir -p libs/darwin_arm64 && cd mrml-capi && cargo build --release --target=aarch64-apple-darwin && cp target/aarch64-apple-darwin/release/libmrml_capi.a ../libs/darwin_arm64

build-capi-darwin-arm64:
	mkdir -p libs/darwin_x64 && cd mrml-capi && cargo build --release --target=x86_64-apple-darwin && cp target/x86_64-apple-darwin/release/libmrml_capi.a ../libs/darwin_x64

build-all: build-capi-linux-x64 build-capi-linux-x86 build-capi-linux-arm64 build-capi-windows-x64 build-capi-windows-x86  build-capi-darwin-arm64 build-capi-darwin-x64 

clean:
	cd mrml-capi && cargo clean
	rm -rf libs/*