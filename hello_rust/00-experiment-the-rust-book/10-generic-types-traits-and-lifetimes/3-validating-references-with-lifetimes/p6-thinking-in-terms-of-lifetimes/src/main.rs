// Thinking in Terms of Lifetimes
// ==================================================
//
// For example in which you need to specify lifetime parameters depends on what your function is
// doing. For example, if we changed the implmentation of the longest function to always return the
// first parameter rather than the longest string slice, we would'nt need to specify a lifetime on
// the y parameter. The following code will compile:

fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// We've specified a lifetime parameter 'a for the parameter x and the return type, but not for the
// parameter y, because the lifetime of y does not have any relationship with the lifetime of x or
// the return value.
//
// When returning a reference from a function, the lifetime parameter for the return type needs to
// match the lifetime parameter for one of the parameters. If the reference returned does not refer
// to one of the parameters, it must refer to a value created within this function. However, this
// would be a dangling reference because the value will go out of scope at the end of the function.
// Consider this attempted implementation of the longest function that won't compile:

fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}

// here, even though we've specified a lifetime parameter 'a for the return type, this
// implementation will fail to compile becaue the return value lifetime is not related to the
// lifetime parameters at all. Here is the error message we get:

/*
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0515]: cannot return reference to local variable `result`
  --> src/main.rs:11:5
   |
11 |     result.as_str()
   |     ^^^^^^^^^^^^^^^ returns a reference to data owned by the current function

For more information about this error, try `rustc --explain E0515`.
error: could not compile `chapter10` due to previous error
*/

// The problem is that result goes out of scope and gets cleaned up at the end of the longest
// function. We're also trying to return a reference to result from the fucntion. There is no way
// we can specify lifetime parameters that would change the dangling reference, and Rust won't let
// us create a dangling reference. In this case, the best fix would be to return an owned data type
// rather than a reference so the calling function is then responsible fore cleaing up the value.
//
// Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return
// values of functions. Once they're connected, Rust has enough information to allow memory-safe
// operations and disallow operations that would create dangling pointers or otherwise violate
// memory safety.
