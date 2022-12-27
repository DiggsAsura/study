// In Struct Definitions
// =====================
//
// We can also define structs to use generic type parameter in one or more fields using the <>
// syntax. Listing 10-6 defines a Point<T> struct to hold x and y coorinate values of any type.

fn main() {
    listing_10_6();
}

fn listing_10_6() {
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("integer.x = {}, integer.y = {}", integer.x, integer.y);
    println!("float.x = {}, float.y = {}", float.x, float.y);
}
// 10-6: A Point<T> struct that holds x and y values of type T

// The syntax for using generics in struct definitions is similar to that used in function
// definitions. First, we declare the name of the type parameter inside angle brackets just
// after the name of the struct. Then we use the generic type in the struct definition where
// we would otherwise specify concrete data types.
//
// Note that because we've used only one generic type to define Point<T>, this definition says
// that the Point<T> struct is generic over some type T, and the fields x and y are both the same
// type, whatever that type may be. If we create an instance of a Point<T> that has values of
// different types, as in Listing 10-7, our code won't compile.
//
fn listing_10_7() {
    struct Point<T> {
        x: T,
        y: T,
    }

    //let wont_work = Point { x: 5, y: 4.0 };
}
// 10-7: The fields x and y must be the same type because both have the same generic data type T.

// In this example, when we assign the integer value 5 to x, we let the compiler know that the
// generic type T will be an integer for this instance of Point<T>. Then when we specify 4.0 for y,
// which we've defined to have the same type as x, we'll get a type mismatch error like this:

/*
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0308]: mismatched types
 --> src/main.rs:7:38
  |
7 |     let wont_work = Point { x: 5, y: 4.0 };
  |                                      ^^^ expected integer, found floating-point number

For more information about this error, try `rustc --explain E0308`.
error: could not compile `chapter10` due to previous error
*/

// To define a Point struct where x and y are both generics but could have different types, we can
// use multiple generic type parameters. For example, in Listing 10-8, we change the definition of
// Point to be generic over types T and U where x is of type T and y is of type U.

fn listing_10_8() {
    struct Point<T, U> {
        x: T,
        y: U,
    }

    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
// 10-8: A Point<T, U> generic over two types so that x and y can be values of different types.

// Now all the instances of Point shown are allowed! You can use as many generic type parameters in
// a definition as you want, but using more than a few makes your code hard to read. If you're
// finding you need lots of generic types in your code, it could indicate that your code needs
// restructuring into smaller pieces.
