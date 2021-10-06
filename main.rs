use std::fmt::Display;

type NodeRef<T> = Option<Box<Node<T>>>;

#[derive(Debug, Default, Clone)]
struct Node<T> {
    value: T,
    left: NodeRef<T>,
    right: NodeRef<T>,
}

fn generate_tree(level: usize, counter: &mut i32) -> NodeRef<i32> {
    if level == 0 {
        None
    } else {
        let mut node = Node {
            value: *counter,
            left: None,
            right: None,
        };
        *counter += 1;
        node.left = generate_tree(level - 1, counter);
        node.right = generate_tree(level - 1, counter);
        Some(Box::new(node))
    }
}

enum Action<T, U> {
    Call(T),
    Handle(U)
}

fn print_tree_nonrec<T: Display>(root: &NodeRef<T>) {
    let mut stack = Vec::<Action<(&NodeRef<T>, usize), (&T, usize)>>::new();
    use Action::*;
    stack.push(Call((root, 0)));
    while let Some(action) = stack.pop() {
        match action {
            Call((root, level)) => if let Some(node) = root {
                stack.push(Call((&node.left, level + 1)));
                stack.push(Handle((&node.value, level)));
                stack.push(Call((&node.right, level + 1)));
            },
            Handle((value, level)) => {
                for _ in 0..level {
                    print!("  ")
                }
                println!("{}", value);
            }
        }
    }
}

fn print_tree<T: Display>(root: &NodeRef<T>, level: usize) {
    if let Some(node) = root {
        print_tree(&node.right, level + 1);
        for _ in 0..level {
            print!("  ")
        }
        println!("{}", node.value);
        print_tree(&node.left, level + 1)
    }
}

#[allow(dead_code)]
fn invert_tree<T: Clone>(root: NodeRef<T>) -> NodeRef<T> {
    match root {
        Some(node) => {
            Some(Box::new(Node {
                value: node.value.clone(),
                left: invert_tree(node.right),
                right: invert_tree(node.left),
            }))
        }
        None => None
    }
}

fn main() {
    let mut counter = 1;
    let tree = generate_tree(3, &mut counter);
    print_tree(&tree, 0);
    println!("------------------------------");
    print_tree_nonrec(&tree);
}
