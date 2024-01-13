use crate::keys::KeyboardKey;

pub const KBD_ROWS: usize = 3;
pub const KBD_COLS: usize = 5;

pub const KBD_MATRIX: [[KeyboardKey; KBD_ROWS]; KBD_COLS] = [[KeyboardKey::None; 3]; 5];
