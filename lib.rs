// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[crate_id = "github.com/mozilla-servo/rust-xlib#xlib:0.1"];
#[crate_type = "lib"];

#[feature(globs)];

use std::libc::*;
use std::ptr;
use std::c_str::CString;
pub use xlib::{Atom, Window, Time};

pub mod xlib;

pub struct XTextProperty {
    raw: xlib::XTextProperty
}

impl Drop for XTextProperty {
    fn drop(&mut self) {
        unsafe {
            std::libc::free(self.raw.value as *mut std::libc::c_void);
        }
    }
}

impl XTextProperty {
    fn new() -> XTextProperty {
        XTextProperty {
            raw:
            xlib::XTextProperty {
                value: ptr::null(),
                encoding: 0,
                format: 0,
                nitems: 0
            }
        }
    }

    pub fn value(&self) -> CString {
        unsafe { CString::new(self.raw.value, false) }
    }

    pub fn encoding(&self) -> Atom {
        self.raw.encoding
    }

    pub fn format(&self) -> int {
        self.raw.format as int
    }

    pub fn nitems(&self) -> uint {
        self.raw.nitems as uint
    }
}

pub struct XWindowAttributes {
    raw: xlib::XWindowAttributes
}

impl XWindowAttributes {
    fn new() -> XWindowAttributes {
        XWindowAttributes {
            raw:
            xlib::XWindowAttributes {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
                border_width: 0,
                depth: 0,
                visual: ptr::null(), // do not free
                root: 0,
                _class: 0,
                bit_gravity: 0,
                win_gravity: 0,
                backing_store: 0,
                backing_planes: 0,
                backing_pixel: 0,
                save_under: 0,
                colormap: 0,
                map_installed: 0,
                map_state: 0,
                all_event_masks: 0,
                your_event_mask: 0,
                do_not_propagate_mask: 0,
                override_redirect: 0,
                screen: ptr::null() // do not free
            }
        }
    }

    pub fn map_state(&self) -> int {
        self.raw.map_state as int
    }

    pub fn override_redirect(&self) -> bool {
        self.raw.override_redirect != 0
    }
}


pub struct Display {
    priv raw: *xlib::Display
}

impl Display {
    pub fn open(display_name: Option<~str>) -> Display {
        unsafe {
            let display = xlib::XOpenDisplay(display_name.map_or(ptr::null(), |s| s.to_c_str().as_bytes().as_ptr() as *i8));
            if display.is_null() {
                fail!("cannot open display")
            } else {
                Display { raw: display }
            }
        }
    }

    pub fn close(&mut self) {
        unsafe {
            xlib::XCloseDisplay(self.raw);
        }
    }

    pub fn default_root_window(&self) -> Window {
        unsafe {
            xlib::XDefaultRootWindow(self.raw)
        }
    }

    pub fn get_wm_name(&self, w: Window) -> XTextProperty {
        unsafe {
            let mut prop = XTextProperty::new();
            xlib::XGetWMName(self.raw, w, &mut prop.raw);
            prop
        }
    }

    pub fn intern_atom(&self, atom_name: ~str, only_if_exists: bool) -> Atom {
        unsafe {
            xlib::XInternAtom(self.raw, atom_name.to_c_str().as_bytes().as_ptr() as *i8, only_if_exists as c_char)
        }
    }

    pub fn query_tree(&self, w: Window) -> (Window, Window, Vec<Window>) {
        unsafe {
            let mut wins = ptr::mut_null();
            let mut rw = 0;
            let mut pw = 0;
            let mut n = 0;
            xlib::XQueryTree(self.raw, w, &mut rw, &mut pw, &mut wins, &mut n);
            (rw, pw, Vec::from_raw_parts(n as uint, n as uint, wins))
        }
    }

    pub fn get_text_property(&self, w: Window, property: Atom) -> XTextProperty {
        unsafe {
            let mut prop = XTextProperty::new();
            xlib::XGetTextProperty(self.raw, w, &mut prop.raw, property);
            prop
        }
    }

    pub fn utf8_text_property_to_text_list(&self, prop: &XTextProperty) -> Vec<CString> {
        unsafe {
            let mut list = ptr::mut_null();
            let mut n = 0;
            xlib::Xutf8TextPropertyToTextList(self.raw, &prop.raw, &mut list, &mut n);
            let v = Vec::from_raw_parts(n as uint, n as uint, list);
            let r = v.iter().map(|&s| CString::new(s, true)).collect();
            r
        }
    }

    pub fn get_window_attributes(&self, w: Window) -> XWindowAttributes {
        unsafe {
            let mut wa = XWindowAttributes::new();
            xlib::XGetWindowAttributes(self.raw, w, &mut wa.raw);
            wa
        }
    }

    pub fn create_simple_window(&self,
                                parent: Window,
                                x: int,
                                y: int,
                                width: uint,
                                height: uint,
                                border_width: uint,
                                border: uint,
                                background: uint) -> Window {
        unsafe {
            xlib::XCreateSimpleWindow(self.raw,
                                      parent,
                                      x as c_int,
                                      y as c_int,
                                      width as c_uint,
                                      height as c_uint,
                                      border_width as c_uint,
                                      border as c_ulong,
                                      background as c_ulong)
        }
    }

    pub fn convert_selection(&self, selection: Atom, target: Atom, property: Atom, requestor: Window, time: Time) {
        unsafe {
            xlib::XConvertSelection(self.raw, selection, target, property, requestor, time);
        }
    }

    pub fn next_event(&self) -> xlib::XEvent {
        unsafe {
            let mut ev = xlib::union__XEvent { data: [0, ..24] };
            xlib::XNextEvent(self.raw, &mut ev);
            ev
        }
    }

    pub fn get_window_property(&self, w: Window, property: Atom,
                               offset: int, length: int, delete: bool,
                               reg_type: Atom) -> (Atom, int, uint, uint, CString) {
        unsafe {
            let mut actual_type = 0;
            let mut actual_format = 0;
            let mut nitems = 0;
            let mut bytes_after = 0;
            let mut prop = ptr::mut_null();
            xlib::XGetWindowProperty(self.raw, w, property, offset as c_long,
                                     length as c_long, delete as c_char,
                                     reg_type, &mut actual_type, &mut actual_format,
                                     &mut nitems, &mut bytes_after, &mut prop);

            (actual_type, actual_format as int, nitems as uint, bytes_after as uint,
                CString::new(prop as *i8, true))
        }
    }
}



