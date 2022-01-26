use adw::prelude::*;
use adw::{HeaderBar, ViewSwitcherTitle};
use gtk::gio;

use crate::application::Application;
use crate::config;
use crate::window::Window;

pub fn build_ui(app: &Application) {
    let window = Window::new(&app);
    let view_stack = window.view_stack();
    let view_switcher_title = ViewSwitcherTitle::builder()
        .stack(&view_stack)
        .title(config::APP_NAME)
        .build();

    view_switcher_title
        .bind_property("title-visible", &window.switcher_bar(), "reveal")
        .build();

    let header_bar_builder =
        gtk::Builder::from_string(include_str!("../data/resources/ui/header_bar.ui"));

    let header_bar: HeaderBar = header_bar_builder.object("header_bar").unwrap();
    header_bar.set_title_widget(Some(&view_switcher_title));

    window.content_box().prepend(&header_bar);
    window.present();

    let about_action = gio::SimpleAction::new("about", None);
    about_action.connect_activate(move |_, _| {
        let dialog = gtk::AboutDialog::builder()
            .program_name(config::APP_NAME)
            .authors(vec!["Zhangyuan Nie".into()])
            .transient_for(&window)
            .modal(true)
            .version(config::APP_VERSION)
            .build();

        dialog.show();
    });
    app.add_action(&about_action);
}
