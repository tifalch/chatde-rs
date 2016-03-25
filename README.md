# chatde-rs
[![Travis Build Status](https://img.shields.io/travis/TibFalch/chatde-rs.svg?style=flat-square)](https://travis-ci.org/LFalch/korome)
[![GitHub licence](https://img.shields.io/github/license/TibFalch/chatde-rs.svg?style=flat-square)](https://github.com/TibFalch/chatde-rs/blob/master/LICENCE)

Rust version of chatde

## Usage
```
chatde-rs [OPTIONS] <IP>[:<PORT>]

IP:     the IP to connect to
PORT:   the port to connect via (default is 15327)

Options:
    --color             Colours the output

    -z,--gzip           Compresses the data using gzip
                        Doesn't currently work :/

    -c,--check          Disables checksum -- to be deprecated

    -D,--debug          Enables debug output

    -p,--passphrase     Parses a passphrase to be used for encryption
```

### Commands

```
§bye, §quit
      quit chatde
```
#### To be added
```
§file <file>
    sends an encrypted file
§ls
    lists the current directory
§cd <dir>
    changes current directory
```

## Encryption Algorithm

[https://github.com/LFalch/delta-l](delta-l)

## Go Port

[https://github.com/TibFalch/gochatde](gochatde)
