// Returning Closures
// ==================
//
// Closures are represented by traits, which means you can't return closures directly. In most
// cases where you might want to return a trait, you can instead use the concrete type that
// implements the trait as the return value of the function. However, you can't do that with
// closures becuase they don't have a concrete type that is returnable; you're not allowed to use
// the function pointer fn as a return type, for example.
//
// The following code tries to return a closure directly, but it won't compile:

/*
fn returns_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}
*/


// The compiler error is a as follows:

/*
$ cargo build
   Compiling functions-example v0.1.0 (file:///projects/functions-example)
error[E0746]: return type cannot have an unboxed trait object
 --> src/lib.rs:1:25
  |
1 | fn returns_closure() -> dyn Fn(i32) -> i32 {
  |                         ^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
  |
  = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
help: use `impl Fn(i32) -> i32` as the return type, as all return paths are of type `[closure@src/lib.rs:2:5: 2:14]`, which implements `Fn(i32) -> i32`
  |
1 | fn returns_closure() -> impl Fn(i32) -> i32 {
  |                         ~~~~~~~~~~~~~~~~~~~

For more information about this error, try `rustc --explain E0746`.
error: could not compile `functions-example` due to previous error
*/

// The error references that Sized trait again! Rust doesn't know how much space it will need to
// store the closure. We saw a solution to this problem earlier. We can use a trait object:

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let closure = returns_closure();
    println!("closure(1) = {}", closure(1));
}

// This code will compile just fine. For more about trait objects, refer to the section "Using
// Trait Objects That Allow for Values of Different Types" in Chapter 17.
//
// Next, let's look at macros!
