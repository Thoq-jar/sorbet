# Sorbet

A simple KVP (Key Value Pair) file format.

Example Usage:
```sorbet
key => value
key => sorbet
     > also
     > supports
     > multiline
```

```rust
use sorbet_kvp::sorbet;

fn main() {
    let contents = "key => value";
    let hash_map = sorbet::parse(contents);

    println!("{:?}", hash_map);
}
```

## License
This project uses the MIT license, check the [LICENSE](LICENSE.md) for more.
