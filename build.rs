fn main() {
    let mut build = cc::Build::new();

    build.include("src"); // For vpx_config.h, etc.

    let files = vec![
        "src/vp8/common/arm/loopfilter_arm.c",
        "src/vp8/common/arm/neon/bilinearpredict_neon.c",
        "src/vp8/common/arm/neon/copymem_neon.c",
        "src/vp8/common/arm/neon/dc_only_idct_add_neon.c",
        "src/vp8/common/arm/neon/dequant_idct_neon.c",
        "src/vp8/common/arm/neon/dequantizeb_neon.c",
        "src/vp8/common/arm/neon/idct_blk_neon.c",
        "src/vp8/common/arm/neon/iwalsh_neon.c",
        "src/vp8/common/arm/neon/loopfiltersimplehorizontaledge_neon.c",
        "src/vp8/common/arm/neon/loopfiltersimpleverticaledge_neon.c",
        "src/vp8/common/arm/neon/mbloopfilter_neon.c",
        "src/vp8/common/arm/neon/shortidct4x4llm_neon.c",
        "src/vp8/common/arm/neon/sixtappredict_neon.c",
        "src/vp8/common/arm/neon/vp8_loopfilter_neon.c",
        "src/vpx_dsp/arm/intrapred_neon.c",
    ];

    for file in files {
        // Only add if the file exists to be safe
        if std::path::Path::new(file).exists() {
            build.file(file);
        }
    }

    build.compile("vpx_asm");
}
