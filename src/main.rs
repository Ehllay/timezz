use std::cell::Cell;

use gtk::prelude::*;
use gtk::{
    glib, Adjustment, Align, Application, ApplicationWindow, Box as GtkBox, Button, Orientation,
    SpinButton,
};

const APP_ID: &str = "org.gtk_rs.TimeZZ";

fn build_ui(app: &Application) {
    // Boxes
    let main_box = GtkBox::new(Orientation::Vertical, 5);

    // Buttons
    let adjustment = Adjustment::builder()
        .lower(5.0)
        .upper(300.0)
        .step_increment(5.0)
        .build();

    let spin_button = SpinButton::builder()
        .adjustment(&adjustment)
        .margin_top(12)
        .margin_bottom(12)
        .halign(Align::Center)
        .valign(Align::End)
        .margin_start(12)
        .margin_end(12)
        .build();

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

    main_box.append(&spin_button);
    main_box.append(&start_button);
    main_box.set_valign(Align::End);

    // Create window
    let window = ApplicationWindow::builder()
        .application(app)
        .height_request(300)
        .width_request(250)
        .default_width(250)
        .default_height(300)
        .title("TimeZZ")
        .child(&main_box)
        .build();
    window.present()
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}
