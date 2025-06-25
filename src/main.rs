// Define a Book struct with a lifetime-annotated reference
// Book struct lifetime-annotated
struct Book<'a> {
    title: &'a str,
    // author: &'a str,
    excerpt: &'a str,
}

// 20250625 1803 CET SDvW If you keep it like this, most of the cases it must fit.
// DateDPE seems to work for this.
impl<'a> Book<'a> {
    // Constructor method
    fn new(title: &'a str, author: &'a str, excerpt: &'a str) -> Self {
        Book { title, author, excerpt }
    }
    
    // Method that returns a reference tied to the same lifetime
    fn get_title(&self) -> &'a str {
        self.title
    }
}

fn main() {
    // The owned strings must live longer than the Book
    let title = String::from("The Rust Programming Language");
    let author = String::from("Steve Klabnik and Carol Nichols");
    let excerpt = String::from("Welcome to The Rust Programming Language...");
    
    // Create a book with references to our strings
    let book = Book::new(&title, &author, &excerpt);
    
    println!(
        "Book: '{}' by {}\nExcerpt: {}",
        book.get_title(),
        book.author,
        book.excerpt
    );
    
    // All references remain valid because:
    // 1. The strings (title, author, excerpt) live until the end of main()
    // 2. The Book's references don't outlive their source data
}