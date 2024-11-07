use std::cell::Cell;
use std::rc::Rc;
use crate::glib::clone;
use gtk4::Orientation;
use gtk4::{prelude::*, Button};
use gtk4::{glib, Application, ApplicationWindow};

const APP_ID: &str = "dc.gtk_rs.ScreenShooter";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_base_ui(app: &Application) {
    let button = Button::with_label("Take Screenshot");

    button.connect_clicked(clone!(
        #[weak]
        button,
        move |_| {
            match take_and_save_screenshot() {
                Ok(path) => println!("Screenshoot saved to : {:?}", path),
                Err(e) => eprintln!("Failed to take screenshot: {:?}", e),
            }
        }
    ));

    let gtk_box = gtk4::Box::builder()
    .orientation(Orientation::Vertical)
    .build();
    gtk_box.append(&button);

    let window =  ApplicationWindow::builder()
    .application(app)
    .default_width(100)
    .default_height(200)
    .title("My Screenshot APP")
    .child(&gtk_box)
    .build();

}