# Preventing Reference Cycles: Turning an Rc<T> into a Weak<T>

So far, we've demonstrated that calling Rc::clone increases the strong_count of an Rc<T>
instance, and an Rc<T> instance is only cleaned up if its strong_count is 0. You can also create a
*weak reference* to the value within an Rc<T> instance by calling Rc::downgrade and passing a
reference to the Rc<T>. Strong references are how you can share ownership of an Rc<T> instance.
Weak references don't express an ownership relationship, and their count doesn't affect when an
Rc<T> instance is cleaned up. They won't caurse a reference cycle because any cycle involving some
weak reference will be broken once the strong reference count of values involved is 0.

When you call Rc::downgrade, you get a smart pointer of type Weak<T>. Instead of increasing the
strong_count in the Rc<T> instance by 1, calling Rc::downgrade increases the weak_count by 1.
The Rc<T> type uses weak_count to keep trak of how many Weak<T> references exist, similar to
strong_count. The difference is the weak_count doesn't need to be 0 for the Rc<T> instance to be
cleaned up.

Because the value that Weak<T> references might have been dropped, to do anything with the value
that a Weak<t> is pointing to, you must make sure the value still exists. Do this by calling the
**upgrade** method on a Weak<t> instance, which will return an Option<Rc<T>>. You'll get a result of
**Some** if the Rc<T> value has not been dropped yet and a result of **None** if the Rc<T> value has
been dropped. Because **upgrade** returns an Option<Rc<T>>, Rust will ensure that the **Some** case
and the **None** case are handled, and there won't be an invalid pointer.

As an example, rather than using a list whose items knows only about the next item, we'll create a
tree whose items know about their children items *and* their parent items.