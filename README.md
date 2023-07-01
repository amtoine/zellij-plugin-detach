# zellij-plugin-detach
A plugin to detach a Zellij session

## installation
from source, run
```shell
cargo install --path .
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
