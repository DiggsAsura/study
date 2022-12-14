/*
 * Tuples
 * -------
 *
 * A tuple is a collection of values of different types. Tuples are constructed using parantheses
 * (), and each tuple itself is a value with type signature (T1, T2, ...), where T1, T2 are the
 * types of its members. Functions can use tuples to return multiple values, as tuples can hold
 * any number of values.
 */

// Tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // 'let' can be used to bind the members of a tuple to variables
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

use std::fmt;

// The following struct is for the activity
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "( {}, {} )", self.0, self.1);
        write!(f, "( {}, {} )", self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    matrix
}

fn main() {
    // A tuple with a bunch of different types
    let long_tuple = (1u8, 2u16, 3u32, 4u64, 
                      -1i8, -2i16, -3i32, -4i64, 
                      0.1f32, 0.264, 
                      'a', true);

    // Values can be extracted from the tuple using tuple indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // But long Tuples (more than 12 elements) cannot be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    // tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{}, {}, {}, {}", b, a, d, c); // example is showing {:?}'s. idk if thats preferable

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);


    // Activity 2
    let trans = transpose(matrix);
    println!("{:?}", trans);
    // Activity 1 was easy, this one is a bit tricky
}
