#[unsafe(no_mangle)]
pub static vp8_block2left: [u8; 25] = [
    0 as u8, 0 as u8, 0 as u8, 0 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 2 as u8, 2 as u8,
    2 as u8, 2 as u8, 3 as u8, 3 as u8, 3 as u8, 3 as u8, 4 as u8, 4 as u8, 5 as u8, 5 as u8,
    6 as u8, 6 as u8, 7 as u8, 7 as u8, 8 as u8,
];
#[unsafe(no_mangle)]
pub static vp8_block2above: [u8; 25] = [
    0 as u8, 1 as u8, 2 as u8, 3 as u8, 0 as u8, 1 as u8, 2 as u8, 3 as u8, 0 as u8, 1 as u8,
    2 as u8, 3 as u8, 0 as u8, 1 as u8, 2 as u8, 3 as u8, 4 as u8, 5 as u8, 4 as u8, 5 as u8,
    6 as u8, 7 as u8, 6 as u8, 7 as u8, 8 as u8,
];
