#![allow(dead_code)] //Who cares if there are unused constants?

pub type BoardCoord = u8;

//in cells
pub const BOARD_WIDTH: BoardCoord = 6;
pub const BOARD_HEIGHT: BoardCoord = 6;

pub const BOARD_LENGTH: usize = BOARD_WIDTH as usize * BOARD_HEIGHT as usize;

//in pixels
pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 240;

pub const BLUE: u32 = 0xFFEE2222;
pub const GREEN: u32 = 0xFF22EE33;
pub const RED: u32 = 0xFF2222EE;
pub const YELLOW: u32 = 0xFF33CCCC;
pub const FLOOR: u32 = 0xFF104010;

//in pixels
pub const CELL_WIDTH: usize = 32;
pub const CELL_HEIGHT: usize = 32;
pub const CELL_DIAMETER: usize = CELL_WIDTH;
pub const CELL_RADIUS: usize = CELL_DIAMETER / 2;

//A spacer pixel after the last cell.
pub const HUD_LEFT_EDGE: usize = ((BOARD_WIDTH as usize) * (CELL_WIDTH + 1) + 1) + 1;
pub const HUD_WIDTH: usize = SCREEN_WIDTH - HUD_LEFT_EDGE;
