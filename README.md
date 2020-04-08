# rmv (regex mv)

`rmv` is a Linux command line tool that allows you to rename (mv) or copy (cp) files using regex (or standard replacement).

## Command line

```
Usage:
  rmv [OPTIONS] [ROOTDIR]

rmv: regex mv, use regular expressions to rename / move files

Positional arguments:
  rootdir               root directory to handle (default ./)
  targetdir             target directory to move / copy the files to (default
                        ./)

Optional arguments:
  -h,--help             Show this help message and exit
  -v,--verbose          verbose mode
  -p,--pretend          pretend, redirect to generate a script
  -R,--recurse          recurse subdirectories
  -c,--copy             copy instead of renaming / moving
  -s,--src SRC          source regex or characters (more on
                        https://docs.rs/regex/#syntax)
  -d,--dst DST          replacement regex or character(s) (more on
                        https://docs.rs/regex/#syntax)
  -r,--replace          replace mode: src contains the characters to replace,
                        dst their replacement, either one single character
                        (replaces all chars from src) or as many characters as
                        src (each one replacing its counterpart in src)
  -a,--nosquare         remove square brackets and their contents (shortcut to
                        --src "(.*)\[.*\](.*)" --dst "\$1\$2")
  -e,--noparen          remove parenthesis and their contents (shortcut to
                        --src "(.*)\(.*\)(.*)" --dst "\$1$2"))
  -u,--unixify          "unixify" the result (shortcut to --replace --src "' +"
                        --dst "_")
```

More information about regex on [https://docs.rs/regex/#syntax](https://docs.rs/regex/#syntax)

## Examples

To change filenames like "31122020filename.ext" to "filename-2020-12-31.ext":

```
rmv --pretend --src "(\d{2})(\d{2})(\d{4})(.*)(\.ext)" --dst "\$4-\$3-\$2-\$1\$5"
```

To replace all digits by an underscore (\_):

```
rmv --pretend --replace --src "0123456789" --dst "_"
```

To replace a by A, b by B, and C by c:

```
rmv --pretend --replace --src "abC" --dst "ABc"
```

## Author

Fred Thomas (https://linkedin.com/in/ctoasaservice)

## Licence

`rmv` is available under the MIT license. See the LICENSE file for more info.
