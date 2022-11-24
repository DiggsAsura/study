/* Tuples
 *
 * A tuple is a general way of gropuing together a number of values with a variety of types into
 * one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in
 * size. 
 *
 * We create a tuple by writing a comma-separated list of values inside parentheses. Each position
 * in the tuple has a type, and the types of the different values in the tuple don't have to be the
 * same. We've added optional type annotations in this example: */


fn main() {
    let tup: (i32, f64, u8) = (500, 5.3, 1);

    /* The variable tup binds to all of the values, because its considered a single compound
     * tlement. To get the individual values of a tuple, we can use pattern matching to destructure
     * a tuple value like this: */

    let (x, y, z) = tup;

    println!("The value of y = {y}");

    /* This program first creates a tuple and binds it to the variable tup. It then uses a pattern
     * with let to make tup and turn it into three separate variables, x, y, and z. This is called
     * destructuring, because it breaks the single tuple into three parts. Finally, the program
     * prints the value of y, which is 6.4.
     *
     * We can also access tuple element directly by using a period (.) followed by the index of the
     * value we want to access. For example: */

    let five_hundred = tup.0;
    let five_point_three = tup.1;
    let one = tup.2;


    /* 
     * The tuple without any values has a special name, unit. This value and its corresponding type
     * are both written () and represents an empty value or an empty return type. Expressions
     * implicitly return the unit value if they don't return any other value.
     */

}
