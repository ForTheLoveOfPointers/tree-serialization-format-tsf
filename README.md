# TSF — Tree Serialization Format

TSF is a compact, deterministic, line-based textual format for serializing ordered trees. Unlike markup languages or presentation formats, its sole purpose is to represent hierarchical structures for transmission, storage, and reconstruction.

## Design Goals

- **O(n)** single-pass parsing, streaming-friendly
- No closing tags, no backtracking, no syntactic ambiguity
- Exactly one canonical textual representation per tree (deterministic hashing, reproducible diffs, digital signatures)
- Compact and human-readable

## Example

```
0 html
1 head
2 title "My page"
1 body
2 h1 "Hello"
2 div class="container"
```

Each line starts with a non-negative integer indicating the node's depth in the tree, followed by the node name, optional `key=value` attributes, and optional quoted string content.

## Current Status

This is an early-stage reference implementation (v0.1.0). The **AST types** and **serializer** (via `Display`) are complete. The lexer, parser, validator, and error handling are placeholder modules ready to be built.

## License

TBD
