# ssb

A Rust implementation that runs all SSB queries on `.tbl` tables.

## Usage

### Compiling the SSB Generator

```
git submodule update --init
cd ssb-dbgen
cmake .
make -j
```

### Generating the Tables

The following command will generate ssb tables with scale factor 1.

```
./dbgen
cd ..
```

### Run It!

If you haven't install Rust toolchains, uses [rustup](https://rustup.rs/) to install them.

Use cargo to compile and run the binary:

```
cargo run --release
```

The output would be:

```
Loading...
Takes 4.336 seconds to load.
q11 takes 60 ms.
q12 takes 29 ms.
q13 takes 27 ms.
q21 takes 141 ms.
q22 takes 152 ms.
q23 takes 108 ms.
q31 takes 244 ms.
q32 takes 176 ms.
q33 takes 185 ms.
q34 takes 97 ms.
q41 takes 277 ms.
q42 takes 261 ms.
q43 takes 110 ms.
```
