extern crate gtk;
extern crate gio;
extern crate gdk;
extern crate gdk_pixbuf;
extern crate cairo;

// To import all needed traits.
use gtk::prelude::*;
use gio::prelude::*;
use std::env;
// use cairo::{Context, Format, ImageSurface};

fn app( app:&gtk::Application) {
        // We create the main window.
        let win = gtk::ApplicationWindow::new(app);

        // Then we set its size and a title.
        win.set_default_size(1920, 1080);
        win.set_title("Basic example");

        // Don't forget to make all widgets visible.
        win.show_all();
        let atom = gdk::Atom::intern("CLIPBOARD");
        let clipboard = gtk::Clipboard::get(&atom);
        let  pixbuf = gdk_pixbuf::Pixbuf::new(
            gdk_pixbuf::Colorspace::Rgb,
            false,
            8,
            42,
            42
        ).unwrap();
        pixbuf.fill(0x90404000);
        for i in 0..42{
            pixbuf.put_pixel(i, i, 0x00, 0x90, 0x10, 0x00);
        }
        clipboard.set_image(&pixbuf);
}

fn main() {
    let uiapp = gtk::Application::new("org.example.clipboard.ops",
                                      gio::ApplicationFlags::FLAGS_NONE)
                                 .expect("Application::new failed");
    uiapp.connect_activate(app);
    uiapp.run(&env::args().collect::<Vec<_>>());
}
