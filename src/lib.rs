#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(clashing_extern_declarations)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::precedence)]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::module_inception)]
#![allow(clippy::needless_return)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::misrefactored_assign_op)]
#![allow(clippy::unnecessary_unwrap)]
#![feature(c_variadic)]
#![feature(extern_types)]

pub mod api;
pub mod thread_shim;
pub mod vp8 {
    pub mod common {
        pub mod alloccommon;
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
    pub mod vpx_write_yuv_frame;
} // mod vpx_util
