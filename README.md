# folder2dmg ![CI](https://github.com/deild/folder2dmg/workflows/CI/badge.svg)

Usage:

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
