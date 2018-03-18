use common::GameState;
use common::Framebuffer;
use common::Button;
use common::{CELL_HEIGHT, CELL_WIDTH};

const BLUE: u32 = 0xFFEE2222;
//const GREEN: u16 = 0xF2E2;
//const RED: u16 = 0xF22E;

#[inline]
pub fn update_and_render(
    state: &mut GameState,
    framebuffer: &mut Framebuffer,
    gamepad: Button::Ty,
) {
    if gamepad.contains(Button::Left) {
        state.x = state.x.wrapping_sub(1);
    }

    if gamepad.contains(Button::Right) {
        state.x = state.x.wrapping_add(1);
    }

    if gamepad.contains(Button::Up) {
        state.y = state.y.wrapping_sub(1);
    }

    if gamepad.contains(Button::Down) {
        state.y = state.y.wrapping_add(1);
    }

    framebuffer.clear();

    framebuffer.draw_rect(state.x, state.y, CELL_WIDTH, CELL_HEIGHT, BLUE);
}
