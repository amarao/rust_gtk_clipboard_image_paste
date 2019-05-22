extern crate gtk;
extern crate gio;
extern crate gdk;
extern crate gdk_pixbuf;

// To import all needed traits.
use gtk::prelude::*;
use gio::prelude::*;
// use gdk_pixbuf;
use std::env;

fn main() {
    let uiapp = gtk::Application::new("org.gtkrsnotes.demo",
                                      gio::ApplicationFlags::FLAGS_NONE)
                                 .expect("Application::new failed");
    uiapp.connect_activate(|app| {
        // We create the main window.
        let win = gtk::ApplicationWindow::new(app);

        // Then we set its size and a title.
        win.set_default_size(320, 200);
        win.set_title("Basic example");

        // Don't forget to make all widgets visible.
        win.show_all();
        // let mut clipboard = gtk::Clipboard::get(&gtk::Clipboard::SELECTION_CLIPBOARD);
        let atom = gdk::Atom::intern("CLIPBOARD");
        let mut clipboard = gtk::Clipboard::get(&atom);
        // let data:Vec<u32>=vec![0x904040;42*42];
        let mut pixbuf = gdk_pixbuf::Pixbuf::new(
            gdk_pixbuf::Colorspace::Rgb,
            false,
            8,
            42,
            42
        ).unwrap();
        pixbuf.fill(0x90404000);
        clipboard.set_image(&pixbuf);
        // let sel = clipboard.get_selection();
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
}
