use adw::prelude::ActionRowExt;
use gtk::{Align, IconSize, Orientation, SelectionMode, SizeGroupMode};
use gtk::prelude::{BoxExt, WidgetExt};

pub fn content() -> gtk::Box {

    let content_box = gtk::Box::builder()
        .hexpand(true)
        .vexpand(true)
        .orientation(Orientation::Vertical)
        .build();

    let tux_icon = gtk::Image::builder()
        .pixel_size(128)
        .halign(Align::Center)
        .hexpand(true)
        .margin_start(10)
        .margin_end(10)
        .margin_bottom(20)
        .margin_top(20)
        .build();

    tux_icon.set_icon_name(Some("tux-symbolic"));

    tux_icon.add_css_class("symbolic-accent-bg");

    let kernel_badges_size_group = gtk::SizeGroup::new(SizeGroupMode::Both);

    content_box.append(&create_kernel_badge("Kernel Branch", "cachy", "background-accent-bg", &kernel_badges_size_group));
    content_box.append(&create_kernel_badge("Latest Version", "6.9", "background-accent-bg", &kernel_badges_size_group));
    content_box.append(&create_kernel_badge("Running Version", "6.8.3", "background-salmon-bg", &kernel_badges_size_group));
    content_box.append(&create_kernel_badge("Running Sched", "sched-ext: rusty", "background-accent-bg", &kernel_badges_size_group));
    content_box.append(&tux_icon);

    content_box
}

fn create_kernel_badge(label0_text: &str, label1_text: &str, css_style: &str, group_size: &gtk::SizeGroup) -> gtk::ListBox {
    let badge_box = gtk::Box::builder()
        .build();

    let label0 = gtk::Label::builder()
        .label(label0_text)
        .margin_start(5)
        .margin_end(5)
        .margin_bottom(1)
        .margin_top(1)
        .hexpand(true)
        .vexpand(true)
        .build();

    let label_seprator = gtk::Separator::builder()
        .build();

    let label1 = gtk::Label::builder()
        .label(label1_text)
        .margin_start(3)
        .margin_end(0)
        .margin_bottom(1)
        .margin_top(1)
        .hexpand(true)
        .vexpand(true)
        .build();

    label1.add_css_class(css_style);

    badge_box.append(&label0);
    badge_box.append(&label_seprator);
    badge_box.append(&label1);

    let boxedlist = gtk::ListBox::builder()
        .selection_mode(SelectionMode::None)
        .halign(Align::Center)
        .valign(Align::End)
        .margin_start(5)
        .margin_end(5)
        .margin_bottom(5)
        .margin_top(5)
        .build();

    boxedlist.add_css_class("boxed-list");
    boxedlist.append(&badge_box);
    group_size.add_widget(&boxedlist);
    boxedlist
}