use std::fmt::{Display, Debug};

type NodeRef<T> = Option<Box<Node<T>>>;

#[derive(Debug, Default, Clone)]
struct Node<T> {
    value: T,
    left: NodeRef<T>,
    right: NodeRef<T>,
}

#[derive(Debug)]
enum Action<T, U> {
    Call(T),
    Handle(U)
}

fn generate_tree(level: usize) -> NodeRef<i32> {
    let mut counter = 1;
    let mut arg_stack = Vec::<Action<usize, i32>>::new();
    let mut ret_stack = Vec::<NodeRef<i32>>::new();

    use Action::*;
    arg_stack.push(Call(level));
    while let Some(action) = arg_stack.pop() {
        match action {
            Call(level) => if level > 0 {
                arg_stack.push(Handle(counter));
                counter += 1;
                arg_stack.push(Call(level - 1));
                arg_stack.push(Call(level - 1));
            } else {
                ret_stack.push(None);
            },
            Handle(value) => {
                let left = ret_stack.pop().unwrap();
                let right = ret_stack.pop().unwrap();
                ret_stack.push(Some(Box::new(Node{value, left, right})));
            },
        }
    }

    ret_stack.pop().unwrap()
}

fn print_tree<T: Display>(root: &NodeRef<T>) {
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

fn invert_tree<T: Clone + Debug>(root: &NodeRef<T>) -> NodeRef<T> {
    let mut arg_stack = Vec::<Action<&NodeRef<T>, &T>>::new();
    let mut ret_stack = Vec::<NodeRef<T>>::new();

    use Action::*;
    arg_stack.push(Call(root));
    while let Some(action) = arg_stack.pop() {
        match action {
            Call(root) => if let Some(node) = root {
                arg_stack.push(Handle(&node.value));
                arg_stack.push(Call(&node.right));
                arg_stack.push(Call(&node.left));
            } else {
                ret_stack.push(None)
            },
            Handle(value) => {
                let left = ret_stack.pop().unwrap();
                let right = ret_stack.pop().unwrap();
                ret_stack.push(Some(Box::new(Node{value: value.clone(), left, right})));
            },
        }
    }

    ret_stack.pop().unwrap()
}

fn main() {
    let tree = generate_tree(3);
    print_tree(&tree);
    println!("------------------------------");
    print_tree(&invert_tree(&tree));
}
