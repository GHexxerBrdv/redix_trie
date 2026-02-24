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

            if common == edge {
                let child = node.children.get_mut(&edge).unwrap();
                let remaining = &key[edge.len()..];
                Self::insert_recursive(child, remaining, value);
                return;
            }

            let old_child = node.children.remove(&edge).unwrap();
            let old_suffix = edge[common.len()..].to_string();
            let new_suffix = key[common.len()..].to_string();

            let mut new_node = Node {
                children: HashMap::new(),
                value: None,
            };

            new_node.children.insert(old_suffix, old_child);
            new_node.children.insert(
                new_suffix,
                Box::new(Node {
                    children: HashMap::new(),
                    value: Some(value),
                }),
            );

            node.children.insert(common, Box::new(new_node));
            return;
        }
        node.children.insert(
            key.to_string(),
            Box::new(Node {
                children: HashMap::new(),
                value: Some(value),
            }),
        );
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
