# audify-rs

- [Description](#description)
- [Getting Started](#getting-started)
  - [Dependencies](#dependencies)
  - [Installing](#installing)
  - [Executing program](#executing-program)
- [License](#license)
- [Acknowledgments](#acknowledgments)

## Description

Convert PDF to audio files, usable in mobile, servers and desktop applications.

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

## License

This project is licensed under the [MIT License](./LICENSE) - see the LICENSE.md file for details

## Acknowledgments

<https://github.com/rhasspy/piper>
<https://github.com/thewh1teagle/piper-rs>
