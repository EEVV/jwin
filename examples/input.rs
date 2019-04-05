extern crate jwin;

use jwin::{Code, Event, Win};

// simple input example
// doesn't use redraw action which means
// that the screen will be wiped on resize
// doesn't use a char buffer as well
fn main() {
    let mut win = Win::new("input".to_string()).unwrap();

    let mut cursor_x = 0;

    win.set_bg(0);
    win.set_fg(1);
    win.set_font(0);

    loop {
        match win.poll() {
            Some(x) => match x {
                Event::Key(Code::Showable(string)) => {
                    // have to iterate because of
                    // paste functionality
                    for chr in string.chars() {
                        match chr {
                            '\n' => (),
                            _ => {
                                win.put_char(cursor_x, 0, chr);
                                cursor_x += 1;
                            }
                        }
                    }
                },
                Event::Key(Code::Backspace) => {
                    if cursor_x != 0 {
                        cursor_x -= 1;
                        win.put_char(cursor_x, 0, ' ');
                    }
                },
                Event::Close => return,
                _ => ()
            },
            None => ()
        }
    }
}