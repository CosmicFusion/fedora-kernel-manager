use crate::content::get_running_kernel_info;
use crate::{KernelBranch, RunningKernelInfo};
use adw::prelude::*;
use duct::cmd;
use glib::*;
use gtk::prelude::*;
use gtk::AccessibleRole::Command;
use gtk::*;
use std::cell::RefCell;
use std::fs::*;
use std::process::Stdio;
use std::rc::Rc;
use std::time::Duration;
use std::{fs, io, thread};

pub fn sched_ext_page(content_stack: &gtk::Stack, window: &adw::ApplicationWindow) -> gtk::Box {
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

    main_icon.set_icon_name(Some("tux-settings-symbolic"));

    main_icon.add_css_class("symbolic-accent-bg");

    let main_label = gtk::Label::builder()
        .label("Sched-EXT Configuration Settings")
        .hexpand(true)
        .margin_start(10)
        .margin_end(10)
        .margin_bottom(20)
        .margin_top(20)
        .build();
    main_label.add_css_class("symbolic-accent-bg");

    let badge_box = gtk::Box::builder()
        .hexpand(true)
        .valign(Align::Start)
        .orientation(Orientation::Vertical)
        .build();

    let initial_running_kernel_info = get_running_kernel_info();

    create_current_sched_badge(
        &badge_box,
        &initial_running_kernel_info,
        &kernel_badges_size_group,
        &kernel_badges_size_group0,
        &kernel_badges_size_group1,
    );

    let selected_scx_sched = Rc::new(RefCell::new(initial_running_kernel_info.clone().sched));

    let cmd_status_dialog = adw::MessageDialog::builder()
        .transient_for(window)
        .hide_on_close(true)
        .build();
    cmd_status_dialog.add_response("cmd_status_dialog_ok", "Ok");

    let scx_sched_expander_row = adw::ExpanderRow::builder()
        .subtitle("Select Sched-EXT Scheduler")
        .build();

    scx_sched_expander_row.add_row(&scx_sched_expandable(
        &scx_sched_expander_row,
        &selected_scx_sched,
    ));

    let scx_sched_expander_row_boxedlist = gtk::ListBox::builder()
        .selection_mode(SelectionMode::None)
        .hexpand(true)
        .vexpand(true)
        .valign(Align::Start)
        .halign(Align::Center)
        .margin_start(10)
        .margin_end(10)
        .margin_bottom(20)
        .margin_top(20)
        .build();

    scx_sched_expander_row_boxedlist.add_css_class("boxed-list");
    scx_sched_expander_row_boxedlist.append(&scx_sched_expander_row);

    let window_bottombar = gtk::Box::builder()
        .hexpand(true)
        .vexpand(true)
        .valign(Align::End)
        .homogeneous(true)
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

    let apply_button = gtk::Button::builder()
        .halign(Align::End)
        .label("Apply Changes")
        .sensitive(false)
        .build();

    apply_button.add_css_class("pill");
    apply_button.add_css_class("destructive-action");

    apply_button.connect_clicked(clone! (@weak badge_box, @weak kernel_badges_size_group, @weak kernel_badges_size_group0, @weak kernel_badges_size_group1, @weak selected_scx_sched => move |_| {
        let selected_scx_sched_clone1 = selected_scx_sched.borrow().clone();

        match change_scx_scheduler(&selected_scx_sched_clone1,
                                   &badge_box,
                                   &kernel_badges_size_group,
                                   &kernel_badges_size_group0,
                                   &kernel_badges_size_group1,) {
            Ok(_) => {
                cmd_status_dialog.set_heading(Some("Success!"));
                cmd_status_dialog.set_body(format!("SCX has been set to: {}", &selected_scx_sched_clone1).as_str());
                cmd_status_dialog.present()

            }
            Err(_) => {
                cmd_status_dialog.set_heading(Some("Failed!"));
                cmd_status_dialog.set_body(format!("SCX couldn't be has been set to: {}", &selected_scx_sched_clone1).as_str());
                cmd_status_dialog.present()
            }
        };
    }));

    //
    let (loop0_sender, loop0_receiver) = async_channel::unbounded();
    let loop0_sender = loop0_sender.clone();

    std::thread::spawn(move || loop {
        loop0_sender.send_blocking(false).expect("error on loop0");
        thread::sleep(Duration::from_millis(100));
    });

    let loop0_context = MainContext::default();
    // The main loop executes the asynchronous block
    loop0_context.spawn_local(clone!(@weak apply_button, @strong selected_scx_sched, @strong initial_running_kernel_info => async move {
        while let Ok(_state) = loop0_receiver.recv().await {
            if *selected_scx_sched.borrow() == initial_running_kernel_info.sched {
                apply_button.set_sensitive(false);
            } else {
                apply_button.set_sensitive(true);
            }
        }
    }));
    //

    window_bottombar.append(&back_button);
    window_bottombar.append(&apply_button);

    main_box.append(&badge_box);
    main_box.append(&scx_sched_expander_row_boxedlist);
    main_box.append(&main_icon);
    main_box.append(&main_label);
    main_box.append(&window_bottombar);

    main_box
}

fn create_current_sched_badge(
    badge_box: &gtk::Box,
    running_kernel_info: &RunningKernelInfo,
    kernel_badges_size_group: &gtk::SizeGroup,
    kernel_badges_size_group0: &gtk::SizeGroup,
    kernel_badges_size_group1: &gtk::SizeGroup,
) {
    while let Some(widget) = badge_box.last_child() {
        badge_box.remove(&widget);
    }

    badge_box.append(&crate::content::create_kernel_badge(
        "Running Sched",
        &running_kernel_info.sched,
        "background-accent-bg",
        &kernel_badges_size_group,
        &kernel_badges_size_group0,
        &kernel_badges_size_group1,
    ));
}

fn scx_sched_expandable(
    expander_row: &adw::ExpanderRow,
    selected_scx_sched: &Rc<RefCell<String>>,
) -> gtk::ListBox {
    let searchbar = gtk::SearchEntry::builder().search_delay(500).build();
    searchbar.add_css_class("round-border-only-top");

    let boxedlist = gtk::ListBox::builder()
        .selection_mode(SelectionMode::None)
        .build();

    boxedlist.append(&searchbar);

    let sched_container = gtk::ListBox::builder()
        .selection_mode(SelectionMode::None)
        .build();
    sched_container.add_css_class("boxed-list");

    let null_checkbutton = gtk::CheckButton::builder()
        .label("No branch selected")
        .build();

    let data = fs::read_to_string("/usr/lib/fedora-kernel-manager/scx_scheds.json")
        .expect("Unable to read file");
    let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");
    if let serde_json::Value::Array(scheds) = &res["scx_schedulers"] {
        for sched in scheds {
            let sched = sched["name"].as_str().to_owned().unwrap().to_string();
            let sched_clone0 = sched.clone();
            let sched_checkbutton = gtk::CheckButton::builder()
                .valign(Align::Center)
                .can_focus(false)
                .build();
            let branch_row = adw::ActionRow::builder()
                .activatable_widget(&sched_checkbutton)
                .title(&sched)
                .build();
            branch_row.add_prefix(&sched_checkbutton);
            sched_checkbutton.set_group(Some(&null_checkbutton));
            sched_container.append(&branch_row);
            sched_checkbutton.connect_toggled(
                clone!(@weak sched_checkbutton, @weak expander_row, @strong selected_scx_sched => move |_| {
                    if sched_checkbutton.is_active() == true {
                        expander_row.set_title(&branch_row.title());
                        *selected_scx_sched.borrow_mut() = sched.to_string();
                    }
                }),
            );
            if format!("scx_{}", get_current_scx_scheduler()).as_str() == sched_clone0 {
                sched_checkbutton.set_active(true)
            }
        }
    };

    let branch_container_viewport = gtk::ScrolledWindow::builder()
        .child(&sched_container)
        .hscrollbar_policy(PolicyType::Never)
        .height_request(160)
        .build();

    sched_container.add_css_class("round-border-only-bottom");

    boxedlist.append(&branch_container_viewport);

    searchbar.connect_search_changed(clone!(@weak searchbar, @weak sched_container => move |_| {
        let mut counter = sched_container.first_child();
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

fn get_current_scx_scheduler() -> String {
    let scx_sched = match fs::read_to_string("/sys/kernel/sched_ext/root/ops") {
        Ok(t) => t,
        Err(_) => "disabled".to_string(),
    };

    scx_sched
}

fn change_scx_scheduler(
    scx_sched: &str,
    badge_box: &gtk::Box,
    kernel_badges_size_group: &gtk::SizeGroup,
    kernel_badges_size_group0: &gtk::SizeGroup,
    kernel_badges_size_group1: &gtk::SizeGroup,
) -> Result<(), io::Error> {
    cmd!(
        "pkexec",
        "bash",
        "-c",
        format!(
            "/usr/lib/fedora-kernel-manager/scripts/scripts/change_scx.sh {}",
            scx_sched
        )
    )
    .run()?;
    create_current_sched_badge(
        &badge_box,
        &get_running_kernel_info(),
        &kernel_badges_size_group,
        &kernel_badges_size_group0,
        &kernel_badges_size_group1,
    );
    Ok(())
}
