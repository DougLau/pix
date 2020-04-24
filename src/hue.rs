// hue.rs       Hue helper functions.
//
// Copyright (c) 2019-2020  Jeron Aldaron Lau
// Copyright (c) 2020  Douglas P Lau
//
use crate::chan::Channel;

/// Hexcone for color hue
#[derive(Clone, Copy, Debug)]
pub enum Hexcone {
    /// Red is at 0 degrees
    Red(f32),
    /// Yellow is at 60 degrees
    Yellow(f32),
    /// Green is at 120 degrees
    Green(f32),
    /// Cyan is at 180 degrees
    Cyan(f32),
    /// Blue is at 240 degrees
    Blue(f32),
    /// Magenta is at 300 degrees
    Magenta(f32),
}

impl Hexcone {
    /// Look up a Hexcone value from hue prime
    ///
    /// * `hp` Hue / 60 degrees (ranging from 0.0 to 6.0)
    pub fn from_hue_prime(hp: f32) -> Self {
        use Hexcone::*;
        let h = hp as i32; // 0..=6
        let hf = hp.fract();
        match h {
            1 => Yellow(hf),
            2 => Green(hf),
            3 => Cyan(hf),
            4 => Blue(hf),
            5 => Magenta(hf),
            _ => Red(hf),
        }
    }

    /// Get the secondary component (after chroma)
    fn secondary<C: Channel>(self, chroma: C) -> C {
        use Hexcone::*;
        match self {
            Red(hf) | Green(hf) | Blue(hf) => chroma * C::from(hf),
            Yellow(hf) | Cyan(hf) | Magenta(hf) => {
                chroma * (C::MAX - C::from(hf))
            }
        }
    }

    /// Get base red, green and blue components
    pub fn rgb<C: Channel>(self, chroma: C) -> (C, C, C) {
        use Hexcone::*;
        let secondary = self.secondary(chroma);
        match self {
            Red(_) => (chroma, secondary, C::MIN),
            Yellow(_) => (secondary, chroma, C::MIN),
            Green(_) => (C::MIN, chroma, secondary),
            Cyan(_) => (C::MIN, secondary, chroma),
            Blue(_) => (secondary, C::MIN, chroma),
            Magenta(_) => (chroma, C::MIN, secondary),
        }
    }
}

/// Convert *red*, *green* and *blue* to *hue*, *chroma* and *value*
pub fn rgb_to_hue_chroma_value<C: Channel>(
    red: C,
    green: C,
    blue: C,
) -> (C, C, C) {
    let val = red.max(green).max(blue);
    let chroma = val - red.min(green).min(blue);

    let hue = if chroma > C::MIN {
        (if val == red {
            if green >= blue {
                (green.to_f32() - blue.to_f32()) / chroma.to_f32()
            } else {
                6.0 - (blue.to_f32() - green.to_f32()) / chroma.to_f32()
            }
        } else if green == val {
            2.0 + (blue.to_f32() - red.to_f32()) / chroma.to_f32()
        } else {
            4.0 + (red.to_f32() - green.to_f32()) / chroma.to_f32()
        }) / 6.0
    } else {
        0.0
    };
    (C::from(hue), chroma, val)
}
