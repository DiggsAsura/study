// All The Places Patterns Can Be Used
// ====================================
//
// Patterns pop up in a number of places in Rust, and you've been using them a lot without
// realizing it! This section discuss all the places where patterns are valid.
//
//
// match Arms
// ----------
//
// As discussed in Chapter 6, we use patterns in the arms of match expressions. Formally, match
// expressions are defined as the keyword match, a value to match on, and one or more match arms
// that consists of a pattern and an expression to run if the value matches that arm's pattern,
// like this:
//
//
// match VALUE {
//    PATTERN => EXPRESSION,
//    PATTERN => EXPRESSION,
//    PATTERN => EXPRESSION,
// }
//
//
// For example, here's the match expression from Listing 6-5 that matches on an Option<i32> value
// in the variable x:

fn ex1() {
    let x = Some(5);

    match x {
        None => None,
        Some(i) => Some(i + 1),
    };
}

// The patterns in this match expression are the None and Some(i) on the left of each arrow.
//
// One requirement for match expressions is that they need to be exhaustive in the sense that all
// possibilities for the value in the match expression must be accounted for. One way to ensure
// you've covered every possibility is to have a catchall pattern for the last arm: for example, a
// variable name matching any value can never fail and thus covers every remaining case.
//
// The particular pattern _ will match anything, but it never binds to a variable, so it's often
// used in the last match arm. The _ pattern can be useful when you want to ignore any value not
// specified, for example. We'll cover the _ pattern in more detail in the "Ignoring Values in a
// Pattern" section later in this chapter.



// Conditional if let Expressions
// ------------------------------
//
// In Chapter 6 we discussed how to use if let expressions mainly as a shorter way to write the
// equivalent of a match that only matches one case. Optionally, if let can have a corresponding
// else containing code to run if the pattern in the if let doesn't match.
//
// Listing 18-1 shows that it's also possible to mix and match if let, else if, and else if let
// expressions. Doing so gives us more flexibility than a match expression in which we can express
// only one value to compare with the patterns. Also, Rust doesn't require that the conditions in a
// series of if let, else if, else if let arms relate to each other.
//
// The code in Listing 18-1 determines what color to make your background based on a series of
// checks for several conditions. For this example, we've created variables with hardcoded values
// that a real program might receive from user input.

fn ex2() {
    let favorite_color: Option<&str> = Some("Greenish");
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

// Listing 18-1: Mixing if let, else if, else if let, and else

// If the user specifies a favorite color, that color is used as the background. If no favorite
// color is specified and today is Tuesday, the background color is green. Otherwise, if the user
// specifies their age as a sgring and we can parse it as a number successfully, the color is
// either purple or orange depending on the value of the number. If none of these conditions apply,
// the background color is blue.
//
// This conditional structure lets us support complex requirements. With the hardcoded values we
// have here, this example will print Using purple as the background color.
//
// You can see that if let can also introduce shadowed variables in the same way taht match arms
// can: the line if let Ok(age) = age introduces a new shadowed age variable that contains the
// value inside the Ok variant. This means we need to place the if age > 30 condition within that
// block: we can't combine these two conditions into if let Ok(age) = age && age > 30. The
// shadowed age we want to compare to 30 isn't valid until the new scope starts with the curly
// brackets.
//
// The downside of using if let expressions is that the compiler doesn't check for exhaustiveness,
// whereas with match expressions it does. If we omitted the last else block and therefor missed
// handling some cases, the compiler would not alert us to the possible logic bug.


// while let Conditional Loops
// ---------------------------
//
// Similar in construction to if let, the while let conditional loop allows a while loop to run for
// as long as a pattern continues to match. In Listing 18-2 we code a while let loop that uses a
// vector as a stack and prints the values in the vector in the opposite order in which they were
// pushed.

fn ex3() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
// Listing 18-2: Using a while let loop to print values for as long as stack.pop() returns Some

// This example prints 3, 2, 1. The pop method takes the last element out of the vector and returns
// Some(value). If the vector is empty, pop returns None. The while loop continues running the code
// in its block as long as pop returns Some. When pop returns None, the loop stops. We can use
// while let to pop every element off our stack.



// for Loops
// ---------
//
// In a for loop, the value that directly follows the keyword for is a pattern. For example, in for
// x in y the x is the pattern. Listing 18-3 demonstrates how to use a pattern in a for loop to
// destructure, or break apart, a tuple as part of the for loop.

fn ex4() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate().map(|(i, v)| (v, i+1)) {
        println!("{} is at index {}", value, index);
    }
}
// Listing 18-3: Using a pattern in a for loop to destructure a tuple

// We adapt an iterator using the enumerate method so it produces a value and the index for that
// value, placed into a tuple. The first value produced is the tuple (0, 'a'). When this value is
// matched to the pattern (index, value), index will be 0 and value will be 'a', printing the first
// line of the output.



// let Statements
// --------------
//
// Prior to this chapter, we had only explicitly discussed using patterns with match and if let,
// but in fact, we've used patterns in other places as well, including in let statements. For
// example, consider this straightforward variable assignment with let:

// let x = 5;

// Every time you've used a let statement like this you've been using patterns, althrough you might
// not have realized it! ore formally, a let statement looks like this:

// let PATTERN = EXPRESSION;

// In statements like let = 5; with a variable name in the PATTERN slot, the variable name is just
// a particularly simple form of a pattern. Rust compares the expression against the pattern and
// assigns any names it finds. So in the let x = 5; example, x is a pattern that means "bind what
// matches here to the variable x." Because the name x is the whole pattern, this pattern
// effectively means "bind everything to the variable x, whatever the value is."
//
// To see the pattern matching aspect of let more clearly, consider Listing 18-4, which uses a
// pattern with let to destructure a tuple.

fn ex5() {
    let (x, y, z) = (1, 2, 3);
    println!("x: {}, y: {}, z: {}", x, y, z);
}

// Listing 18-4: Using a paattern to destructure a tuple and create three variables at once

// Here, we match a tuple against a pattern. Rust compares the value (1, 2, 3) to the pattern (x,
// y, z) and sees that the value matches the pattern, so Rust binds 1 to x, 2 to y, and 3 to z. You
// can think of this tuple pattern as nesting three individual variable patterns inside it.
//
// If the number of elements in the pattern doesn't match the number of elements in the tuple, the
// overall type won't match and we'll get a compiler error. For example, Listing 18-5 shows an
// attempt to destructure a tuple with three elements into two variables, which won't work.

// let (x, y) = (1, 2, 3);

// Listing 18-5: Incorrectly constructing a pattern whose variables don't match the number of
// elements in the tuple

// To fix this error, we could ignore one or more of the values in the tuple using _ or .., as
// you'll see in the "Ignoring Values in a Pattern" section. If the problem is that we have too
// many variables in the pattern, the solution is to make the types match by removing variables so
// the number of variables equals the number of elements in the tuple.


// Function Parameters
// -------------------
//
// Function parameters can also be patterns. The code in Listing 18-6, which declares a function
// named foo that takes one parameter named x of type i32, should by now look familiar.

fn ex6(x: i32) {
    // code goes here
}
// Listing 18-6: A function signature uses patterns in the parameters

// The x part is a pattern! As we did with let, we could match a tuple in a function's arguments to
// the pattern. Listing 18-7 splits the values in a tuple as we pass it to a function.

fn ex7(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

// Listing 18-7: A function with parameters that destructure a tuple

// This code prints Current locations: (3, 5). The values &(3, 5) match the pattern &(x, y), so x
// is the value 3 and y is the value 5.
//
// We can also use patterns in closure parameter lists in the same way as in function parameter
// lists, because closures are similar to functions, as discussed in Chapter 13.
//
// At this point, you've seen several ways of using patterns, but patterns don't work the same in
// every place we can use them. In some places, the patterns must be irrefutable; in other
// circumstances, they can be refutable. We'll discuss these two concepts next.

fn main() {
    ex1();
    ex2();
    ex3();
    ex4();
    ex5();

    let point = (3, 5);
    ex7(&point);
}


