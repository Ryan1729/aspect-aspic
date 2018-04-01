#![allow(dead_code)] //Who cares if there are unused constants?

pub type BoardCoord = u8;

//in cells
pub const BOARD_WIDTH: BoardCoord = 6;
pub const BOARD_HEIGHT: BoardCoord = 6;

pub const BOARD_LENGTH: usize = BOARD_WIDTH as usize * BOARD_HEIGHT as usize;

//in pixels
pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 240;

// reportedly colourblind friendly colours
// https://twitter.com/ea_accessible/status/968595073184092160
pub const BLUE: u32 = 0xFFE15233;
pub const GREEN: u32 = 0xFF6EB030;
pub const RED: u32 = 0xFF4949DE;
pub const YELLOW: u32 = 0xFF37B9FF;
pub const PURPLE: u32 = 0xFF543353;
pub const GREY: u32 = 0xFF8B7D5A;
pub const GRAY: u32 = GREY;

pub const FLOOR: u32 = PURPLE;

//in pixels
pub const CELL_WIDTH: usize = 32;
pub const CELL_HEIGHT: usize = 32;
pub const CELL_DIAMETER: usize = CELL_WIDTH;
pub const CELL_RADIUS: usize = CELL_DIAMETER / 2;

//A spacer pixel after the last cell.
pub const HUD_LEFT_EDGE: usize = ((BOARD_WIDTH as usize) * (CELL_WIDTH + 1) + 1) + 1;
pub const HUD_WIDTH: usize = SCREEN_WIDTH - HUD_LEFT_EDGE;

pub const INVENTORY_WIDTH: usize = HUD_WIDTH / 3;
pub const INVENTORY_HEIGHT: usize = INVENTORY_WIDTH;
pub const INVENTORY_LEFT_EDGE: usize = HUD_LEFT_EDGE + INVENTORY_WIDTH;

pub const ORB_RADIUS: usize = CELL_DIAMETER / 9;
