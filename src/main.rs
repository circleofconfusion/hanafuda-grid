extern crate hanafuda_deck_rs;
extern crate gtk;
extern crate gio;
extern crate glib;
extern crate cairo;

use std::env::args;
use gio::prelude::*;
use gio::ApplicationFlags;
use gtk::prelude::*;
use gtk::{Application, Image, Orientation, Align};
use glib::clone;
use cairo::ImageSurface;

use hanafuda_deck_rs::HanafudaDeck;

const CARD_WIDTH: i32 = 232;
const CARD_HEIGHT: i32 = 400;
const COLUMNS: i32 = 4;
const CARD_PADDING: i32 = 5;

fn display_cards(app: &Application, deck: &HanafudaDeck) {
    let window = gtk::ApplicationWindow::new(app);
    
    window.set_title("Card Grid");
    window.set_position(gtk::WindowPosition::Center);
    window.get_preferred_width();
    window.set_default_size(CARD_WIDTH * COLUMNS + CARD_PADDING * (COLUMNS - 1), CARD_HEIGHT * 2 + CARD_PADDING);

    window.set_application(Some(app));

    // Overall scrolling panel 
    let scrolling_container = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    scrolling_container.set_widget_name("scrolling-container");
    
    // Box to center the card grid
    let centering_container = gtk::Box::new(Orientation::Horizontal, 0);
    centering_container.set_halign(Align::Center);
    
    let card_grid = gtk::Grid::new();
    card_grid.set_column_spacing(CARD_PADDING as u32);
    card_grid.set_row_spacing(CARD_PADDING as u32);
    
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
    
    // Assemble all the widgets
    scrolling_container.add(&centering_container);
    centering_container.add(&card_grid);
    window.add(&scrolling_container);

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
