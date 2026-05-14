#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(raw_ref_op)]

pub mod vp8 {
    pub mod common {
        pub mod alloccommon;
        pub mod arm {
            pub mod loopfilter_arm;
            pub mod neon {
                pub mod bilinearpredict_neon;
                pub mod copymem_neon;
                pub mod dc_only_idct_add_neon;
                pub mod dequant_idct_neon;
                pub mod dequantizeb_neon;
                pub mod idct_blk_neon;
                pub mod iwalsh_neon;
                pub mod loopfiltersimplehorizontaledge_neon;
                pub mod loopfiltersimpleverticaledge_neon;
                pub mod mbloopfilter_neon;
                pub mod shortidct4x4llm_neon;
                pub mod sixtappredict_neon;
                pub mod vp8_loopfilter_neon;
            } // mod neon
        } // mod arm
        pub mod blockd;
        pub mod dequantize;
        pub mod entropy;
        pub mod entropymode;
        pub mod entropymv;
        pub mod extend;
        pub mod filter;
        pub mod findnearmv;
        pub mod generic {
            pub mod systemdependent;
        } // mod generic
        pub mod idct_blk;
        pub mod idctllm;
        pub mod loopfilter_filters;
        pub mod mbpitch;
        pub mod modecont;
        pub mod quant_common;
        pub mod reconinter;
        pub mod reconintra;
        pub mod reconintra4x4;
        pub mod rtcd;
        pub mod setupintrarecon;
        pub mod swapyv12buffer;
        pub mod treecoder;
        pub mod vp8_loopfilter;
    } // mod common
    pub mod decoder {
        pub mod dboolhuff;
        pub mod decodeframe;
        pub mod decodemv;
        pub mod detokenize;
        pub mod onyxd_if;
        pub mod threading;
    } // mod decoder
    pub mod vp8_dx_iface;
} // mod vp8
pub mod vpx {
    pub mod src {
        pub mod vpx_codec;
        pub mod vpx_decoder;
        pub mod vpx_encoder;
        pub mod vpx_image;
    } // mod src
} // mod vpx
pub mod vpx_config;
pub mod vpx_dsp {
    pub mod arm {
        pub mod intrapred_neon;
    } // mod arm
    pub mod bitreader;
    pub mod bitreader_buffer;
    pub mod intrapred;
    pub mod prob;
    pub mod skin_detection;
    pub mod vpx_dsp_rtcd;
} // mod vpx_dsp
pub mod vpx_mem {
    pub mod vpx_mem;
} // mod vpx_mem
pub mod vpx_ports {
    pub mod aarch64_cpudetect;
} // mod vpx_ports
pub mod vpx_scale {
    pub mod generic {
        pub mod gen_scalers;
        pub mod vpx_scale;
        pub mod yv12config;
        pub mod yv12extend;
    } // mod generic
    pub mod vpx_scale_rtcd;
} // mod vpx_scale
pub mod vpx_util {
    pub mod vpx_thread;
    pub mod vpx_write_yuv_frame;
} // mod vpx_util
