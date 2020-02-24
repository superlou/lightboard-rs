use ggez::event::KeyCode;

pub fn mutate_from_key(buffer: &mut String, key: KeyCode) {
    match key {
        KeyCode::A => buffer.push('a'),
        KeyCode::B => buffer.push('b'),
        KeyCode::C => buffer.push('c'),
        KeyCode::D => buffer.push('d'),
        KeyCode::E => buffer.push('e'),
        KeyCode::F => buffer.push('f'),
        KeyCode::G => buffer.push('g'),
        KeyCode::H => buffer.push('h'),
        KeyCode::I => buffer.push('i'),
        KeyCode::J => buffer.push('j'),
        KeyCode::K => buffer.push('k'),
        KeyCode::L => buffer.push('l'),
        KeyCode::M => buffer.push('m'),
        KeyCode::N => buffer.push('n'),
        KeyCode::O => buffer.push('o'),
        KeyCode::P => buffer.push('p'),
        KeyCode::Q => buffer.push('q'),
        KeyCode::R => buffer.push('r'),
        KeyCode::S => buffer.push('s'),
        KeyCode::T => buffer.push('t'),
        KeyCode::U => buffer.push('u'),
        KeyCode::V => buffer.push('v'),
        KeyCode::W => buffer.push('w'),
        KeyCode::X => buffer.push('x'),
        KeyCode::Y => buffer.push('y'),
        KeyCode::Z => buffer.push('z'),
        KeyCode::Key0 | KeyCode::Numpad0 => buffer.push('0'),
        KeyCode::Key1 | KeyCode::Numpad1 => buffer.push('1'),
        KeyCode::Key2 | KeyCode::Numpad2 => buffer.push('2'),
        KeyCode::Key3 | KeyCode::Numpad3 => buffer.push('3'),
        KeyCode::Key4 | KeyCode::Numpad4 => buffer.push('4'),
        KeyCode::Key5 | KeyCode::Numpad5 => buffer.push('5'),
        KeyCode::Key6 | KeyCode::Numpad6 => buffer.push('6'),
        KeyCode::Key7 | KeyCode::Numpad7 => buffer.push('7'),
        KeyCode::Key8 | KeyCode::Numpad8 => buffer.push('8'),
        KeyCode::Key9 | KeyCode::Numpad9 => buffer.push('9'),
        KeyCode::Space => buffer.push(' '),
        KeyCode::Back => {let _ = buffer.pop();},
        KeyCode::Escape => buffer.clear(),
        _ => (),
    }
}
