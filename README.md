[![Check](https://github.com/ztroop/defang/actions/workflows/check.yml/badge.svg?branch=main)](https://github.com/ztroop/defang/actions/workflows/check.yml)

# defang

Yet another defanging tool! Defanging is a reliable way to share suspicious or malicious URLs without endangering the recipient by modifying the URL to be inaccessible.

## Usage

```
A utility to defang or refang URL strings

USAGE:
    defang [OPTIONS]

OPTIONS:
    -d, --defang <URL>    Convert a URL into a harmless and shareable format
    -h, --help            Print help information
    -r, --refang <URL>    Restore a defanged URL to its original format
    -V, --version         Print version information
```

## Demonstration

```sh
$ defang -d http://malware.example.com
hxxp://malware[.]example[.]com
```
