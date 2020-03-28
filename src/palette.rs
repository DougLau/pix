// palette.rs   Color palette
//
// Copyright (c) 2019-2020  Douglas P Lau
//
use crate::rgb::Rgb;
use crate::SRgb8;

/// Color table for use with indexed `Raster`s.
#[derive(Clone)]
pub struct Palette {
    table: Vec<SRgb8>,
    threshold_fn: fn(usize) -> SRgb8,
}

impl Palette {
    /// Create a new color `Palette`.
    ///
    /// * `capacity` Maximum number of entries.
    pub fn new(capacity: usize) -> Self {
        let table = Vec::with_capacity(capacity);
        let threshold_fn = |_| SRgb8::default();
        Palette {
            table,
            threshold_fn,
        }
    }
    /// Get the number of entries.
    pub fn len(&self) -> usize {
        self.table.len()
    }
    /// Check if the palette is empty.
    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }
    /// Set the threshold function for matching entries.
    ///
    /// * `threshold_fn` Called when checking whether a color matches an
    ///                  existing entry.  The parameter is the palette table
    ///                  size.  Returns the maximum `Channel`-wise difference
    ///                  to match.
    pub fn set_threshold_fn(&mut self, threshold_fn: fn(usize) -> SRgb8) {
        self.threshold_fn = threshold_fn;
    }
    /// Get view of a color slice as a `u8` slice.
    fn u8_slice(colors: &[SRgb8]) -> &[u8] {
        unsafe { colors.align_to::<u8>().1 }
    }
    /// Get view of `Palette` as a `u8` slice.
    pub fn as_u8_slice(&self) -> &[u8] {
        Self::u8_slice(&self.table)
    }
    /// Get a `Palette` entry.
    ///
    /// * `i` Index of entry.
    pub fn entry(&self, i: usize) -> Option<SRgb8> {
        if i < self.table.len() {
            Some(self.table[i])
        } else {
            None
        }
    }
    /// Set a `Palette` entry.
    ///
    /// The table is searched for the best matching color within the threshold.
    /// If none found, a new entry is added.
    ///
    /// * `clr` Color to lookup or add.
    ///
    /// # Returns
    /// Index of best matching or added entry if successful.  Otherwise, when
    /// no matches are found and the table is full, `None` is returned.
    pub fn set_entry(&mut self, clr: SRgb8) -> Option<usize> {
        if let Some((i, dif)) = self.best_match(clr) {
            if Rgb::within_threshold(dif, (self.threshold_fn)(self.table.len()))
            {
                return Some(i);
            }
        }
        let i = self.table.len();
        if i < self.table.capacity() {
            self.table.push(clr);
            Some(i)
        } else {
            None
        }
    }
    /// Find the best match for a color.
    ///
    /// The first of equal matches will be returned.
    fn best_match(&self, clr: SRgb8) -> Option<(usize, SRgb8)> {
        let mut best = None;
        for (i, c) in self.table.iter().enumerate() {
            let dif = Rgb::difference(clr, *c);
            if match best {
                Some((_, d)) => Rgb::within_threshold(dif, d) && dif != d,
                _ => true,
            } {
                best = Some((i, dif));
            }
        }
        best
    }
    /// Replace a `Palette` entry.
    ///
    /// * `i` Index of entry.
    /// * `clr` Color to replace entry with.
    ///
    /// # Returns
    /// Previous entry, or `None` if index is larger than table size.
    pub fn replace_entry(&mut self, i: usize, clr: SRgb8) -> Option<SRgb8> {
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
    where
        T: Copy,
        usize: From<T>,
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
            let idx = p.set_entry(SRgb8::new(i, i, i)).unwrap();
            assert_eq!(i as usize, idx);
        }
        assert_eq!(p.set_entry(SRgb8::new(255, 255, 255)), None);
        // test lookup
        for i in 0..16 {
            let idx = p.set_entry(SRgb8::new(i, i, i)).unwrap();
            assert_eq!(i as usize, idx);
        }
        assert_eq!(p.entry(5), Some(SRgb8::new(5, 5, 5)));
        p.replace_entry(5, SRgb8::new(0x55, 0x55, 0x55));
        let v = vec![
            0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x02, 0x02, 0x02, 0x03, 0x03,
            0x03, 0x04, 0x04, 0x04, 0x55, 0x55, 0x55, 0x06, 0x06, 0x06, 0x07,
            0x07, 0x07, 0x08, 0x08, 0x08, 0x09, 0x09, 0x09, 0x0A, 0x0A, 0x0A,
            0x0B, 0x0B, 0x0B, 0x0C, 0x0C, 0x0C, 0x0D, 0x0D, 0x0D, 0x0E, 0x0E,
            0x0E, 0x0F, 0x0F, 0x0F,
        ];
        assert_eq!(p.as_u8_slice(), &v[..]);
    }
    #[test]
    fn check_hist() {
        let mut p = Palette::new(8);
        for i in 0..7 {
            p.set_entry(SRgb8::new(i, i, i));
        }
        let v: Vec<u8> = vec![
            0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x01, 0x02, 0x04, 0x01,
            0x02, 0x02, 0x04, 0x02, 0x06, 0x04, 0x03, 0x03, 0x03, 0x06, 0x03,
            0x01, 0x02, 0x01, 0x02, 0x04, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x01, 0x02, 0x00, 0x00, 0x00, 0x02, 0x02, 0x04, 0x01, 0x00,
            0x00, 0x00, 0x02, 0x04,
        ];
        assert_eq!(p.histogram(&v[..]), Some(vec![18, 6, 10, 4, 8, 0, 2]));
    }
    #[test]
    fn matching() {
        let mut p = Palette::new(8);
        assert_eq!(p.set_entry(SRgb8::new(10, 10, 10)), Some(0));
        assert_eq!(p.set_entry(SRgb8::new(20, 20, 20)), Some(1));
        assert_eq!(p.set_entry(SRgb8::new(30, 30, 30)), Some(2));
        assert_eq!(p.set_entry(SRgb8::new(40, 40, 40)), Some(3));
        p.set_threshold_fn(|_| SRgb8::new(4, 5, 6));
        assert_eq!(p.set_entry(SRgb8::new(15, 15, 15)), Some(4));
        p.set_threshold_fn(|_| SRgb8::new(5, 5, 5));
        assert_eq!(p.set_entry(SRgb8::new(35, 35, 35)), Some(2));
    }
}
