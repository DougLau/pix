// mask.rs    A 2D image mask.
//
// Copyright (c) 2017-2019  Douglas P Lau
//
use std::fs::File;
use std::io;
use std::io::Write;
use std::ptr;

/// A Mask is an image with only an 8-bit alpha channel.
///
pub struct Mask {
    width  : u32,
    height : u32,
    pixels : Vec<u8>,
}

impl Mask {
    /// Create a new mask
    ///
    /// * `width` Width in pixels.
    /// * `height` Height in pixels.
    pub fn new(width: u32, height: u32) -> Mask {
        let len = (width * height) as usize;
        // Capacity must be 8-element multiple (for SIMD)
        let cap = ((len + 7) >> 3) << 3;
        let mut pixels = vec![0; cap];
        // Remove excess pixels
        for _ in 0..cap-len { pixels.pop(); };
        Mask { width, height, pixels }
    }
    /// Get mask width.
    pub fn width(&self) -> u32 {
        self.width
    }
    /// Get mask height.
    pub fn height(&self) -> u32 {
        self.height
    }
    /// Get pixel slice
    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }
    /// Clear the mask.
    pub fn clear(&mut self) {
        let len = self.pixels.len();
        self.fill(0, len, 0);
    }
    /// Fill a range of pixels with a single value
    pub fn fill(&mut self, x: usize, len: usize, v: u8) {
        assert!(x + len <= self.pixels.len());
        unsafe {
            let pix = self.pixels.as_mut_ptr().offset(x as isize);
            ptr::write_bytes(pix, v, len);
        }
    }
    /// Get one scan line (row)
    pub fn scan_line(&mut self, row: u32) -> &mut [u8] {
        let s = (row * self.width) as usize;
        let t = s + self.width as usize;
        &mut self.pixels[s..t]
    }
    /// Write the mask to a PGM (portable gray map) file.
    ///
    /// * `filename` Name of file to write.
    pub fn write_pgm(&self, filename: &str) -> io::Result<()> {
        let fl = File::create(filename)?;
        let mut bw = io::BufWriter::new(fl);
        let w = bw.get_mut();
        w.write_all(format!("P5\n{} {}\n255\n", self.width, self.height)
         .as_bytes())?;
        w.write_all(&self.pixels[..])?;
        w.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Mask;
    #[test]
    fn test_mask() {
        let mut m = Mask::new(10, 10);
        m.clear();
        assert!(m.width == 10);
        assert!(m.height == 10);
        assert!(m.pixels.len() == 100);
        m.fill(40, 20, 255);
        assert!(m.pixels[0] == 0);
        assert!(m.pixels[39] == 0);
        assert!(m.pixels[40] == 255);
        assert!(m.pixels[59] == 255);
        assert!(m.pixels[60] == 0);
        assert!(m.pixels[99] == 0);
    }
}
