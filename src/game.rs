use common::*;

#[inline]
pub fn update_and_render(
    state: &mut GameState,
    framebuffer: &mut Framebuffer,
    gamepad: Button::Ty,
) {
    for i in 0..GameState::ENTITY_COUNT {
        let entity = state.entities[i];
        if entity.contains(Component::PlayerControlled) {
            let (mut x, mut y) = state.positions[i];

            if gamepad.contains(Button::Left) {
                x = x.wrapping_sub(1);
            }

            if gamepad.contains(Button::Right) {
                x = x.wrapping_add(1);
            }

            if gamepad.contains(Button::Up) {
                y = y.wrapping_sub(1);
            }

            if gamepad.contains(Button::Down) {
                y = y.wrapping_add(1);
            }

            state.positions[i] = (x, y);
        }
    }

    framebuffer.clear();

    for i in 0..GameState::ENTITY_COUNT {
        let entity = state.entities[i];
        if entity.contains(Component::Position | Component::Appearance) {
            let pos = state.positions[i];

            let appearance = &mut state.appearances[i];
            appearance.render_positioned(framebuffer, pos);
        }
    }

    framebuffer.draw_rect(HUD_LEFT_EDGE, 0, HUD_WIDTH, SCREEN_HEIGHT, 0xFF333333);
}
