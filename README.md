KeCS
====

[![CI](https://github.com/forbjok/kecs/actions/workflows/ci.yml/badge.svg)](https://github.com/forbjok/kecs/actions/workflows/ci.yml)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/forbjok/kecs)

## Introduction

KeCS is a tool for generating a checksum set from the contents of a directory, which can then be used to verify those files or compared
with a different checksum set. A checksum set is stored as a JSON file containing a list of files and their corresponding checksums, as well as other necessary metadata.

Comparing checksum sets can be used as a way to determine whether two copies of the same content are identical without requiring both to be accessible from the same computer.

### Supported hashing algorithms:
* BLAKE2b-512 (blake2b512)
* BLAKE2s-256 (blake2s256)
* BLAKE3 (blake3)
* CRC32 (crc32)
* MD5 (md5)
* SHA1 (sha1)
* SHA256 (sha256, the default if not specified)
* SHA3-256 (sha3_256)

## Compiling
1. Install Rust using the instructions [here](https://www.rust-lang.org/tools/install) or your distro's package manager.
2. Clone this repository and execute the following command in it:
```
$ cargo build --release
```

Voila! You should now have a usable executable in the `target/release` subdirectory.

## Generating a checksum set

Example: Generating a checksum set using the blake3 hashing algorithm:
```
$ kecs generate <path/to/content> -t blake3
```

## Verifying using a checksum set

```
$ kecs verify <path/to/file.checksums.json>
```

## Comparing two checksum sets

```
$ kecs diff <path/to/file1.checksums.json> <path/to/file2.checksums.json>
```
