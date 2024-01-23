#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Node {
        Node {
            value,
            left: None,
            right: None,
        }
    }
    fn insert(&mut self, val: i32) {
        if val < self.value {
            match self.left {
                Some(ref mut node) => node.insert(val),
                None => self.left = Some(Box::new(Node::new(val))),
            }
        } else {
            match self.right {
                Some(ref mut node) => node.insert(val),
                None => self.right = Some(Box::new(Node::new(val))),
            }
        }
    }
}

fn main() {
    let mut node = Node::new(0);

    node.insert(-1);
    node.insert(1);
    node.insert(10);
    node.insert(2);

    println!("{node:?}");
}
