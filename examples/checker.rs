use pix::Raster;
use pix::gray::SGray8;
use std::fs::File;
use std::io;
use std::io::Write;

fn main() -> Result<(), io::Error> {
    let v = SGray8::new(255);
    let mut r = Raster::<SGray8>::with_clear(16, 16);
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
    let fl = File::create(filename)?;
    let mut bw = io::BufWriter::new(fl);
    let w = bw.get_mut();
    w.write_all(
        format!("P5\n{} {}\n255\n", raster.width(), raster.height()).as_bytes(),
    )?;
    w.write_all(&raster.as_u8_slice())?;
    w.flush()?;
    Ok(())
}
