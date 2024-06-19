use crate::{content, kernel_pkg, sched_ext, KernelBranch, PRETTY_NAME};
use adw::prelude::*;
use adw::*;
use glib::property::PropertyGet;
use glib::{clone, MainContext};
use gtk::prelude::*;
use gtk::*;
use std::cell::RefCell;
use std::process::Command;
use std::rc::Rc;
use std::{thread, time};

pub fn build_ui(app: &adw::Application) {
    let internet_connected = Rc::new(RefCell::new(false));
    let selected_kernel_branch: Rc<RefCell<KernelBranch>> = Rc::new(RefCell::new(KernelBranch {
        name: "?".to_owned(),
        db_url: "?".to_owned(),
        db: "?".to_owned(),
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
            let banner_text = "Warning: No internet connection";
            if state == true {
                *internet_connected_status.borrow_mut()=true;
                if window_banner.title() == banner_text {
                    window_banner.set_revealed(false)
                }
            } else {
                *internet_connected_status.borrow_mut()=false;
                window_banner.set_title(banner_text);
                window_banner.set_revealed(true)
            }
        }
    }));

    let window_headerbar = adw::HeaderBar::builder()
        .title_widget(&adw::WindowTitle::builder().title(PRETTY_NAME).build())
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
        .build();

    content_stack.add_named(
        &content::content(
            &content_stack,
            &selected_kernel_branch2,
            &db_load_complete,
            &window,
        ),
        Some("content_page"),
    );
    content_stack.add_named(
        &sched_ext::sched_ext_page(&content_stack, &window),
        Some("sched_ext_page"),
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

    window.present();
}

fn load_icon_theme(window: &adw::ApplicationWindow) {
    let icon_theme = gtk::IconTheme::for_display(&WidgetExt::display(window));

    icon_theme.add_resource_path("/com/github/cosmicfusion/fedora-kernel-manager/icons/");
    icon_theme.add_resource_path(
        "/com/github/cosmicfusion/fedora-kernel-manager/icons/scalable/actions/",
    );
}
