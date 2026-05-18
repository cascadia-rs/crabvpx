static mut dc_qlookup: [i32; 128] = [
    4 as i32, 5 as i32, 6 as i32, 7 as i32, 8 as i32, 9 as i32, 10 as i32, 10 as i32, 11 as i32,
    12 as i32, 13 as i32, 14 as i32, 15 as i32, 16 as i32, 17 as i32, 17 as i32, 18 as i32,
    19 as i32, 20 as i32, 20 as i32, 21 as i32, 21 as i32, 22 as i32, 22 as i32, 23 as i32,
    23 as i32, 24 as i32, 25 as i32, 25 as i32, 26 as i32, 27 as i32, 28 as i32, 29 as i32,
    30 as i32, 31 as i32, 32 as i32, 33 as i32, 34 as i32, 35 as i32, 36 as i32, 37 as i32,
    37 as i32, 38 as i32, 39 as i32, 40 as i32, 41 as i32, 42 as i32, 43 as i32, 44 as i32,
    45 as i32, 46 as i32, 46 as i32, 47 as i32, 48 as i32, 49 as i32, 50 as i32, 51 as i32,
    52 as i32, 53 as i32, 54 as i32, 55 as i32, 56 as i32, 57 as i32, 58 as i32, 59 as i32,
    60 as i32, 61 as i32, 62 as i32, 63 as i32, 64 as i32, 65 as i32, 66 as i32, 67 as i32,
    68 as i32, 69 as i32, 70 as i32, 71 as i32, 72 as i32, 73 as i32, 74 as i32, 75 as i32,
    76 as i32, 76 as i32, 77 as i32, 78 as i32, 79 as i32, 80 as i32, 81 as i32, 82 as i32,
    83 as i32, 84 as i32, 85 as i32, 86 as i32, 87 as i32, 88 as i32, 89 as i32, 91 as i32,
    93 as i32, 95 as i32, 96 as i32, 98 as i32, 100 as i32, 101 as i32, 102 as i32, 104 as i32,
    106 as i32, 108 as i32, 110 as i32, 112 as i32, 114 as i32, 116 as i32, 118 as i32, 122 as i32,
    124 as i32, 126 as i32, 128 as i32, 130 as i32, 132 as i32, 134 as i32, 136 as i32, 138 as i32,
    140 as i32, 143 as i32, 145 as i32, 148 as i32, 151 as i32, 154 as i32, 157 as i32,
];
static mut ac_qlookup: [i32; 128] = [
    4 as i32, 5 as i32, 6 as i32, 7 as i32, 8 as i32, 9 as i32, 10 as i32, 11 as i32, 12 as i32,
    13 as i32, 14 as i32, 15 as i32, 16 as i32, 17 as i32, 18 as i32, 19 as i32, 20 as i32,
    21 as i32, 22 as i32, 23 as i32, 24 as i32, 25 as i32, 26 as i32, 27 as i32, 28 as i32,
    29 as i32, 30 as i32, 31 as i32, 32 as i32, 33 as i32, 34 as i32, 35 as i32, 36 as i32,
    37 as i32, 38 as i32, 39 as i32, 40 as i32, 41 as i32, 42 as i32, 43 as i32, 44 as i32,
    45 as i32, 46 as i32, 47 as i32, 48 as i32, 49 as i32, 50 as i32, 51 as i32, 52 as i32,
    53 as i32, 54 as i32, 55 as i32, 56 as i32, 57 as i32, 58 as i32, 60 as i32, 62 as i32,
    64 as i32, 66 as i32, 68 as i32, 70 as i32, 72 as i32, 74 as i32, 76 as i32, 78 as i32,
    80 as i32, 82 as i32, 84 as i32, 86 as i32, 88 as i32, 90 as i32, 92 as i32, 94 as i32,
    96 as i32, 98 as i32, 100 as i32, 102 as i32, 104 as i32, 106 as i32, 108 as i32, 110 as i32,
    112 as i32, 114 as i32, 116 as i32, 119 as i32, 122 as i32, 125 as i32, 128 as i32, 131 as i32,
    134 as i32, 137 as i32, 140 as i32, 143 as i32, 146 as i32, 149 as i32, 152 as i32, 155 as i32,
    158 as i32, 161 as i32, 164 as i32, 167 as i32, 170 as i32, 173 as i32, 177 as i32, 181 as i32,
    185 as i32, 189 as i32, 193 as i32, 197 as i32, 201 as i32, 205 as i32, 209 as i32, 213 as i32,
    217 as i32, 221 as i32, 225 as i32, 229 as i32, 234 as i32, 239 as i32, 245 as i32, 249 as i32,
    254 as i32, 259 as i32, 264 as i32, 269 as i32, 274 as i32, 279 as i32, 284 as i32,
];
#[unsafe(no_mangle)]
pub unsafe fn vp8_dc_quant(mut QIndex: i32, mut Delta: i32) -> i32 {
    unsafe {
        let mut retval: i32 = 0;
        QIndex += Delta;
        if QIndex > 127 as i32 {
            QIndex = 127 as i32;
        } else if QIndex < 0 as i32 {
            QIndex = 0 as i32;
        }
        retval = dc_qlookup[QIndex as usize];
        retval
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_dc2quant(mut QIndex: i32, mut Delta: i32) -> i32 {
    unsafe {
        let mut retval: i32 = 0;
        QIndex += Delta;
        if QIndex > 127 as i32 {
            QIndex = 127 as i32;
        } else if QIndex < 0 as i32 {
            QIndex = 0 as i32;
        }
        retval = dc_qlookup[QIndex as usize] * 2 as i32;
        retval
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_dc_uv_quant(mut QIndex: i32, mut Delta: i32) -> i32 {
    unsafe {
        let mut retval: i32 = 0;
        QIndex += Delta;
        if QIndex > 127 as i32 {
            QIndex = 127 as i32;
        } else if QIndex < 0 as i32 {
            QIndex = 0 as i32;
        }
        retval = dc_qlookup[QIndex as usize];
        if retval > 132 as i32 {
            retval = 132 as i32;
        }
        retval
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_ac_yquant(mut QIndex: i32) -> i32 {
    unsafe {
        let mut retval: i32 = 0;
        if QIndex > 127 as i32 {
            QIndex = 127 as i32;
        } else if QIndex < 0 as i32 {
            QIndex = 0 as i32;
        }
        retval = ac_qlookup[QIndex as usize];
        retval
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_ac2quant(mut QIndex: i32, mut Delta: i32) -> i32 {
    unsafe {
        let mut retval: i32 = 0;
        QIndex += Delta;
        if QIndex > 127 as i32 {
            QIndex = 127 as i32;
        } else if QIndex < 0 as i32 {
            QIndex = 0 as i32;
        }
        retval = (ac_qlookup[QIndex as usize] * 101581 as i32) >> 16 as i32;
        if retval < 8 as i32 {
            retval = 8 as i32;
        }
        retval
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_ac_uv_quant(mut QIndex: i32, mut Delta: i32) -> i32 {
    unsafe {
        let mut retval: i32 = 0;
        QIndex += Delta;
        if QIndex > 127 as i32 {
            QIndex = 127 as i32;
        } else if QIndex < 0 as i32 {
            QIndex = 0 as i32;
        }
        retval = ac_qlookup[QIndex as usize];
        retval
    }
}
