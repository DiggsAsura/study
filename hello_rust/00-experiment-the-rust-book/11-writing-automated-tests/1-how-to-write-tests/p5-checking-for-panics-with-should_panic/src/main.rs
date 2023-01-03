// Checking for Panics with should_panic
// ======================================
//
// In addition to checking return values, it's important to check that our code handles error
// conditions as we expect. For example, consider the Guess type that we created in Chapter 9,
// Listing 9-13. Other code that uses Guess depends on the guarntee that Guess instance will
// contain only values between 1 and 100. We can write a test that ensures that attempting to
// create a Guess insteance with a value outside that range panics.
//
// We do this by adding the attribute should_panic to our test function. The test passes if the
// code inside the function panics; the test fails if the code inside the function doesn't panic.
//
// Listing 11-8 shows a test that checks that the error conditions of Guess::new happens when we
// expect them to.

pub struct Guess {
    value: i32,
}
/*
impl Guess {
    pub fn new(value: i32) -> Guess {
        //if value < 1 || value > 100 {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, goet {}.", value);
        }

        Guess { value }
    }
}
*/

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.", value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.", value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")] // note this extre message
    fn greater_than_100() {
        Guess::new(200);
    }
}
// 11-8: Testing that a condition will cause a panic!
// 11-9: Testing for a panic! with a panic message containting a specified substring

// We place the #[should_panic] attribute after the #[test] attribute and before the test function
// it applies to. Let's look at the result when this test passes:

/*
$ cargo test
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished test [unoptimized + debuginfo] target(s) in 0.58s
     Running unittests src/lib.rs (target/debug/deps/guessing_game-57d70c3acb738f4d)

running 1 test
test tests::greater_than_100 - should panic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests guessing_game

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

*/

// Looks good! Now let's introduce a but in our code by removeing the condition that the new
// function will pani if the value is greater than 100:

// changed the original impl block code above

// When we run the test in Listing 11-8, it will fail.

// We don't get a very helpful message in this case, but when we look at the test function, we see
// that it's annotated with #[should_panic]. The failure we got means that the code in the test
// function did not cause a panic.
//
// Tests that use should_panic can be imprecise. A should_panic test would pass even if the test
// panics for a different reason from the one we were expecting. To make should_panic tests more
// precise, we can add an optional expected parameter to the should_panic attribute. The test
// harness will make sure that the failure message contains the provided text. For example,
// consider the modified code for Guess in Listing 11-9 where the new function panics with
// different messages depending on wheter the value is too small or too large.

// writing new impl block above

// This test will pass because the value we put in the should_panic attribute's expected parameter
// is a substring of the message that the Guesss::new function panics with. We could have specified
// the entire panic message that we expect, which in this case would be Guess value must be less
// than or equal to 100, got 200. Wehat you choose to specify depeinds on how much of the panic
// message is unique or dynamic and how precise you want your test to be. In this case, a substring
// of the panic message is enough to ensure that the code in the test function executes the else if
// values > 100 case.
//
// To see what happens when a should_panic test with an expected message fails, let's again
// intorduce a bug into our code by swapping the bodies of the if value < 1 and the else if value >
// 100 blocks:

// swapped above


// This time when we run the should_panic test, it will fail:


/*
$ cargo test
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished test [unoptimized + debuginfo] target(s) in 0.66s
     Running unittests src/lib.rs (target/debug/deps/guessing_game-57d70c3acb738f4d)

running 1 test
test tests::greater_than_100 - should panic ... FAILED

failures:

---- tests::greater_than_100 stdout ----
thread 'main' panicked at 'Guess value must be greater than or equal to 1, got 200.', src/lib.rs:13:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: panic did not contain expected string
      panic message: `"Guess value must be greater than or equal to 1, got 200."`,
 expected substring: `"less than or equal to 100"`

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
*/

// The failure message indicates that this test did indeed panic as we expected, but the panic
// message did not include the expected string 'Guess value must be less than or equal to 100'. The
// panic message that we did get in this case was Guess value must be greater than or equal to 1,
// got 200. Now we can start figuring out where our bug is!

fn main() {}
