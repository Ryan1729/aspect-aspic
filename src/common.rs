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

macro_rules! red {
    ($colour: expr) => {
        $colour & 0xFF
    };
}

macro_rules! green {
    ($colour: expr) => {
        ($colour & 0xFF_00) >> 8
    };
}

macro_rules! blue {
    ($colour: expr) => {
        ($colour & 0xFF_00_00) >> 16
    };
}

macro_rules! alpha {
    ($colour: expr) => {
        ($colour & 0xFF_00_00_00) >> 24
    };
}

macro_rules! colour {
    ($red: expr, $green: expr, $blue: expr, $alpha: expr) => {
        $red | $green << 8 | $blue << 16 | $alpha << 24
    };
}

macro_rules! set_alpha {
    ($colour: expr, $alpha: expr) => {
        ($colour & 0x00_FF_FF_FF) | $alpha << 24
    };
}

impl Framebuffer {
    pub fn new() -> Framebuffer {
        Framebuffer::default()
    }

    pub fn xy_to_i(x: usize, y: usize) -> usize {
        y.saturating_mul(SCREEN_WIDTH).saturating_add(x)
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize, colour: u32) {
        let one_past_right_edge = x + width;
        let one_past_bottom_edge = y + height;

        for current_y in y..one_past_bottom_edge {
            for current_x in x..one_past_right_edge {
                let i = Framebuffer::xy_to_i(current_x, current_y);
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

    //see http://members.chello.at/~easyfilter/bresenham.html
    pub fn draw_solid_circle(&mut self, xMid: usize, yMid: usize, radius: usize, colour: u32) {
        if xMid < radius || yMid < radius {
            if cfg!(debug_assertions) {
                console!(log, "draw_solid_circle xMid < radius || yMid < radius");
            }

            return;
        }
        let mut r = radius as isize;
        let mut x = -r;
        let mut y = 0isize;
        let mut err = 2 - 2 * r; /* II. Quadrant */
        while {
            self.buffer[Framebuffer::xy_to_i(
                (xMid as isize - x) as usize,
                (yMid as isize + y) as usize,
            )] = colour; /*   I. Quadrant */
            self.buffer[Framebuffer::xy_to_i(
                (xMid as isize - y) as usize,
                (yMid as isize - x) as usize,
            )] = colour; /*  II. Quadrant */
            self.buffer[Framebuffer::xy_to_i(
                (xMid as isize + x) as usize,
                (yMid as isize - y) as usize,
            )] = colour; /* III. Quadrant */
            self.buffer[Framebuffer::xy_to_i(
                (xMid as isize + y) as usize,
                (yMid as isize + x) as usize,
            )] = colour; /*  IV. Quadrant */
            r = err;
            if r <= y {
                y += 1;
                err += y * 2 + 1; /* e_xy+e_y < 0 */
            }
            if r > x || err > y {
                x += 1;
                err += x * 2 + 1; /* e_xy+e_x > 0 or no 2nd y-step */
            }

            x < 0
        } {}
    }

    #[inline]
    //see https://stackoverflow.com/a/12016968/4496839
    pub fn blend(&mut self, i: usize, colour: u32) {
        let background = self.buffer[i];
        let alpha = alpha!(colour) + 1;
        let inv_alpha = 256 - alpha!(colour);
        self.buffer[i] = colour!(
            (alpha * red!(colour) + inv_alpha * red!(background)) >> 8,
            (alpha * green!(colour) + inv_alpha * green!(background)) >> 8,
            (alpha * blue!(colour) + inv_alpha * blue!(background)) >> 8,
            0xFF
        );
    }

    #[inline]
    pub fn blend_xy(&mut self, x: usize, y: usize, colour: u32) {
        self.blend(Framebuffer::xy_to_i(x, y), colour);
    }

    //see http://members.chello.at/easyfilter/bresenham.c
    pub fn draw_circle(&mut self, xMid: usize, yMid: usize, radius: usize, colour: u32) {
        if xMid < radius || yMid < radius {
            if cfg!(debug_assertions) {
                console!(log, "draw_circle xMid < radius || yMid < radius");
            }

            return;
        }
        let xm = xMid as isize;
        let ym = yMid as isize;

        /* II. quadrant from bottom left to top right */
        let mut x: isize = -(radius as isize);
        let mut y: isize = 0;

        let mut alpha;

        /* error of 1.step */
        let mut err: isize = 2 - 2 * (radius as isize);

        //equivalent to 2 * radius - 1
        let diameter = 1 - err;
        while {
            /* get blend value of pixel */
            alpha = 255 * isize::abs(err - 2 * (x + y) - 2) / diameter;

            {
                let new_colour = set_alpha!(colour, 255 - (alpha as u32));

                /*   I. Quadrant */
                self.blend_xy((xm - x) as usize, (ym + y) as usize, new_colour);
                /*  II. Quadrant */
                self.blend_xy((xm - y) as usize, (ym - x) as usize, new_colour);
                /* III. Quadrant */
                self.blend_xy((xm + x) as usize, (ym - y) as usize, new_colour);
                /*  IV. Quadrant */
                self.blend_xy((xm + y) as usize, (ym + x) as usize, new_colour);
            }

            /* remember values */
            let e2 = err;
            let x2 = x;

            /* x step */
            if err + y > 0 {
                alpha = 255 * (err - 2 * x - 1) / diameter;

                /* outward pixel */
                if alpha < 256 {
                    let new_colour = set_alpha!(colour, 255 - (alpha as u32));

                    self.blend_xy((xm - x) as usize, (ym + y + 1) as usize, new_colour);
                    self.blend_xy((xm - y - 1) as usize, (ym - x) as usize, new_colour);
                    self.blend_xy((xm + x) as usize, (ym - y - 1) as usize, new_colour);
                    self.blend_xy((xm + y + 1) as usize, (ym + x) as usize, new_colour);
                }
                x += 1;
                err += x * 2 + 1;
            }

            /* y step */
            if e2 + x2 <= 0 {
                alpha = 255 * (2 * y + 3 - e2) / diameter;

                /* inward pixel */
                if alpha < 256 {
                    let new_colour = set_alpha!(colour, 255 - (alpha as u32));
                    self.blend_xy((xm - x2 - 1) as usize, (ym + y) as usize, new_colour);
                    self.blend_xy((xm - y) as usize, (ym - x2 - 1) as usize, new_colour);
                    self.blend_xy((xm + x2 + 1) as usize, (ym - y) as usize, new_colour);
                    self.blend_xy((xm + y) as usize, (ym + x2 + 1) as usize, new_colour);
                }
                y += 1;
                err += y * 2 + 1;
            }

            x < 0
        } {}
    }
}

impl Default for Framebuffer {
    fn default() -> Self {
        let mut buffer = Vec::new();
        buffer.resize(SCREEN_WIDTH * SCREEN_HEIGHT, 0);

        Framebuffer { buffer }
    }
}

pub const BLUE: u32 = 0xFFEE2222;
pub const GREEN: u32 = 0xFF22EE22;
pub const RED: u32 = 0xFF2222EE;
pub const FLOOR: u32 = 0xFF104010;

//in pixels
pub const CELL_WIDTH: usize = 32;
pub const CELL_HEIGHT: usize = 32;
pub const CELL_DIAMETER: usize = CELL_WIDTH;
pub const CELL_RADIUS: usize = CELL_DIAMETER / 2;

pub fn cell_x_to_px_x(x: usize) -> usize {
    x * (CELL_WIDTH + 1) + 1
}
pub fn cell_y_to_px_y(y: usize) -> usize {
    y * (CELL_HEIGHT + 1) + 1
}

//A spacer pixel after the last cell.
pub const HUD_LEFT_EDGE: usize = ((BOARD_WIDTH as usize) * (CELL_WIDTH + 1) + 1) + 1;
pub const HUD_WIDTH: usize = SCREEN_WIDTH - HUD_LEFT_EDGE;

#[derive(Clone, Copy, Default)]
pub struct Appearance {
    pub colour: u32,
    pub shape: Shape,
    pub offset: (isize, isize),
}

pub fn offset_by(value: usize, offset: isize) -> usize {
    if offset > 0 {
        value.saturating_add(offset as usize)
    } else {
        value.saturating_sub(offset.abs() as usize)
    }
}

impl Appearance {
    pub fn render_intra_positioned(
        &self,
        framebuffer: &mut Framebuffer,
        pos: Position,
        intra_pos: IntraCellPosition,
    ) {
        self.render_positioned_at_offset(framebuffer, pos, intra_pos.get_offset());
    }

    pub fn render_positioned(&self, framebuffer: &mut Framebuffer, pos: Position) {
        self.render_positioned_at_offset(framebuffer, pos, self.offset);
    }

    pub fn render_positioned_at_offset(
        &self,
        framebuffer: &mut Framebuffer,
        (x, y): Position,
        (x_off, y_off): (isize, isize),
    ) {
        let px_x = offset_by(cell_x_to_px_x(x as usize), x_off);
        let px_y = offset_by(cell_y_to_px_y(y as usize), y_off);

        let colour = self.colour;

        match self.shape {
            Shape::FullCell => {
                framebuffer.draw_rect(px_x, px_y, CELL_WIDTH, CELL_HEIGHT, colour);
            }
            Shape::Player => {
                framebuffer.draw_rect(
                    px_x.saturating_add(4),
                    px_y.saturating_add(4),
                    CELL_WIDTH.saturating_sub(8),
                    CELL_HEIGHT.saturating_sub(6),
                    colour,
                );
            }
            Shape::DeadOrb0 => {
                framebuffer.draw_circle(px_x, px_y, CELL_DIAMETER / 9, colour);
            }
            Shape::Blob0 => {
                framebuffer.draw_circle(
                    px_x - CELL_WIDTH / 9,
                    px_y - CELL_HEIGHT / 9,
                    CELL_DIAMETER / 9,
                    colour,
                );
                framebuffer.draw_circle(
                    px_x + CELL_WIDTH / 9,
                    px_y + CELL_HEIGHT / 9,
                    CELL_DIAMETER / 9,
                    colour,
                );
            }
        }
    }

    pub fn is_offset(&self) -> bool {
        let (x_off, y_off) = self.offset;

        x_off != 0 || y_off != 0
    }

    pub fn reduce_offset(&mut self, offset: isize) {
        let (x_off, y_off) = (&mut self.offset.0, &mut self.offset.1);

        if *x_off > 0 {
            *x_off -= offset;
        } else if *x_off < 0 {
            *x_off += offset;
        } else {
            //do nothing
        }

        if *y_off > 0 {
            *y_off -= offset;
        } else if *y_off < 0 {
            *y_off += offset;
        } else {
            //do nothing
        }
    }
}

#[derive(Clone, Copy)]
pub enum Shape {
    FullCell,
    Player,
    DeadOrb0,
    Blob0,
}

impl Default for Shape {
    fn default() -> Self {
        Shape::FullCell
    }
}

pub type Position = (BoardCoord, BoardCoord);

#[derive(Clone, Copy)]
pub enum _2by2 {
    _0_0,
    _0_1,
    _1_0,
    _1_1,
}

#[derive(Clone, Copy)]
pub enum _3by3 {
    _0_0,
    _0_1,
    _0_2,
    _1_0,
    _1_1,
    _1_2,
    _2_0,
    _2_1,
    _2_2,
}

#[derive(Clone, Copy)]
pub enum IntraCellPosition {
    Four(_2by2),
    Nine(_3by3),
}
use IntraCellPosition::*;

impl IntraCellPosition {
    fn get_offset(&self) -> (isize, isize) {
        let w = CELL_WIDTH as isize;
        let h = CELL_HEIGHT as isize;
        match *self {
            Four(pos) => {
                let (x, y) = match pos {
                    _2by2::_0_0 => (0, 0),
                    _2by2::_1_0 => (w / 2, 0),
                    _2by2::_0_1 => (0, h / 2),
                    _2by2::_1_1 => (w / 2, h / 2),
                };

                (x + w / 4, y + h / 4)
            }
            Nine(pos) => {
                let (x, y) = match pos {
                    _3by3::_0_0 => (0, 0),
                    _3by3::_1_0 => (w / 3, 0),
                    _3by3::_2_0 => (w * 2 / 3, 0),
                    _3by3::_0_1 => (0, h / 3),
                    _3by3::_1_1 => (w / 3, h / 3),
                    _3by3::_2_1 => (w * 2 / 3, h / 3),
                    _3by3::_0_2 => (0, h * 2 / 3),
                    _3by3::_1_2 => (w / 3, h * 2 / 3),
                    _3by3::_2_2 => (w * 2 / 3, h * 2 / 3),
                };

                (x + w / 6, y + h / 6)
            }
        }
    }
}

pub struct GameState {
    pub entities: [Component::Ty; GameState::ENTITY_COUNT],

    pub positions: [Position; GameState::ENTITY_COUNT],
    pub appearances: [Appearance; GameState::ENTITY_COUNT],
    pub intra_cell_positions: [IntraCellPosition; GameState::ENTITY_COUNT],
}

impl GameState {
    pub const ENTITY_COUNT: usize = 256;

    pub fn new() -> GameState {
        let mut entities = [Component::Ty::empty(); GameState::ENTITY_COUNT];
        let mut positions = [(0, 0); GameState::ENTITY_COUNT];
        let mut appearances = [Appearance::default(); GameState::ENTITY_COUNT];
        let mut intra_cell_positions = [Four(_2by2::_0_0); GameState::ENTITY_COUNT];

        {
            let mut i = 0;
            while let Some(pos) = get_board_xy(i) {
                entities[i].insert(Component::Position | Component::Appearance);
                positions[i] = pos;
                appearances[i].colour = FLOOR;

                i += 1;
            }
        }

        let playerId = BOARD_LENGTH;

        entities[playerId] |=
            Component::PlayerControlled | Component::Position | Component::Appearance;
        positions[playerId] = get_board_xy(playerId).unwrap_or((0, 0));
        appearances[playerId].colour = BLUE;
        appearances[playerId].shape = Shape::Player;

        let nineCircleIdBase = playerId + 1;

        for circleId in nineCircleIdBase..nineCircleIdBase + 9 {
            entities[circleId] |=
                Component::Position | Component::Appearance | Component::IntraCellPosition;
            positions[circleId] = (3, 4);
            appearances[circleId].colour = RED;
            appearances[circleId].shape = Shape::DeadOrb0;
            intra_cell_positions[circleId] = match circleId - nineCircleIdBase {
                0 => Nine(_3by3::_0_0),
                1 => Nine(_3by3::_0_1),
                2 => Nine(_3by3::_0_2),
                3 => Nine(_3by3::_1_0),
                4 => Nine(_3by3::_1_1),
                5 => Nine(_3by3::_1_2),
                6 => Nine(_3by3::_2_0),
                7 => Nine(_3by3::_2_1),
                _ => Nine(_3by3::_2_2),
            };
        }

        let fourCircleIdBase = nineCircleIdBase + 9;

        for circleId in fourCircleIdBase..fourCircleIdBase + 4 {
            entities[circleId] |=
                Component::Position | Component::Appearance | Component::IntraCellPosition;
            positions[circleId] = (4, 3);
            appearances[circleId].colour = RED;
            appearances[circleId].shape = Shape::Blob0;
            intra_cell_positions[circleId] = match circleId - fourCircleIdBase {
                0 => Four(_2by2::_0_0),
                1 => Four(_2by2::_1_0),
                2 => Four(_2by2::_0_1),
                _ => Four(_2by2::_1_1),
            };
        }

        GameState {
            entities,
            positions,
            appearances,
            intra_cell_positions,
        }
    }
}

pub mod Component {
    bitflags! {
        pub flags Ty: u16 {
            const Position         = 1 << 0,
            const Appearance       = 1 << 1,
            const PlayerControlled = 1 << 2,
            const Player = Position.bits
             | Appearance.bits
             | PlayerControlled.bits,
             const IntraCellPosition = 1 << 3,
        }
    }
}

pub struct State {
    pub game_state: GameState,
    pub framebuffer: Framebuffer,
    pub input: Input,
}

impl State {
    pub fn new() -> State {
        let framebuffer = Framebuffer::new();

        State {
            game_state: GameState::new(),
            framebuffer,
            input: Input::new(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Input {
    pub gamepad: Button::Ty,
    pub previous_gamepad: Button::Ty,
}

impl Input {
    pub fn new() -> Self {
        Input {
            gamepad: Button::Ty::empty(),
            previous_gamepad: Button::Ty::empty(),
        }
    }

    pub fn pressed_this_frame(&self, buttons: Button::Ty) -> bool {
        !self.previous_gamepad.contains(buttons) && self.gamepad.contains(buttons)
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

    let result = (
        index as BoardCoord % BOARD_WIDTH,
        index as BoardCoord / BOARD_WIDTH,
    );

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
