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
            let (x, y) = state.positions[i];

            let px_x = cell_x_to_px_x(x as usize);
            let px_y = cell_y_to_px_y(y as usize);

            let colour = state.appearances[i].colour;

            framebuffer.draw_rect(px_x as _, px_y as _, CELL_WIDTH, CELL_HEIGHT, colour);
        }
    }

    framebuffer.draw_rect(HUD_LEFT_EDGE, 0, HUD_WIDTH, SCREEN_HEIGHT, 0xFF333333);
}
