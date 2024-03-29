// Ensuring the Content of a Draft Post Is Empty
// =============================================
//
// Even after we've called add_text and added some content to our post, we still want the content
// method to return an empty string slice because the post is still in the draft state, as shown on
// line 7 of Listing 7-11. For now, let's implement the content method with the simplest thing that
// will fullfill this requirement: always returning an empty string slice. We'll change this later
// once we implemente the ability to change a post's state so it can be published. So far, posts
// can only be in the draft state, so the post content should always be empty. Listing 17-14 shows
// this placehodler implementation:

impl Post {
    // --snip--
    pub fn content(&self) -> &str {
        ""
    }
}
// Listing 17-14: Adding a placeholder implementation for the content method on Post that always
// returns an empty string slice

// With this added content method, everything in Listing 17-11 up to line 7 works as intended.
