/* Static
 * =======
 *
 * Rust has a few reserved lifetime names. One of those is 'static. You might encounter it in two
 * situations: */

// A reference with 'static lifetime
fn ex1() {
    let s: &'static str = "hello world";

// 'static as part of a trait cound:
    fn generic<T>(x: T) where T: 'static {}
}

/* Both are related but subtly different and this is a common source for confusion when learning
 * Rust. Here are some examples for each situation:
 *
 *
 * Reference lifetime
 * --------------------
 *
 * As a reference lifetime 'static indicates that the data pointed to by the reference lives for
 * the entire lifetime of the running program. It can still be coerced to a shorter lifetime. 
 *
 * There are two ways to make a variable with 'static lifetime, and both are stored in the
 * read-only memory of the binary:
 *
 *  - Make a constant with the static declaration
 *  - Make a string literal which has type: &'static str
 * 
 * See the following example for a display of each method: */

// Make a constant with 'static lifetime
static NUM: i32 = 18;

// Returns a reference to 'NUM' where its 'static lifetime is coerced to that of the input
// argument.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // Make a 'string' literal and print it:
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // When 'static_string' goes out of scope, the reference can no longer be used, but the data
        // remains in the binary.
    }

    {
        // Make an integer to use for 'coerce_static':
        let lifetime_num = 9;

        // Coerce 'NUM' to lifetime of 'lifetime_num':
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);

    // Part 2
    // i is owned and contains no references, thus it's 'static:
    let i = 5;
    print_it(i);

    // oops, &i only has the lifetime defined by the scope of 
    // main(), so it's not 'static:
    print_it(&i);
}


/* Trait bound
 * -------------
 *
 * As a trait bound, it means the type does not contain any non-static references. Eg. the receiver
 * can hold on to the type for as long as they want and it will never become invalid until they
 * drop it.
 *
 * It's important to understand this means that any owned data always passes a 'static lifetime
 * bound, but a reference to that owned data generally does not: */

use std::fmt::Debug;

fn print_it( input: impl Debug + 'static ) {
    println!( "'static value passed in is: {:?}", input );
}


