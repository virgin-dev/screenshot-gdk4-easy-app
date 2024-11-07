use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::glib::clone;
use gtk4::gio::SrvTarget;
use gtk4::Orientation;
use gtk4::{prelude::*, Button};
use gtk4::{glib, Application, ApplicationWindow};
extern crate screenshot;
use screenshot::get_screenshot;


const APP_ID: &str = "dc.gtk_rs.ScreenShooter";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_base_ui);
    app.run()
}

fn take_and_save_screenshot() -> Result<String, Box<dyn std::error::Error>> {
    let screenshots_folder = "C:\\Users\\games";
    fs::create_dir_all(screenshots_folder);

    let screen = get_screenshot(0)?;

    let path = Path::new(&filename);

    let width = screenshot.width() as u32;
    let height = screenshot.height() as u32;
    let image: RgbaImage = ImageBuffer::from_raw(width, height, screenshot.into_vec())
        .ok_or("Failed to create image buffer")?;

        let filename = format!(
            "{}/screenshot_{}.png",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)?
                .as_secs()
        );

    let path = Path::new(&filename);

    // Сохраняем изображение как PNG
    image.save(path)?;
    Ok(filename)

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