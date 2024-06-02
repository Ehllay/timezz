use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use gtk::gdk::Display;

use gtk::prelude::*;
use gtk::{
    gio, glib, Adjustment, Align, Application, ApplicationWindow, Box as GtkBox, Button,
    CssProvider, Label, Orientation, ProgressBar, SpinButton,
};
use notify_rust::Notification;

const APP_ID: &str = "org.gtk_rs.TimeZZ";

fn build_ui(app: &Application) {
    // Boxes
    let main_box = GtkBox::new(Orientation::Vertical, 5);

    // Time indicator
    let time_left_label = Label::builder()
        .margin_top(12)
        .margin_bottom(12)
        .halign(Align::Center)
        .valign(Align::Center)
        .margin_start(12)
        .margin_end(12)
        .label("5")
        .build();

    let progress = ProgressBar::builder()
        .margin_top(12)
        .margin_bottom(12)
        .fraction(1.0)
        .halign(Align::Center)
        .valign(Align::End)
        .margin_start(12)
        .margin_end(12)
        .hexpand(true)
        .build();

    // Buttons
    let adjustment = Adjustment::builder()
        .lower(5.0)
        .upper(300.0)
        .step_increment(5.0)
        .build();

    let spin_button = SpinButton::builder()
        .adjustment(&adjustment)
        .value(5.0)
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

    // Update label with spin button
    let label = time_left_label.clone();
    let is_paused = Arc::new(Mutex::new(false));
    let is_paused_clone = is_paused.clone();

    spin_button.connect_value_changed(move |sb| {
        let value = sb.value() as u32;
        label.set_text(&format!("{}", value));
    });

    let spin_button_clone = spin_button.clone();

    start_button.connect_clicked(move |button| {
        let mut paused = is_paused_clone.lock().unwrap();
        // Lock SpinButton when timer is on
        let sensitive = spin_button_clone.get_sensitive();
        spin_button_clone.set_sensitive(!sensitive);

        if !*paused {
            *paused = true;
            button.set_label("Stop the timer");

            let is_paused_clone_2 = is_paused_clone.clone();

            gio::spawn_blocking(move || {
                let duration = 5;
                for _ in (1..=duration).rev() {
                    if !*is_paused_clone_2.lock().unwrap() {
                        break;
                    }
                    thread::sleep(Duration::from_secs(1));
                }
                let mut paused = is_paused_clone_2.lock().unwrap();
                *paused = false;

                Notification::new()
                    .summary("Timer has stopped!")
                    .body("Good job! Do you want to start it again?")
                    .icon("alarm-symbolic")
                    .show()
                    .unwrap();
            });
        } else {
            *paused = false;
            button.set_label("Start the timer!");
        };
    });

    // Append widgets
    main_box.append(&time_left_label);
    main_box.append(&progress);
    main_box.append(&spin_button);
    main_box.append(&start_button);
    main_box.set_valign(Align::Center);

    // Apply Css
    time_left_label.add_css_class("time-left");

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

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    app.run()
}
