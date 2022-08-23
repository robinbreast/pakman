# pakman

## File compression(zip) **PA**c**K**aging **MAN**ager

File packing manager based on the configuration file where to specify all the files to be packed

- Define a package file list with a set of files in a configuration file
- Define the multiple packages with the different package names in a configuration file
- Define shortcut files (batch file for Windows) in a package
- Generate output zip file from a package definition
- All the file paths are relative file paths based on the given file path from command line

## Available config names in the configuration file (json)
- **version**: current compatiable version of pakman
- **packages**: array of package definitions
    - **name**: package name
    - **filepaths**: file path list of package name for source files
    - **shortcuts**: optional, shortcut file lists to be created
        - **name**: shorcut file name to be created
        - **target**: target file path to be linked
        - **cwd**: optional, to be used in bat file when creating the shortcut file

## Example
```
$ pakman --help
pakman 0.1.0

USAGE:
    pakman --config-filepath <CONFIG_FILEPATH> --input-dirname <INPUT_DIRNAME> --package-name <PACKAGE_NAME> --output-filepath <OUTPUT_FILEPATH>

OPTIONS:
    -c, --config-filepath <CONFIG_FILEPATH>    config json filepath
    -h, --help                                 Print help information
    -i, --input-dirname <INPUT_DIRNAME>        input root directory
    -o, --output-filepath <OUTPUT_FILEPATH>    output zip filepath
    -p, --package-name <PACKAGE_NAME>          select package
    -V, --version                              Print version information
$ pakman --config-filepath ./example/config.json --input-dirname ./example --package-name package-1 --output-filepath ./output.zip
n shortcut-1.bat => dir1/test1.bat ...
n shortcut-2.bat => dir1/test2.cfg ...
[
    "example/file1.txt",
    "example/file2.txt",
    "example/dir1/dir_file1.txt",
    "example/dir2",
    "example/dir3/",
    "example/shortcut-1.bat",
    "example/shortcut-2.bat",
]
+ "example/file1.txt" ...
warning: IO error for operation on example/file2.txt: No such file or directory (os error 2)
warning: IO error for operation on example/dir1/dir_file1.txt: No such file or directory (os error 2)
adding dir "dir2"
+ "example/dir2/dir_file1.txt" ...
+ "example/dir2/dir_file2.txt" ...
warning: IO error for operation on example/dir3/: No such file or directory (os error 2)
+ "example/shortcut-1.bat" ...
+ "example/shortcut-2.bat" ...
successfully done!
```
