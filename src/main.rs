#![allow(dead_code)]
#![allow(non_snake_case)]

extern crate gtk;

mod gba;
mod util;

use gio::prelude::*;
use gtk::Application;
use std::cell::RefCell;
use std::rc::Rc;
use std::env;

use gba::GBA;
use gba::ui::debug::DebugWindow;


fn main() {
    let gba = Rc::new(RefCell::new(GBA::new()));
    
    let args: Vec<String> = env::args().collect();
    if args.len() > 0 {
        gba.borrow_mut().load_rom(&args[0]);
    }

    let application = Application::new(Some("rgba.emu"), Default::default())
        .expect("GTK initialization failed");
    application.connect_activate(move |app| {
        DebugWindow::new(app, gba.clone());
    });
    application.run(&[]);
}

