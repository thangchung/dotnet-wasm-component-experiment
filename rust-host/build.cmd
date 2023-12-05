cargo build --target wasm32-wasi --release
wasm-tools component new target/wasm32-wasi/release/my_rust_app.wasm --adapt ..\wasi_snapshot_preview1.reactor.wasm -o target/wasm32-wasi/release/my_rust_app.component.wasm

set CSHARP_COMPONENT=..\Adder\bin\Debug\net8.0\wasi-wasm\native\Adder.component.wasm

wasm-tools compose -o target\wasm32-wasi\release\my_rust_app.composed.wasm target\wasm32-wasi\release\my_rust_app.component.wasm -d %CSHARP_COMPONENT%