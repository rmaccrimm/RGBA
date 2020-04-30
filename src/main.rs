#![allow(dead_code)]
#![allow(non_snake_case)]

extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{
    ApplicationWindow, CellRendererText, ListStore, Orientation, ScrolledWindow, TreeView,
    TreeViewColumn,
};

mod gba;
mod util;
use gba::GBA;

// use gtk::{ButtonsType, DialogFlags, MessageType, MessageDialog, Window};

fn append_column(tree: &TreeView, id: i32) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();

    column.pack_start(&cell, true);
    // Association of the view's column with the model's `id` column.
    column.add_attribute(&cell, "text", id);
    tree.append_column(&column);
}

fn main() {
    let gba = GBA::new();

    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("GTK initialization failed");

    application.connect_activate(|app| {
        let store = ListStore::new(&[u32::static_type(), String::static_type()]);
        let entries = &["Michel", "Sara", "Liam", "Zelda", "Neo", "Octopus master"];
        for i in 0..100 {
            let j = i % 6;
            store.insert_with_values(None, &[0, 1], &[&(i as u32), &entries[j]]);
        }
        // for (i, entry) in entries.iter().enumerate() {
        // store.insert_with_values(None, &[0, 1], &[&(i as u32 + 1), &entry]);
        // }

        let tree = TreeView::new();
        append_column(&tree, 0);
        append_column(&tree, 1);
        tree.set_model(Some(&store));

        let scrolled = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        scrolled.add(&tree);
        // let vertical_layout = gtk::Box::new(Orientation::Vertical, 0);
        // vertical_layout.add(&tree);

        let window = gtk::ApplicationWindow::new(app);
        window.set_title("Test");
        window.set_border_width(10);
        window.set_position(gtk::WindowPosition::Center);
        window.set_default_size(350, 70);
        window.add(&scrolled);
        window.show_all();
    });

    application.run(&[]);
}
