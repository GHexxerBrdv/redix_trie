use std::collections::HashMap;

#[derive(Debug)]
struct Node<V> {
    children: HashMap<String, Box<Node<V>>>,
    value: Option<V>,
}

impl<V> Node<V> {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            value: None,
        }
    }
}

#[derive(Debug)]
struct Trie<V> {
    root: Node<V>,
}

impl<V> Trie<V> {
    fn new() -> Self {
        Self { root: Node::new() }
    }

    fn insert(&mut self, key: String, value: V) {
        Self::insert_recursive(&mut self.root, &key, value);
    }

    fn insert_recursive(node: &mut Node<V>, key: &str, value: V) {
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

    fn get(&self, key: &str) -> Option<&V> {
        Self::get_recursive(&self.root, key)
    }

    fn get_recursive<'a>(node: &'a Node<V>, key: &str) -> Option<&'a V> {
        if key.is_empty() {
            return node.value.as_ref();
        }

        for (edge, child) in &node.children {
            if key.starts_with(edge) {
                let remaining = &key[edge.len()..];
                return Self::get_recursive(child, remaining);
            }
        }

        None
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

#[test]
fn insert_test() {
    let mut trie = Trie::<String>::new();

    trie.insert("cat".to_string(), "1".to_string());
    trie.insert("bull".to_string(), "2".to_string());
    trie.insert("car".to_string(), "14".to_string());

    assert_eq!(trie.get("cat"), Some(&"1".to_string()));
    assert_eq!(trie.get("car"), Some(&"14".to_string()));
    assert_eq!(trie.get("bull"), Some(&"2".to_string()));

    let mut trie = Trie::<i32>::new();

    trie.insert("cat".to_string(), 1);
    trie.insert("bull".to_string(), 2);
    trie.insert("car".to_string(), 14);

    assert_eq!(trie.get("cat"), Some(&1));
    assert_eq!(trie.get("car"), Some(&14));
    assert_eq!(trie.get("bull"), Some(&2));
}
