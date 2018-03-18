use common::GameState;
use common::Framebuffer;
use common::Button;
use common::{CELL_HEIGHT, CELL_WIDTH};
use common::Component;

const BLUE: u32 = 0xFFEE2222;
const GREEN: u32 = 0xFF22E22;
const RED: u32 = 0xFF2222EE;

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



            framebuffer.draw_rect(x as _, y as _, CELL_WIDTH, CELL_HEIGHT, BLUE);
        }
    }


}
