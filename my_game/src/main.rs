#![deny(missing_docs)]

//! The interface for the game.
//! You can specify the ais to play with in the main function of this file.
//! You can specify the size of the grid as the SIZE const in this file.
//!
//! You can also specify the number of units per player in the main function.
//! at the creation of the game (game::new_random(size, size, nb_unit).
//!
extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate mission3;
extern crate time;
extern crate rand;


use piston::window::WindowSettings;
use piston::event_loop::{Events, EventSettings};
use glutin_window::GlutinWindow;
use piston::input::RenderEvent;
use opengl_graphics::{OpenGL, Filter, GlGraphics, GlyphCache, TextureSettings};

use rand::XorShiftRng;

use mission3::game::Game;
use mission3::randomai::RandomAI;
use mission3::naive::NaiveAI;
use mission3::minmax::NegaMaxAI;
use mission3::montecarlo::MonteCarloAI;
use mission3::comparator::unit_eval_double;


pub use gameboard_controller::GameboardController;
pub use gameboard_view::GameboardView;
pub use gameboard_view::GameboardViewSettings;

mod gameboard_controller;
mod gameboard_view;

static SIZE: (usize, usize) = (7, 7);

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Sudoku", [912; 2])
        .opengl(opengl)
        .exit_on_esc(true);

    let mut window: GlutinWindow = settings.build().expect("Could not create window");

    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);

    let gameboard = Game::new_random(SIZE.0, SIZE.1, 7, &mut XorShiftRng::new_unseeded());
    let mut gameboard_controller = GameboardController::new(
        gameboard,
        vec![
            Box::new(MonteCarloAI::new( unit_eval_double())),
            Box::new(NaiveAI::new( unit_eval_double())),
        ],
    );
    let gameboard_view_settings = GameboardViewSettings::new();
    let gameboard_view = GameboardView::new(gameboard_view_settings);


    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings)
        .expect("Could not load font");

    while let Some(e) = events.next(&mut window) {
        gameboard_controller.event(
            gameboard_view.settings.position,
            gameboard_view.settings.size,
            &e,
        );
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;

                clear([1.0; 4], g);
                gameboard_view.draw(&gameboard_controller, glyphs, &c, g);
            });
        }
    }
}
