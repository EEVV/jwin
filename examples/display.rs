extern crate jwin;

use jwin::{Event, Win};

fn main() {
    let mut win = Win::new("display".to_string()).unwrap();

    win.set_bg(0);
    win.set_fg(1);

    loop {
        match win.poll() {
            Some(x) => match x {
                Event::Redraw(_, _) => {
                    win.set_font(0);
                    win.put_str(0, 0, "First line");
                    win.put_str(0, 1, "Second line");
                    win.put_str(0, 2, "Third line");
                    win.set_font(1);
                    win.put_str(0, 3, "First line bold");
                    win.put_str(0, 4, "Second line bold");
                    win.put_str(0, 5, "Third line bold");
                },
                Event::Close => return,
                _ => ()
            },
            None => ()
        }
    }
}