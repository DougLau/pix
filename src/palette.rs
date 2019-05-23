// palette.rs   Color palette
//
// Copyright (c) 2019  Douglas P Lau
//
use crate::{Ch8, Format};

/// Color table for use with indexed `Raster`s.
#[derive(Clone)]
pub struct Palette<F>
    where F: Format<Chan = Ch8>
{
    capacity: usize,
    table: Vec<F>,
}

impl<F> Palette<F>
    where F: Format<Chan = Ch8>
{
    /// Create a new color `Palette`.
    ///
    /// * `capacity` Maximum number of entries.
    pub fn new(capacity: usize) -> Self {
        let table = Vec::with_capacity(capacity);
        Palette { capacity, table }
    }
    /// Get view of a color slice as a `u8` slice.
    fn u8_slice(colors: &[F]) -> &[u8] {
        unsafe { colors.align_to::<u8>().1 }
    }
    /// Get view of `Palette` as a `u8` slice.
    pub fn as_u8_slice(&self) -> &[u8] {
        Self::u8_slice(&self.table)
    }
    /// Get a `Palette` entry.
    ///
    /// * `i` Index of entry.
    pub fn entry(&self, i: usize) -> Option<F> {
        if i < self.table.len() {
            Some(self.table[i])
        } else {
            None
        }
    }
    /// Set a `Palette` entry.
    ///
    /// The table is searched for a matching color.  If not found, a new entry
    /// is added.
    ///
    /// * `clr` Color to lookup or add.
    ///
    /// # Returns
    /// An index is returned if matching color is found or an entry is added.
    /// Otherwise, when the table is full, `None` is returned.
    pub fn set_entry(&mut self, clr: F) -> Option<usize> {
        for (i, c) in self.table.iter().enumerate() {
            if clr == *c {
                return Some(i);
            }
        }
        let i = self.table.len();
        if i < self.capacity {
            self.table.push(clr);
            Some(i)
        } else {
            None
        }
    }
    /// Replace a `Palette` entry.
    ///
    /// * `i` Index of entry.
    /// * `clr` Color to replace entry with.
    ///
    /// # Returns
    /// Previous entry, or `None` if index is larger than table size.
    pub fn replace_entry(&mut self, i: usize, clr: F) -> Option<F> {
        if i < self.table.len() {
            let old = self.table[i];
            self.table[i] = clr;
            Some(old)
        } else {
            None
        }
    }
    /// Create a histogram of `Palette` entries.
    ///
    /// * `ent` Slice of entry indices (pixel values).
    pub fn histogram<T>(&self, ent: &[T]) -> Option<Vec<usize>>
        where T: Copy, usize: From<T>
    {
        let mut hist = vec![0; self.table.len()];
        for e in ent {
            let i = usize::from(*e);
            if i < hist.len() {
                hist[i] += 1;
            } else {
                return None;
            }
        }
        Some(hist)
    }
}

#[cfg(test)]
mod test {
    use super::super::*;
    #[test]
    fn fill_16() {
        let mut p = Palette::new(16);
        assert_eq!(p.entry(4), None);
        // test insertion
        for i in 0..16 {
            let idx = p.set_entry(Rgb8::new(i, i, i)).unwrap();
            assert_eq!(i as usize, idx);
        }
        assert_eq!(p.set_entry(Rgb8::new(255, 255, 255)), None);
        // test lookup
        for i in 0..16 {
            let idx = p.set_entry(Rgb8::new(i, i, i)).unwrap();
            assert_eq!(i as usize, idx);
        }
        assert_eq!(p.entry(5), Some(Rgb8::new(5, 5, 5)));
        p.replace_entry(5, Rgb8::new(0x55, 0x55, 0x55));
        let v = vec![
            0x00,0x00,0x00, 0x01,0x01,0x01, 0x02,0x02,0x02, 0x03,0x03,0x03,
            0x04,0x04,0x04, 0x55,0x55,0x55, 0x06,0x06,0x06, 0x07,0x07,0x07,
            0x08,0x08,0x08, 0x09,0x09,0x09, 0x0A,0x0A,0x0A, 0x0B,0x0B,0x0B,
            0x0C,0x0C,0x0C, 0x0D,0x0D,0x0D, 0x0E,0x0E,0x0E, 0x0F,0x0F,0x0F,
        ];
        assert_eq!(p.as_u8_slice(), &v[..]);
    }
    #[test]
    fn check_hist() {
        let mut p = Palette::new(8);
        for i in 0..7 {
            p.set_entry(Rgb8::new(i, i, i));
        }
        let v: Vec<u8> = vec![
            0x00,0x00,0x00,0x00,0x04,0x00,0x00,0x01,0x02,0x04,0x01,0x02,
            0x02,0x04,0x02,0x06,0x04,0x03,0x03,0x03,0x06,0x03,0x01,0x02,
            0x01,0x02,0x04,0x00,0x04,0x00,0x00,0x00,0x00,0x00,0x01,0x02,
            0x00,0x00,0x00,0x02,0x02,0x04,0x01,0x00,0x00,0x00,0x02,0x04,
        ];
        assert_eq!(p.histogram(&v[..]), Some(vec![18, 6, 10, 4, 8, 0, 2]));
    }
}
