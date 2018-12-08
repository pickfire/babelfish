Babelfish
=========

The universal translater for encodings.

## Install

Installing with different features provides different binaries. By default all
features are selected.

```sh
$ cargo install babelfish
```

To install babelfish with only support for `cbor` and `json` which provides
`json2cbor` and `cbor2json`.

```sh
$ cargo install babelfish --features="cbor json"  # cbor2json, json2cbor
```

## Usage

Currently, conversion can only be done with input output.

```sh
$ cbor2json < file.cbor
$ cat file.json | jq ... | json2cbor > file.cbor
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
