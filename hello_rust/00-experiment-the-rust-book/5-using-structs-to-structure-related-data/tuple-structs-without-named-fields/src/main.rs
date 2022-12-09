/* Using Tuple Structs without Named Fields to Create Different Types 
 *
 * Rust also supports structs that look similar to tuples, called tuple structs. Tuple structs have
 * the added meaning the struct name provides but don't have names associated with their fields;
 * rather, they just ahve the types of the fields. Tuple structs are useful when you want to give
 * the whole tuple a name and make the tuple a name and make  the tuple a different type from other
 * tuples, and when naming each field as in a regular struct would be verbose or reduntant. 
 *
 * To define a tuple struct, start with the struct keyword and the struct name followed by the
 * types in the tuple. For example, here we define and use two tuple structs named Color and Point:
 */

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

/* Note that the black and origin values are different types, because they're instances of
 * different tuple structs. Each struct you define is its own type, even though the fields within
 * the struct might have the same types. For example, a function that takes a parameter of type
 * Color cannot take a Point as an argument, even though both types are made up of three i32
 * values. Otherwise, tuple struct instances are similar to tuples in that you can destructure them
 * into their individual pieces, and you can use a . followed by the index to access individual
 * value. */
