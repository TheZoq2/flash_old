#![allow(dead_code)]

extern crate gtk;
extern crate glob;

use gtk::prelude::*;

use gtk::{Window, WindowType, Image};

use std::path::{Path, PathBuf};

use glob::glob;

mod tags;

const WINDOW_NAME: &'static str = "main_window";

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
    
    let glade_src = include_str!("../resources/main_ui.glade");
    let builder = gtk::Builder::new_from_string(glade_src);

    let window: Window = builder.get_object(&WINDOW_NAME).unwrap();

    window.connect_delete_event(|_, _| {
                gtk::main_quit();
                Inhibit(false)
    });

    window.show_all();

    gtk::main();
}
