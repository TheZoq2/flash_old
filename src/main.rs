
extern crate gtk;

use gtk::prelude::*;

use gtk::{Button, Window, WindowType, Image};

use std::path::{Path, PathBuf};

use glob::glob;

fn get_images_in_directory(path: &Path) -> Vec<PathBuf>
{
    for path in glob.
    {
        
    }
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("First GTK+ Program");
    window.set_default_size(350, 70);
    //let button = Button::new_with_label("Click me!");
    let image = Image::new_from_file("media/test.png");
    window.add(&image);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
