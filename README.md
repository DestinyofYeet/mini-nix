# mini-nix

Attempt to do a little nix interpreting

# How to use
This is currently under development, but you can play with it a bit.

Either with an input file
```bash
cargo run -- -f <input-file>
```

Or repl-like
```bash
cargo run --
```

Currently it just builds an AST and nothing else. You currently need to add `-vv` to enable tracing output and see something.

# Performance-Optimization-Bingo
- [ ] Use References in `Token` instead of a full-blown String
