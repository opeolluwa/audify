# audify-rs
Convert PDF to audio files, usable in mobile, servers and desktop applications.


## Why use Rust? 
Rust in audify-rs is encouraged by the possiblity of running Rust on mutiple platforms and architectures with very little hardware footprint 

## Getting Started

### Dependencies

- [wget](https://www.gnu.org/software/wget/)
- [rust](https://rust-lang.org)
- [just](https://just.systems)

### Installing

```sh
git clone https://github.com/opeolluwa/audify-rs
cd audify-rs
just download-models
```

### Executing program

```sh
cargo run --example audio
```

## Use cases 
- [Audify Client](https://github.com/opeolluwa/audify-client) An application targeting major desktop and mobile operating system, built on [audify-rs](https://github.com/opeolluwa/audify-rs) and [Tauri apps](https://tauri.app/) 
  
## License

This project is licensed under the [MIT License](./LICENSE) - see the LICENSE.md file for details

## Acknowledgments

- <https://github.com/rhasspy/piper>
- <https://github.com/thewh1teagle/piper-rs>
- <https://github.com/microsoft/onnxruntime>
