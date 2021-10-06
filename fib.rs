#[allow(dead_code)]
fn fib(n: usize) -> usize {
    if n < 2 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

#[derive(Debug)]
enum Action<T, U> {
    Call(T),
    Handle(U),
}

fn fib_nonrec(n: usize) -> usize {
    let mut arg_stack = Vec::<Action<usize, ()>>::new();
    let mut ret_stack = Vec::<usize>::new();

    use Action::*;
    arg_stack.push(Call(n));
    while let Some(action) = arg_stack.pop() {
        println!("action = {:?}", &action);
        match action {
            Call(n) => if n < 2 {
                ret_stack.push(n)
            } else {
                arg_stack.push(Handle(()));
                arg_stack.push(Call(n - 2));
                arg_stack.push(Call(n - 1));
            },

            Handle(()) => {
                let a = ret_stack.pop().unwrap();
                let b = ret_stack.pop().unwrap();
                ret_stack.push(a + b);
            },
        }
        println!("arg_stack = {:?}", &arg_stack);
        println!("ret_stack = {:?}", &ret_stack);
        println!("------------------------------")
    }

    ret_stack.pop().unwrap()
}

fn main() {
    fib_nonrec(3);
}
