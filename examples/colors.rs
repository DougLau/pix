mod bmp;

use pix::Raster;
use pix::hwb::SHwb8;
use std::io;

fn main() -> Result<(), io::Error> {
    let mut r = Raster::with_clear(256, 256);
    for (y, row) in r.rows_mut(()).enumerate() {
        for (x, p) in row.iter_mut().enumerate() {
            let h = ((x + y) >> 1) as u8;
            let w = y.saturating_sub(x) as u8;
            let b = x.saturating_sub(y) as u8;
            *p = SHwb8::new(h, w, b);
        }
    }
    bmp::write(&r, "colors.bmp")
}
