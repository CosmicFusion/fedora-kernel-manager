use gtk::*;
use gtk::prelude::*;
use adw::*;
use adw::prelude::*;
use crate::{content, PRETTY_NAME, sched_ext};

pub fn build_ui(app: &adw::Application) {
    let window = adw::ApplicationWindow::new(app);

    load_icon_theme(&window);

    window.connect_close_request(move |window| {
        if let Some(application) = window.application() {
            application.remove_window(window);
        }
        glib::Propagation::Proceed
    });

    let window_headerbar = adw::HeaderBar::builder()
        .title_widget(&adw::WindowTitle::builder().title(PRETTY_NAME).build())
        .build();

    let content_stack = gtk::Stack::builder()
        .transition_type(StackTransitionType::Crossfade)
        .build();

    let window_toolbar = adw::ToolbarView::builder()
        .content(&content_stack)
        .build();

    content_stack.add_named(&content::content(&content_stack), Some("content_page"));
    content_stack.add_named(&sched_ext::sched_ext_page(&content_stack), Some("sched_ext_page"));

    window_toolbar.add_top_bar(&window_headerbar);

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .content(&window_toolbar)
        .width_request(600)
        .height_request(600)
        .resizable(false)
        .build();

    window.present();
}

fn load_icon_theme(window: &adw::ApplicationWindow) {
    let icon_theme = gtk::IconTheme::for_display(&WidgetExt::display(window));

    icon_theme.add_resource_path("/com/github/cosmicfusion/fedora-kernel-manager/icons/");
    icon_theme.add_resource_path("/com/github/cosmicfusion/fedora-kernel-manager/icons/scalable/actions/");
}