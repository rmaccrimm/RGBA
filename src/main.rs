#![allow(dead_code)]
#![allow(non_snake_case)]

extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

mod gba;
mod util;

fn append_text_column(treeview: &gtk::TreeView, model_index: i32, title: &str) {
    let column = gtk::TreeViewColumn::new();
    let cell = gtk::CellRendererText::new();
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", model_index);
    column.set_title(title);
    treeview.append_column(&column);
}

fn append_toggle_column(treeview: &gtk::TreeView) {
    let toggle_column = gtk::TreeViewColumn::new();
    let cell = gtk::CellRendererToggle::new();
    toggle_column.pack_start(&cell, false);
    treeview.append_column(&toggle_column);
}

pub fn build_debug_window(app: &gtk::Application) {
    let glade_src = include_str!("gba/ui/glade/DebugWindow.glade");
    let builder = gtk::Builder::new_from_string(glade_src);

    let model: gtk::ListStore = builder
        .get_object("liststore1")
        .expect("Failed to load liststore1");
    let entries = &["Michel", "Sara", "Liam", "Zelda", "Neo", "Octopus master"];
    for i in 0..100 {
        let j = i % 6;
        model.insert_with_values(None, &[0, 1], &[&format!("{:08X}", i), &entries[j]]);
    }

    let tree: gtk::TreeView = builder.get_object("tree").expect("Failed to load tree");
    append_toggle_column(&tree);
    append_text_column(&tree, 0, "Address");
    append_text_column(&tree, 1, "Value");
    
    let window: gtk::ApplicationWindow = builder
        .get_object("app_window")
        .expect("Failed to load app_window");
    window.set_application(Some(app));
    window.show_all();
}

// use gba::ui::build_debug_window;

fn main() {
    let application = gtk::Application::new(Some("com.me.what"), Default::default())
        .expect("GTK initialization failed");
    application.connect_activate(|app| {
        build_debug_window(app);
    });
    application.run(&[]);
}
