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
