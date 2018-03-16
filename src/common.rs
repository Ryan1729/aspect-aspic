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

pub struct State {
    pub framebuffer: Framebuffer,
    pub gamepad: Button::Ty,
    pub game_state: GameState,
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

pub struct GameState {
    pub x: u8,
    pub y: u8,
}

impl GameState {
    pub fn new() -> GameState {
        let x = 128;
        let y = 120;

        GameState { x, y }
    }
}
