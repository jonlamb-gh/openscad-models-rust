# openscad-models-rust
Rust workspace for my OpenScad projects

## Dependencies

### Rust-Scad

- [crates.io](https://crates.io/crates/scad)
- [docs](http://thezoq2.github.io/Rust-Scad/doc/scad_generator/index.html)

## Building the Projects

The runner used in this workspace expects OpenScad binary name to be `openscad`.

```bash
cargo build

# generates package-name.scad and opens it with OpenScad
cargo run
```

## Projects

- [dimdraw](libraries/dimdraw) A Rust library to generate annotated engineering drawings in OpenScad
