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

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
