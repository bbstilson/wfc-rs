# wfc-rs

Original Algorithm: <https://github.com/mxgmn/WaveFunctionCollapse>

Helpful basic explanation for tiled model: <https://robertheaton.com/2018/12/17/wavefunction-collapse-algorithm/>

Helpful explanation in Rust: <https://www.gridbugs.org/wave-function-collapse/>

## Renderings

![animated flowers](./flowers.gif)

```bash
cargo run --release -- ./input/flowers.png -o 80,40 -t 3,3 -m
```

![animated city](./city.gif)

```bash
cargo run --release -- ./input/smog-city.png -o 80,40 -t 4,4 -m
```

![animated village](./village.gif)

```bash
cargo run --release -- ./input/village.png -o 80,40 -t 3,3 -m
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

    -m, --make-gif
            whether or not to make a gif (warning: very slow)

    -t, --tile-dimensions <TILE_DIMENSIONS>
            tile dimensions to parse from input image

    -V, --version
            Print version information

    -w, --with-tile-variations
            whether or not create all variations (rotations and reflections) of tiles
```

## TODOs

- tiled mode
- reflections and rotations of tiles
- weights in directions
- global weights of tiles
- shannon entropy of cells
- detect ground tiles and do something with them
- diagonal directions?
- 3d models?
