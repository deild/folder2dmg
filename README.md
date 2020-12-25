# foldertodmg

Usage:

```bash
foldertodmg 0.1.2
Samuel Marcaille
foldertodmg is a personnal helper to create image from directory

USAGE:
    foldertodmg [FLAGS] [OPTIONS] <srcfolder>

FLAGS:
    -d, --debug      turn on debugging information
    -e, --erase      delete the srcfolder after created the image
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --image <image>        the image to be created. Use the folder name and local Location if not set
    -v, --volname <volname>    the newly-created filesystem will be named volname. Use the folder name if not set

ARGS:
    <srcfolder>    the source directory, copies file-by-file the contents of source into
```