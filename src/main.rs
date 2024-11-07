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

fn build_ui(app: &Application) {

    let button_increase = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let button_decrease = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let number = Rc::new(Cell::new(0));

    button_increase.connect_clicked(clone!(
        #[weak]
        number,
        #[weak]
        button_decrease,
        move |_| {
            number.set(number.get() + 1);
            button_decrease.set_label(&number.get().to_string());
        }
    ));
    button_decrease.connect_clicked(clone!(
        #[weak]
        button_increase,
        move |_| {
            number.set(number.get() - 1);
            button_increase.set_label(&number.get().to_string());
        }
    ));

    let gtk_box = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My Screenshot APP")
        .child(&gtk_box)

        .build();
    window.present();
}