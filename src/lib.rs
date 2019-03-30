extern crate x11;

use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::*;
use std::ptr;
use std::cmp;

use x11::xlib;
use x11::xft;
use x11::xrender;

mod config;

#[derive(Debug)]
pub enum Code {
    Showable(String),
    Backspace,
    Return
}

#[derive(Debug)]
pub enum Event {
    Redraw(usize, usize),
    Key(Code),
    Close
}

pub struct Win {
    // used for selecting text
    // might get removed
    buffer: Vec<Vec<char>>,
    buffer_width: usize,
    buffer_height: usize,

    width: usize,
    height: usize,

    // xlib shit
    display: *mut xlib::Display,
    visual: *mut xlib::Visual,
    window: xlib::Window,

    // protocols
    wm_protocols: c_ulong,
    wm_delete_window: c_ulong,

    // xft shit
    font: *mut xft::XftFont,
    draw: *mut xft::XftDraw,
    colors: [xft::XftColor; 2]
}

impl Win {
    pub fn new(title: String) -> Option<Win> {
        unsafe {
            let display = xlib::XOpenDisplay(ptr::null());
            if display.is_null() {
                return None;
            }

            let screen = xlib::XDefaultScreen(display);
            let root = xlib::XRootWindow(display, screen);
            let window = xlib::XCreateSimpleWindow(
                display,
                root,
                0, 0,
                512, 256,
                0,
                0, 0
            );

            xlib::XMapWindow(display, window);

            xlib::XSelectInput(display, window, xlib::ExposureMask | xlib::KeyPressMask);

            let visual = xlib::XDefaultVisual(display, screen);
            let colormap = xlib::XDefaultColormap(display, screen);

            let title_str = CString::new(title).ok()?;
            xlib::XStoreName(display, window, title_str.as_ptr() as *mut c_char);

            let wm_protocols_str = CString::new("WM_PROTOCOLS").ok()?;
            let wm_delete_window_str = CString::new("WM_DELETE_WINDOW").ok()?;

            let wm_protocols = xlib::XInternAtom(display, wm_protocols_str.as_ptr(), xlib::False);
            let wm_delete_window = xlib::XInternAtom(display, wm_delete_window_str.as_ptr(), xlib::False);

            let mut protocols = [wm_delete_window];
            xlib::XSetWMProtocols(display, window, protocols.as_mut_ptr(), protocols.len() as c_int);

            let font = xft::XftFontOpenName(display, screen, CString::new(config::FONT).ok()?.as_ptr());
            if font.is_null() {
                xlib::XCloseDisplay(display);
                return None;
            }

            let draw = xft::XftDrawCreate(
                display,
                window,
                visual,
                colormap
            );
            if draw.is_null() {
                xlib::XCloseDisplay(display);
                return None;
            }

            let mut colors: [xft::XftColor; 2] = mem::uninitialized();

            for i in 0..config::COLORS_LEN {
                let (r, g, b, a) = config::COLORS[i];

                xft::XftColorAllocValue(
                    display,
                    visual,
                    colormap,
                    &xrender::XRenderColor {
                        red: r,
                        green: g,
                        blue: b,
                        alpha: a
                    },
                    &mut colors[i]
                );
            }

            let win = Win {
                buffer: vec![],
                width: 0, height: 0,
                buffer_width: 0, buffer_height: 0,

                display: display,
                visual: visual,
                window: window,

                wm_protocols: wm_protocols,
                wm_delete_window: wm_delete_window,

                font: font,
                draw: draw,
                colors: colors
            };
            return Some(win);
        }
    }

    fn resize_buffer(&mut self, w: usize, h: usize) {
        self.buffer_width = w;
        self.buffer_height = h;

        self.buffer = vec![vec![' '; w]; h];
    }

    unsafe fn update_dimensions(&mut self) {
        let mut root = mem::uninitialized();
        let mut c_x = mem::uninitialized();
        let mut c_y = mem::uninitialized();
        let mut c_width = mem::uninitialized();
        let mut c_height = mem::uninitialized();
        let mut c_border = mem::uninitialized();
        let mut c_depth = mem::uninitialized();
        xlib::XGetGeometry(
            self.display,
            self.window,
            &mut root,
            &mut c_x, &mut c_y,
            &mut c_width, &mut c_height,
            &mut c_border,
            &mut c_depth
        );

        self.width = c_width as usize;
        self.height = c_height as usize;

        let width = self.width / ((*self.font).max_advance_width as usize);
        let height = self.height / ((*self.font).height as usize);

        self.resize_buffer(width, height);
    }

    pub fn redraw(&mut self) {
        unsafe {
            let mut y = 1;

            for line in &self.buffer {
                let line_string: String = line.into_iter().collect();

                xft::XftDrawString8(
                    self.draw,
                    &self.colors[1],
                    self.font,
                    0, y * (*self.font).height,
                    CString::new(line_string).unwrap().as_ptr() as *const u8,
                    line.len() as i32
                );

                y += 1;
            }
        }
    }

    pub fn poll(&mut self) -> Option<Event> {
        unsafe {
            let mut event = mem::uninitialized();
            xlib::XNextEvent(self.display, &mut event);

            match event.get_type() {
                xlib::Expose => {
                    self.update_dimensions();
                    self.redraw();

                    return Some(Event::Redraw(self.buffer_width, self.buffer_height));
                },
                xlib::KeyPress => {
                    let keysym = xlib::XKeycodeToKeysym(self.display, event.key.keycode as u8, 0);
                    let string = CStr::from_ptr(xlib::XKeysymToString(keysym)).to_str().unwrap();

                    return Some(Event::Key(config::map_keystring(string)?));
                },
                xlib::ClientMessage => {
                    let xclient = xlib::XClientMessageEvent::from(event);

                    if xclient.message_type == self.wm_protocols && xclient.format == 32 {
                        let protocol = xclient.data.get_long(0) as xlib::Atom;

                        if protocol == self.wm_delete_window {
                            return Some(Event::Close);
                        }
                    }
                },
                _ => ()
            }
        }

        return None;
    }

    pub fn put_string(&mut self, x: usize, y: usize, string: String) {
        if self.buffer_height <= y {
            return;
        }

        let start = x;
        let end = cmp::min(self.buffer_width, string.len() + x);
        let range = start..end;

        let mut curr_x = start;
        for chr in string.chars() {
            if !range.contains(&curr_x) {
                break;
            }
            self.buffer[y][curr_x] = chr;

            curr_x += 1;
        }

        let len = (end - start) as i32;

        unsafe {
            let x_pos = (x as i32) * (*self.font).max_advance_width;
            let y_pos = (y as i32) * (*self.font).height;

            xft::XftDrawRect(
                self.draw,
                &self.colors[0],
                x_pos, y_pos + (*self.font).descent,
                (len * (*self.font).max_advance_width) as u32, (*self.font).height as u32
            );

            xft::XftDrawString8(
                self.draw,
                &self.colors[1],
                self.font,
                x_pos, y_pos + (*self.font).height,
                CString::new(string).unwrap().as_ptr() as *const u8,
                len
            );
        }
    }
}

impl Drop for Win {
    fn drop(&mut self) {
        unsafe {
            for i in 0..config::COLORS_LEN {
                xft::XftColorFree(
                    self.display,
                    self.visual,
                    self.window,
                    &mut self.colors[i]
                );
            }
            xft::XftDrawDestroy(self.draw);
            xlib::XCloseDisplay(self.display);
        }
    }
}

