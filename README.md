# `rustr`: stringology in Rust

`rustr` contains several implementations for stringology.

## Implementations
### String Tree Family

- [Sufffix Trie](./src/suffix_trie.rs)
- [Suffix Tree](./src/suffix_tree)
- [Suffix Array](./src/sa.rs)
- [Longest Common Prefix Array](./src/lcp.rs)

### String Classes

- [Fibonacci String](./src/fib.rs)
- [Lyndon Word](./src/lyndon.rs)
- [Palindrome](./src/palindrome.rs)

### Others

- [Minimal Unique Substrings](./src/mus.rs)
- [Debruijn Graph](./src/debruijn.rs)

## Build Docs

```bash
$ cargo make doc-katex
```