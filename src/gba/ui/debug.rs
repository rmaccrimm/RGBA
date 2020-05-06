extern crate gio;
extern crate gtk;

use gtk::prelude::*;
use gio::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::gba::GBA;

pub struct DebugWindow {
    mem_model: gtk::ListStore,
    reg_model: gtk::ListStore,
}

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

fn update_models(
    reg_model: &gtk::ListStore,
    mem_model: &gtk::ListStore,
    gba_ref: &Rc<RefCell<GBA>>,
) {
    reg_model.clear();
    let pc = gba_ref.borrow().bus.cpu.PC;
    reg_model.insert_with_values(None, &[0, 1], &[&"PC:", &format!["{:08X}", pc]]);
    mem_model.clear();
    for i in 0u32..100 {
        let val = gba_ref.borrow().bus.mmu.read(i);
        mem_model.insert_with_values(
            None,
            &[0, 1],
            &[&format!("{:08X}", i), &format!("{:08X}", val)],
        );
    }
}

impl DebugWindow {
    pub fn new(application: &gtk::Application, gba_ref: Rc<RefCell<GBA>>) -> Self {
        let glade_src = include_str!("glade/DebugWindow.glade");
        let builder = gtk::Builder::new_from_string(glade_src);

        let col_types = &[String::static_type(), String::static_type()];

        let mem_model = gtk::ListStore::new(col_types);
        let mem_view: gtk::TreeView = builder
            .get_object("memory_view")
            .expect("Failed to load memory view");

        let reg_model = gtk::ListStore::new(col_types);
        let reg_view: gtk::TreeView = builder
            .get_object("register_view")
            .expect("Failed to load register view");

        let window: gtk::ApplicationWindow = builder
            .get_object("app_window")
            .expect("Failed to load app_window");

        let step_button: gtk::Button = builder
            .get_object("step_button")
            .expect("Failed to load step button");

        update_models(&reg_model, &mem_model, &gba_ref);

        // These values all need to be cloned so we can move them into the closure
        let mem_model_clone = mem_model.clone();
        let reg_model_clone = reg_model.clone();
        let gba_ref_clone = gba_ref.clone();
        step_button.connect_clicked(move |_| {
            gba_ref_clone.borrow_mut().step();
            update_models(&reg_model_clone, &mem_model_clone, &gba_ref_clone);
        });

        append_toggle_column(&mem_view);
        append_text_column(&mem_view, 0, "Address");
        append_text_column(&mem_view, 1, "Value");
        mem_view.set_model(Some(&mem_model));

        append_text_column(&reg_view, 0, "");
        append_text_column(&reg_view, 1, "");
        reg_view.set_model(Some(&reg_model));

        window.set_application(Some(application));
        window.show_all();

        DebugWindow {
            mem_model,
            reg_model,
        }
    }
}

