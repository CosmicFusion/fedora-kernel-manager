use glib::*;
use adw::prelude::*;
use gtk::*;
use gtk::prelude::*;
use crate::content::get_running_kernel_info;
use crate::RunningKernelInfo;

pub fn sched_ext_page(content_stack: &gtk::Stack) -> gtk::Box {
    let main_box = gtk::Box::builder()
        .hexpand(true)
        .vexpand(true)
        .orientation(Orientation::Vertical)
        .build();

    let kernel_badges_size_group = gtk::SizeGroup::new(SizeGroupMode::Both);
    let kernel_badges_size_group0 = gtk::SizeGroup::new(SizeGroupMode::Both);
    let kernel_badges_size_group1 = gtk::SizeGroup::new(SizeGroupMode::Both);

    let main_icon = gtk::Image::builder()
        .pixel_size(128)
        .halign(Align::Center)
        .hexpand(true)
        .margin_start(10)
        .margin_end(10)
        .margin_bottom(20)
        .margin_top(20)
        .build();

    main_icon.set_icon_name(Some("tux-symbolic"));

    main_icon.add_css_class("symbolic-accent-bg");

    let badge_box = gtk::Box::builder()
        .hexpand(true)
        .vexpand(true)
        .orientation(Orientation::Vertical)
        .build();

    create_current_sched_badge(&badge_box, &get_running_kernel_info(), &kernel_badges_size_group, &kernel_badges_size_group0, &kernel_badges_size_group1);

    main_box.append(&badge_box);
    main_box.append(&main_icon);

    main_box
}

fn create_current_sched_badge(badge_box: &gtk::Box, running_kernel_info: &RunningKernelInfo, kernel_badges_size_group: &gtk::SizeGroup, kernel_badges_size_group0: &gtk::SizeGroup, kernel_badges_size_group1: &gtk::SizeGroup) {
    while let Some(widget) = badge_box.last_child() {
        badge_box.remove(&widget);
    }

    badge_box.append(&crate::content::create_kernel_badge("Running Sched", &running_kernel_info.sched, "background-accent-bg", &kernel_badges_size_group, &kernel_badges_size_group0, &kernel_badges_size_group1));
}