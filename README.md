# Prerequisites

Make sure you have Rust and Cargo installed on your system.  You can download them from [https://www.rust-lang.org/](https://www.rust-lang.org/).

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run --release
```

The simulation will prompt you for the number of rows and the number of balls to simulate. If you press Enter without typing anything, default values will be used.  The output will show the mean, standard deviation, variance, and a visual representation of the distribution of the balls in the bins.

## Dependencies

* `rand`:  Used for generating random numbers to simulate the ball's path.
