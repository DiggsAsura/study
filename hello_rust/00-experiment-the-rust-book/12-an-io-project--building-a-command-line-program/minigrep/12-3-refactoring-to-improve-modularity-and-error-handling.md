# Refactoring to Improve Modularity and Error Handling

To improve our program, we'll fix four problems that have to do with the program's structure and how
it's handling potential errors. First, our main function now performs two tasks: it parses
arguments and reads files. As our program grows, the number of separate tasks the main function
handles will increase. As function gains responsibilities, it becomes more difficult to reason about,
harder to test, and harder to change without breaking one of its parts. It's best to separate
functionality so each function is responsible for one task.

This issue also ties into the second problem: although query anf file_path are configuration
variables to our program, variables like contents are used to perform the program's logic. The
Longer main becomes, the more variables we'll need to bring into scope; the more variables we have
in scope, the harder it will be to keep track of the purpose of each. It's best to group the
configuration variables into one structure to make their purpose clear.

The third problem is that we've used expect to print an error message when reading the file fails,
but the error message just prints Should have been able to read the file. Reading a file can fail
in a number of ways: for example, the file could be missing, or we might not have permission to
open it. Right now, regardless of the situation, we'd print the same error message for everything,
which wouldn't give the user any information!

Fourth, we use expect repeatedly to handle different errors, and if the user runs our program
without specifying enough arguments, they'll get an index out of bounds error from Rust that
doesn't clearly explain the problem. It would be best if all the error-handling code were in one place
so future maintainers had only one place to consult the code if the error-handling logic needed to
change. Having all the error-handling code in one place will also ensure that we're printing messages
that will be meaningful to our end users.

Let's address these four problems by refactoring our project.


## Separation of Concerns for Binary Projects

The organizational problem of allocating responsibility for multiple tasks to the main function is
common to many binary projects. As a result, the Rust community has developed guidelines for
splitting the separate concerns of a binary program when main starts getting large. This process has
the following steps:

    - Split our program into a main.rs and lib.rs and move your program's logic to lib.rs
    - As long as your command line parsing logic is small, it can remain in main.rs
    - When the command line parsing logic starts getting complicated, extract it from main.rs and
      move it to lib.rs.

The responsibilities that remain in the main function after this process should be limited to the
following:

    - Calling the command line parsing logic with the argument values
    - Setting up any other configuration
    - Calling a run function in lib.rs
    - Handling the error if run returns an error

This pattern is about separating concerns: main.rs handles running the program, and libr.rs handles
all the logic of the task at hand. Because you can't test the main function directly, this structure lets
you test all of your program's logic by moving it into functions in lib.rs. The code that remains in
main.rs will be small enough to verify its correctness by reading it. Let's rework our program by
following this process.


### Extracting the Argument Parser

We'll extract the functionality for parsing arguments into a function that **main** will call to prepare for
moving the command line parsing logic to *src/lib.rs*. Listing 12-5 shows the new start of **main** that
calls a new function parse_config, which we'll define in *src/main.rs* for the moment.

*(editing main.rs)*

12-5: Extracting a parse_config function from main

We're still collecting the command line arguments into a vector, but instead of assigning the
argument value at index 1 to the variable **query** and the argument value at index 2 to the variable
**file_path** within the **main** funciton, we pass the whole vector to the parse_config function. The
**parse_config** function then holds the logic that determines which argument goes in which variable
and passes the values back to **main**. We still create the **quesry** and **file_path** variables in main,
but **main** no longer has the responsibility of determining how the command line arguments and
variables correspond.

This rework may seem like overkill for our small program, but we're refactoring in small, incremental
steps. After making this change, run the program again to verify that the arguments parsing still
works. It's good to check your progress often, to help identify the cause of problems when they
occur.


### Grouping Configuration Values

We can take another small step to improve the parse_config function further. At the moment,
we're returning a tuple, but then we immediately break that tuple into individual parts again. This is
a sign that perhaps we don't have the right abstraction yet.

Another indicator that shows there's room for improvement is the config part of parse_config,
which implies that the two values we return are related and are both part of one configuration value.
Wer're not currently conveying this meaning in the structure of the data other than by grouping the
two values into a tuple; we'll instead put the two values into one struct and give each of the struct
fields a meaningful name. Doing so will make it easier for future maintainers of this code to
understand how the different values relate to each other and what their purpose is.

Listing 12-6 shows the improvements to the parse_config function.

*editing main.rs*
12-6: Refactoring parse_config to return an instance of a Config struct

We've added a struct named Config defined to have fields named query and file_path. The
signature of parse_config now indicates that it returns a Config value. In the body of
parse_config, where we used to return string slices that reference String values in args, we now
define Config to contain owned String values. The args variable in main is the owner of the
argument values and is only letting the parse_config functiong borrow them, which means we'd
violate Rust's borrowing rules if Config tried to take ownership of the values in args.

There are a number of ways we could manage the String data; the easiest, though somewhat
inefficient, route is to call the clone method on the values. This will make a full copy of the data for
the Config instance to own, which takes more time and memory than storing a reference to the
string data. However, cloning the data also makes our code very straightforward because we don't
have to manage the lifetimes of the references; in this circumstance, giving up a little performance
to gain simplicity is a worthwihile trade-off.


- ---------
### The Trade-Offs of Using clone

There's a tendency among many Rustaceans to avoid using clone to fix ownership problems
because of its runtime cost. In Chapter 13, you'll learn how to use more efficient methods in
this type of situation. But for now, it's okay to copy a few strings to continue making progress
because you'll make these copies only once and your file path and query string are very small.
It's better to have a working program that's a bit inefficient than to try to hyperoptimize code
on your first pass. As you become more experienced with Rust, it'll be easier to start with the
most efficient solution, but for now, it's perfectly acceptable to call clone.
- ---------


We've updated main so it places the instance of Config returned by parse_config into a variable
named config, and we updated the code that previously used the separate query and file_path
variables so it now uses the fields on the Config struct instead.

Now our code more clearly conveys that **query** and **file_path** are related and that their purpose is
to configure how the program will work. Any code that uses these values knows to find them in the
**config** instance in the fields named for their purpose.