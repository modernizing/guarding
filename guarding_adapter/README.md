# Guarding Adapter

## Usage



## Dev Setup

1. install CBindgen

```bash
cargo install --force cbindgen
```

2. generate header

```
cbindgen --config cbindgen.toml --crate guarding_adapter --output guarding_adapter.h
```
