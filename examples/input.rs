extern crate jwin;

use jwin::{Code, Event, Win};

// simple input example
// doesn't use redraw action which means
// that the screen will be wiped on resize
// doesn't use a char buffer as well
fn main() {
    let mut win = Win::new("input".to_string()).unwrap();

    let mut cursor_x = 0;
    let mut cursor_y = 0;

    loop {
        match win.poll() {
            Some(x) => match x {
                Event::Key(Code::Showable(string)) => {
                    let len = string.len();
                    win.put_string(cursor_x, cursor_y, string);
                    cursor_x += len;
                },
                Event::Key(Code::Backspace) => {
                    if cursor_x != 0 {
                        cursor_x -= 1;
                        win.put_string(cursor_x, cursor_y, String::from(" "));
                    }
                },
                Event::Key(Code::Return) => {
                    cursor_x = 0;
                    cursor_y += 1;
                },
                Event::Close => return,
                _ => ()
            },
            None => ()
        }
    }
}