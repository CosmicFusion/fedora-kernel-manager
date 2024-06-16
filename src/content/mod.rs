use gtk::{Align, IconSize};
use gtk::prelude::{BoxExt, WidgetExt};

pub fn content() -> gtk::Box {

    let content_box = gtk::Box::builder()
        .hexpand(true)
        .vexpand(true)
        .build();

    let tux_icon = gtk::Image::builder()
        .pixel_size(128)
        .halign(Align::Center)
        .hexpand(true)
        .build();

    tux_icon.set_icon_name(Some("tux-symbolic"));

    tux_icon.add_css_class("symbolic-accent-bg");

    content_box.append(&tux_icon);

    content_box
}