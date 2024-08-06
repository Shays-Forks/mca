# mca

A simple but effective & fast parser for Minecrafts Region Files (mca).

It does use **`unsafe`** rust with a few `get_unchecked` calls.  
I've thrown some few hundred MB of region files and no problems occured.

Getting a single chunk is done in just `3.1ns` (on my machine), which is pretty blazingly fast.

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

## Benchmarks

There is one benchmark included that compares against the only other  
mca parser that i could find (`mca-parser`) and this crate is just like `1-3ns` faster.  
A very stupid, marginal error difference, but uhh this seems "faster".

you can run it with `cargo bench`.

## Unsafe part

All unsafe calls are `get_unchecked`, i haven't tested with the safe version yet.  
But the perf might just not be a difference at all or be better, who knows. gotta test it.  
but so far this crate is **unsafe**