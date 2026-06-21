static dc_qlookup: [i32; 128] = [
    4_i32, 5_i32, 6_i32, 7_i32, 8_i32, 9_i32, 10_i32, 10_i32, 11_i32, 12_i32, 13_i32, 14_i32,
    15_i32, 16_i32, 17_i32, 17_i32, 18_i32, 19_i32, 20_i32, 20_i32, 21_i32, 21_i32, 22_i32, 22_i32,
    23_i32, 23_i32, 24_i32, 25_i32, 25_i32, 26_i32, 27_i32, 28_i32, 29_i32, 30_i32, 31_i32, 32_i32,
    33_i32, 34_i32, 35_i32, 36_i32, 37_i32, 37_i32, 38_i32, 39_i32, 40_i32, 41_i32, 42_i32, 43_i32,
    44_i32, 45_i32, 46_i32, 46_i32, 47_i32, 48_i32, 49_i32, 50_i32, 51_i32, 52_i32, 53_i32, 54_i32,
    55_i32, 56_i32, 57_i32, 58_i32, 59_i32, 60_i32, 61_i32, 62_i32, 63_i32, 64_i32, 65_i32, 66_i32,
    67_i32, 68_i32, 69_i32, 70_i32, 71_i32, 72_i32, 73_i32, 74_i32, 75_i32, 76_i32, 76_i32, 77_i32,
    78_i32, 79_i32, 80_i32, 81_i32, 82_i32, 83_i32, 84_i32, 85_i32, 86_i32, 87_i32, 88_i32, 89_i32,
    91_i32, 93_i32, 95_i32, 96_i32, 98_i32, 100_i32, 101_i32, 102_i32, 104_i32, 106_i32, 108_i32,
    110_i32, 112_i32, 114_i32, 116_i32, 118_i32, 122_i32, 124_i32, 126_i32, 128_i32, 130_i32,
    132_i32, 134_i32, 136_i32, 138_i32, 140_i32, 143_i32, 145_i32, 148_i32, 151_i32, 154_i32,
    157_i32,
];
static ac_qlookup: [i32; 128] = [
    4_i32, 5_i32, 6_i32, 7_i32, 8_i32, 9_i32, 10_i32, 11_i32, 12_i32, 13_i32, 14_i32, 15_i32,
    16_i32, 17_i32, 18_i32, 19_i32, 20_i32, 21_i32, 22_i32, 23_i32, 24_i32, 25_i32, 26_i32, 27_i32,
    28_i32, 29_i32, 30_i32, 31_i32, 32_i32, 33_i32, 34_i32, 35_i32, 36_i32, 37_i32, 38_i32, 39_i32,
    40_i32, 41_i32, 42_i32, 43_i32, 44_i32, 45_i32, 46_i32, 47_i32, 48_i32, 49_i32, 50_i32, 51_i32,
    52_i32, 53_i32, 54_i32, 55_i32, 56_i32, 57_i32, 58_i32, 60_i32, 62_i32, 64_i32, 66_i32, 68_i32,
    70_i32, 72_i32, 74_i32, 76_i32, 78_i32, 80_i32, 82_i32, 84_i32, 86_i32, 88_i32, 90_i32, 92_i32,
    94_i32, 96_i32, 98_i32, 100_i32, 102_i32, 104_i32, 106_i32, 108_i32, 110_i32, 112_i32, 114_i32,
    116_i32, 119_i32, 122_i32, 125_i32, 128_i32, 131_i32, 134_i32, 137_i32, 140_i32, 143_i32,
    146_i32, 149_i32, 152_i32, 155_i32, 158_i32, 161_i32, 164_i32, 167_i32, 170_i32, 173_i32,
    177_i32, 181_i32, 185_i32, 189_i32, 193_i32, 197_i32, 201_i32, 205_i32, 209_i32, 213_i32,
    217_i32, 221_i32, 225_i32, 229_i32, 234_i32, 239_i32, 245_i32, 249_i32, 254_i32, 259_i32,
    264_i32, 269_i32, 274_i32, 279_i32, 284_i32,
];
pub fn vp8_dc_quant(mut QIndex: i32, mut Delta: i32) -> i32 {
    let mut retval: i32 = 0;
    QIndex += Delta;
    QIndex = QIndex.clamp(0, 127);
    retval = dc_qlookup[QIndex as usize];
    retval
}
pub fn vp8_dc2quant(mut QIndex: i32, mut Delta: i32) -> i32 {
    let mut retval: i32 = 0;
    QIndex += Delta;
    QIndex = QIndex.clamp(0, 127);
    retval = dc_qlookup[QIndex as usize] * 2_i32;
    retval
}
pub fn vp8_dc_uv_quant(mut QIndex: i32, mut Delta: i32) -> i32 {
    let mut retval: i32 = 0;
    QIndex += Delta;
    QIndex = QIndex.clamp(0, 127);
    retval = dc_qlookup[QIndex as usize];
    if retval > 132_i32 {
        retval = 132_i32;
    }
    retval
}
pub fn vp8_ac_yquant(mut QIndex: i32) -> i32 {
    let mut retval: i32 = 0;
    QIndex = QIndex.clamp(0, 127);
    retval = ac_qlookup[QIndex as usize];
    retval
}
pub fn vp8_ac2quant(mut QIndex: i32, mut Delta: i32) -> i32 {
    let mut retval: i32 = 0;
    QIndex += Delta;
    QIndex = QIndex.clamp(0, 127);
    retval = (ac_qlookup[QIndex as usize] * 101581_i32) >> 16_i32;
    if retval < 8_i32 {
        retval = 8_i32;
    }
    retval
}
pub fn vp8_ac_uv_quant(mut QIndex: i32, mut Delta: i32) -> i32 {
    let mut retval: i32 = 0;
    QIndex += Delta;
    QIndex = QIndex.clamp(0, 127);
    retval = ac_qlookup[QIndex as usize];
    retval
}
