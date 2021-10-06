# Non-Recursive Inverting of Binary Tree in Rust

The idea is to implement the classical [Inverting of Binary Tree](https://twitter.com/mxcl/status/608682016205344768?lang=en) but without using recursion.

## Quick Start

```console
$ rustc main.rs
$ ./main
```

## Main Idea

The Main Idea is to basically simulate the recursion by managing two stacks: one for the arguments (`arg_stack`) and one for the return values (`ret_stack`). The `arg_stack` contains a sequence of two kinds of actions:

```rust
#[derive(Debug)]
enum Action<T, U> {
    Call(T),
    Handle(U),
}
```

`Call` simulates the recursive function call with the given arguments `T`, `Handle` allows to postpone the handling of the return values on `ret_stack` to achieve a specific order of execution that is usually achieved by using recursive functions.

This forms a general purpose framework that enables us to convert any (I believe so, can't formally prove it) recursive function into a non-recursive one.

Let's take a look at a simple recursive function that calculates `n`-th Fibonacci number:

```console
fn fib(n: usize) -> usize {
    if n < 2 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
```

This is how you would implement such function in a non-recursive fashion using the aforementioned framework:

```console
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
```

Calling `fib_nonrec(3)` generates the following log:

```
action = Call(3)
arg_stack = [Handle(()), Call(1), Call(2)]
ret_stack = []
------------------------------
action = Call(2)
arg_stack = [Handle(()), Call(1), Handle(()), Call(0), Call(1)]
ret_stack = []
------------------------------
action = Call(1)
arg_stack = [Handle(()), Call(1), Handle(()), Call(0)]
ret_stack = [1]
------------------------------
action = Call(0)
arg_stack = [Handle(()), Call(1), Handle(())]
ret_stack = [1, 0]
------------------------------
action = Handle(())
arg_stack = [Handle(()), Call(1)]
ret_stack = [1]
------------------------------
action = Call(1)
arg_stack = [Handle(())]
ret_stack = [1, 1]
------------------------------
action = Handle(())
arg_stack = []
ret_stack = [2]
------------------------------
```

The top of the stacks is on the right.

Notice how the `arg_stack` grows until it exhausts `n` and shrinks back computing the final result into the `ret_stack`. This is basically the simulation of the recursive process.

The resulting code is admittedly bigger, less readable and more difficult to extend, so I do not generally recommend to develop in this style. This entire example was created as a coding exercise. Although this approach might be useful for doing very deep recursion to prevent stack overflows, since [Vec<T>](https://doc.rust-lang.org/std/vec/struct.Vec.html) can extend for as long as there is available memory and the call stack is usually limited.
