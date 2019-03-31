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
pub enum Code<'a> {
    Showable(&'a str),
    Backspace,
    Left,
    Right,
    Up,
    Down
}

#[derive(Debug)]
pub enum Event<'a> {
    Redraw(usize, usize),
    Key(Code<'a>),
    Close
}

pub struct Win {
    buffer_width: usize, buffer_height: usize,
    width: usize, height: usize,

    // xlib shit
    display: *mut xlib::Display,
    visual: *mut xlib::Visual,
    window: xlib::Window,

    // protocols
    wm_protocols: c_ulong,
    wm_delete_window: c_ulong,

    // xft shit
    fonts: [*mut xft::XftFont; config::FONTS_LEN],
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

            let mut fonts: [*mut xft::XftFont; config::FONTS_LEN] = mem::uninitialized();
            for i in 0..config::FONTS_LEN {
                fonts[i] = xft::XftFontOpenName(display, screen, CString::new(config::FONTS[i]).ok()?.as_ptr());
                if fonts[i].is_null() {
                    xlib::XCloseDisplay(display);
                    return None;
                }
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
                buffer_width: 0, buffer_height: 0,
                width: 0, height: 0,

                display: display,
                visual: visual,
                window: window,

                wm_protocols: wm_protocols,
                wm_delete_window: wm_delete_window,

                fonts: fonts,
                draw: draw,
                colors: colors
            };
            return Some(win);
        }
    }

    fn resize_buffer(&mut self, w: usize, h: usize) {
        self.buffer_width = w;
        self.buffer_height = h;
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

        let width = self.width / ((*self.fonts[0]).max_advance_width as usize);
        let height = self.height / ((*self.fonts[0]).height as usize);

        self.resize_buffer(width, height);
    }

    pub fn poll<'a, 'b: 'a>(&'a mut self) -> Option<Event<'b>> {
        unsafe {
            let mut event = mem::uninitialized();
            xlib::XNextEvent(self.display, &mut event);

            match event.get_type() {
                xlib::Expose => {
                    self.update_dimensions();

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

    pub fn put_str(&mut self, x: usize, y: usize, string: &str, i: usize) {
        if self.buffer_height <= y {
            return;
        }

        let end = cmp::min(self.buffer_width, string.len() + x);
        let len = (end as i32) - (x as i32);
        if len < 0 {
            return;
        }

        unsafe {
            let x_pos = (x as i32) * (*self.fonts[0]).max_advance_width;
            let y_pos = (y as i32) * (*self.fonts[0]).height;

            let font = self.fonts[i];

            xft::XftDrawRect(
                self.draw,
                &self.colors[0],
                x_pos, y_pos + (*font).descent,
                (len * (*font).max_advance_width) as u32, (*self.fonts[0]).height as u32
            );

            xft::XftDrawString8(
                self.draw,
                &self.colors[1],
                self.fonts[i],
                x_pos, y_pos + (*font).height,
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

