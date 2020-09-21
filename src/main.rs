extern crate hanafuda_deck_rs;
extern crate gtk;
extern crate gio;
extern crate glib;
extern crate cairo;

use std::env::args;
use gio::prelude::*;
use gio::ApplicationFlags;
use gtk::prelude::*;
use gtk::{Application, Image};
use glib::clone;
use cairo::ImageSurface;

use hanafuda_deck_rs::HanafudaDeck;

fn display_cards(app: &Application, deck: &HanafudaDeck) {
    let window = gtk::ApplicationWindow::new(app);
    
    window.set_title("Card Grid");
    window.set_position(gtk::WindowPosition::Center);
    window.get_preferred_width();
    window.set_default_size(630, 400);

    window.set_application(Some(app));
    
    let container = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    container.set_widget_name("container");

    let card_grid = gtk::Grid::new();
    card_grid.set_column_spacing(5);
    card_grid.set_row_spacing(5);

    let mut card_num = 0;
    for card in deck.cards.iter() {
        let row = card_num / 4;
        let column = card_num % 4;
        let mut img_copy = card.image_raw_bytes.clone();
        let surface = ImageSurface::create_from_png(&mut img_copy).unwrap();
        let img = Image::from_surface(Some(&surface));
        card_grid.attach(&img, column, row, 1, 1);

        card_num += 1;
    }

    container.add(&card_grid);

    window.add(&container);

    app.connect_activate(clone!(@weak window => move |_| {
        window.show_all();
    }));
}

fn main() {
    let mut hanafuda_deck = HanafudaDeck::new();
    hanafuda_deck.shuffle();

    let application = Application::new(Some("com.knudsen.hanafuda_grid"), ApplicationFlags::empty())
        .expect("Initialization failed");

    application.connect_startup(move |app: &Application| {
        display_cards(app, &hanafuda_deck);
    });

    glib::set_application_name("Hanafuda Grid");
    application.run(&args().collect::<Vec<_>>());
}
