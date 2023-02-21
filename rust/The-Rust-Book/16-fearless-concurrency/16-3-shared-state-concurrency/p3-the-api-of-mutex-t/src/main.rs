// The API of Mutex<T>
// ===================
//
// As an example of how to use a mutex, let's start by using a mutex in a single threaded context,
// as shown in Listing 16-12:

use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

// Listing 16-2: Exploring the API of Mutex<T> in a single-threaded context for simplicity

// As with many types, we cretae a Mutex<T> using the associated function new. To access the data
// inside the mutex, we use the lock method to aquire the lock. This call will block the current
// thread so it can't do any work until it's our turn to have the lock.
//
// The call to lock would fail if another thread holding the lock panicked. In that case, no one
// would ever be able to get the lock, so we've chosen to unwrap and have this thread panic if
// we're in that situation.
//
// After we've aquired the lock, we can treat the return value, named num in this case, as a
// mutable reference to the data inside. The type system ensures that we acquire a lock before
// using the vaue in m. The type of m is Mutex<i32>, not i32, so we must call lock to be able to
// use the i32 value. We can't forget; the type system won't let us access the inner i32 otherwise.
//
// As you might suspect, Mutex<T> is a smart pointer. More accurately, the call to lock returns a
// smart pointer called MutexGuard, wrapped in a LockResult that we handle with the call to unwrap.
// The MutexGuard smart pointer implements Deref to point at our inner data; the smart pointer also
// has a Drop implementation that releases the lock automatically when a MutexGuard goes out of
// scope. As a result, we don't risk forgetting to release the lock and blocking the mutex from
// being used by other threads, because the lock release happens automatically.
//
// After dropping the lock, we can print the mutex value and see that we were able to change the
// inner i32 to 6.
