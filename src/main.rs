use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Box, Label};
use std::process::Command;

fn main() {
    let app = Application::new(Some("com.example.roblox_multi"), Default::default());
    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Roblox Multi");
        window.set_default_size(300, 200);

        let vbox = Box::new(gtk::Orientation::Vertical, 5);
        let label = Label::new(Some("Welcome to Roblox Multi"));
        let start_button = Button::with_label("Start Roblox");

        start_button.connect_clicked(|_| {
            Command::new("roblox-player.exe")
                .arg("--start")
                .spawn()
                .expect("Failed to start Roblox");
        });

        vbox.pack_start(&label, true, true, 0);
        vbox.pack_start(&start_button, true, true, 0);
        window.add(&vbox);
        window.show_all();
    });

    app.run();
}