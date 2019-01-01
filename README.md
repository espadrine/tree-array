# Insert Benchmark

Insertion, to me, means placing an item in a sequence at any given index,
incrementing all subsequent elements’ index to maintain a bijection from index
to entry.

(Which, sadly, is inconsistently named in Rust’s HashMap and BTreeMap API,
presumably under the influence of C++. `set` would have been a much clearer
name.)

## Set up

Use the nightly channel with `rustup default nightly`.

Run the benchmark with `cargo bench`.

## Development

We recommend installing inotify-tools on Linux and running this to have the
tests run on every file write:

```bash
while inotifywait -qe close_write --exclude '^\.git$' $(find . -type d); do cargo test; done
```
