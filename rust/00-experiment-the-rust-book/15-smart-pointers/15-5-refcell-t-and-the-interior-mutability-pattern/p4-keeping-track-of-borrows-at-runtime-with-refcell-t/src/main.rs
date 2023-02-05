// Keeping Track of Borrows at Runtime with RefCell<t>
// ===================================================
//
// When creating immutable and mutable references, we use the & and &mut syntax, respectively. With
// RefCell<T>, we ust eh borrow and borrow_mut methods instead, which are part of the safe API that
// belongs to RefCell<T>. The borrow method returns the smart pointer type Ref<T>, and borrowmut
// returns the smart pointer type RefMut<T>. Both types implements Deref, so we can treat them like
// regular references.
//
// The RefCell<T> keeps track of how many Ref<T> and RefMut<T> smart pointers are currently active.
// Every time we call borrow, the RefCell<t> increases its count of how many immutable borrows are
// active. When a Ref<T> value goes out of scope, the cound of immutable borrows goes down by one.
// Just like the compile-time borrowing rules, RefCell<t> lets us have many immutable borrows of
// one mutable borrow at any point in time.
//
// If we try to violate these rules, rather than getting a compiler error as we sould with
// references, the implementation of RefCell<t> will panic at runtime. Listing 15-23 shows a
// modification of the implementation of send in Listing 15-22. We're deliberately trying to create
// two mutable borrows active for the same scope to illustrate that RefCell<t> prevents us from
// doing this at runtime.

// Filename: src/lib.rs (but in this main.rs)

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        let mut one_borrow = self.sent_messages.borrow_mut();
        let mut two_borrow = self.sent_messages.borrow_mut();

        one_borrow.push(String::from(message));
        two_borrow.push(String::from(message));
    }
}

// Listing 15-23: Creating two mutable references in the same scope to see that RefCell<t> will
// panic

// We create a variable one_borrow for the RefMut<T> smart pointer returned from borrow_mut. Then
// we create another mutable borrow in the same way in the variable two_borrow. This makes two
// mutable references in the same scope, which isn't allowed. When we run the tests for our
// library, the code in Listing 15-23 will compile without any errors, but the test will fail:


/*
$ cargo test
   Compiling limit-tracker v0.1.0 (file:///projects/limit-tracker)
    Finished test [unoptimized + debuginfo] target(s) in 0.91s
     Running unittests src/lib.rs (target/debug/deps/limit_tracker-e599811fa246dbde)

running 1 test
test tests::it_sends_an_over_75_percent_warning_message ... FAILED

failures:

---- tests::it_sends_an_over_75_percent_warning_message stdout ----
thread 'main' panicked at 'already borrowed: BorrowMutError', src/lib.rs:60:53
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::it_sends_an_over_75_percent_warning_message

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
*/

// Notice that the code panicked with the message already borrowed: BorrowMutError. This is how
// RefCell<t> handles violations of the borrowing rules at runtime.
//
// Choosing to catch borrowing errors at runtime rather than compile time, as we've done here,
// means you'd potentially be finding mistakes in your code later in the development process:
// possible not until your code was deployed to production. Also, your code would incur a small
// runtime performance penalty as a result of keeping trakof the borrows at runtime rather than
// compile time. However, using RefCell<T> makes it possible to write a mock object that can modify
// itself to keep trak of the messages it has seen while you're using it in a context where only
// immutable values are allowed. You can use RefCell<t> despite its trade-offs to get more
// funtionality than regular referneces provide.
//
