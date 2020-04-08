use enigo::*;

fn main() {
    let mut enigo = Enigo::new();
    //enigo.key_click(Key::Meta);
    //paste
    enigo.mouse_move_to(1000, 800);
    enigo.mouse_click(MouseButton::Right);
}
