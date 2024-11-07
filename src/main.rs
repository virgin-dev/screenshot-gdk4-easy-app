use std::cell::Cell;
use std::rc::Rc;
use crate::glib::clone;

use gtk4::{prelude::*, Button};
use gtk4::{glib, Application, ApplicationWindow};

const APP_ID: &str = "dc.gtk_rs.ScreenShooter";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {

    let button = Button::builder()
        .label("-")
        .build();
    let button2 = Button::builder()
        .label("+")
        .margin_bottom(50)
        .margin_end(50)
        .margin_start(50)
        .margin_top(50)
        .build();

    let number = Rc::new(Cell::new(0));
    let number_copy = number.clone();

    button.connect_clicked(clone!(
        #[weak]
        number,
        #[strong]
        button,
        move |_| {
            number.set(number.get() + 1);
            button.set_label(&number.get().to_string());
        }
    ));
    button2.connect_clicked(clone!(
        #[strong]
        button2,
        move |_| {
            number.set(number.get() - 1);
            button2.set_label(&number.get().to_string());
        }
    ));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My Screenshot APP")
        .child(&button)
        .build();
    window.present();
}