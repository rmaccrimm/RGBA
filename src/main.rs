#![allow(dead_code)]
#![allow(non_snake_case)]

extern crate gtk;

mod gba;
mod util;

use gio::prelude::*;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use gba::GBA;
use gba::ui::debug::DebugWindow;


fn main() {
    let gba = Rc::new(RefCell::new(GBA::new()));
    let application = gtk::Application::new(Some("rgba.emu"), Default::default())
        .expect("GTK initialization failed");
    application.connect_activate(move |app| {
        DebugWindow::new(app, gba.clone());
    });
    application.run(&[]);
}
