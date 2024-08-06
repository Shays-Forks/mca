# mca

A simple but effective & fast parser for Minecrafts Region Files (mca).

## Example

```rust
use std::{fs::File, io::Read};
use mca::Region;

let mut data = Vec::new();
File::open("r.0.0.mca")?.read_to_end(&mut data)?;

// Initialize the region
// This mostly just validates the header
let region = Region::new(&data)?;

// Get a specific chunk based of it's chunk coordinates
let chunk = region.get_chunk(0, 0)?.unwrap();

// Decompress the chunk data
// This will most commonly be either ZLib or LZ4 compressed
let decompressed = chunk.decompress()?;

// You can now bring your own NBT parser to parse the actual chunk data here
// I recommend either `simdnbt` or `fastnbt` for this.
```

## Unsafe Feature

Toggling the `unsafe` feature will add unsafe `get_unchecked` calls to the code.  
And this improves the performance by about 50% - 100% (were talking 2-3ns faster).  
I *think* i have added enough manual bound checks to make this safe, but i can't guarantee it.  

I've tested this on a few hundred MBs of region files and no issues at all.  

*Do note that enabling `unsafe` changes the function signature of `Region::get_timestamp` to return a result*

## Benchmarks

There is one benchmark included that compares against the only other  
mca parser that i could find (`mca-parser`) and this crate is just like `1-3ns` faster (with `unsafe`).  
A very stupid, marginal error difference, but uhh this seems "faster".

you can run it with `cargo bench` or `cargo bench --features unsafe` for the unsafe version.

