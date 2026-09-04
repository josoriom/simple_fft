# simple_fft

Cooley-Tukey FFT and IFFT in Rust.

## Install

```bash
cargo add --git https://github.com/josoriom/simple_fft simple_fft
```

## How to use

A signal is a `Data`, two arrays of the same length. Both the real and the
imaginary parts are always given. Lengths must be a power of two.

```rust
use simple_fft::{convolution, deconvolution, fft, ifft, Data, Plan};

let real = vec![1.0, 2.0, 3.0, 4.0];
let imag = vec![0.0, 0.0, 0.0, 0.0];

let spectrum = fft(Data { real, imag });
let signal = ifft(spectrum);
```

Build a `Plan` when you transform the same length more than once. It keeps the
twiddle table instead of rebuilding it on every call.

```rust
let plan = Plan::new(1024);
let spectrum = plan.fft(Data { real, imag });
```

**Safe by default.** The default build contains no `unsafe` code. The wide
kernels sit behind the `simd` feature, which you turn on yourself:

```toml
simple_fft = { git = "https://github.com/josoriom/simple_fft", features = ["simd"] }
```

All `unsafe` lives in `src/simd`, and that folder is not compiled unless the
feature is on. The feature buys speed only; the answers agree to the last bit
or two, since the wide kernels fuse multiply and add and so round once where
the plain ones round twice.

## Support

| Architecture | Default build | With `simd` |
| --- | --- | --- |
| aarch64 (arm64) | scalar | NEON, radix 2 and radix 4 |
| x86_64 | scalar | AVX2 radix 2 and radix 4, found at run time |
| everything else | scalar | scalar |
