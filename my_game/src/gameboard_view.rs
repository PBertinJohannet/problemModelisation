//! Gameboard view.
use graphics::character::CharacterCache;
use graphics::types::Color;
use graphics::{Context, Graphics};
use graphics::{Image, Line, Rectangle, Transformed};

use GameboardController;

/// Stores gameboard view settings.
pub struct GameboardViewSettings {
    /// Position from left-top corner.
    pub position: [f64; 2],
    /// Size of gameboard along horizontal and vertical edge.
    pub size: f64,
    /// Background color.
    pub background_color: Color,
    /// Border color.
    pub border_color: Color,
    /// Edge color around the whole board.
    pub board_edge_color: Color,
    /// Edge color between the 3x3 sections.
    pub section_edge_color: Color,
    /// Edge color between cells.
    pub cell_edge_color: Color,
    /// Edge color between cells.
    pub death_color: Color,
    /// Edge radius around the whole board.
    pub board_edge_radius: f64,
    /// Edge radius between the 3x3 sections.
    pub section_edge_radius: f64,
    /// Edge radius between cells.
    pub cell_edge_radius: f64,
    /// Selected cell background color.
    pub selected_cell_background_color: Color,
    /// Text color.
    pub text_color: Color,
    /// Colors of the armys,
    pub armys_color: [Color; 2],
    /// Colors of the armys,
    pub cell_attack: Color,
    /// Colors of the armys,
    pub cell_move: Color,
}

impl GameboardViewSettings {
    /// Creates new gameboard view settings.
    pub fn new() -> GameboardViewSettings {
        GameboardViewSettings {
            position: [10.0; 2],
            size: 800.0,
            background_color: [0.2, 0.2, 0.3, 1.0],
            border_color: [0.0, 0.0, 0.0, 1.0],
            board_edge_color: [0.0, 0.0, 0.2, 1.0],
            section_edge_color: [0.0, 0.0, 0.2, 1.0],
            cell_edge_color: [0.0, 0.0, 0.2, 1.0],
            death_color: [0.0, 0.0, 0.0, 1.0],
            board_edge_radius: 3.0,
            section_edge_radius: 2.0,
            cell_edge_radius: 1.0,
            selected_cell_background_color: [0.9, 0.9, 1.0, 1.0],
            text_color: [1.0, 1.0, 0.1, 1.0],
            armys_color: [[0.5, 0.5, 0.0, 1.0], [1.0, 0.5, 0.5, 1.0]],
            cell_attack: [1.0, 0.0, 0.0, 0.8],
            cell_move: [0.0, 0.0, 1.0, 0.8],
        }
    }
}

/// Stores visual information about a gameboard.
pub struct GameboardView {
    /// Stores gameboard view settings.
    pub settings: GameboardViewSettings,
}

impl GameboardView {
    /// Creates a new gameboard view.
    pub fn new(settings: GameboardViewSettings) -> GameboardView {
        GameboardView { settings: settings }
    }
    /// Draws the background for a line/row of index i with color *Color*
    pub fn draw_back<G: Graphics>(
        &self,
        settings: &GameboardViewSettings,
        c: &Context,
        g: &mut G,
        i: usize,
    ) {
        // Set up coordinates.
        let offset = i as f64 / (::SIZE.0 as f64) * settings.size;
        let x = settings.position[0] + offset;
        let y = settings.position[1] + offset;

        let color = settings.background_color;
        let vrect = [
            x,
            settings.position[1],
            settings.size / (::SIZE.0 as f64),
            settings.size,
        ];
        Rectangle::new(color).draw(vrect, &c.draw_state, c.transform, g);

        let hrect = [
            settings.position[0],
            y,
            settings.size,
            settings.size / (::SIZE.0 as f64),
        ];
        Rectangle::new(color).draw(hrect, &c.draw_state, c.transform, g);
    }

    /// Draws the grid on the screen.
    pub fn draw_grid<G: Graphics>(
        &self,
        controller: &GameboardController,
        settings: &GameboardViewSettings,
        c: &Context,
        g: &mut G,
    ) {

        let board_rect = [
            settings.position[0],
            settings.position[1],
            settings.size,
            settings.size,
        ];
        // rects :

        for i in 0..::SIZE.0 {
            self.draw_back(settings, c, g,  i);
        }

        // lines
        for i in 0..::SIZE.0 {


            let section_edge = Line::new(settings.section_edge_color, settings.section_edge_radius);
            // Set up coordinates.
            let x = settings.position[0] + i as f64 / (::SIZE.0 as f64) * settings.size;
            let y = settings.position[1] + i as f64 / (::SIZE.0 as f64) * settings.size;
            let x2 = settings.position[0] + settings.size;
            let y2 = settings.position[1] + settings.size;


            let vline = [x, settings.position[1], x, y2];
            section_edge.draw(vline, &c.draw_state, c.transform, g);

            let hline = [settings.position[0], y, x2, y];
            section_edge.draw(hline, &c.draw_state, c.transform, g);
        }

        // Draw board edge.
        Rectangle::new_border(settings.board_edge_color, settings.board_edge_radius)
            .draw(board_rect, &c.draw_state, c.transform, g);


    }

    /// Draw selected cell.
    pub fn draw_cell<G: Graphics>(
        &self,
        color: Color,
        cell: (usize, usize),
        c: &Context,
        g: &mut G,
        settings: &GameboardViewSettings,
    ) {

        let cell_size = settings.size / (::SIZE.0 as f64);
        let pos = [cell.0 as f64 * cell_size, cell.1 as f64 * cell_size];
        let cell_rect = [
            settings.position[0] + pos[0],
            settings.position[1] + pos[1],
            cell_size,
            cell_size,
        ];
        Rectangle::new(color).draw(cell_rect, &c.draw_state, c.transform, g);

    }



    /// Draw a string in a cell.
    pub fn draw_in_cell<G: Graphics, C>(
        &self,
        glyphs: &mut C,
        c: &Context,
        g: &mut G,
        settings: &GameboardViewSettings,
        (i, j): (usize, usize),
        ch: char,
    ) where
        C: CharacterCache<Texture = G::Texture>,
    {
        let text_image = Image::new_color(settings.text_color);
        let cell_size = settings.size / (::SIZE.0 as f64);
        let font_size = self.settings.size as u32 / (::SIZE.0 as u32);
        let pos = [
            settings.position[0] + i as f64 * cell_size + font_size as f64 / 2.0,
            settings.position[1] + j as f64 * cell_size + font_size as f64,
        ];
        if let Ok(character) = glyphs.character(font_size, ch) {
            let ch_x = pos[0] + character.left();
            let ch_y = pos[1] - character.top();
            text_image.draw(
                character.texture,
                &c.draw_state,
                c.transform.trans(ch_x, ch_y),
                g,
            );
        }

    }


    /// Draw cells.
    pub fn draw_cells<G: Graphics, C>(
        &self,
        settings: &GameboardViewSettings,
        controller: &GameboardController,
        glyphs: &mut C,
        c: &Context,
        g: &mut G,
    ) where
        C: CharacterCache<Texture = G::Texture>,
    {
        // Draw characters.
        for player in 0..2 {
            for i in controller.gameboard.player(player) {
                self.draw_cell(
                    settings.armys_color[player],
                    (i.pos.x, i.pos.y),
                    c,
                    g,
                    settings,
                );
                self.draw_in_cell(
                    glyphs,
                    c,
                    g,
                    settings,
                    (i.pos.x, i.pos.y),
                    i.unit_char(),
                );
            }
        }
    }




    /// Draw gameboard.
    pub fn draw<G: Graphics, C>(
        &self,
        controller: &GameboardController,
        glyphs: &mut C,
        c: &Context,
        g: &mut G,
    ) where
        C: CharacterCache<Texture = G::Texture>,
    {
        let ref settings = self.settings;

        self.draw_grid(&controller, settings, c, g);

        if controller.coloring {

            for action in controller.gameboard.moves(controller.player) {
                self.draw_cell(
                    settings.cell_attack,
                    (action.shoot.x, action.shoot.y),
                    c,
                    g,
                    settings,
                );
                self.draw_cell(
                    settings.cell_move,
                    (action.mv.x, action.mv.y),
                    c,
                    g,
                    settings,
                );
            }
        }
        if controller.selected_cell.is_some() {
            for action in controller.selected_actions() {
                self.draw_cell(
                    settings.cell_attack,
                    (action.shoot.x, action.shoot.y),
                    c,
                    g,
                    settings,
                );
                self.draw_cell(
                    settings.cell_move,
                    (action.mv.x, action.mv.y),
                    c,
                    g,
                    settings,
                );
            }

        }
        //self.draw_selected_cell(controller, c, g, settings);

        self.draw_cells(settings, controller, glyphs, c, g);
    }
}
