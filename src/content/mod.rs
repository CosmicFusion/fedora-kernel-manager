use crate::{kernel_pkg, sched_ext, KernelBranch, RunningKernelInfo};
use adw::prelude::*;
use adw::ExpanderRow;
use async_channel::Receiver;
use duct::cmd;
use glib::property::PropertyGet;
use glib::*;
use gtk::prelude::*;
use gtk::*;
use homedir::get_my_home;
use std::cell::RefCell;
use std::path::Path;
use std::process::{Command, Stdio};
use std::rc::Rc;
use std::{fs, time};
use version_compare::Version;
use Vec;

pub fn content(
    content_stack: &gtk::Stack,
    selected_kernel_branch: &Rc<RefCell<KernelBranch>>,
    db_load_complete: &Rc<RefCell<bool>>,
    window: &adw::ApplicationWindow,
    window_banner: &adw::Banner,
) -> gtk::Box {
    let (get_kernel_branches_sender, get_kernel_branches_receiver) = async_channel::unbounded();
    let get_kernel_branches_sender = get_kernel_branches_sender.clone();

    std::thread::spawn(move || {
        get_kernel_branches_sender
            .send_blocking(get_kernel_branches())
            .expect("channel needs to be open.");
    });

    let loading_spinner = gtk::Spinner::builder()
        .hexpand(true)
        .valign(Align::Start)
        .halign(Align::Center)
        .spinning(true)
        .height_request(128)
        .width_request(128)
        .build();

    let loading_label = gtk::Label::builder()
        .hexpand(true)
        .valign(Align::Start)
        .halign(Align::Center)
        .label(t!("loading_label_label"))
        .build();

    let loading_box = gtk::Box::builder()
        .hexpand(true)
        .vexpand(true)
        .orientation(Orientation::Vertical)
        .build();

    loading_box.append(&loading_spinner);
    loading_box.append(&loading_label);

    let content_box = gtk::Box::builder()
        .hexpand(true)
        .vexpand(true)
        .orientation(Orientation::Vertical)
        .sensitive(false)
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

    let sched_ext_badge_box = adw::Bin::builder()
        .hexpand(true)
        .vexpand(true)
        .build();

    let kernel_branch_expander_row = adw::ExpanderRow::builder()
        .subtitle(t!("kernel_branch_expander_row_subtitle"))
        .build();

    kernel_branch_expander_row.add_row(&kernel_branch_expandable(
        &kernel_branch_expander_row,
        &window_banner,
        &loading_box,
        selected_kernel_branch,
        db_load_complete,
        get_kernel_branches_receiver.clone(),
    ));

    let kernel_branch_expander_row_boxedlist = gtk::ListBox::builder()
        .selection_mode(SelectionMode::None)
        .hexpand(true)
        .halign(Align::Center)
        .margin_start(10)
        .margin_end(10)
        .margin_bottom(20)
        .margin_top(20)
        .build();

    let button_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .margin_start(10)
        .margin_end(10)
        .margin_bottom(20)
        .margin_top(20)
        .hexpand(true)
        .halign(Align::Center)
        .build();

    let browse_kernels_button = gtk::Button::builder()
        .icon_name("tux-settings-symbolic")
        .halign(Align::Start)
        .margin_start(10)
        .margin_end(10)
        .height_request(50)
        .width_request(50)
        .tooltip_text(t!("browse_kernels_button_tooltip_text"))
        .hexpand(true)
        .build();
    browse_kernels_button.add_css_class("circular");

    browse_kernels_button.connect_clicked(
        clone!(@weak window, @weak content_stack, @strong selected_kernel_branch => move |_| {
                content_stack.add_named(
            &kernel_pkg::kernel_pkg_page(&content_stack, &window, &selected_kernel_branch),
            Some("kernel_pkg_page"),
        );
            content_stack.set_visible_child_name("kernel_pkg_page")
        }),
    );

    let config_kernel_button = gtk::Button::builder()
        .icon_name("tux-download-symbolic")
        .halign(Align::End)
        .margin_start(10)
        .margin_end(10)
        .height_request(50)
        .width_request(50)
        .tooltip_text(t!("config_kernel_button_tooltip_text"))
        .sensitive(!is_scx_kernel())
        .hexpand(true)
        .build();
    config_kernel_button.add_css_class("circular");

    if !is_scx_kernel() {
        config_kernel_button
            .set_tooltip_text(Some(&t!("config_kernel_button_tooltip_text_no_scx").to_string()));
    }

    config_kernel_button.connect_clicked(clone!(@weak content_stack, @weak window, @weak sched_ext_badge_box => move |_| {
            content_stack.add_named(
        &sched_ext::sched_ext_page(&content_stack, &window, &sched_ext_badge_box),
        Some("sched_ext_page"),
    );
        content_stack.set_visible_child_name("sched_ext_page")
    }));

    button_box.append(&browse_kernels_button);
    button_box.append(&config_kernel_button);

    kernel_branch_expander_row_boxedlist.add_css_class("boxed-list");
    kernel_branch_expander_row_boxedlist.append(&kernel_branch_expander_row);

    content_box.append(&loading_box);
    content_box.append(&kernel_badge_box);
    content_box.append(&tux_icon);
    content_box.append(&kernel_branch_expander_row_boxedlist);
    content_box.append(&button_box);

    let (load_badge_async_sender, load_badge_async_receiver) = async_channel::unbounded();
    let load_badge_async_sender = load_badge_async_sender.clone();
    // The long running operation runs now in a separate thread
    std::thread::spawn(move || loop {
        load_badge_async_sender
            .send_blocking(true)
            .expect("The channel needs to be open.");
        std::thread::sleep(time::Duration::from_secs(5));
    });

    let load_badge_async_context = MainContext::default();
    // The main loop executes the asynchronous block
    load_badge_async_context.spawn_local(clone!(@weak content_box, @weak loading_box, @weak kernel_badge_box, @strong selected_kernel_branch, @strong db_load_complete => async move {
            while let Ok(_state) = load_badge_async_receiver.recv().await {
            if *db_load_complete.borrow() == true {
                let running_kernel_info = get_running_kernel_info();
                create_kernel_badges(&kernel_badge_box, &running_kernel_info, &selected_kernel_branch);
                create_current_sched_badge(&sched_ext_badge_box, &running_kernel_info);
                loading_box.set_visible(false);
                content_box.set_sensitive(true)
            }
            }
    }));

    content_box
}

fn kernel_branch_expandable(
    expander_row: &adw::ExpanderRow,
    window_banner: &adw::Banner,
    loading_box: &gtk::Box,
    selected_kernel_branch: &Rc<RefCell<KernelBranch>>,
    db_load_complete: &Rc<RefCell<bool>>,
    get_kernel_branches_receiver: Receiver<Result<Vec<KernelBranch>, reqwest::Error>>,
) -> gtk::ListBox {
    let searchbar = gtk::SearchEntry::builder().search_delay(500).build();
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
        .label(t!("null_checkbutton_label"))
        .build();

    let get_kernel_branches_loop_context = MainContext::default();
    // The main loop executes the asynchronous block
    get_kernel_branches_loop_context.spawn_local(clone!(@weak expander_row, @weak branch_container, @strong selected_kernel_branch, @weak loading_box, @weak window_banner, @strong db_load_complete => async move {
        while let Ok(data) = get_kernel_branches_receiver.recv().await {
            match data {
                Ok(t) => {
                    for branch in t {
        let branch_clone0 = branch.clone();
        let branch_clone1 = branch.clone();
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
        let selected_kernel_branch_clone0 = selected_kernel_branch.clone();
        branch_checkbutton.connect_toggled(
            clone!(@weak branch_checkbutton, @weak expander_row => move |_| {
                if branch_checkbutton.is_active() == true {
                    expander_row.set_title(&branch_row.title());
                    save_branch_config(&branch_row.title().to_string());
                    *selected_kernel_branch_clone0.borrow_mut()=branch_clone0.clone()
                }
            }),
        );

        match get_my_home().unwrap().unwrap().join(".config/fedora-kernel-manager/branch").exists() {
            true if fs::read_to_string(get_my_home().unwrap().unwrap().join(".config/fedora-kernel-manager/branch")).unwrap()== branch_clone1.name && std::fs::metadata(get_my_home().unwrap().unwrap().join(".config/fedora-kernel-manager/branch")).expect("file metadata not found").len() == 0 =>
            {
                branch_checkbutton.set_active(true)
            }
            _ => branch_container.first_child().unwrap().property::<gtk::CheckButton>("activatable_widget").set_property("active", true),
        };

                *db_load_complete.borrow_mut() = true;
                println!("{}", t!("db_load_complete"))
    }
                }
                _ => {
                    window_banner.set_title(&t!("banner_text_url_error").to_string());
                    window_banner.set_revealed(true);
                    loading_box.set_visible(false);
                }
            }
        }
    }));

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

pub fn create_kernel_badge(
    label0_text: &str,
    label1_text: &str,
    css_style: &str,
    group_size: &gtk::SizeGroup,
    group_size0: &gtk::SizeGroup,
    group_size1: &gtk::SizeGroup,
) -> gtk::ListBox {
    let badge_box = gtk::Box::builder().build();

    let label0 = gtk::Label::builder()
        .label(label0_text)
        .margin_start(5)
        .margin_end(5)
        .margin_bottom(1)
        .margin_top(1)
        .valign(Align::Center)
        .halign(Align::Center)
        .hexpand(true)
        .vexpand(true)
        .build();
    group_size0.add_widget(&label0);

    let label_seprator = gtk::Separator::builder().build();

    let label1 = gtk::Label::builder()
        .label(label1_text)
        .margin_start(3)
        .margin_end(0)
        .margin_bottom(1)
        .margin_top(1)
        .valign(Align::Center)
        .halign(Align::Center)
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

fn get_kernel_branches() -> Result<Vec<KernelBranch>, reqwest::Error> {
    let mut kernel_branches_array: Vec<KernelBranch> = Vec::new();
    let data = fs::read_to_string("/usr/lib/fedora-kernel-manager/kernel_branches.json")
        .expect("Unable to read file");
    let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");
    if let serde_json::Value::Array(branches) = &res["branches"] {
        for branch in branches {
            let branch_name = branch["name"].as_str().to_owned().unwrap().to_string();
            let branch_db_url = branch["db_url"].as_str().to_owned().unwrap().to_string();
            let branch_init_script = branch["init_script"]
                .as_str()
                .to_owned()
                .unwrap()
                .to_string();
            println!("{} {}.",t!("db_downloading"), &branch_name);
            let branch_db =
                reqwest::blocking::get(branch["db_url"].as_str().to_owned().unwrap().to_string())?
                    .text()
                    .unwrap();
            let branch = KernelBranch {
                name: branch_name,
                db_url: branch_db_url,
                init_script: branch_init_script,
                db: branch_db,
            };
            println!("{}", t!("db_download_complete"));
            println!("{} {} {}", t!("db_init_script_run_p1"), &branch.name, t!("db_init_script_run_p2"));
            match cmd!("bash", "-c", &branch.init_script).run() {
                Ok(t) => println!("{} {}", &branch.name, t!("db_init_script_successful")),
                _ => println!("{} {}", &branch.name, t!("db_init_script_failed")),
            };
            kernel_branches_array.push(branch)
        }
    };

    Ok(kernel_branches_array)
}
pub fn get_running_kernel_info() -> RunningKernelInfo {
    let kernel = match Command::new("uname")
        .arg("-r")
        .stdout(Stdio::piped())
        .output()
    {
        Ok(t) => String::from_utf8(t.stdout).unwrap().trim().to_owned(),
        Err(_) => t!("unknown").to_string(),
    };

    let version = match linux_version::linux_kernel_version() {
        Ok(t) => {
            if t.patch == 0 {
                format!("{}.{}", t.major, t.minor)
            } else {
                format!("{}.{}.{}", t.major, t.minor, t.patch)
            }
        }
        Err(_) => t!("unknown").to_string(),
    };

    let info = RunningKernelInfo {
        kernel: kernel,
        version: version.clone(),
        // didn't find a way to accurately get this, outside of sched-ext (https://github.com/CachyOS/kernel-manager/blob/develop/src/schedext-window.cpp)
        sched: get_current_scheduler(version),
    };

    info
}

fn is_scx_kernel() -> bool {
    if Path::new("/sys/kernel/sched_ext/root/ops").exists() {
        true
    } else {
        false
    }
}
pub fn get_current_scheduler(version: String) -> String {
    if is_scx_kernel() {
        println!("{}", t!("get_current_scheduler_sched_ext_detected"));
        let scx_sched = match fs::read_to_string("/sys/kernel/sched_ext/root/ops") {
            Ok(t) => t,
            Err(_) => "disabled".to_string(),
        };
        "sched_ext: ".to_owned() + &scx_sched
    } else if bore_check() {
        "BORE".to_string()
    } else if Version::from(&version) >= Version::from("6.6") {
        "EEVDF?".to_string()
    } else {
        "CFS?".to_string()
    }
}

fn bore_check() -> bool {
    let is_bore = match cmd!("sysctl", "-n", "kernel.sched_bore").read() {
        Ok(t) => {
            if t == "1" {
                true
            } else {
                false
            }
        }
        Err(_) => false,
    };
    is_bore
}

fn create_kernel_badges(
    badge_box: &gtk::Box,
    running_kernel_info: &RunningKernelInfo,
    selected_kernel_branch: &Rc<RefCell<KernelBranch>>,
) {
    let selected_kernel_branch_clone = selected_kernel_branch.borrow().clone();

    let kernel_badges_size_group = gtk::SizeGroup::new(SizeGroupMode::Both);
    let kernel_badges_size_group0 = gtk::SizeGroup::new(SizeGroupMode::Both);
    let kernel_badges_size_group1 = gtk::SizeGroup::new(SizeGroupMode::Both);

    let json: serde_json::Value =
        serde_json::from_str(&selected_kernel_branch_clone.db).expect("Unable to parse");

    let kernel_version = match json["latest_version"].as_str() {
        Some(t) => t,
        _ => "Unknown",
    };

    let version_css_style = if &running_kernel_info.version == &kernel_version {
        "background-green-bg"
    } else {
        "background-red-bg"
    };

    while let Some(widget) = badge_box.last_child() {
        badge_box.remove(&widget);
    }

    badge_box.append(&create_kernel_badge(
        &t!("kernel_badge_branch_label").to_string(),
        &selected_kernel_branch_clone.name,
        "background-accent-bg",
        &kernel_badges_size_group,
        &kernel_badges_size_group0,
        &kernel_badges_size_group1,
    ));
    badge_box.append(&create_kernel_badge(
        &t!("kernel_badge_latest_version_label").to_string(),
        kernel_version,
        "background-accent-bg",
        &kernel_badges_size_group,
        &kernel_badges_size_group0,
        &kernel_badges_size_group1,
    ));
    badge_box.append(&create_kernel_badge(
        &t!("kernel_badge_running_version_label").to_string(),
        &running_kernel_info.version,
        &version_css_style,
        &kernel_badges_size_group,
        &kernel_badges_size_group0,
        &kernel_badges_size_group1,
    ));
    badge_box.append(&create_kernel_badge(
        &t!("kernel_badge_running_kernel_label").to_string(),
        &running_kernel_info.kernel,
        &version_css_style,
        &kernel_badges_size_group,
        &kernel_badges_size_group0,
        &kernel_badges_size_group1,
    ));
    badge_box.append(&create_kernel_badge(
        &t!("kernel_badge_running_sched_label").to_string(),
        &running_kernel_info.sched,
        "background-accent-bg",
        &kernel_badges_size_group,
        &kernel_badges_size_group0,
        &kernel_badges_size_group1,
    ));
}

fn save_branch_config(branch: &str) {
    let config_path = get_my_home()
        .unwrap()
        .unwrap()
        .join(".config/fedora-kernel-manager");
    match &config_path.exists() {
        true => fs::write(config_path.join("branch"), branch).unwrap(),
        _ => {
            fs::create_dir(&config_path).unwrap();
            fs::write(config_path.join("branch"), branch).unwrap();
        }
    }
}

fn create_current_sched_badge(
    badge_box: &adw::Bin,
    running_kernel_info: &RunningKernelInfo,
) {
    //while let Some(widget) = badge_box.last_child() {
    //    badge_box.remove(&widget);
    //}

    let kernel_badges_size_group = gtk::SizeGroup::new(SizeGroupMode::Both);
    let kernel_badges_size_group0 = gtk::SizeGroup::new(SizeGroupMode::Both);
    let kernel_badges_size_group1 = gtk::SizeGroup::new(SizeGroupMode::Both);

    badge_box.set_child(Some(&crate::content::create_kernel_badge(
        &t!("kernel_badge_running_sched_label").to_string(),
        &running_kernel_info.sched,
        "background-accent-bg",
        &kernel_badges_size_group,
        &kernel_badges_size_group0,
        &kernel_badges_size_group1,
    )));
}