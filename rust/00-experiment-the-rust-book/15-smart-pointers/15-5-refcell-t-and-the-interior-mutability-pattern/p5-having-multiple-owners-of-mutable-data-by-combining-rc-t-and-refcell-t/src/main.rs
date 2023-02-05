// Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>
// =========================================================================
//
// A common way to use RefCell<t> is in combination with Rc<T>. Recall that Rc<T> lets you have
// multiple owners of some data, but it only gives immutable access to that data. If you have an
// Rc<T> that holds a RefCell<t>, you can get a value that can have multiple owners and that you
// can mutate!
//
// For example, recall the cons list example in Listing 15-18 where we used Rc<T> to allow mutliple
// lists to share ownership of another list. Because Rc<T> holds only immutable values, we can't
// change any of the values in the list once we've created them. Let's add in RefCell<T> to gain
// the ability to change the values in the lists. Listing 15-24 shows that by using a RefCell<T> in
// the Cons definition, we can modify the values stored in all the lists:

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

// Listing 15-24: Using Rc<RefCell<i32>> to create a List that we can mutate

// We create a value that is an instance of Rc<RefCell<i32>> and store it in a variable named value
// so we can access it directly later. Then we create a List in a with a Cons variant that holds
// value. We need to clone value so both a and value have ownership of the inner 5 value rather
// than transferring ownership from value to a or having a borrow from value.
//
// We wrap the list in an Rc<T> so when we create lists b and c, they can both refer to a, which is
// what we did in Listing 15-18.
//
// After we've created the lists in a, b, and c, we want to add 10 to the value in value. We do
// this by calling borrow_mut on value, which uses the automatic dereferencing features we
// discussed in Chapter 5 (see the section "Wehere's the -> Operator?") to dereference the Rc<T> to
// the inner RefCell<t> value. The borrow_mut method returns a RefMut<T> smart pointer, and we use
// the dereference operator on it and change the inner value.
//
// When we print a, b, and c, ew can see that they all have the modified value of 15 rather than 5:


/*
$ cargo run
   Compiling cons-list v0.1.0 (file:///projects/cons-list)
    Finished dev [unoptimized + debuginfo] target(s) in 0.63s
     Running `target/debug/cons-list`
a after = Cons(RefCell { value: 15 }, Nil)
b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
*/

// This technique is pretty nead! By using RefCell<t>, we have an outwardly immutable List value.
// But we can use the methods on RefCell<t> that provide access to its interior mutablility so we
// can modify our data when we need to. The runtime checks of the borrowing rules protect us from
// data races, and it's sometimes worth trading a bit of speed for this flexibility in our data
// structures. Note that RefCell<t> does not work for multithreaded code! Mutex<T> is the
// thread-safe version of RefCell<t> and we'll discuss Mutex<T> in Chapter 16.


