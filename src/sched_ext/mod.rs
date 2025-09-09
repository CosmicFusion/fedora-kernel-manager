use crate::content::get_running_kernel_info;
use adw::prelude::*;
use duct::cmd;
use glib::*;
use gtk::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::rc::Rc;
use std::time::Duration;
use std::{fs, io, thread};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ScxSchedMode {
    name: String,
    flags: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ScxSched {
    name: String,
    modes: Vec<ScxSchedMode>,
}

impl ScxSched {
    fn get_sched_from_name(name: &str) -> Option<Self> {
        if name.starts_with("sched_ext:") {
            let name = name.strip_prefix("sched_ext: ").unwrap();
            let data = fs::read_to_string("/usr/lib/pika/kernel-manager/scx_scheds.json")
                .expect("Unable to read file");

            let scx_schedulers: ScxSchedulers = serde_json::from_str(&data).unwrap();

            let scx_scheds: Vec<ScxSched> = scx_schedulers.scx_schedulers;

            scx_scheds.iter().filter(|x| x.name == name).next().cloned()
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ScxSchedulers {
    scx_schedulers: Vec<ScxSched>,
}

pub fn sched_ext_page(
    content_stack: &gtk::Stack,
    window: &adw::ApplicationWindow,
    badge_box: &adw::Bin,
) -> gtk::Box {
    let main_box = gtk::Box::builder()
        .hexpand(true)
        .vexpand(true)
        .orientation(Orientation::Vertical)
        .build();

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
        .label(t!("sched_ext_main_label_label"))
        .hexpand(true)
        .margin_start(10)
        .margin_end(10)
        .margin_bottom(20)
        .margin_top(20)
        .build();
    main_label.add_css_class("symbolic-accent-bg");

    let initial_running_kernel_info = get_running_kernel_info();

    let selected_scx_sched = Rc::new(RefCell::new(
        ScxSched::get_sched_from_name(&initial_running_kernel_info.clone().sched).unwrap_or(
            ScxSched {
                name: "scx_disabled".to_string(),
                modes: Vec::new(),
            },
        ),
    ));

    let cmd_status_dialog = adw::AlertDialog::builder().build();
    cmd_status_dialog.add_response("cmd_status_dialog_ok", "Ok");

    let scx_sched_expander_row = adw::ExpanderRow::builder()
        .subtitle(t!("scx_sched_expander_row_subtitle"))
        .build();

    let scx_sched_mode_expander_row = adw::ExpanderRow::builder()
        .subtitle(t!("scx_sched_mode_expander_row_subtitle"))
        .build();

    let scx_sched_mode_extra_flags_entry_row = adw::EntryRow::builder()
        .title(t!("scx_sched_mode_extra_flags_entry_row_title"))
        .text(get_scx_flags())
        .build();

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
    scx_sched_expander_row_boxedlist.append(&scx_sched_mode_expander_row);
    scx_sched_expander_row_boxedlist.append(&scx_sched_mode_extra_flags_entry_row);

    scx_sched_expander_row.add_row(&scx_sched_expandable(
        &scx_sched_expander_row,
        &scx_sched_mode_expander_row,
        &scx_sched_mode_extra_flags_entry_row,
        &selected_scx_sched,
    ));

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

    back_button.connect_clicked(clone!(
        #[weak]
        content_stack,
        #[weak]
        main_box,
        move |_| {
            content_stack.set_visible_child_name("content_page");
            content_stack.remove(&main_box);
        }
    ));

    let apply_button = gtk::Button::builder()
        .halign(Align::End)
        .label(t!("sched_ext_apply_button_label"))
        .sensitive(false)
        .build();

    apply_button.add_css_class("pill");
    apply_button.add_css_class("destructive-action");

    apply_button.connect_clicked(clone!(
        #[weak]
        selected_scx_sched,
        #[weak]
        scx_sched_mode_extra_flags_entry_row,
        #[strong]
        window,
        move |_| {
            let selected_scx_sched_clone1 = selected_scx_sched.borrow().clone().name;
            let additional_options_flag = &scx_sched_mode_extra_flags_entry_row.text();
            let flags = if additional_options_flag != "" {
                additional_options_flag
            } else {
                ""
            };
            match change_scx_scheduler(&selected_scx_sched_clone1, flags) {
                Ok(_) => {
                    cmd_status_dialog.set_heading(Some(
                        &t!("sched_ext_cmd_status_dialog_heading_success").to_string(),
                    ));
                    cmd_status_dialog.set_body(
                        format!(
                            "{}: {}",
                            t!("sched_ext_cmd_status_dialog_body_success"),
                            &selected_scx_sched_clone1
                        )
                        .as_str(),
                    );
                    cmd_status_dialog.present(Some(&window));
                }
                Err(_) => {
                    cmd_status_dialog.set_heading(Some(
                        &t!("sched_ext_cmd_status_dialog_heading_failed").to_string(),
                    ));
                    cmd_status_dialog.set_body(
                        format!(
                            "{}: {}",
                            t!("sched_ext_cmd_status_dialog_body_failed"),
                            &selected_scx_sched_clone1
                        )
                        .as_str(),
                    );
                    cmd_status_dialog.present(Some(&window));
                }
            };
        }
    ));

    //
    let (loop0_sender, loop0_receiver) = async_channel::unbounded();
    let loop0_sender = loop0_sender.clone();

    std::thread::spawn(move || loop {
        loop0_sender.send_blocking(false).expect("error on loop0");
        thread::sleep(Duration::from_millis(100));
    });

    let loop0_context = MainContext::default();
    // The main loop executes the asynchronous block
    loop0_context.spawn_local(clone!(
        #[weak]
        apply_button,
        #[strong]
        selected_scx_sched,
        #[strong]
        initial_running_kernel_info,
        async move {
            while let Ok(_state) = loop0_receiver.recv().await {
                if *selected_scx_sched.borrow().name == initial_running_kernel_info.sched {
                    apply_button.set_sensitive(false);
                } else {
                    apply_button.set_sensitive(true);
                }
            }
        }
    ));
    //

    window_bottombar.append(&back_button);
    window_bottombar.append(&apply_button);

    main_box.append(badge_box);
    main_box.append(&scx_sched_expander_row_boxedlist);
    main_box.append(&main_icon);
    main_box.append(&main_label);
    main_box.append(&window_bottombar);

    main_box
}

fn scx_sched_expandable(
    expander_row: &adw::ExpanderRow,
    scx_sched_mode_expander_row: &adw::ExpanderRow,
    scx_sched_mode_extra_flags_entry_row: &adw::EntryRow,
    selected_scx_sched: &Rc<RefCell<ScxSched>>,
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
        .label(t!("sched_ext_null_checkbutton_label"))
        .build();

    let data = fs::read_to_string("/usr/lib/fedora-kernel-manager/scx_scheds.json")
        .expect("Unable to read file");

    let scx_schedulers: ScxSchedulers = serde_json::from_str(&data).unwrap();

    let (modes_expandable_listbox, modes_boxedlist, modes_null_check) =
        scx_sched_modes_expandable();

    scx_sched_mode_expander_row.add_row(&modes_expandable_listbox);

    let scx_scheds: Vec<ScxSched> = scx_schedulers.scx_schedulers;
    for sched in scx_scheds {
        let sched_name = &sched.name;
        let sched_checkbutton = gtk::CheckButton::builder()
            .valign(Align::Center)
            .can_focus(false)
            .build();
        let branch_row = adw::ActionRow::builder()
            .activatable_widget(&sched_checkbutton)
            .title(sched_name)
            .subtitle(t!(&format!("{}_description", sched_name)))
            .build();
        branch_row.add_prefix(&sched_checkbutton);
        sched_checkbutton.set_group(Some(&null_checkbutton));
        sched_container.append(&branch_row);
        sched_checkbutton.connect_toggled(clone!(
            #[weak]
            sched_checkbutton,
            #[weak]
            expander_row,
            #[strong]
            selected_scx_sched,
            #[strong]
            sched,
            #[strong]
            modes_boxedlist,
            #[strong]
            modes_null_check,
            #[strong]
            scx_sched_mode_extra_flags_entry_row,
            move |_| {
                let sched_clone = sched.clone();
                if sched_checkbutton.is_active() == true {
                    scx_sched_mode_extra_flags_entry_row.set_text("");
                    expander_row.set_title(&branch_row.title());
                    *selected_scx_sched.borrow_mut() = sched_clone;
                    while let Some(child) = modes_boxedlist.last_child() {
                        modes_boxedlist.remove(&child)
                    }
                    let null_mode_row = adw::ActionRow::builder()
                        .activatable_widget(&modes_null_check)
                        .title(t!("sched_ext_mode_default_checkbutton_label"))
                        .subtitle(t!("default_mode_description"))
                        .build();
                    null_mode_row.add_prefix(&modes_null_check);
                    modes_boxedlist.append(&null_mode_row);
                    for mode in sched.modes.clone() {
                        let mode_checkbutton = gtk::CheckButton::builder()
                            .valign(Align::Center)
                            .can_focus(false)
                            .build();
                        let mode_row = adw::ActionRow::builder()
                            .activatable_widget(&mode_checkbutton)
                            .title(&mode.name)
                            .subtitle(t!(&format!("{}_mode_description", &mode.name)).to_string())
                            .build();
                        mode_row.add_prefix(&mode_checkbutton);
                        mode_checkbutton.set_group(Some(&modes_null_check));
                        modes_boxedlist.append(&mode_row);
                        mode_checkbutton.connect_toggled(clone!(
                            #[strong]
                            scx_sched_mode_extra_flags_entry_row,
                            #[weak]
                            mode_checkbutton,
                            move |_| {
                                if mode_checkbutton.is_active() {
                                    let flag = &mode.flags;
                                    scx_sched_mode_extra_flags_entry_row.set_text(flag);
                                }
                            }
                        ));
                        modes_null_check.connect_toggled(clone!(
                            #[strong]
                            scx_sched_mode_extra_flags_entry_row,
                            #[weak]
                            modes_null_check,
                            move |_| {
                                if modes_null_check.is_active() {
                                    scx_sched_mode_extra_flags_entry_row.set_text("");
                                }
                            }
                        ));
                    }
                }
            }
        ));
        if format!("scx_{}", get_current_scx_scheduler()).as_str() == sched_name {
            sched_checkbutton.set_active(true)
        }
    }
    let branch_container_viewport = gtk::ScrolledWindow::builder()
        .child(&sched_container)
        .hscrollbar_policy(PolicyType::Never)
        .height_request(160)
        .build();

    sched_container.add_css_class("round-border-only-bottom");

    boxedlist.append(&branch_container_viewport);

    searchbar.connect_search_changed(clone!(
        #[weak]
        searchbar,
        #[weak]
        sched_container,
        move |_| {
            let mut counter = sched_container.first_child();
            while let Some(row) = counter {
                if row.widget_name() == "AdwActionRow" {
                    if !searchbar.text().is_empty() {
                        if row
                            .property::<String>("subtitle")
                            .to_lowercase()
                            .contains(&searchbar.text().to_string().to_lowercase())
                            || row
                                .property::<String>("title")
                                .to_lowercase()
                                .contains(&searchbar.text().to_string().to_lowercase())
                        {
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
        }
    ));

    boxedlist
}

fn scx_sched_modes_expandable() -> (ListBox, ListBox, CheckButton) {
    let searchbar = gtk::SearchEntry::builder().search_delay(500).build();
    searchbar.add_css_class("round-border-only-top");

    let boxedlist = gtk::ListBox::builder()
        .selection_mode(SelectionMode::None)
        .build();

    boxedlist.append(&searchbar);

    let mode_boxedlist = gtk::ListBox::builder()
        .selection_mode(SelectionMode::None)
        .build();
    mode_boxedlist.add_css_class("boxed-list");

    let null_checkbutton = gtk::CheckButton::builder().build();

    let branch_container_viewport = gtk::ScrolledWindow::builder()
        .child(&mode_boxedlist)
        .hscrollbar_policy(PolicyType::Never)
        .height_request(160)
        .build();

    mode_boxedlist.add_css_class("round-border-only-bottom");

    boxedlist.append(&branch_container_viewport);

    searchbar.connect_search_changed(clone!(
        #[weak]
        searchbar,
        #[weak]
        mode_boxedlist,
        move |_| {
            let mut counter = mode_boxedlist.first_child();
            while let Some(row) = counter {
                if row.widget_name() == "AdwActionRow" {
                    if !searchbar.text().is_empty() {
                        if row
                            .property::<String>("subtitle")
                            .to_lowercase()
                            .contains(&searchbar.text().to_string().to_lowercase())
                            || row
                                .property::<String>("title")
                                .to_lowercase()
                                .contains(&searchbar.text().to_string().to_lowercase())
                        {
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
        }
    ));

    (boxedlist, mode_boxedlist, null_checkbutton)
}

fn get_current_scx_scheduler() -> String {
    let scx_sched = match fs::read_to_string("/sys/kernel/sched_ext/root/ops") {
        Ok(t) => t,
        Err(_) => "disabled".to_string(),
    };

    scx_sched
}

fn change_scx_scheduler(scx_sched_name: &str, scx_sched_options: &str) -> Result<(), io::Error> {
    cmd!(
        "pkexec",
        "/usr/lib/fedora-kernel-manager/scripts/change_scx.sh",
        scx_sched_name,
        scx_sched_options
    )
    .run()?;
    Ok(())
}

fn get_scx_flags() -> String {
    let file_path = "/etc/default/scx";
    let path = Path::new(file_path);
    let file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return String::new(), // Return empty if file doesn't exist
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue, // Skip if line can't be read
        };

        let trimmed = line.trim();

        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Check if this is the SCX_FLAGS line
        if trimmed.starts_with("SCX_FLAGS=") {
            // Split on the first '=' and get the value part
            let parts: Vec<&str> = trimmed.splitn(2, '=').collect();
            if parts.len() == 2 {
                return parts[1].trim().to_string();
            }
        }
    }

    String::new() // Return empty if no uncommented SCX_FLAGS found
}
