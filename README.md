# wolves-rust-defs
Rust definitions for use across projects.

## Background

The internal `rustwebfileserver` project and the client's `rustloader` utilise the same structs - the server encodes in JSON, but the client deserialises them. 

This create gives both access to the same definitions; reducing duplication.
