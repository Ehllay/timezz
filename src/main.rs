use std::cell::Cell;

use gtk::prelude::*;
use gtk::{glib, Align, Application, ApplicationWindow, Button};

const APP_ID: &str = "org.gtk_rs.TimeZZ";

fn build_ui(app: &Application) {
    // Buttons
    let start_button = Button::builder()
        .label("Start the timer!")
        .margin_top(12)
        .margin_bottom(12)
        .halign(Align::Center)
        .valign(Align::End)
        .margin_start(12)
        .margin_end(12)
        .build();

    let paused = Cell::new(false);

    start_button.connect_clicked(move |button| {
        if !paused.get() {
            button.set_label("Stop the timer");
            paused.set(true);
        } else {
            button.set_label("Start the timer!");
            paused.set(false);
        };
    });

    // Create window
    let window = ApplicationWindow::builder()
        .application(app)
        .height_request(300)
        .width_request(250)
        .default_width(250)
        .default_height(300)
        .title("TimeZZ")
        .child(&start_button)
        .build();
    window.present()
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}
