use std::cell::RefCell;
use crate::content::get_running_kernel_info;
use crate::{KernelBranch, RunningKernelInfo};
use adw::prelude::*;
use glib::*;
use gtk::prelude::*;
use gtk::*;
use std::fs;
use std::fs::*;
use std::rc::Rc;

pub fn kernel_pkg_page(content_stack: &gtk::Stack, selected_kernel_branch: &Rc<RefCell<KernelBranch>>) -> gtk::Box {
    let selected_kernel_branch_clone0 = selected_kernel_branch.borrow().clone();

    let main_box = gtk::Box::builder()
        .hexpand(true)
        .vexpand(true)
        .orientation(Orientation::Vertical)
        .build();

    let main_label = gtk::Label::builder()
        .label(format!("Available Kernel Packages for {}", &selected_kernel_branch_clone0.name))
        .hexpand(true)
        .margin_start(10)
        .margin_end(10)
        .margin_bottom(20)
        .margin_top(20)
        .build();
    main_label.add_css_class("symbolic-accent-bg");
    main_label.add_css_class("size-20-font");

    let searchbar = gtk::SearchEntry::builder()
        .search_delay(500)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .margin_start(15)
        .build();
    searchbar.add_css_class("rounded-all-25");

    let packages_boxedlist = gtk::ListBox::builder()
        .selection_mode(SelectionMode::None)
        .build();
    packages_boxedlist.add_css_class("boxed-list");
    add_package_rows(&packages_boxedlist, &selected_kernel_branch_clone0.db);

    let packages_viewport = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vexpand(true)
        .hexpand(true)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .margin_start(15)
        .height_request(390)
        .child(&packages_boxedlist)
        .build();

    let window_bottombar = gtk::Box::builder()
        .hexpand(true)
        .homogeneous(true)
        .vexpand(true)
        .valign(Align::End)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .margin_start(15)
        .build();

    let back_button = gtk::Button::builder()
        .halign(Align::Start)
        .label("Back")
        .build();

    back_button.add_css_class("pill");

    back_button.connect_clicked(clone!(@weak content_stack, @weak main_box => move |_| {
        content_stack.set_visible_child_name("content_page");
        content_stack.remove(&main_box);
    }));

    window_bottombar.append(&back_button);

    main_box.append(&main_label);
    main_box.append(&searchbar);
    main_box.append(&packages_viewport);
    main_box.append(&window_bottombar);

    main_box
}

fn add_package_rows(boxedlist: &gtk::ListBox, data: &str) {
    let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");
    if let serde_json::Value::Array(scheds) = &res["scx_schedulers"] {
        for sched in scheds {