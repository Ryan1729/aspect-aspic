use common::GameState;
use common::Framebuffer;
use common::Button;

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

    framebuffer.rainbow(state.x, state.y);
}
