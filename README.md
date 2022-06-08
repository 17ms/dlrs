<div align="center">
    <span><img src="https://github.com/einisto/rusty-downloader/blob/main/doc/ferris.png" width="400"></span>
</div>

## Rust-powered image downloader for 4chan

Minimal concurrent image downloader for 4chan threads/boards.

<p align="left">
<a href="https://www.gnu.org/licenses/gpl-3.0"><img src="https://img.shields.io/badge/License-GPLv3-blue.svg"></a>
<a href="https://github.com/einisto"><img src="https://img.shields.io/badge/GitHub-Follow%20on%20GitHub-inactive.svg?logo=github"></a>
</p>

### Installation and usage

Build and run with

```shell
cargo run -- <OPTIONS>
```

```shell
USAGE:
    rusty-downloader --output <PATH> <--thread <URL>|--board <URL>>

OPTIONS:
    -b, --board <URL>      Set a board URL
    -h, --help             Print help information
    -o, --output <PATH>    Set an output directory
    -t, --thread <URL>     Set a thread URL
    -V, --version          Print version information
```

### Dependencies

![DependenciesGraph](https://github.com/einisto/rusty-downloader/blob/main/doc/structure.svg)
