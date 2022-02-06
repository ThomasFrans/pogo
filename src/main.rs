use std::cell::{Cell, RefCell};
use std::rc::Rc;
use gtk::glib::clone;
use gtk::glib;
use gtk::prelude::*;
use mines::mines::Board;

struct Game {
    app: gtk::Application,
    board: Rc<RefCell<Board>>
}

impl Game {
    fn new() -> Self {

        let app = gtk::Application::builder()
            .application_id("com.github.ThomasFrans.pogo")
            .build();
        let board = Rc::new(RefCell::new(Board::new(24, 24, 64).expect("Board except")));


        app.connect_activate(clone!(@weak board => move |app| {
            let header = gtk::HeaderBar::builder()
                .build();
            let window = gtk::ApplicationWindow::builder()
                .resizable(false)
                .title("Pogo")
                .application(app)
                .build();
            let grid = gtk::Grid::builder()
                .halign(gtk::Align::Center)
                .valign(gtk::Align::Center)
                .build();
            for i in 0..24 {
                for j in 0..24 {
                    let but = gtk::Button::builder()
                        .label("#")
                        .height_request(50)
                        .width_request(50)
                        .build();

                    but.connect_clicked(clone!(@weak grid, @weak board => move |_| {
                        board.borrow_mut().uncover(i, j);
                        for a in 0..24 {
                            for b in 0..24 {
                                let board_tmp = &*board.borrow();
                                let cell = &board_tmp.get_state()[a][b];
                                if !cell.is_covered() && !cell.has_mine() && cell.get_neighbor_amount() == 0 {
                                    grid.attach(&gtk::Button::builder()
                                        .label(" ")
                                        .height_request(50)
                                        .width_request(50)
                                        .build(), b as i32, a as i32, 1, 1);
                                } else if !cell.is_covered() && cell.has_mine() {
                                    grid.attach(&gtk::Button::builder()
                                        .label("O")
                                        .height_request(50)
                                        .width_request(50)
                                        .build(), b as i32, a as i32, 1, 1);
                                } else if !cell.is_covered() && cell.get_neighbor_amount() > 0 {
                                    grid.attach(&gtk::Button::builder()
                                        .label(format!("{}", cell.get_neighbor_amount()).as_str())
                                        .height_request(50)
                                        .width_request(50)
                                        .build(), b as i32, a as i32, 1, 1);
                                }

                            }
                        }
                }));
                    grid.attach(&but, i, j, 1, 1);
                }
            }
            window.set_titlebar(Some(&header));
            window.set_child(Some(&grid));
            window.present();
        }));

        Game {
            app,
            board
        }
    }

    fn run (&self) {
        self.app.run();
    }
}

fn main() {
    let geam = Game::new();
    geam.run();
}