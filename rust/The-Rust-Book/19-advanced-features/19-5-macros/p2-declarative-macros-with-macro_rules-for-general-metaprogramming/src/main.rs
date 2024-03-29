// Declarative Macros with macro_rules! for General Metaprogramming
// =================================================================
//
// The most widely used form of macros in Rust is the declarative macro. These are also sometimes
// referred to as "macros by example", "macro_rules! macros", or just plain "macros". At their
// core, declarative macros allow you to write something similar to a Rust match expression. As
// discussed in Chapter 6, match expressions are control structures that take an expression,
// compare the resulting value of the expression to patterns, and then run the code associated with
// the matching pattern. Macros also compare a value to patterns that are associated with
// particular code: in this situation, the value is the literal Rust source code passed to the
// macro; the patterns are compared with the structure of that source code; and the code associated
// with each pattern, when matched, replaces the code passed to the macro. This all happens during
// compilation.
//
// To define a macro, you use the macro_rules! construct. Let's explore how to use macro_rules! by
// looking at how the vec! macro is defined. Chapter 8 covered how we can use the vec! macro to
// create a new vector with particular values. For example, the following macro creates a new
// vector containing three integers:

fn ex1() {
    let v: Vec<u32> = vec![1, 2, 3];
    println!("v = {:?}", v);
}

// We could also use the vec! macro to make a vector of two integers or a vector of five string
// slices. We wouldn't be able to use a function to do the same because we wouln't know the number
// or type of values up front.
//
// Listing 19-28 shows a slightly simplified definition of the vec! macro.

fn ex2() {
    #[macro_export]
    macro_rules! vec {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }
}

// Listing 19-28: A simplified version of the vec! macro definition

// --
// Note: The actual definition of the vec! macro is the standard library includes code to
// preallocate the correct amount of memory up front. That code is an optimization that we don't
// include here to make the example simpler.
// ----
//
//
// The #[macro_export] annotation indicates that this macro should be made available whenever the
// crate in which the macro is defined is brought into scope. Without this annotation, the macro
// can't be brought into scope.
//
// We then start the macro definition with macro_rules! and the name of the macro we're defining
// without the exclamation mark. The name, in this case vec, is followed by curly brackets denoting
// the body of the macro definition.
//
// The structure in the vec! body is similar to the structure of a match expression. Here we have
// one arm with the pattern ( $( $x:expr ),* ), followed by => and the block of code associated
// with this pattern. If the pattern matches, the associated block of code will be emitted. Given
// that this is the only pattern in this macro, there is only one valid way to match; andy other
// pattern will result in an error. More complex macros will have more than one arm.
//
// Valid pattern syntax in macro definitions is different than the pattern syntax covered in
// Chapter 18 because macro patterns are matched against Rust code structure rather than values.
// Let's walk through what the pattern pieces in Listing 19-28 mean; for the full macro pattern
// syntax, see the Rust Reference: https://doc.rust-lang.org/reference/macros-by-example.html
//
// First, we use a set of parantheses to encompass the whole pattern. We use a dollar sign ($) to
// declare a variable in the macro system that will contain the Rust code matching the pattern. The
// dollar sign makes it clear this is a macro variable as opposed to a regular Rust variable. Next
// comes a set of parantheses that captures values that match the pattern within the paranthesese
// for use in the replacement code. Within $() is $x:expr, which matches any Rust expression and
// gives the expression the name $x.
//
// The comma following $() indicates that a literal comma separator character could optionlally
// appear after the code that matches the code in $(). The * specifies that the pattern matches
// zero or more of whatever precedes the *.
//
// When we call this macro with vec![1, 2, 3], the $x pattern matches three times with the three
// expressions 1, 2, and 3.
//
// Now let's look at the pattern in the body of the code associated with this arm: temp_vec.push()
// within $()* is generated for each part that matcehs $() in the pattern zero or more times
// depending on how many times the pattern matches. The $x is replaced with each expression
// matched. When we call this macro with vec![1, 2, 3];, the code generated that replace this macro
// call will be the following

fn ex2() {
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}

// We've defined a macro that can take any number or arguments of any type and can generate code to
// create a vector containing the specified elements.
//
// To learn more about how to write macros, consult the online documentation or other resources,
// such as "The Little Book of Rust Macros" started by Daniel Keep an continued by Lukas Wirth.
//
// https://veykril.github.io/tlborm/
//
//
