// The Static Lifetime
// ===================
//
// One special lifetime we need to discuss is 'static, which denotes that the affected reference
// can life for the entire duration of the program. All string literals have the 'static lifetime,
// which can annotate as follows:
let s: &'static str = "I have a static lifetime.";

// The text of this string is stored directly in the program's binary, which always is available.
// Therefore, the lifetime of all string literals is 'static.
//
// You might see suggestions to use the 'static lifetime in error messages. But before specifying
// 'static as the lifetime for a reference, think about wheter the reference you have actually
// lives the entire lifetime of your program or not, and wheter you want it to. Most of the time,
// an error message suggesting the 'static lifetime results from attempting to create a dangling
// reference or a mismatch of the available lifetimes. In such cases, the solution is fixing those
// problems, not specifying the 'static lifetime.
