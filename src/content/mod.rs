use glib::*;
use adw::prelude::*;
use gtk::*;
use gtk::prelude::*;
use std::process::{Command, Stdio};
use crate::{KernelBranch, RunningKernelInfo};
use Vec;
use std::fs;
use std::path::Path;
use adw::ExpanderRow;
use duct::cmd;
use version_compare::Version;

pub fn content() -> gtk::Box {

    let running_kernel_info = get_running_kernel_info();

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

    let kernel_badge_box = gtk::Box::builder()
        .hexpand(true)
        .vexpand(true)
        .orientation(Orientation::Vertical)
        .build();

    let kernel_branch_expander_row = adw::ExpanderRow::builder()
        .subtitle("Kernel Branch")
        .build();

    kernel_branch_expander_row.add_row(&kernel_branch_expandable(&kernel_branch_expander_row));

    let kernel_branch_expander_row_boxedlist = gtk::ListBox::builder()
        .selection_mode(SelectionMode::None)
        .hexpand(true)
        .vexpand(true)
        .halign(Align::Center)
        .margin_start(10)
        .margin_end(10)
        .margin_bottom(20)
        .margin_top(20)
        .build();
    kernel_branch_expander_row_boxedlist.add_css_class("boxed-list");
    kernel_branch_expander_row_boxedlist.append(&kernel_branch_expander_row);

    create_kernel_badges(&kernel_badge_box, &running_kernel_info);

    content_box.append(&kernel_badge_box);
    content_box.append(&tux_icon);
    content_box.append(&kernel_branch_expander_row_boxedlist);

    content_box
}

fn kernel_branch_expandable(expander_row: &adw::ExpanderRow) -> gtk::ListBox {
    let searchbar = gtk::SearchEntry::builder()
        .search_delay(500)
        .build();
    searchbar.add_css_class("round-border-only-top");

    let boxedlist = gtk::ListBox::builder()
        .selection_mode(SelectionMode::None)
        .build();

    boxedlist.append(&searchbar);

    let branch_container = gtk::ListBox::builder()
        .selection_mode(SelectionMode::None)
        .build();
    branch_container.add_css_class("boxed-list");

    let null_checkbutton = gtk::CheckButton::builder()
        .label("No branch selected")
        .build();

    for branch in get_kernel_branches() {
        let branch_checkbutton = gtk::CheckButton::builder()
            .valign(Align::Center)
            .can_focus(false)
            .build();
        let branch_row = adw::ActionRow::builder()
            .activatable_widget(&branch_checkbutton)
            .title(branch.name)
            .build();
        branch_row.add_prefix(&branch_checkbutton);
        branch_checkbutton.set_group(Some(&null_checkbutton));
        branch_container.append(&branch_row);
        branch_checkbutton.connect_toggled(clone!(@weak branch_checkbutton, @weak expander_row => move |_| {
            if branch_checkbutton.is_active() == true {
                expander_row.set_title(&branch_row.title());
            }
        }));
        //if current_keyboard.contains(&(keyboard_layout_clone)) {
        //    keyboard_layout_checkbutton.set_active(true);
        //}
    }

    let branch_container_viewport = gtk::ScrolledWindow::builder()
        .child(&branch_container)
        .hscrollbar_policy(PolicyType::Never)
        .build();

    branch_container.add_css_class("round-border-only-bottom");

    boxedlist.append(&branch_container_viewport);

    searchbar.connect_search_changed(clone!(@weak searchbar, @weak branch_container => move |_| {
        let mut counter = branch_container.first_child();
        while let Some(row) = counter {
            if row.widget_name() == "AdwActionRow" {
                if !searchbar.text().is_empty() {
                    if row.property::<String>("subtitle").to_lowercase().contains(&searchbar.text().to_string().to_lowercase()) || row.property::<String>("title").to_lowercase().contains(&searchbar.text().to_string().to_lowercase()) {
                        //row.grab_focus();
                        //row.add_css_class("highlight-widget");
                        row.set_property("visible", true);
                        searchbar.grab_focus();
                    } else {
                        row.set_property("visible", false);
                    }
                } else {
                    row.set_property("visible", true);
                }
            }
            counter = row.next_sibling();
        }
    }));

    boxedlist
}

fn create_kernel_badge(label0_text: &str, label1_text: &str, css_style: &str, group_size: &gtk::SizeGroup, group_size0: &gtk::SizeGroup, group_size1: &gtk::SizeGroup) -> gtk::ListBox {
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
    group_size0.add_widget(&label0);

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
    group_size1.add_widget(&label1);

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

fn get_kernel_branches() -> Vec<KernelBranch> {
    let test_branch = KernelBranch {
      name: "kernel-cachy".to_string(),
      db: "https://raw.githubusercontent.com/CosmicFusion/fedora-kernel-manager/main/data/db-kernel-cachy.json".to_string()
    };

    let test_branch2 = KernelBranch {
        name: "kernel-cachy".to_string(),
        db: "https://raw.githubusercontent.com/CosmicFusion/fedora-kernel-manager/main/data/db-kernel-cachy.json".to_string()
    };

    vec![test_branch, test_branch2]
}
fn get_running_kernel_info() -> RunningKernelInfo {
    let kernel = match Command::new("uname").arg("-r").stdout(Stdio::piped()).output() {
        Ok(t) =>  String::from_utf8(t.stdout).unwrap().trim().to_owned(),
        Err(_) => "Unknown".to_string()
    };

    let version = match linux_version::linux_kernel_version() {
        Ok(t) => {
            if t.patch == 0 {
                format!("{}.{}", t.major, t.minor)
            } else {
                format!("{}.{}.{}", t.major, t.minor, t.patch)
            }
        }
        Err(_) => "Unknown".to_string()
    };

    let info = RunningKernelInfo {
        kernel: kernel,
        version: version.clone(),
        // didn't find a way to accurately get this, outside of sched-ext (https://github.com/CachyOS/kernel-manager/blob/develop/src/schedext-window.cpp)
        sched: get_current_scheduler(version)
    };

    info
}

fn get_current_scheduler(version: String) -> String {
    if Path::new("/sys/kernel/sched_ext/root/ops").exists() {
        println!("sched_ext is detected, getting scx scheduler");
        let scx_sched = match fs::read_to_string("/sys/kernel/sched_ext/root/ops") {
            Ok(t) => t,
            Err(_) => "unknown!".to_string()
        };
        "sched_ext: ".to_owned() + &scx_sched
    } else if bore_check() {
        "BORE".to_string()
    } else if Version::from(&version) >= Version::from("6.6")  {
        "EEVDF?".to_string()
    } else {
        "CFS?".to_string()
    }
}

fn bore_check() -> bool {
   let is_bore= match cmd!("sysctl", "-n", "kernel.sched_bore").read() {
     Ok(t) => {
         if t == "1" {
             true
         } else {
             false
         }
     }
       Err(_) => false
   };
    is_bore
}

fn create_kernel_badges(badge_box: &gtk::Box, running_kernel_info: &RunningKernelInfo) {
    let kernel_badges_size_group = gtk::SizeGroup::new(SizeGroupMode::Both);
    let kernel_badges_size_group0 = gtk::SizeGroup::new(SizeGroupMode::Both);
    let kernel_badges_size_group1 = gtk::SizeGroup::new(SizeGroupMode::Both);

    let kernel_version = "6.9";

    let version_css_style = if &running_kernel_info.version.as_str() == &kernel_version {
        "background-green-bg"
    }
    else {
        "background-red-bg"
    };

    while let Some(widget) = badge_box.last_child() {
        badge_box.remove(&widget);
    }

    badge_box.append(&create_kernel_badge("Kernel Branch", "cachy", "background-accent-bg", &kernel_badges_size_group, &kernel_badges_size_group0, &kernel_badges_size_group1));
    badge_box.append(&create_kernel_badge("Latest Version", "6.9", "background-accent-bg", &kernel_badges_size_group, &kernel_badges_size_group0, &kernel_badges_size_group1));
    badge_box.append(&create_kernel_badge("Running Version", &running_kernel_info.version, &version_css_style, &kernel_badges_size_group, &kernel_badges_size_group0, &kernel_badges_size_group1));
    badge_box.append(&create_kernel_badge("Running Kernel", &running_kernel_info.kernel, &version_css_style, &kernel_badges_size_group, &kernel_badges_size_group0, &kernel_badges_size_group1));
    badge_box.append(&create_kernel_badge("Running Sched", &running_kernel_info.sched, "background-accent-bg", &kernel_badges_size_group, &kernel_badges_size_group0, &kernel_badges_size_group1));
}