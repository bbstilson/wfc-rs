# wfc-rs

Original Algorithm: <https://github.com/mxgmn/WaveFunctionCollapse>

Helpful basic explanation for tiled model: <https://robertheaton.com/2018/12/17/wavefunction-collapse-algorithm/>

Helpful explanation in Rust: <https://www.gridbugs.org/wave-function-collapse/>

## Renderings

![animated flowers](./flowers.gif)

```bash
cargo run --release -- ./input/flowers.png -o 80,40 -t 3,3
```

![animated city](./city.gif)

```bash
cargo run --release -- ./input/city.png -o 100,50 -t 4,4
```

## Running

```bash
cargo run --release -- --help

wfc-rs 0.1.0
Brandon Stilson
Run wfc-rs

USAGE:
    wfc-rs [OPTIONS] --output-dimensions <OUTPUT_DIMENSIONS> --tile-dimensions <TILE_DIMENSIONS> <INPUT>

ARGS:
    <INPUT>    input image location

OPTIONS:
    -h, --help
            Print help information

    -o, --output-dimensions <OUTPUT_DIMENSIONS>
            output dimensions in pixels

        --parse-method <PARSE_METHOD>
            parse input as a tiled map [default: overlap] [possible values: overlap, tiled]

    -s, --snapshots
            whether or not to take snapshot images

    -t, --tile-dimensions <TILE_DIMENSIONS>
            tile dimensions to parse from input image

    -V, --version
            Print version information

    -w, --with-tile-variations
            whether or not create all variations (rotations and reflections) of tiles
```

To build the animated gif, the program must be run with the `-s, --snapshots` flag, then you can run the python script to compile all the images into a gif.

Depending on the number of snapshots taken -- for `-o 150,100`, 15,000 images will be produced -- you might need to increase your `ulimit` (or whatever the equivalent is for your OS).

```bash
ulimit -n 30000
```

```bash
poetry run python animate.py
```

## Profiling Notes

The original implementation was very slow, and it's still quite slow. Here are scattered notes on profiling:

Enable:

```toml
[profile.release]
debug = true
```

Build a release build:

```bash
cargo build --release
```

Install [flamegraph-rs](https://github.com/flamegraph-rs/flamegraph) and linux (Pop!_OS. See flamegraph-rs readme for other os instructions) deps:

```bash
sudo apt install linux-tools-common linux-tools-generic
cargo install flamegraph
```

Build a flamegraph:

```bash
flamegraph -- ./target/release/wfc-rs ./input/city.png -o 40,20 -t 3,3
```
