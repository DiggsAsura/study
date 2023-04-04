use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro, My name is Pancaces!");
    }
}

fn main() {
    Pancakes::hello_macro();
}

