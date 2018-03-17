//in pixels
pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 240;

pub struct Framebuffer {
    pub buffer: Vec<u16>,
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

    pub fn rainbow(&mut self, x: u8, y: u8) {
        for i in 0..self.buffer.len() {
            self.buffer[i] = (i + x as usize - (y as usize * 2)) as _;
        }
    }
}

impl Default for Framebuffer {
    fn default() -> Self {
        let mut buffer = Vec::new();
        buffer.resize(256 * 240, 0);

        Framebuffer { buffer }
    }
}

pub struct GameState {
    pub x: u8,
    pub y: u8,
    pub board: Board,
}

impl GameState {
    pub fn new() -> GameState {
        let x = 128;
        let y = 120;
        let board = [None; BOARD_LENGTH];

        GameState { x, y, board }
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

    pub fn framebuffer(&self, framebuffer: &mut [u32; 256 * 240]) {
        for (pixel_in, pixel_out) in self.framebuffer.buffer.iter().zip(framebuffer.iter_mut()) {
            let r = Self::beside(((pixel_in & 0x000F) >> 0) as u32);
            let g = Self::beside(((pixel_in & 0x00F0) >> 4) as u32);
            let b = Self::beside(((pixel_in & 0x0F00) >> 8) as u32);
            let a = Self::beside(((pixel_in & 0xF000) >> 12) as u32);

            *pixel_out = r | g << 8 | b << 16 | a << 24
        }
    }

    #[inline]
    fn beside(x: u32) -> u32 {
        x | x << 4
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

type Piece = bool;

//in pixels
pub const CELL_WIDTH: usize = SCREEN_WIDTH / 20;
pub const CELL_HEIGHT: usize = SCREEN_HEIGHT / 20;
pub const HUD_WIDTH: usize = 40;

//in cells
pub const BOARD_WIDTH: usize = (SCREEN_WIDTH - HUD_WIDTH) / CELL_WIDTH;
pub const BOARD_HEIGHT: usize = SCREEN_HEIGHT / CELL_HEIGHT;
pub const BOARD_LENGTH: usize = BOARD_WIDTH * BOARD_HEIGHT;

pub type Board = [Option<Piece>; BOARD_LENGTH];

pub fn get_board_index(x: usize, y: usize) -> Option<usize> {
    if !xy_on_board(x, y) {
        return None;
    }

    let result = y.saturating_mul(BOARD_WIDTH).saturating_add(x);

    if is_index_on_board(result) {
        Some(result)
    } else {
        None
    }
}

pub fn get_board_xy(index: usize) -> Option<(usize, usize)> {
    if !is_index_on_board(index) {
        return None;
    }

    let result = (index % BOARD_WIDTH, index / BOARD_WIDTH);

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
        fn xy_i_xy(x: usize, y: usize) -> bool {
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

fn xy_on_board(x: usize, y: usize) -> bool {
    x < BOARD_WIDTH && y < BOARD_HEIGHT
}
