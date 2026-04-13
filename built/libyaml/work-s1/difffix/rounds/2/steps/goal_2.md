# Goal 2: Fix node count pointer arithmetic in yaml_emitter_dump

## Function
`yaml_emitter_dump` (node count computation for anchor allocation)

## C Source
/home/leochanj/Desktop/libyaml/src/emitter.c

## Rust Source
/home/leochanj/Desktop/PHC2Rust/built/libyaml/rust-s1/src/lib.rs (line 8303-8305)

## What's Wrong
Same pointer arithmetic pattern as the round 1 bug, but compensated by a manual division:
```rust
let node_count = (*document).nodes.top as usize - (*document).nodes.start as usize;
let node_count_items = node_count / core::mem::size_of::<yaml_node_t>();
```

`nodes.top` and `nodes.start` are `*mut yaml_node_t` pointers. The subtraction gives byte offset, then divides by struct size to recover element count. This produces the correct result but is fragile — if anyone removes the compensating division or copies the pattern without it, it breaks.

The C code simply does:
```c
document->nodes.top - document->nodes.start
```
which gives element count directly via C pointer arithmetic.

## What Needs to Change
Replace lines 8303-8304 with:
```rust
let node_count_items = (*document).nodes.top.offset_from((*document).nodes.start) as usize;
```
Then update line 8305 and 8310 to use `node_count_items` directly (remove the intermediate `node_count` variable).

## Success Criteria
- `yaml_emitter_dump` correctly allocates anchors array with proper count
- roundtrip and canonical_emitter tests pass (these exercise the emitter path)
- No memory corruption or incorrect anchor allocation
- The compensating `/ size_of` division is eliminated
