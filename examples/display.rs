extern crate jwin;

use jwin::{Event, Win};

fn main() {
    let mut win = Win::new("display".to_string()).unwrap();

    loop {
        match win.poll() {
            Some(x) => match x {
                Event::Redraw(x, y) => {
                    win.put_string(0, 0, String::from("First line"));
                    win.put_string(0, 1, String::from("Second line"));
                    win.put_string(0, 2, String::from("Third line"));
                },
                Event::Close => return,
                _ => ()
            },
            None => ()
        }
    }
}