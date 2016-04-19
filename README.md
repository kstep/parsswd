# parsswd

A native Rust passwd and group files parser.

Cargo.toml:

```toml
[dependencies]
parsswd = "0.1.0"
```

Usage:

```rust
extern crate parsswd;

use parsswd::{PwEnt, GrpEnt};
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let passwd = BufReader::new(File::open("/etc/passwd").unwrap());
    for line in passwd.lines() {
        let line = line.unwrap();
        let entry = PwEnt::from_str(&*line).unwrap();
        println!("User #{}: {} ", entry.uid, entry.name);
    }

    let group = BufReader::new(File::open("/etc/group").unwrap());
    for line in group.lines() {
        let line = line.unwrap();
        let entry = GrpEnt::from_str(&*line).unwrap();
        println!("Group #{}: {} ", entry.gid, entry.name);
    }
}
```
