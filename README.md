# Add solution

This repository is for experiment `https://github.com/SteveSandersonMS/wasm-component-sdk`.

> Run on Windows 11 only. It couldn't work on Ubuntu or MacOS 😂

## wasm/wasi component with C#

```sh
> dotnet new sln -n calculator
> dotnet sln calculator.sln add MyApp/MyApp.csproj 
> dotnet sln calculator.sln add Adder/Adder.csproj
```

```sh
> cd Adder 
> dotnet build
```

You should see the wasm/wasi component at `bin\Debug\net8.0\wasi-wasm\native\Adder.component.wasm`

## wasm/wasi component with Spin (Rust)

```sh
> cd rust-host
> spin build
> spin run
> curl http://127.0.0.1:3000/?"x=10&y=40"
```

## wasm-tools (compose and run `Adder` on MyApp - C#)

```sh
> wasm-tools component wit Adder\bin\Debug\net8.0\wasi-wasm\native\Adder.component.wasm
```

```sh
> wasm-tools compose -o composed.wasm MyApp\bin\Debug\net8.0\wasi-wasm\native\MyApp.component.wasm -d Adder\bin\Debug\net8.0\wasi-wasm\native\Adder.component.wasm
> wasmtime --wasm component-model composed.wasm
```

## Refs
- https://learn.microsoft.com/en-us/windows/dev-environment/rust/setup
