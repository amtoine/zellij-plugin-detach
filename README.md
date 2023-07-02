# zellij-plugin-detach
A plugin to detach a Zellij session

## installation
first, you'll need to install the `wasm32-wasi` target to your `rustup` installation
```shell
rustup target add wasm32-wasi
```
once that's done and installed, you can
- build the plugin
```shell
cargo build --target wasm32-wasi
```
- install the plugin from source with
```shell
cargo install --target wasm32-wasi --path .
```

## some ideas of usage
in Nushell, we can define
```nushell
def "zellij detach" [] {
    if $env.ZELLIJ? != null {
        zellij action start-or-reload-plugin $"file:($env.CARGO_HOME | path join "bin" "zellij-plugin-detach.wasm")"
    }
}
```
