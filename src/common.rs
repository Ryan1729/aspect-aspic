//in pixels
pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 240;

pub struct Framebuffer {
    pub buffer: Vec<u32>,
}

impl PartialEq for Framebuffer {
    fn eq(&self, other: &Framebuffer) -> bool {
        &self.buffer[..] == &other.buffer[..]
    }
}

impl Eq for Framebuffer {}

impl Framebuffer {
    pub fn new() -> Framebuffer {
        Framebuffer::default()
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize, colour: u32) {
        let one_past_right_edge = x + width;
        let one_past_bottom_edge = y + height;

        for current_y in y..one_past_bottom_edge {
            for current_x in x..one_past_right_edge {
                let i = current_y
                    .saturating_mul(SCREEN_WIDTH)
                    .saturating_add(current_x);
                if i < self.buffer.len() {
                    self.buffer[i] = colour;
                }
            }
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.buffer.len() {
            self.buffer[i] = 0;
        }
    }
}

impl Default for Framebuffer {
    fn default() -> Self {
        let mut buffer = Vec::new();
        buffer.resize(SCREEN_WIDTH * SCREEN_HEIGHT, 0);

        Framebuffer { buffer }
    }
}

pub type Position = (BoardCoord, BoardCoord);

pub struct GameState {
    pub entities: [Component::Ty; GameState::ENTITY_COUNT],

    pub positions: [Position; GameState::ENTITY_COUNT],
    pub appearances: [Appearance; GameState::ENTITY_COUNT],
}

impl GameState {
    pub const ENTITY_COUNT: usize = 256;

    pub fn new() -> GameState {
        let mut entities = [Component::Ty::empty(); GameState::ENTITY_COUNT];
        let mut positions = [(0, 0); GameState::ENTITY_COUNT];
        let mut appearances = [Appearance::empty(); GameState::ENTITY_COUNT];

        for i in 0..GameState::ENTITY_COUNT {
            entities[i].insert(Component::Position | Component::Appearance);
            if let Some(pos) = get_board_xy(i) {
                positions[i] = pos;
            }
            appearances[i].colour = GREEN;
        }

        let playerId = 24;

        entities[playerId] |= Component::PlayerControlled;
        appearances[playerId].colour = BLUE;

        GameState {
            entities,
            positions,
            appearances
        }
    }
}

pub mod Component {
    bitflags! {
        pub flags Ty: u16 {
            const Position         = 1 << 0,
            const Appearance       = 1 << 1,
            const PlayerControlled = 1 << 2,
        }
    }
}

pub const BLUE: u32 = 0xFFEE2222;
pub const GREEN: u32 = 0xFF22EE22;
pub const RED: u32 = 0xFF2222EE;

#[derive(Clone, Copy)]
pub struct Appearance {
    pub colour: u32,
}

impl Appearance {
    fn empty() -> Self {
        Appearance {
            colour: 0,
        }
    }

    pub fn render_positioned(&self, framebuffer: &mut Framebuffer, (x, y): Position) {
        let px_x = cell_x_to_px_x(x as usize);
        let px_y = cell_y_to_px_y(y as usize);

        let colour = self.colour;

        framebuffer.draw_rect(px_x as _, px_y as _, CELL_WIDTH, CELL_HEIGHT, colour);
    }
}

pub struct State {
    pub framebuffer: Framebuffer,
    pub gamepad: Button::Ty,
    pub game_state: GameState,
}

impl State {
    pub fn new() -> State {
        let framebuffer = Framebuffer::new();

        State {
            framebuffer,
            gamepad: Button::Ty::empty(),
            game_state: GameState::new(),
        }
    }
}

// These values are deliberately picked to be the same as the ones in NES' input registers.
pub mod Button {
    bitflags! {
        pub flags Ty: u8 {
            const A          = 1 << 0,
            const B          = 1 << 1,
            const Select     = 1 << 2,
            const Start      = 1 << 3,
            const Up         = 1 << 4,
            const Down       = 1 << 5,
            const Left       = 1 << 6,
            const Right      = 1 << 7
        }
    }
}

pub type BoardCoord = u8;

//in cells
pub const BOARD_WIDTH: BoardCoord = 6;
pub const BOARD_HEIGHT: BoardCoord = 6;

pub const BOARD_LENGTH: usize = BOARD_WIDTH as usize * BOARD_HEIGHT as usize;

#[allow(dead_code)]
pub fn get_board_index(x: BoardCoord, y: BoardCoord) -> Option<usize> {
    if !xy_on_board(x, y) {
        return None;
    }

    let result = (y.saturating_mul(BOARD_WIDTH).saturating_add(x)) as usize;

    if is_index_on_board(result) {
        Some(result)
    } else {
        None
    }
}

pub fn get_board_xy(index: usize) -> Option<(BoardCoord, BoardCoord)> {
    if !is_index_on_board(index) {
        return None;
    }

    let result = (index as BoardCoord % BOARD_WIDTH, index as BoardCoord / BOARD_WIDTH);

    if xy_on_board(result.0, result.1) {
        Some(result)
    } else {
        None
    }
}

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(test)]
mod board_indices {
    use ::*;

    quickcheck! {
        fn i_xy_i(i: usize) -> bool {
              let expected = if is_index_on_board(i) {
                  Some(i)
              } else {
                  None
              };

              expected == get_board_xy(i).and_then(|(x,y)| get_board_index(x,y))
        }
    }

    quickcheck! {
        fn xy_i_xy(x: BoardCoord, y: BoardCoord) -> bool {
             let expected = if xy_on_board(x, y) {
                 Some((x, y))
             } else {
                 None
             };

             expected == get_board_index(x,y).and_then(|i| get_board_xy(i))
        }
    }

}

fn is_index_on_board(piece_index: usize) -> bool {
    piece_index < BOARD_LENGTH
}

fn xy_on_board(x: BoardCoord, y: BoardCoord) -> bool {
    x < BOARD_WIDTH && y < BOARD_HEIGHT
}

//in pixels
pub const CELL_WIDTH: usize = 32;
pub const CELL_HEIGHT: usize = 32;

pub fn cell_x_to_px_x(x: usize) -> usize {
     x * (CELL_WIDTH + 1) + 1
}
pub fn cell_y_to_px_y(y: usize) -> usize {
     y * (CELL_HEIGHT + 1) + 1
}

//A spacer pixel after the last cell.
pub const HUD_LEFT_EDGE: usize = ((BOARD_WIDTH as usize) * (CELL_WIDTH + 1) + 1) + 1;
pub const HUD_WIDTH: usize = SCREEN_WIDTH - HUD_LEFT_EDGE;
