// Creating Type Synonymous with Type Aliases
// ==========================================
//
// Rust provides the ability to declare a type alias to give an existing type another name. For
// this we use the type keyword. For example, we can create the alias Kilometers to i32 like so:

fn ex1() {
    type Kilometers = i32;

    let x: Kilometers = 5;
    println!("x = {} kilometers", x);
}

// Now, the alias Kilometers is a synonym for i32; unlike the Millimeters and Meters types we
// created in Listing 19-15, Kilometers is not a separate, new type. Values that have the type
// Kilometers will be treated the same as values of type i32:

fn ex2() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}

// Because Kilometers and i32 are the same type, we can add values of both types and we can pass
// Kilometers values to functions that take i32 parameters. Howwever, using this method, we don't
// get the type checking benefits that we get from the newtype pattern discussed earlier. In other
// words, if we mix up Kilometers and i32 values somewhere, the compiler will not give us an error.
//
// The main use case fot type synonymous is to reduce repetition. For example, we might have a
// lengthy type like this:

//    Box<dyn Fn() + Send + 'static>

// Writing this lengty type in function signatures and as type annotations all over the code can be
// tiresome and error prone. Imagine having a project full of code like that in Listing 19-24:

fn ex3() {
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
        // --snip--
    }

    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        // --snip--
    }
}
// Listing 19-24: Using a long type in many places

// A type alias makes this code more manageable by reducing the repetition. In Listing 19-25, we've
// indtroduced an alias named T-hunk for the verb ose type and can replace all uses of the type
// with the shorter alias Thunk.

fn ex4() {
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        // --snip--
    }
}
// Listing 19-25: Introducing a type alias Thunk to reduce repitition

// This code is much easier to read and write! Choosing a meaningful name for a type alias can help
// communicate your intent as well (thunk is a word for code to be evaluated at a later time, so
// it's an appropriate name for a closure that gets stored).
//
// Type aliases are also commonly used with the Result<T, E> type for reducing repetition. Consider
// the std::io module in the standard library. I/O operations often return a Result<T, E> to handle
// situations when operations fail to work. This library has a std::io::Error struct that
// represents all possible I/O errors. Many of the functions in std::io returning Result<T, E>
// where the E is std::io::Error, such as these functions in the Write trait:

fn ex5() {
    use std::fmt;
    use std::io::Error;

    pub trait Write {
        fn Write(&mut self, buf: &[u8]) -> Result<usize, Error>;
        fn flush(&mut self) -> Result<(), Error>;

        fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
    }
}

// The Result<..., Error> is repeated a lot. As such, std::io has this type alias declaration:

// type Result<T> = std::result::Result<T, std::io::Error>;

// Because this declaration is in the std::io module, we can use the fully qualified alias
// std::io::Result<T>; that is, a Result<T, E> with the E filled in as std::io::Error. The Write
// trait function signatures end up looking like this:

fn ex6() {
    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
        fn flush(&mut self) -> Result<()>;

        fn write_all(&mut self, buf: &[u8]) -> Result<()>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
    }
}
// The type alias helps in two ways: it makes code easier to write and it gives us a consistent
// interface accross all of std::io. Because it's an alias, it's just another Resulst<T, E>, which
// means we can use any methods that work on Result<T, E> with it, as well as special syntax like
// the ? operator.

fn main() {
    ex1();
    ex2();
    ex3();
    ex4();
}
