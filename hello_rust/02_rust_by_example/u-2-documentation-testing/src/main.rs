/* Documentation testing
 * =======================
 *
 * The primary way of documenting a Rust project is through annotating the source code. 
 * Documentation comments are written in markdown and support code blocks in them. Rust takes care
 * about correctness, so these code blocks are compiles and used as documentation tests. */

/// First line is a short summary describing function.
///
/// The next lines present detailed documentation. Code blocks start with
/// triple backquotes and have implicit 'fn main()' inside
/// and 'extern crate <cratename>'. Assume we're testing 'doccoments' crate:
///
/// ```
/// let result = doccoments::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Usually doc comments may include sections "Examples", "Panics" and "Failures".
///
/// The next function divides two numbers.
///
/// # Examples
///
/// ```
/// let result = doccoments::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// // panics on division by zero
/// doccoments::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }
    a/b
}

fn main() {
    println!("Hello, world!");
}

/* Motivation behind documentation tests
 * ----------------------------------------
 *
 * The main purpose of documentation tests is to serve as examples that exersise the functionality,
 * which is one of the most important guidelines. It allows using examples from docs as complete
 * code snippets. But using ? makes compilation fail since main returns unit. The ability to hide
 * some source lines from documentation comes to the resuce: one may write fn try_main() ->
 * Result<(), ErrorType>, hide it and unwrap it in hidden main. Sounds complicated? Here's an
 * example: */

/// Using hidden 'try_main' in doc tests.
///
/// ```
/// # // hidden lines start with '#' symbol, but they're still combileable!
/// # fn try_main() -> Result<(), String> { // line that wraps the body shown in doc
/// let res = try::try_div(10, 2)?;
/// # Ok(()) // returning from try_main
/// # }
/// # fn main() { // starting main that'll unwrap()
/// #     try_main().unwrap(); // calling try_main and unwrapping
/// #                        // so that test will panic isn case of error
/// # }
/// ```
pub fun try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Divide-by-zero"))
    } else {
        Ok(a / b)
    }
}