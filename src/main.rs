#![allow(dead_code)]

extern crate gtk;
extern crate glob;

use gtk::prelude::*;

use gtk::{Window, WindowType, Image};

use std::path::{Path, PathBuf};

use glob::glob;

fn get_images_in_directory(path: String) -> Vec<PathBuf>
{
    let mut result = Vec::<PathBuf>::new();

    let full_path = path + "/*";

    for entry in glob(&full_path).expect("Failed to read glob")
    {
        match entry
        {
            Ok(path) => result.push(path),
            Err(e) => println!("{}", e)
        }
    }

    result
}

fn main() 
{
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Flash");
    window.set_default_size(350, 70);
    //let button = Button::new_with_label("Click me!");
    let image = Image::new_from_file("media/test.png");
    window.add(&image);
    window.show_all();

    window.
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let paths = get_images_in_directory(String::from("/tmp"));
    
    for path in paths
    {
        println!("{}", path.display());
    }

    gtk::main();
}
