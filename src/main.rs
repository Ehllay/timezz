use glib::clone;
use gtk::gdk::Display;

use gtk::prelude::*;
use gtk::{
    glib, Adjustment, Align, Application, ApplicationWindow, Box as GtkBox, Button, CssProvider,
    Label, Orientation, ProgressBar, SpinButton,
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

    spin_button.connect_value_changed(move |sb| {
        let value = sb.value() as u32;
        label.set_text(&format!("{}", value));
    });

    // Clones
    let spin_button_clone = spin_button.clone();
    let progress_clone = progress.clone();
    let time_left_label_clone = time_left_label.clone();

    start_button.connect_clicked(move |button| {
        let spin_button = spin_button_clone.clone();
        let progress = progress_clone.clone();
        let label = time_left_label_clone.clone();

        glib::spawn_future_local(
            clone!(@weak button, @weak spin_button, @weak progress, @weak label => async move {
                button.set_label("Stop the timer");

                let duration = spin_button.value_as_int() * 60;
                for t in (1..=duration).rev() {
                    progress.set_fraction(t as f64 / duration as f64);
                    label.set_text(&format!("{}", (t+59)/60));
                    glib::timeout_future_seconds(1).await;
                }

                button.set_label("Start the timer!");
                progress.set_fraction(1.0);
                label.set_text(&format!("{}", duration/60));

                Notification::new()
                    .summary("Timer has stopped!")
                    .body("Good job! Do you want to start it again?")
                    .icon("alarm-symbolic")
                    .show()
                    .unwrap();
            }),
        );
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
