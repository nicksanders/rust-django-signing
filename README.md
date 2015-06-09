django-signing
==============

Utilities for decoding django signed cookies in rust

[![Build Status](https://travis-ci.org/nicksanders/rust-django-signing.svg?branch=master)](https://travis-ci.org/nicksanders/rust-django-signing)

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies.django-signing]
git = "https://github.com/nicksanders/rust-django-signing.git"
```

And this in your crate root:

```rust
extern crate django_signing;
```
