extern crate gtk;

use std::env;
use gtk::glib;
use gtk::prelude::*;
use gtk::Builder;
use gtk::{
    Toolbar,
    ToolButton,
};


const PLAY_STOCK: &str = "gtk-media-play";

fn main() {

    let application = gtk::Application::new(Some("com.github.ssword.nilplayer"), Default::default());
    application.connect_activate(|application| {
        let window = gtk::ApplicationWindow::new(application);
        window.set_title("NilPlayer");

        let toolbar = Toolbar::new();
        window.add(&toolbar);

        let open_button = ToolButton::new_from_stock("gtk-open");
        toolbar.add(&open_button);
        window.show();
    });
    application.run();
}
