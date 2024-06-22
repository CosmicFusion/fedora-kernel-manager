use crate::APP_GITHUB;
use crate::APP_ICON;
use crate::APP_ID;
use crate::VERSION;
use crate::{content, KernelBranch};
use adw::prelude::*;
use adw::*;
use glib::{clone, MainContext};
use gtk::*;
use std::cell::RefCell;
use std::process::Command;
use std::rc::Rc;
use std::{thread, time};

pub fn build_ui(app: &adw::Application) {
    gtk::glib::set_prgname(Some(t!("application_name").to_string()));
    glib::set_application_name(&t!("application_name").to_string());

    let internet_connected = Rc::new(RefCell::new(false));
    let selected_kernel_branch: Rc<RefCell<KernelBranch>> = Rc::new(RefCell::new(KernelBranch {
        name: "?".to_owned(),
        db_url: "?".to_owned(),
        db: "?".to_owned(),
        init_script: "?".to_owned(),
    }));
    let db_load_complete = Rc::new(RefCell::new(false));

    let (internet_loop_sender, internet_loop_receiver) = async_channel::unbounded();
    let internet_loop_sender = internet_loop_sender.clone();

    std::thread::spawn(move || loop {
        match Command::new("ping").arg("google.com").arg("-c 1").output() {
            Ok(t) if t.status.success() => internet_loop_sender
                .send_blocking(true)
                .expect("The channel needs to be open"),
            _ => internet_loop_sender
                .send_blocking(false)
                .expect("The channel needs to be open"),
        };
        thread::sleep(time::Duration::from_secs(5));
    });

    let window_banner = adw::Banner::builder().revealed(false).build();

    let internet_connected_status = internet_connected.clone();

    let selected_kernel_branch2 = selected_kernel_branch.clone();

    let internet_loop_context = MainContext::default();
    // The main loop executes the asynchronous block
    internet_loop_context.spawn_local(clone!(@weak window_banner => async move {
        while let Ok(state) = internet_loop_receiver.recv().await {
            let banner_text = t!("banner_text_no_internet").to_string();
            if state == true {
                *internet_connected_status.borrow_mut()=true;
                if window_banner.title() == banner_text {
                    window_banner.set_revealed(false)
                }
            } else {
                *internet_connected_status.borrow_mut()=false;
                if window_banner.title() != t!("banner_text_url_error").to_string() {
                window_banner.set_title(&banner_text);
                window_banner.set_revealed(true)
                    }
            }
        }
    }));

    let window_headerbar = adw::HeaderBar::builder()
        .title_widget(
            &adw::WindowTitle::builder()
                .title(t!("application_name"))
                .build(),
        )
        .build();

    let content_stack = gtk::Stack::builder()
        .transition_type(StackTransitionType::Crossfade)
        .build();

    let window_toolbar = adw::ToolbarView::builder().content(&content_stack).build();

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .content(&window_toolbar)
        .width_request(600)
        .height_request(600)
        .resizable(false)
        .icon_name(APP_ICON)
        .startup_id(APP_ID)
        .build();

    content_stack.add_named(
        &content::content(
            &content_stack,
            &selected_kernel_branch2,
            &db_load_complete,
            &window,
            &window_banner,
        ),
        Some("content_page"),
    );

    window_toolbar.add_top_bar(&window_headerbar);
    window_toolbar.add_top_bar(&window_banner);

    load_icon_theme(&window);

    window.connect_close_request(move |window| {
        if let Some(application) = window.application() {
            application.remove_window(window);
        }
        glib::Propagation::Proceed
    });

    let credits_button = gtk::Button::builder()
        .icon_name("dialog-information-symbolic")
        .build();

    let credits_window = adw::AboutWindow::builder()
        .application_icon(APP_ICON)
        .application_name(t!("application_name"))
        .transient_for(&window)
        .version(VERSION)
        .hide_on_close(true)
        .developer_name(t!("developer_name"))
        .license_type(License::Gpl20)
        .issue_url(APP_GITHUB.to_owned() + "/issues")
        .build();

    window_headerbar.pack_end(&credits_button);
    credits_button
        .connect_clicked(clone!(@weak credits_button => move |_| credits_window.present()));

    window.present();
}

fn load_icon_theme(window: &adw::ApplicationWindow) {
    let icon_theme = gtk::IconTheme::for_display(&WidgetExt::display(window));

    icon_theme.add_resource_path("/com/github/cosmicfusion/fedora-kernel-manager/icons/");
    icon_theme.add_resource_path(
        "/com/github/cosmicfusion/fedora-kernel-manager/icons/scalable/actions/",
    );
}
