//! Gameboard controller.

use piston::input::GenericEvent;
use mission3::game::Game;
use mission3::unit::Action;
use mission3::ai::AI;
use mission3::unit::Position;



/// Handles events for Fifteen puzzle game.
pub struct GameboardController {
    /// Stores the gameboard state.
    pub gameboard: Game,
    /// Selected cell.
    pub selected_cell: Option<[usize; 2]>,
    /// Position of the cursor
    cursor_pos: [f64; 2],
    /// The current playing ai.
    pub player: usize,
    /// Time since last move was made.
    time_since_last_move: f64,
    /// The ais controlling the players.
    ais: Vec<Box<AI>>,
    /// tells if ais are playing
    playing: bool,
    /// Tells if it is coloring a cell.
    pub coloring: bool,
    /// Tells if it is debugging.
    pub debug: bool,
    /// Tells the moves we are debugging.
    pub debuging: Vec<(Game, Action)>,
    /// Tells the old game
    pub old_game: Option<Game>,
}

impl GameboardController {
    /// Creates a new gameboard controller.
    pub fn new(state: Game, ais: Vec<Box<AI>>) -> GameboardController {
        GameboardController {
            gameboard: state,
            selected_cell: None,
            cursor_pos: [0.0; 2],
            time_since_last_move: 0.0,
            player: 0,
            ais: ais,
            playing: false,
            coloring: false,
            debug: false,
            debuging: vec![],
            old_game: None,
        }
    }
    /// Returns the disponibles actions for the selected cell.
    pub fn selected_actions(&self) -> Vec<Action> {
        if self.selected_cell.is_some() {
            let sel = self.selected_cell.unwrap();
            match self.gameboard.at(Position::new(sel[0], sel[1])) {
                Some(u) => {
                    u.actions(
                        self.gameboard.width(),
                        self.gameboard.height(),
                        &self.gameboard.positions(),
                    )
                }
                None => vec![],
            }
        } else {
            vec![]
        }
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, e: &E) {
        use piston::input::{Button, Key, MouseButton};
        if let Some(idle) = e.idle_args() {
            if self.playing {
                self.time_since_last_move += idle.dt;
            }
        }
        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            // Find coordinates relative to upper left corner.
            let x = self.cursor_pos[0] - pos[0];
            let y = self.cursor_pos[1] - pos[1];
            // Check that coordinates are inside board boundaries.
            if x >= 0.0 && x <= size && y >= 0.0 && y <= size {
                // Compute the cell position.
                let cell_x = (x / size * ::SIZE.1 as f64) as usize;
                let cell_y = (y / size * ::SIZE.0 as f64) as usize;
                self.selected_cell = Some([cell_x, cell_y]);
                println!("selected cell : {}, {}", cell_x, cell_y);
            }
        }
        match e.press_args() {
            Some(Button::Keyboard(key)) => {
                match key {
                    Key::Space => {
                        self.playing = !self.playing;
                    }
                    Key::M => {
                        self.coloring = !self.coloring;
                    }
                    Key::D => {
                        self.debug = !self.debug;
                        if self.debug {
                            let moves = self.gameboard.moves(self.player);
                            println!("debug ");
                            self.debuging = moves
                                .iter()
                                .map(|mv| {
                                    let mut g = self.gameboard.clone();
                                    g.apply_move(mv);
                                    (g, mv.clone())
                                })
                                .collect();
                            self.old_game = Some(self.gameboard.clone());
                            self.gameboard = self.debuging.pop().unwrap().0;
                        }
                    }
                    Key::N => {
                        let (mut g, mut m) = self.debuging.pop().unwrap();
                        self.gameboard = g;
                        println!("debug : {}", self.debuging.len());
                        println!("move : {:?}", m);
                        if self.debuging.is_empty() {
                            self.debug = false;
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        };
        if !self.debug && self.old_game.is_some() {
            self.gameboard = self.old_game.clone().unwrap();
            self.old_game = None;
        }
        if self.playing {
            if self.time_since_last_move > 0.5 {
                self.time_since_last_move -= 0.5;
                if self.gameboard.player(self.player).len() == 0 {
                    return;
                }
                let mv = &self.ais[self.player].play(&self.gameboard, self.player);
                self.gameboard.apply_move(mv);
                self.player = 1 - self.player;
            }
        }
    }
}
