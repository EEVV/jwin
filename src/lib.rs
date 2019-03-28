extern crate x11;

use std::ffi::CString;
use std::mem;
use std::os::raw::*;
use std::ptr;

use x11::xlib;
use x11::xft;
use x11::xrender;

mod config;

pub struct Term {
    buffer: Vec<String>,
    should_close_readonly: bool,

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
    color: xft::XftColor
}

impl Term {
    pub fn new(title: String) -> Option<Term> {
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
            xlib::XSelectInput(display, window, xlib::ExposureMask);

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

            xlib::XMapWindow(display, window);

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

            let mut color: xft::XftColor = mem::uninitialized();
            
            xft::XftColorAllocValue(
                display,
                visual,
                colormap,
                &xrender::XRenderColor {
                    red: 0xffff,
                    green: 0xffff,
                    blue: 0xffff,
                    alpha: 0xffff
                },
                &mut color
            );

            return Some(Term {
                buffer: vec![
                    String::from("hello"),
                    String::from("world"),
                    String::from("and"),
                    String::from("stuff"),
                    String::from("yes....")
                ],
                should_close_readonly: false,

                display: display,
                visual: visual,
                window: window,

                wm_protocols: wm_protocols,
                wm_delete_window: wm_delete_window,

                font: font,
                draw: draw,
                color: color
            });
        }
    }

    pub fn poll(&mut self) {
        unsafe {
            let mut event = mem::uninitialized();
            xlib::XNextEvent(self.display, &mut event);

            match event.get_type() {
                xlib::Expose => {
                    let mut y = 1;

                    for line in &self.buffer {
                        xft::XftDrawString8(
                            self.draw,
                            &self.color,
                            self.font,
                            0, y * (*self.font).height,
                            CString::new(line.clone()).unwrap().as_ptr() as *const u8,
                            line.len() as i32
                        );

                        y += 1;
                    }
                },
                xlib::ClientMessage => {
                    let xclient = xlib::XClientMessageEvent::from(event);

                    if xclient.message_type == self.wm_protocols && xclient.format == 32 {
                        let protocol = xclient.data.get_long(0) as xlib::Atom;

                        if protocol == self.wm_delete_window {
                            self.should_close_readonly = true;
                        }
                    }
                },
                _ => ()
            }
        }
    }

    pub fn should_close(&self) -> bool {
        self.should_close_readonly
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        unsafe {
            xft::XftColorFree(
                self.display,
                self.visual,
                self.window,
                &mut self.color
            );
            xft::XftDrawDestroy(self.draw);
            xlib::XCloseDisplay(self.display);
        }
    }
}

