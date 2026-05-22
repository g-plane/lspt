# lspt

Types of [Language Server Protocol](https://microsoft.github.io/language-server-protocol/).

## Proposed Types

To use proposed types, enable the `proposed` feature. This is disabled by default.

## Method Macros

Enable the `macros` feature to map LSP method names to request and notification types:

```rust,ignore
type Request = lspt::lsp_request!("textDocument/definition");
type Notification = lspt::lsp_notification!("textDocument/didOpen");
```

## URI Implementation

By default, this library uses [`String`] as URI.
If this doesn't satisfy your requirements,
enable the `url` feature and it will switch to `url::Url`.

## Hashmap Implementation

By default, this library uses [`rustc_hash::FxHashMap`] as hashmap implementation, but can be disabled.

You can also enable the `indexmap` feature for preserve the order in map.
