// Recoverable Errors with Result
// =============================
//
// Most errors aren't serious enough to require the program to stop entirely. Sometimes,
// when a function fails, it's for a reason that you can easily interpret and respond to.
// For example, if you try to open a file and that operation fails because the file
// doesn't exist, you might want to create the file instead of terminating the process.
//
// Recall from "Handling Potential Failure with the Result Type" in Chapter 2 that
// the Result enum is defined as having to variants, Ok and Err, ass follows:

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// The T and E are generic type parameters: we'll discuss generics in more detail
// in Chapter 10. What you need to know right now is that T represents the type of the
// value that will be returned in a success case within the Ok variant, and E represents
// the type of the error that will be returned in a failure case within the Err variant.
// Because Result has these generic type parameters, we can use the Result type
// and the functions defined on it in many different situations where the successful
// value and error value we want to return may differ.
//
// Let's call a function that returns a Result value because the function could fail.
// Listing 9-3 we try to open a file.

use std::fs::File;

fn main() {
    //let greeting_file_result = File::open("hello.txt");
    ex2();
}
// 9-3: Opening a file

// The return type of File::open is a Result<T, E>. The generic parameter T has been filled
// in by the implementation of File::open with the type of the success value, std::fs::File,
// which is a file handle. The type of E used in the error value is std::io::Error.
// This return type means the call to File::open might succeed and reutrn a file handle
// that we can read from or write to. The function call also might fail: for example,
// the file might not exist, or we might not have permission to access the file. The
// File::open function needs to have a way to tell us whether it succeeded or failed
// and at the same time give us either the file handle or error information. This
// information is exactly what the Result enum conveys.
//
// In the case where File::open succeeds, the value in the variable greeting_file_result
// will be an instance of Ok that contains a file handle. In the case where it fails,
// the value in greeting_file_result will be an instance of Err that contains more
// information about the kind of error that happened.
//
// We need to add to the code in Listing 9-3 to take different actions depending on the
// value File::open returns. Listing 9-4 shows one way to handle the Result using a
// basic tool, the match expression that we discussed in Chapter 6.
fn ex2() {
    let greeting_file_result = File::open("hello2.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
// 9-4: Using a match expression to handle the Result variants that might be returned

// Note that, like the Option enum, the Result enum and its variants have been brought
// into scope by the prelude, so we don't need to specify Result:: before Ok and Err
// variants in the match arms.
//
// When the result is Ok, this code will return the inner file value out of the Ok variant,
// and we then assign that file handle value to the variable greeting_file. After the match,
// we can use the file handle for reading or writing.
//
// The other arm of the match handles the case where we get an Err value from File::open.
// In this example, we've chosen to call the panic! macro. If there's no file named hello.txt
// in our current directory and we run this code, we'1ll see the following output from
// the panic! macro:

/*
$ cargo run
   Compiling error-handling v0.1.0 (file:///projects/error-handling)
    Finished dev [unoptimized + debuginfo] target(s) in 0.73s
     Running `target/debug/error-handling`
thread 'main' panicked at 'Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:8:23
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
*/

// As usual, this output tells us exactly what has gone wrong.
