# simple_fft

Cooley-Tukey FFT and IFFT in Rust.

## Install

```bash
cargo add --git https://github.com/josoriom/simple_fft simple_fft
```

## How to use

Lengths must be a power of two.

```rust
use simple_fft::{convolution, deconvolution, fft, ifft, Input, Plan};

let spectrum = fft(Input::Real(vec![1.0, 2.0, 3.0, 4.0]));
let signal = ifft(Input::Complex(spectrum));
```

Build a `Plan` when you transform the same length more than once. It keeps the
twiddle table instead of rebuilding it on every call.

```rust
let plan = Plan::new(1024);
let spectrum = plan.fft(Input::Real(samples));
```


**Safe by default.** The default build contains no `unsafe` code.
Wide kernels are behind the `simd` feature, which you have to turn on by using simd feature:

```toml
simple_fft = { version = "0.1", features = ["simd"] }
```

All `unsafe` lives in `src/simd`, and that folder is not compiled unless the
feature is on. Results are the same either way, so the feature buys speed only.

## Support

| Architecture | Default build | With `simd` |
| --- | --- | --- |
| aarch64 (arm64) | scalar | NEON, radix 2 and radix 4 |
| x86_64 | scalar | AVX2 radix 2, found at run time |
| everything else | scalar | scalar |




