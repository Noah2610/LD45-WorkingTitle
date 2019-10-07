# Cross-Compiling from Linux to Windows
## Install Windows target
```
rustup target x86_64-pc-windows-gnu
```

## Compiling
```
cargo +stable build --release --target x86_64-pc-windows-gnu
```

## `lexical-core` compile issue
https://stackoverflow.com/a/56602123/10927893

Copying the files `/usr/x86_64-w64-mingw32/lib/{crt2.o,dllcrt2.o}` to `~/.rustup/toolchains/<CURRENT-TOOLCHAIN>/lib/rustlib/x86_64-pc-windows-gnu/lib/`  
solved the issue for me!
