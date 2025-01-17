# unicase string-cache

A fork of [Servo](https://github.com/servo/servo)'s string cache library. Each string compare in the library uses ascii-unicase comparison.

## Simple usage

In `Cargo.toml`:

```toml
[dependencies.unicase_string_cache]
git = "https://github.com/jwillbold/string-cache.git"
```

In `lib.rs`:

```rust
extern crate unicase_string_cache;
use string_cache::DefaultAtom as Atom;
```

## With static atoms

In `Cargo.toml`:

```toml
[package]
build = "build.rs"

[dependencies]
string_cache = "0.8"

[build-dependencies]
string_cache_codegen = "0.5"
```

In `build.rs`:

```rust
extern crate string_cache_codegen;

use std::env;
use std::path::Path;

fn main() {
    string_cache_codegen::AtomType::new("foo::FooAtom", "foo_atom!")
        .atoms(&["foo", "bar"])
        .write_to_file(&Path::new(&env::var("OUT_DIR").unwrap()).join("foo_atom.rs"))
        .unwrap()
}
```

In `lib.rs`:

```rust
extern crate string_cache;

mod foo {
    include!(concat!(env!("OUT_DIR"), "/foo_atom.rs"));
}
```

The generated code will define a `FooAtom` type and a `foo_atom!` macro.
The macro can be used in expression or patterns, with strings listed in `build.rs`.
For example:

```rust
fn compute_something(input: &foo::FooAtom) -> u32 {
    match *input {
        foo_atom!("foo") => 1,
        foo_atom!("bar") => 2,
        _ => 3,
    }
}
```
