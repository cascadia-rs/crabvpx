#[derive(Copy, Clone)]
#[repr(C)]
pub struct MvContext {
    pub prob: [u8; 19],
}
#[unsafe(no_mangle)]
pub static vp8_mv_update_probs: [MvContext; 2] = [
    MvContext {
        prob: [
            237 as u8, 246 as u8, 253 as u8, 253 as u8, 254 as u8, 254 as u8, 254 as u8, 254 as u8,
            254 as u8, 254 as u8, 254 as u8, 254 as u8, 254 as u8, 254 as u8, 250 as u8, 250 as u8,
            252 as u8, 254 as u8, 254 as u8,
        ],
    },
    MvContext {
        prob: [
            231 as u8, 243 as u8, 245 as u8, 253 as u8, 254 as u8, 254 as u8, 254 as u8, 254 as u8,
            254 as u8, 254 as u8, 254 as u8, 254 as u8, 254 as u8, 254 as u8, 251 as u8, 251 as u8,
            254 as u8, 254 as u8, 254 as u8,
        ],
    },
];
#[unsafe(no_mangle)]
pub static vp8_default_mv_context: [MvContext; 2] = [
    MvContext {
        prob: [
            162 as u8, 128 as u8, 225 as u8, 146 as u8, 172 as u8, 147 as u8, 214 as u8, 39 as u8,
            156 as u8, 128 as u8, 129 as u8, 132 as u8, 75 as u8, 145 as u8, 178 as u8, 206 as u8,
            239 as u8, 254 as u8, 254 as u8,
        ],
    },
    MvContext {
        prob: [
            164 as u8, 128 as u8, 204 as u8, 170 as u8, 119 as u8, 235 as u8, 140 as u8, 230 as u8,
            228 as u8, 128 as u8, 130 as u8, 130 as u8, 74 as u8, 148 as u8, 180 as u8, 203 as u8,
            236 as u8, 254 as u8, 254 as u8,
        ],
    },
];
