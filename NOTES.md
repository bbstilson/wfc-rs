# Notes

## Scaling up GIFs

```bash
convert output.gif -filter point -resize 600% output_resized.gif
```

## Profiling

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
