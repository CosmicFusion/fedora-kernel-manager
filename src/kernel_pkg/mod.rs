use crate::{kernel_package_row, KernelBranch, KernelPackage};
use adw::prelude::*;
use duct::cmd;
use glib::*;
use gtk::*;
use std::cell::RefCell;
use std::error::Error;
use std::io::BufRead;
use std::io::BufReader;
use std::process::Command;
use std::process::Stdio;
use std::rc::Rc;
use std::time::*;

pub fn kernel_pkg_page(
    content_stack: &gtk::Stack,
    window: &adw::ApplicationWindow,
    selected_kernel_branch: &Rc<RefCell<KernelBranch>>,
) {
    let selected_kernel_branch_clone0 = selected_kernel_branch.borrow().clone();

    let parse_loading_dialog = adw::MessageDialog::builder()
        .transient_for(window)
        .extra_child(&gtk::Spinner::builder()
        .hexpand(true)
        .valign(Align::Start)
        .halign(Align::Center)
        .spinning(true)
        .height_request(128)
        .width_request(128)
        .build())
        .heading(t!("parse_loading_dialog_heading"))
        .body(t!("parse_loading_dialog_body"))
        .build();

    parse_loading_dialog.present();

    let main_box = gtk::Box::builder()
        .hexpand(true)
        .vexpand(true)
        .orientation(Orientation::Vertical)
        .build();

    let main_label = gtk::Label::builder()
        .label(format!(
            "{} {}", t!("kernel_main_label_label"),
            &selected_kernel_branch_clone0.name
        ))
        .hexpand(true)
        .halign(Align::Center)
        .margin_start(10)
        .margin_end(10)
        .margin_bottom(20)
        .margin_top(20)
        .build();
    main_label.add_css_class("symbolic-accent-bg");
    main_label.add_css_class("size-20-font");

    let main_icon = gtk::Image::builder()
        .pixel_size(48)
        .halign(Align::Start)
        .margin_start(20)
        .margin_end(20)
        .margin_bottom(20)
        .margin_top(20)
        .build();

    main_icon.set_icon_name(Some("tux-settings-symbolic"));

    main_icon.add_css_class("symbolic-accent-bg");

    let main_label_box = gtk::Box::new(Orientation::Horizontal, 0);
    main_label_box.append(&main_icon);
    main_label_box.append(&main_label);

    let searchbar = gtk::SearchEntry::builder()
        .search_delay(500)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .margin_start(15)
        .build();
    searchbar.add_css_class("rounded-all-25");

    content_stack.add_named(
        &main_box,
        Some("kernel_pkg_page"),
    );

    let packages_boxedlist = gtk::ListBox::builder()
        .selection_mode(SelectionMode::None)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .margin_start(15)
        .build();
    packages_boxedlist.add_css_class("boxed-list");
    let rows_size_group = gtk::SizeGroup::new(SizeGroupMode::Both);
    add_package_rows(
        &packages_boxedlist,
        selected_kernel_branch_clone0.db,
        &window,
        &rows_size_group,
        &searchbar,
        &parse_loading_dialog
    );

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
    }));

    window_bottombar.append(&back_button);

    main_box.append(&main_label_box);
    main_box.append(&searchbar);
    main_box.append(&packages_viewport);
    main_box.append(&window_bottombar);

    //parse_loading_dialog.close();
}

fn add_package_rows(
    boxedlist: &gtk::ListBox,
    data: String,
    window: &adw::ApplicationWindow,
    rows_size_group: &gtk::SizeGroup,
    searchbar: &gtk::SearchEntry,
    parse_loading_dialog: &adw::MessageDialog
) {
    let cpu_feature_level: u32 = match get_cpu_feature_level().as_str() {
        "x86-64-v4" => 4,
        "x86-64-v3" => 3,
        "x86-64-v2" => 2,
        _ => 1,
    };

    let (kernel_package_sender, kernel_package_receiver) = async_channel::unbounded();
    let kernel_package_sender = kernel_package_sender.clone();

    let (kernel_package_done_sender, kernel_package_done_receiver) = async_channel::unbounded();
    let kernel_package_done_sender = kernel_package_done_sender.clone();

    std::thread::spawn(move || {
        let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");
        if let serde_json::Value::Array(kernels) = &res["kernels"] {
            for kernel in kernels {
                let kernel_name = kernel["name"].as_str().to_owned().unwrap().to_string();
                let kernel_main_package = kernel["main_package"]
                    .as_str()
                    .to_owned()
                    .unwrap()
                    .to_string();
                let kernel_packages = kernel["packages"].as_str().to_owned().unwrap().to_string();
                let kernel_min_x86_march = kernel["min_x86_march"]
                    .as_str()
                    .to_owned()
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();
                let kernel_package_version = match Command::new(
                    "/usr/lib/fedora-kernel-manager/scripts/generate_package_info.sh",
                )
                    .args(["description", &kernel_main_package])
                    .output()
                {
                    Ok(t) => String::from_utf8(t.stdout).unwrap(),
                    _ => "Error".to_owned(),
                };
                let kernel_description = match Command::new(
                    "/usr/lib/fedora-kernel-manager/scripts/generate_package_info.sh",
                )
                    .args(["description", &kernel_main_package])
                    .output()
                {
                    Ok(t) => String::from_utf8(t.stdout).unwrap(),
                    _ => "Error".to_owned(),
                };

                let kernel_package = KernelPackage{
                    name: kernel_name,
                    main_package: kernel_main_package,
                    packages: kernel_packages,
                    min_x86_march: kernel_min_x86_march,
                    package_version: kernel_package_version,
                    description: kernel_description
                };
                kernel_package_sender.send_blocking(kernel_package).expect("Kernel Package sender channel closed")
            }};
        kernel_package_done_sender.send_blocking(true).expect("Kernel Package done sender channel closed")
    });

    let kernel_package_context = MainContext::default();
    // The main loop executes the asynchronous block
    kernel_package_context.spawn_local(clone!(@strong boxedlist, @strong window, @strong rows_size_group => async move {
        while let Ok(kernel_pkg_status) = kernel_package_receiver.recv().await {
            let kernel_name = kernel_pkg_status.name;
            let kernel_main_package = kernel_pkg_status.main_package;
            let kernel_packages = kernel_pkg_status.packages;
            let kernel_min_x86_march = kernel_pkg_status.min_x86_march;
            let kernel_package_version = kernel_pkg_status.package_version;
            let kernel_description = kernel_pkg_status.description;

            let (log_loop_sender, log_loop_receiver) = async_channel::unbounded();
            let log_loop_sender: async_channel::Sender<String> = log_loop_sender.clone();

            let (log_status_loop_sender, log_status_loop_receiver) = async_channel::unbounded();
            let log_status_loop_sender: async_channel::Sender<bool> =
                log_status_loop_sender.clone();

            let (kernel_status_loop_sender, kernel_status_loop_receiver) =
                async_channel::unbounded();
            let kernel_status_loop_sender: async_channel::Sender<bool> =
                kernel_status_loop_sender.clone();

            let kernel_main_package_clone0 = kernel_main_package.clone();

            std::thread::spawn(move || loop {
                let command_installed_status = Command::new("rpm")
                    .args(["-q", &kernel_main_package_clone0])
                    .output()
                    .unwrap();
                if command_installed_status.status.success() {
                    kernel_status_loop_sender
                        .send_blocking(true)
                        .expect("channel needs to be open")
                } else {
                    kernel_status_loop_sender
                        .send_blocking(false)
                        .expect("channel needs to be open")
                }
                std::thread::sleep(Duration::from_secs(6));
            });

                let kernel_expander_row = kernel_package_row::KernelPackageRow::new();
                kernel_expander_row.set_package(kernel_main_package);
                let kernel_status_icon = gtk::Image::builder()
                    .icon_name("emblem-default")
                    .pixel_size(24)
                    .valign(Align::Center)
                    .visible(false)
                    .tooltip_text(t!("Installed"))
                    .build();
                let kernel_description_label = gtk::Label::builder()
                    .label(&kernel_description)
                    .valign(Align::Center)
                    .build();
                let kernel_content_row = adw::ActionRow::builder().build();
                let kernel_install_button = gtk::Button::builder()
                    .margin_start(5)
                    .margin_top(5)
                    .margin_bottom(5)
                    .valign(gtk::Align::Center)
                    .label(t!("kernel_install_button_label"))
                    .tooltip_text(t!("kernel_install_button_tooltip_text"))
                    .sensitive(false)
                    .build();
                kernel_install_button.add_css_class("suggested-action");
                let kernel_remove_button = gtk::Button::builder()
                    .margin_end(5)
                    .margin_top(5)
                    .margin_bottom(5)
                    .valign(gtk::Align::Center)
                    .label(t!("kernel_uninstall_button_label"))
                    .tooltip_text(t!("kernel_uninstall_button_tooltip_text"))
                    .sensitive(false)
                    .build();
                let kernel_action_box = gtk::Box::builder().homogeneous(true).build();
                kernel_remove_button.add_css_class("destructive-action");
                kernel_expander_row.add_suffix(&kernel_status_icon);
                kernel_expander_row.set_title(&kernel_name);
                kernel_expander_row.set_subtitle(&kernel_package_version);
                kernel_content_row.add_prefix(&kernel_description_label);
                kernel_action_box.append(&kernel_remove_button);
                kernel_action_box.append(&kernel_install_button);
                kernel_content_row.add_suffix(&kernel_action_box);
                kernel_expander_row.add_row(&kernel_content_row);
                rows_size_group.add_widget(&kernel_action_box);
                //
                let kernel_status_loop_context = MainContext::default();
                // The main loop executes the asynchronous block
                kernel_status_loop_context.spawn_local(clone!(@weak kernel_remove_button, @weak kernel_install_button, @strong kernel_status_loop_receiver => async move {
                    while let Ok(kernel_status_state) = kernel_status_loop_receiver.recv().await {
                            if kernel_status_state == true {
                                kernel_status_icon.set_visible(true);
                                kernel_install_button.set_sensitive(false);
                            kernel_remove_button.set_sensitive(true);
                            } else {
                                kernel_status_icon.set_visible(false);
                                kernel_remove_button.set_sensitive(false);
                                kernel_install_button.set_sensitive(true);
                            }
                        }
                    }));
                //
                let kernel_install_log_terminal_buffer = gtk::TextBuffer::builder().build();

                let kernel_install_log_terminal = gtk::TextView::builder()
                    .vexpand(true)
                    .hexpand(true)
                    .editable(false)
                    .buffer(&kernel_install_log_terminal_buffer)
                    .build();

                let kernel_install_log_terminal_scroll = gtk::ScrolledWindow::builder()
                    .width_request(400)
                    .height_request(200)
                    .vexpand(true)
                    .hexpand(true)
                    .child(&kernel_install_log_terminal)
                    .build();

                let kernel_install_dialog = adw::MessageDialog::builder()
                    .transient_for(&window)
                    .hide_on_close(true)
                    .extra_child(&kernel_install_log_terminal_scroll)
                    .width_request(400)
                    .height_request(200)
                    .heading(t!("kernel_install_dialog_heading"))
                    .build();
                kernel_install_dialog.add_response("kernel_install_dialog_ok", &t!("kernel_install_dialog_ok_label").to_string());
                kernel_install_dialog
                    .add_response("kernel_install_dialog_reboot", &t!("kernel_install_dialog_reboot_label").to_string());
                kernel_install_dialog.set_response_appearance(
                    "kernel_install_dialog_reboot",
                    adw::ResponseAppearance::Suggested,
                );
                //

                //
                let log_loop_context = MainContext::default();
                // The main loop executes the asynchronous block
                log_loop_context.spawn_local(clone!(@weak kernel_install_log_terminal_buffer, @weak kernel_install_dialog, @strong log_loop_receiver => async move {
                while let Ok(state) = log_loop_receiver.recv().await {
                    kernel_install_log_terminal_buffer.insert(&mut kernel_install_log_terminal_buffer.end_iter(), &("\n".to_string() + &state))
                }
                }));

                let log_status_loop_context = MainContext::default();
                // The main loop executes the asynchronous block
                log_status_loop_context.spawn_local(clone!(@weak kernel_install_dialog, @strong log_status_loop_receiver => async move {
                        while let Ok(state) = log_status_loop_receiver.recv().await {
                            if state == true {
                                kernel_install_dialog.set_response_enabled("kernel_install_dialog_ok", true);
                                //if get_current_username().unwrap() == "pikaos" {
                                //    kernel_install_dialog.set_response_enabled("kernel_install_dialog_reboot", false);
                                //} else {
                                //    kernel_install_dialog.set_response_enabled("kernel_install_dialog_reboot", true);
                                //}
                                kernel_install_dialog.set_response_enabled("kernel_install_dialog_reboot", true);
                                kernel_install_dialog.set_body(&t!("kernel_install_dialog_body_successful").to_string());
                            } else {
                                kernel_install_dialog.set_response_enabled("kernel_install_dialog_ok", true);
                                kernel_install_dialog.set_body(&t!("kernel_install_dialog_body_failed").to_string());
                                kernel_install_dialog.set_response_enabled("kernel_install_dialog_reboot", false);
                            }
                        }
                }));
                //
                kernel_install_log_terminal_buffer.connect_changed(clone!(@weak kernel_install_log_terminal, @weak kernel_install_log_terminal_buffer,@weak kernel_install_log_terminal_scroll => move |_|{
                   if kernel_install_log_terminal_scroll.vadjustment().upper() - kernel_install_log_terminal_scroll.vadjustment().value() > 100.0 {
                        kernel_install_log_terminal_scroll.vadjustment().set_value(kernel_install_log_terminal_scroll.vadjustment().upper())
                    }
                }));
                //
                kernel_install_button.connect_clicked(clone!(@weak kernel_install_log_terminal,@weak kernel_install_log_terminal_buffer, @weak kernel_install_dialog, @strong log_loop_sender, @strong log_status_loop_sender, @strong kernel_packages => move |_| {
                    kernel_install_log_terminal_buffer.delete(&mut kernel_install_log_terminal_buffer.bounds().0, &mut kernel_install_log_terminal_buffer.bounds().1);
                    kernel_install_dialog.set_response_enabled("kernel_install_dialog_ok", false);
                    kernel_install_dialog.set_response_enabled("kernel_install_dialog_reboot", false);
                    kernel_install_dialog.set_body("");
                    kernel_install_dialog.choose(None::<&gio::Cancellable>, move |choice| {
                    if choice == "kernel_install_dialog_reboot" {
                            Command::new("systemctl")
                            .arg("reboot")
                            .spawn()
                            .expect("systemctl reboot failed to start");
                    }
                    });
                    let log_status_loop_sender_clone = log_status_loop_sender.clone();
                    let log_loop_sender_clone= log_loop_sender.clone();
                    let kernel_packages_clone = kernel_packages.clone();
                            std::thread::spawn(move || {
                            let command = kernel_modify(log_loop_sender_clone, &kernel_packages_clone);
                            match command {
                                Ok(_) => {
                                    println!("{}", t!("log_status_kernel_modify_successful"));
                                    log_status_loop_sender_clone.send_blocking(true).expect("The channel needs to be open.");
                                }
                                Err(_) => {
                                    println!("{}",  t!("log_status_kernel_modify_failed"));
                                    log_status_loop_sender_clone.send_blocking(false).expect("The channel needs to be open.");
                                }
                            }
                    });
                }));
                kernel_remove_button.connect_clicked(clone!(@weak kernel_install_log_terminal,@weak kernel_install_log_terminal_buffer, @weak kernel_install_dialog, @strong log_loop_sender, @strong log_status_loop_sender, @strong kernel_packages  => move |_| {
                    kernel_install_log_terminal_buffer.delete(&mut kernel_install_log_terminal_buffer.bounds().0, &mut kernel_install_log_terminal_buffer.bounds().1);
                    kernel_install_dialog.set_response_enabled("kernel_install_dialog_ok", false);
                    kernel_install_dialog.set_response_enabled("kernel_install_dialog_reboot", false);
                    kernel_install_dialog.set_body("");
                    kernel_install_dialog.choose(None::<&gio::Cancellable>, move |choice| {
                    if choice == "kernel_install_dialog_reboot" {
                            Command::new("systemctl")
                            .arg("reboot")
                            .spawn()
                            .expect("systemctl reboot failed to start");
                    }
                    });
                    let log_status_loop_sender_clone = log_status_loop_sender.clone();
                    let log_loop_sender_clone= log_loop_sender.clone();
                    let kernel_packages_clone = kernel_packages.clone();
                            std::thread::spawn(move || {
                            let command = kernel_modify(log_loop_sender_clone, &kernel_packages_clone);
                            match command {
                                Ok(_) => {
                                    println!("{}", t!("log_status_kernel_modify_successful"));
                                    log_status_loop_sender_clone.send_blocking(true).expect("The channel needs to be open.");
                                }
                                Err(_) => {
                                    println!("{}", t!("log_status_kernel_modify_failed"));
                                    log_status_loop_sender_clone.send_blocking(false).expect("The channel needs to be open.");
                                }
                            }
                    });
                }));
                if cpu_feature_level >= kernel_min_x86_march {
                    boxedlist.append(&kernel_expander_row);
                }
        }
    }));

    let kernel_package_done_context = MainContext::default();
    kernel_package_done_context.spawn_local(clone!(@weak parse_loading_dialog => async move {
        while let Ok(_state) = kernel_package_done_receiver.recv().await {
            parse_loading_dialog.close();
        }
    }));

    searchbar.connect_search_changed(clone!(@weak searchbar, @weak boxedlist => move |_| {
        let mut counter = boxedlist.first_child();
        while let Some(row) = counter {
            if row.widget_name() == "KernelPackageRow" {
                if !searchbar.text().is_empty() {
                    if row.property::<String>("title").to_lowercase().contains(&searchbar.text().to_string().to_lowercase()) || row.property::<String>("package").to_lowercase().contains(&searchbar.text().to_string().to_lowercase()) {
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
}

const KERNEL_MODIFY_PROG: &str = r###"
#! /bin/bash
PACKAGE="$0"
pkexec /usr/lib/fedora-kernel-manager/scripts/modify_package.sh "${PACKAGE}"
"###;
fn kernel_modify(
    log_loop_sender: async_channel::Sender<String>,
    kernel_pkg: &str,
) -> Result<(), std::boxed::Box<dyn Error + Send + Sync>> {
    let (pipe_reader, pipe_writer) = os_pipe::pipe()?;
    let child = cmd!("bash", "-c", KERNEL_MODIFY_PROG, kernel_pkg)
        .stderr_to_stdout()
        .stdout_file(pipe_writer)
        .start()?;
    for line in BufReader::new(pipe_reader).lines() {
        log_loop_sender
            .send_blocking(line?)
            .expect("Channel needs to be opened.")
    }
    child.wait()?;

    Ok(())
}

fn get_cpu_feature_level() -> String {
    let base_command = Command::new("/lib64/ld-linux-x86-64.so.2") // `ps` command...
        .arg("--help")
        .env("LANG" ,"en_US")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let grep_command = Command::new("grep")
        .arg("(supported, searched)")
        .stdin(Stdio::from(base_command.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = grep_command.wait_with_output().expect("Output failed");
    let result = match String::from_utf8(output.stdout)
        .expect("stringing failed")
        .lines()
        .next()
    {
        Some(t) => t
            .trim_end_matches("(supported, searched)")
            .trim()
            .to_string(),
        _ => "x86-64-v1".to_string(),
    };
    result
}
