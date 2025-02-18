# lspt

Types of [Language Server Protocol](https://microsoft.github.io/language-server-protocol/).

## Proposed Types

To use proposed types, enable the `proposed` feature. This is disabled by default.

## URI Implementation

By default, this library uses [`String`] as URI.
If this doesn't satisfy your requirements,
enable the `url` feature and it will switch to `url::Url`.

## Hashmap Implementation

By default, this library uses [`std::collections::HashMap`] as hashmap implementation.

You can enable the `rustc-hash` feature for faster hashing,
or enable the `indexmap` feature for preserve the order in map.
Or, enable both of them.
