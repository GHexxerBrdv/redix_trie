# Overview
This project implements a minimal compressed Radix Trie (Patricia Trie) in Rust.

A Redix trie is prefix tree where:
- keys are stored as path
- common prefixes are shared 
- chains of single child nodes are compressed into a single edge

Unlike regular trie, redix trie stores entire string segment as edge, making it space efficient.

# Features
- Insert key-value pair
- Lookup values by key
- Path compression
- Automatic edge splitting on partial matches
- O(K) time complexity where K is the length of key.

## Core idea

Instead of storing:

```
b -> r -> d -> v
```
we store:

```
"brdv"
```

if we later insert "brdp", it becomes:

```
      -> v
"brd"           (node splitting)
      -> p  
```

# Data Structure

Node is consist of:
- children: Hash map that maps string to node
- value: stores a value if node represents a complete key

```rust
struct Node {
    children: HashMap<String, Box<Node>>,
    value: Option<String>,
}
```
