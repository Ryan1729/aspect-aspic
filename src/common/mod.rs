pub mod rendering;
pub use rendering::Framebuffer;
pub use rendering::Position;
pub use rendering::Appearance;

pub use rendering::IntraCellPosition::{self, Four, Nine};
pub use rendering::{_2by2, _3by3};

pub use rendering::Shape;

pub mod constants;
pub use constants::*;

pub struct GameState {
    pub entities: [Component::Ty; GameState::ENTITY_COUNT],

    pub positions: [Position; GameState::ENTITY_COUNT],
    pub appearances: [Appearance; GameState::ENTITY_COUNT],
    pub intra_cell_positions: [IntraCellPosition; GameState::ENTITY_COUNT],
    pub player_types: [PlayerType; GameState::ENTITY_COUNT],

    pub mode: Mode,

    //TODO Depending on how much I have to do things like this,
    //consider having multiple ways to retrieve compoents effiecently,
    //e.g. by compoent type, position etc.
    pub selectrixId: usize,
}

impl GameState {
    pub const ENTITY_COUNT: usize = 256;

    pub fn new() -> GameState {
        let mut entities = [Component::Ty::empty(); GameState::ENTITY_COUNT];
        let mut positions = [(0, 0); GameState::ENTITY_COUNT];
        let mut appearances = [Appearance::default(); GameState::ENTITY_COUNT];
        let mut intra_cell_positions = [Four(_2by2::_0_0); GameState::ENTITY_COUNT];
        let mut player_types = [PlayerType::default(); GameState::ENTITY_COUNT];

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
        player_types[playerId] = PlayerType::Avatar;

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

        let solidNineCircleIdBase = fourCircleIdBase + 4;

        for circleId in solidNineCircleIdBase..solidNineCircleIdBase + 9 {
            entities[circleId] |=
                Component::Position | Component::Appearance | Component::IntraCellPosition;
            positions[circleId] = (3, 3);
            appearances[circleId].colour = RED;
            appearances[circleId].shape = Shape::LiveOrb0;
            intra_cell_positions[circleId] = match circleId - solidNineCircleIdBase {
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

        let selectrixId = solidNineCircleIdBase + 9;

        entities[selectrixId] |= Component::Position | Component::Appearance
            | Component::PlayerControlled
            | Component::IntraCellPosition;
        appearances[selectrixId].colour = YELLOW;
        appearances[selectrixId].shape = Shape::Selectrix;
        player_types[selectrixId] = PlayerType::Selectrix;

        GameState {
            entities,
            positions,
            appearances,
            intra_cell_positions,
            player_types,
            mode: Mode::MoveAvatar,
            selectrixId,
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

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum PlayerType {
    NonPlayer,
    Avatar,
    Selectrix,
}

impl Default for PlayerType {
    fn default() -> Self {
        PlayerType::NonPlayer
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Mode {
    MoveAvatar,
    MoveSelectrix,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::MoveAvatar
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
