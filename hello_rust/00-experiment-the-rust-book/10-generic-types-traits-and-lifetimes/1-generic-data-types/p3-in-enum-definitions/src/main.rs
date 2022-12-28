// In Enum Definitions
// ===================
//
// As we did with structs, we can define enums to hold generic data types in their variants. Let's
// take another look at the Option<T> enum that the standard library provides, which we used in
// Chapter 6:

fn main() {
}

fn ex1() {
    enum Option<T> {
        Some(T),
        None,
    }
}
// This definition should now make more sense to you. As you can see, the Option<T>  enum is
// generic over type T and has two variants: Some, which holds the value of type T, and a None
// variant that doent't hold any value. By using the Option<T> enum, we can express the abstract
// concept of an optional value, and because Option<T> is generic, we can use this abstraction no
// matter what the type of the optional value is.
//
// Enums can use multiple generic types as well. The definition of the Result enum that we used in
// Chapter 9 is one example:

fn ex2() {
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}
// The Result enum is generic over two types, T and E, and has two variants: Ok, which holds a
// value of type T, and Err, which holds a value of type E. This definition makes it convenient to
// use the Result enum anywhere we have an operation that might succeed (return a value of some
// type T) or fail (return an error of some type E). In fact, this is what we used to open a file
// in Listing 9-3, where T was filled with the type std::fs::File when the file was opened
// successfully and E was filled in with the type std::io::Error when there were problems opening
// the file.
//
// When you recognize situations in your code with multiple struct or enum definitions that differ
// only in the types of the values they hold, you can avoid duplication by using generic types
// instead.
