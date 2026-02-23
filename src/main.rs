use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    children: HashMap<String, Box<Node>>,
    value: Option<String>,
}

impl Node {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            value: None,
        }
    }
}

#[derive(Debug)]
struct Trie {
    root: Node,
}

impl Trie {
    fn new() -> Self {
        Self { root: Node::new() }
    }

    fn insert(&mut self, key: String, value: String) {
        Self::insert_recursive(&mut self.root, &key, value);
    }

    fn insert_recursive(node: &mut Node, key: &str, value: String) {
        if key.is_empty() {
            node.value = Some(value);
            return;
        }
        let keys: Vec<String> = node.children.keys().cloned().collect();

        for edge in keys {
            let common = lcp(&edge, key);

            if common.is_empty() {
                continue;
            }
        }
    }
}

fn main() {
    println!("Hello from redix trie");
}

fn lcp(a: &str, b: &str) -> String {
    let mut result = String::new();

    for (c1, c2) in a.chars().zip(b.chars()) {
        if c1 == c2 {
            result.push(c1);
        } else {
            break;
        }
    }
    result
}
