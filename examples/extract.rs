extern crate crowbook_localize;

use crowbook_localize::Extractor;

fn main() {
    let mut extractor = Extractor::new();
    extractor.add_messages_from_file("/tmp/book.rs").unwrap();
    extractor.print_messages();
}
