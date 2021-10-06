# Non-Recursive Inverting of Binary Tree in Rust

The idea is to implement the classical [Inverting of Binary Tree](https://twitter.com/mxcl/status/608682016205344768?lang=en) but without using recursion.

## Quick Start

```console
$ rustc main.rs
$ ./main
```

## Main Idea

The Main Idea is to basically simulate recursion by managing two stacks: one for the arguments (`arg_stack`) and one for the return values (`ret_stack`).
