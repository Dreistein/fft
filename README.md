# (forward) FFT Rust Implementation

This is a simple and unoptimized FFT implementation written in Rust for educational purposes.  
The implementation was tested by comparing the results to numpy.fft.fft, which seem to match.  
However, I do not guarantee the correctness of the algorithm.

## Build
Run `cargo build` and `cargo run`

For copy/pasting into a python shell:  
`RUSTFLAGS='--cfg print_python' cargo run`
