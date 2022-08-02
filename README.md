# wfc-rs

Original Algorithm: <https://github.com/mxgmn/WaveFunctionCollapse>

<https://robertheaton.com/2018/12/17/wavefunction-collapse-algorithm/>

<https://www.gridbugs.org/wave-function-collapse/>

![animated collapse](./animated.gif)

## Running

```bash
cargo run --release -- ./input/flowers.png -o 80,40 -t 3,3
```

For more info:

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

```bash
poetry run python animate.py
```
