use pix::gray::{Gray, SGray8};
use pix::Raster;
use std::fs::File;
use std::io;
use std::io::Write;

fn main() -> Result<(), io::Error> {
    let v = SGray8::new(255);
    let mut r = Raster::with_clear(16, 16);
    for y in 0..16 {
        for x in 0..16 {
            if x + y & 1 != 0 {
                *r.pixel_mut(x, y) = v;
            }
        }
    }
    write_pgm(&r, "checker.pgm")
}

fn write_pgm(raster: &Raster<SGray8>, filename: &str) -> io::Result<()> {
    let mut buf = [0u8; 1];
    let fl = File::create(filename)?;
    let mut bw = io::BufWriter::new(fl);
    let w = bw.get_mut();
    write!(w, "P5\n{} {}\n255\n", raster.width(), raster.height())?;
    for p in raster.pixels() {
        buf[0] = u8::from(Gray::value(*p));
        w.write_all(&buf[..])?;
    }
    w.flush()?;
    Ok(())
}
