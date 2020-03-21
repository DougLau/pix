// build.rs     Create look-up tables
//
// Copyright (c) 2020  Douglas P Lau
//
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

include!("src/srgb_gamma.rs");

/// Create sRGB gamma look-up tables
fn gamma_lut() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("gamma_lut.rs");
    let mut w = BufWriter::new(File::create(dest_path).unwrap());
    writeln!(w, "const ENCODE_SRGB_U8: &[u8] = &[").unwrap();
    for i in 0..256 {
        if i % 8 == 0 {
            write!(w, "    ").unwrap();
        }
        let s = i as f32 / 255.0;
        let v = (srgb_gamma_encode(s) * 255.0).round() as u8;
        write!(w, "0x{:02X?}, ", v).unwrap();
        if i % 8 == 7 {
            writeln!(w).unwrap();
        }
    }
    writeln!(w, "];").unwrap();
    writeln!(w, "const DECODE_SRGB_U8: &[u8] = &[").unwrap();
    for i in 0..256 {
        if i % 8 == 0 {
            write!(w, "    ").unwrap();
        }
        let s = i as f32 / 255.0;
        let v = (srgb_gamma_decode(s) * 255.0).round() as u8;
        write!(w, "0x{:02X?}, ", v).unwrap();
        if i % 8 == 7 {
            writeln!(w).unwrap();
        }
    }
    writeln!(w, "];").unwrap();

    println!("cargo:rerun-if-changed=src/srgb_gamma.rs");
}

fn main() {
    gamma_lut();

    println!("cargo:rerun-if-changed=build.rs");
}
