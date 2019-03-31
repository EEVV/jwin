extern crate jwin;

use jwin::{Event, Win};

fn main() {
    let mut win = Win::new("display".to_string()).unwrap();

    loop {
        match win.poll() {
            Some(x) => match x {
                Event::Redraw(_, _) => {
                    win.put_str(0, 0, "First line", 0);
                    win.put_str(0, 1, "Second line", 0);
                    win.put_str(0, 2, "Third line", 0);
                    win.put_str(0, 3, "First line bold", 1);
                    win.put_str(0, 4, "Second line bold", 1);
                    win.put_str(0, 5, "Third line bold", 1);
                },
                Event::Close => return,
                _ => ()
            },
            None => ()
        }
    }
}