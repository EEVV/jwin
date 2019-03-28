extern crate jift_term_x11;

use jift_term_x11::Term;

fn main() {
    let mut term = Term::new("example test".to_string()).unwrap();

    while !term.should_close() {
        term.poll();
    }
}