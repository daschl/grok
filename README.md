grok
====
The `grok` library allows you to quickly parse and match potentially unstructured data into a structed result. It is especially helpful when parsing logfiles of all kinds. This [Rust](http://rust-lang.org) version is mainly a port from the [java version](https://github.com/thekrakken/java-grok) which in drew inspiration from the original [ruby version](https://github.com/logstash-plugins/logstash-filter-grok).

## Usage
Add this to your `Cargo.toml`:

```toml
[dependencies]
grok = "0.1"
```

and this to your crate root:

```rust
extern crate grok;
```

Here is a simple example which stores a pattern, compiles it and then matches a line on it:

```rust
extern crate grok;

use grok::Grok;

fn main() {
    // Instantiate Grok
    let mut grok = Grok::new();

    // Insert a definition which might be a regex or an alias
    grok.insert_definition("USERNAME", r"[a-zA-Z0-9._-]+");

    // Compile the definitions into the pattern you want
    let pattern = grok.compile("%{USERNAME}", false);

    //  Match the compiled pattern against a string
    match pattern.match_against("root") {
        Some(m) => println!("Found username {:?}", m.get("USERNAME")),
        None => println!("No matches found!"),
    }
}
```

## License
`grok` is distributed under the terms of the Apache License (Version 2.0). 
See LICENSE for details.