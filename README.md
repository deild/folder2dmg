# folder2dmg
[![GITHUB: folder2dmg](https://img.shields.io/badge/GitHub-deild%2Ffolder2dmg-blue?logo=github&style=for-the-badge)](https://github.com/deild/folder2dmg/)
[![Continuous integration](https://img.shields.io/github/workflow/status/deild/folder2dmg/Continuous%20integration/main?label=Continuous%20integration&logo=github-actions&style=for-the-badge)](https://github.com/deild/folder2dmg/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://raw.githubusercontent.com/deild/folder2dmg/main/LICENSE)
[![crates.io](https://img.shields.io/crates/v/folder2dmg?style=for-the-badge&logo=rust)](https://crates.io/crates/folder2dmg)

## Installation

Install the **folder2dmg** binary:

#### From source on [crates.io](https://crates.io/):

```sh
cargo install folder2dmg
```

### Install via Package Manager

#### With [Homebrew](https://brew.sh/):

```sh
brew tap deild/tap
brew install folder2dmg
```

## Usage:

```bash
folder2dmg 0.1.6-alpha.0
Samuel Marcaille
folder2dmg is a personnal helper to create image from directory

USAGE:
    folder2dmg [FLAGS] [OPTIONS] <srcfolder>

FLAGS:
    -d, --debug      turn on debugging information
    -e, --erase      delete the srcfolder after created the image
    -h, --help       Prints help information
    -q, --quiet      no output printed to stdout
    -V, --version    Prints version information

OPTIONS:
    -i, --image <image>        the image to be created. Use the folder name and local Location if not set
    -v, --volname <volname>    the newly-created filesystem will be named volname. Use the folder name if not set

ARGS:
    <srcfolder>    the source directory, copies file-by-file the contents of source into image
```
