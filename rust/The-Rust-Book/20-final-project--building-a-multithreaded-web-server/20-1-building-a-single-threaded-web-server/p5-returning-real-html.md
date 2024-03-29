# Returning Real HTML

Let's implement the functionality for returning more than a blank page. Create the new file *hello.html*
in the root of your project directory, not in the *src* directory. You can input any HTML you want;
Listing 20-4 shows one possibility.

Filename: hello.html
```
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
  </body>
</html>
```
*Listing 20-4: A sample HTML file to return in a response*

This is a minimal HTML5 document with a heading and some text. To return this from the server
when a request is received, we'll modify **handle_connection** as shown in Listing 20-5 to read the
HTML file, add it to the response as a body, and send it.

Filename: src/main.rs
```rust
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
// --snip--

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\{contents}");
}
```
*Listing 20-5: Sending the contents of **hello.html** as the body of the response*

We've added **fs** to the **use** statement to bring the standard library's filesystem module into scope.
The code for reading the contents of a file to a string should look familiar; we used it in Chapter 12
when we read the contents of a file for our I/O project in Listing 12-4.

Next, we use **format!** to add the fil's contents as the body of the success response. To ensure a
valid HTTP response, we add the **Content-Length** header which is set to the size of our response
body, in this case the size of **hello.html**.

Run this code with **cargo run** and load *127.0.0.1:7878* in your browser; you should see your HTML
rendered!

Currently, we're ignoring the request data in **http_request** and just sending back the contents of
the HTML file unconditionally. That means if you try requesting *127.0.0.1:7878/something-else* in your
browser, you'll still get back this same HTML response. At the moment, our server is very limited and
does not do what most web servers do. We want to customize our response depending on the
request and only send bak the HTML file for a well-formed request to /.
