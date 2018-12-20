# openscad-models-rust
Rust workspace for my OpenSCAD projects

## Dependencies

### Rust-Scad

- [crates.io](https://crates.io/crates/scad)
- [docs](http://thezoq2.github.io/Rust-Scad/doc/scad_generator/index.html)

## Building the Projects

The [runner](.cargo/config) and used in this workspace expects OpenSCAD binary name to be `openscad`.

```bash
cargo build

# generates package-name.scad and opens it with OpenSCAD
cargo run
```

## Projects

- [dimdraw](libraries/dimdraw) A Rust library to generate annotated engineering drawings in OpenSCAD

- [parts](libraries/parts) A Rust library of OpenSCAD parts and things

- [House](sandbox/house) A house project for experimenting with OpenSCAD

- [Rammed Earth House](sandbox/rammed-earth) A Rammed Earth house project

- [Slab Table](wood-projects/slab-table) A live edge slab coffee table

- [Table](wood-projects/table) Smaller sized table
