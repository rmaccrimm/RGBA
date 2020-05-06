extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::gba::GBA;

#[derive(Clone)]
struct MemoryView {
    model: gtk::ListStore
}

#[derive(Clone)]
struct RegisterView {
    model: gtk::ListStore
}

pub struct DebugWindow {
    mem_view: MemoryView,
    reg_view: RegisterView,
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

impl MemoryView {
    pub fn new(tview: &gtk::TreeView) -> Self {
        let model = gtk::ListStore::new(&[String::static_type(); 3]);
        append_toggle_column(&tview);
        append_text_column(&tview, 0, "Address");
        append_text_column(&tview, 1, "Value");
        append_text_column(&tview, 2, "Instruction");
        tview.set_model(Some(&model));
        MemoryView { model }
    }

    pub fn update(&self, gba: &Rc<RefCell<GBA>>) {
        self.model.clear();
        for i in 0u32..100 {
            let val = gba.borrow().bus.mmu.read(i);
            self.model.insert_with_values(
                None,
                &[0, 1, 2],
                &[&format!("{:08X}", i), &format!("{:08X}", val), &""],
            );
        }
    }
}

impl RegisterView {
    pub fn new(tview: &gtk::TreeView) -> Self {
        let model = gtk::ListStore::new(&[String::static_type(); 2]);
        append_text_column(&tview, 0, "");
        append_text_column(&tview, 1, "");
        tview.set_model(Some(&model));
        RegisterView{ model }
    }

    pub fn update(&self, gba: &Rc<RefCell<GBA>>) {
        let pc = gba.borrow().bus.cpu.PC;
        self.model.clear();
        self.model.insert_with_values(None, &[0, 1], &[&"PC:", &format!["{:08X}", pc]]);
    }
}

impl DebugWindow {
    pub fn new(application: &gtk::Application, gba: Rc<RefCell<GBA>>) -> Self {
        let glade_src = include_str!("glade/DebugWindow.glade");
        let builder = gtk::Builder::new_from_string(glade_src);

        let mem_tree: gtk::TreeView = builder
            .get_object("memory_view")
            .expect("Failed to load memory view");
        let mem_view = MemoryView::new(&mem_tree);
        mem_view.update(&gba);

        let reg_tree: gtk::TreeView = builder
            .get_object("register_view")
            .expect("Failed to load register view");
        let reg_view = RegisterView::new(&reg_tree);
        reg_view.update(&gba);

        let window: gtk::ApplicationWindow = builder
            .get_object("app_window")
            .expect("Failed to load app_window");

        let step_button: gtk::Button = builder
            .get_object("step_button")
            .expect("Failed to load step button");

        // These values all need to be cloned so we can move them into the closure
        let mem_view_clone = mem_view.clone();
        let reg_view_clone = reg_view.clone();
        let gba_ref_clone = gba.clone();
        step_button.connect_clicked(move |_| {
            gba_ref_clone.borrow_mut().step();
            mem_view_clone.update(&gba_ref_clone);
            &reg_view_clone.update(&gba_ref_clone);
        });

        window.set_application(Some(application));
        window.show_all();
        DebugWindow {
            mem_view,
            reg_view,
        }
    }
}
