// hsx.rs       HSV, HSL and HSI color models.
//
// Copyright (c) 2019-2020  Jeron Aldaron Lau
// Copyright (c) 2020  Douglas P Lau
//
use crate::alpha::{
    self, AChannel, Mode as _, Opaque, Premultiplied, Straight, Translucent,
};
use crate::gamma::{self, Linear};
use crate::{Ch16, Ch32, Ch8, Channel, ColorModel, Pixel};
use std::marker::PhantomData;

/// Hexcone for color hue
#[derive(Clone, Copy, Debug)]
enum Hexcone {
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
    fn from_hue_prime(hp: f32) -> Self {
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
            },
        }
    }

    /// Get base red, green and blue components
    fn rgb<C: Channel>(self, chroma: C) -> (C, C, C) {
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
fn rgb_to_hue_chroma_value<C: Channel>(red: C, green: C, blue: C) -> (C, C, C) {
    let val = red.max(green).max(blue);
    let chroma = val - red.min(green).min(blue);

    let hue = if chroma > C::MIN {
        (if val == red {
            if green >= blue {
                (green.into() - blue.into()) / chroma.into()
            } else {
                6.0 - (blue.into() - green.into()) / chroma.into()
            }
        } else if green == val {
            2.0 + (blue.into() - red.into()) / chroma.into()
        } else {
            4.0 + (red.into() - green.into()) / chroma.into()
        }) / 6.0
    } else {
        0.0
    };
    (C::from(hue), chroma, val)
}

/// `HSV` hexcone [color model], also known as `HSB`.
///
/// The components are *hue*, *saturation* and *value* (or *brightness*), with
/// optional *[alpha]*.
///
/// [alpha]: alpha/trait.AChannel.html
/// [channel]: trait.Channel.html
/// [color model]: trait.ColorModel.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Hsv<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    hue: C,
    components: [C; 2],
    alpha: A,
    mode: PhantomData<M>,
    gamma: PhantomData<G>,
}

impl<C, A, M, G> Hsv<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    /// Create an `Hsv` color.
    ///
    /// ## Example
    /// ```
    /// # use pix::*;
    /// let opaque_hsv = Hsv8::new(50, 255, 128, ());
    /// let translucent_hsv = Hsva8::new(100, 128, 255, 200);
    /// ```
    pub fn new<H, B>(hue: H, saturation: H, value: H, alpha: B) -> Self
    where
        C: From<H>,
        A: From<B>,
    {
        let hue = C::from(hue);
        let saturation = C::from(saturation);
        let value = C::from(value);
        let components = [saturation, value];
        let alpha = A::from(alpha);
        Hsv {
            hue,
            components,
            alpha,
            mode: PhantomData,
            gamma: PhantomData,
        }
    }
    /// Get the *hue* component.
    pub fn hue(self) -> C {
        self.hue
    }
    /// Get the *saturation* component.
    pub fn saturation(self) -> C {
        self.components[0]
    }
    /// Get the *value* component.
    pub fn value(self) -> C {
        self.components[1]
    }
}

impl<C, A, M, G> ColorModel for Hsv<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    type Chan = C;

    /// Get all components affected by alpha/gamma
    fn components(&self) -> &[Self::Chan] {
        &self.components
    }

    /// Get the *alpha* component
    fn alpha(self) -> Self::Chan {
        self.alpha.value()
    }

    /// Convert to *red*, *green*, *blue* and *alpha* components
    fn to_rgba(self) -> [Self::Chan; 4] {
        let v = self.value();
        let chroma = v * self.saturation();
        let hp = self.hue().into() * 6.0; // 0.0..=6.0
        let hc = Hexcone::from_hue_prime(hp);
        let (red, green, blue) = hc.rgb(chroma);
        let m = v - chroma;
        [red + m, green + m, blue + m, self.alpha()]
    }

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn with_rgba(rgba: [Self::Chan; 4]) -> Self {
        let red = rgba[0];
        let green = rgba[1];
        let blue = rgba[2];
        let alpha = rgba[3];
        let (hue, chroma, val) = rgb_to_hue_chroma_value(red, green, blue);
        let sat_v = if val > C::MIN {
            chroma / val
        } else {
            C::MIN
        };
        Hsv::new(hue, sat_v, val, alpha)
    }
}

impl<C, A, M, G> Pixel for Hsv<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    type Alpha = M;
    type Gamma = G;
}

impl<C, A, M, G> Iterator for Hsv<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        Some(*self)
    }
}

impl<C, M, G> From<Hsv<C, Translucent<C>, M, G>> for Hsv<C, Opaque<C>, M, G>
where
    C: Channel,
    M: alpha::Mode,
    G: gamma::Mode,
{
    fn from(c: Hsv<C, Translucent<C>, M, G>) -> Self {
        Hsv::new(c.hue(), c.saturation(), c.value(), ())
    }
}

impl<C, M, G> From<Hsv<C, Opaque<C>, M, G>> for Hsv<C, Translucent<C>, M, G>
where
    C: Channel,
    M: alpha::Mode,
    G: gamma::Mode,
{
    fn from(c: Hsv<C, Opaque<C>, M, G>) -> Self {
        Hsv::new(c.hue(), c.saturation(), c.value(), ())
    }
}

impl<C, A, G> From<Hsv<C, A, Straight, G>> for Hsv<C, A, Premultiplied, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    G: gamma::Mode,
{
    fn from(c: Hsv<C, A, Straight, G>) -> Self {
        let hue = c.hue();
        let alpha = c.alpha();
        let saturation = Premultiplied::encode(c.saturation(), alpha);
        let value = Premultiplied::encode(c.value(), alpha);
        Hsv::new(hue, saturation, value, alpha)
    }
}

impl<C, A, G> From<Hsv<C, A, Premultiplied, G>> for Hsv<C, A, Straight, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    G: gamma::Mode,
{
    fn from(c: Hsv<C, A, Premultiplied, G>) -> Self {
        let hue = c.hue();
        let alpha = c.alpha();
        let saturation = Premultiplied::decode(c.saturation(), alpha);
        let value = Premultiplied::decode(c.value(), alpha);
        Hsv::new(hue, saturation, value, alpha)
    }
}

/// [Hsv](struct.Hsv.html) 8-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsv8 = Hsv<Ch8, Opaque<Ch8>, Straight, Linear>;
/// [Hsv](struct.Hsv.html) 16-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsv16 = Hsv<Ch16, Opaque<Ch16>, Straight, Linear>;
/// [Hsv](struct.Hsv.html) 32-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsv32 = Hsv<Ch32, Opaque<Ch32>, Straight, Linear>;

/// [Hsv](struct.Hsv.html) 8-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsva8 = Hsv<Ch8, Translucent<Ch8>, Straight, Linear>;
/// [Hsv](struct.Hsv.html) 16-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsva16 = Hsv<Ch16, Translucent<Ch16>, Straight, Linear>;
/// [Hsv](struct.Hsv.html) 32-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsva32 = Hsv<Ch32, Translucent<Ch32>, Straight, Linear>;

/// [Hsv](struct.Hsv.html) 8-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsva8p = Hsv<Ch8, Translucent<Ch8>, Premultiplied, Linear>;
/// [Hsv](struct.Hsv.html) 16-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsva16p = Hsv<Ch16, Translucent<Ch16>, Premultiplied, Linear>;
/// [Hsv](struct.Hsv.html) 32-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsva32p = Hsv<Ch32, Translucent<Ch32>, Premultiplied, Linear>;

/// `HSL` bi-hexcone [color model].
///
/// The components are *hue*, *saturation* and *lightness*, with optional
/// *[alpha]*.
///
/// [alpha]: alpha/trait.AChannel.html
/// [channel]: trait.Channel.html
/// [color model]: trait.ColorModel.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Hsl<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    hue: C,
    components: [C; 2],
    alpha: A,
    mode: PhantomData<M>,
    gamma: PhantomData<G>,
}

impl<C, A, M, G> Hsl<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    /// Create an `Hsl` color.
    ///
    /// ## Example
    /// ```
    /// # use pix::*;
    /// let opaque_hsl = Hsl8::new(50, 255, 128, ());
    /// let translucent_hsl = Hsla8::new(100, 128, 255, 200);
    /// ```
    pub fn new<H, B>(hue: H, saturation: H, lightness: H, alpha: B) -> Self
    where
        C: From<H>,
        A: From<B>,
    {
        let hue = C::from(hue);
        let saturation = C::from(saturation);
        let lightness = C::from(lightness);
        let components = [saturation, lightness];
        let alpha = A::from(alpha);
        Hsl {
            hue,
            components,
            alpha,
            mode: PhantomData,
            gamma: PhantomData,
        }
    }
    /// Get the *hue* component.
    pub fn hue(self) -> C {
        self.hue
    }
    /// Get the *saturation* component.
    pub fn saturation(self) -> C {
        self.components[0]
    }
    /// Get the *lightness* component.
    pub fn lightness(self) -> C {
        self.components[1]
    }
}

impl<C, A, M, G> ColorModel for Hsl<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    type Chan = C;

    /// Get all components affected by alpha/gamma
    fn components(&self) -> &[Self::Chan] {
        &self.components
    }

    /// Get the *alpha* component
    fn alpha(self) -> Self::Chan {
        self.alpha.value()
    }

    /// Convert to *red*, *green*, *blue* and *alpha* components
    fn to_rgba(self) -> [Self::Chan; 4] {
        let vl = 1.0 - (2.0 * self.lightness().into() - 1.0).abs();
        let chroma = C::from(vl) * self.saturation();
        let hp = self.hue().into() * 6.0; // 0.0..=6.0
        let hc = Hexcone::from_hue_prime(hp);
        let (red, green, blue) = hc.rgb(chroma);
        let m = self.lightness() - chroma * C::from(0.5);
        [red + m, green + m, blue + m, self.alpha()]
    }

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn with_rgba(rgba: [Self::Chan; 4]) -> Self {
        let red = rgba[0];
        let green = rgba[1];
        let blue = rgba[2];
        let alpha = rgba[3];
        let (hue, chroma, val) = rgb_to_hue_chroma_value(red, green, blue);
        let lightness = val - chroma * C::from(0.5);
        let min_l = lightness.min(C::MAX - lightness);
        let sat_l = if min_l > C::MIN {
            (val - lightness) / min_l
        } else {
            C::MIN
        };
        Hsl::new(hue, sat_l, lightness, alpha)
    }
}

impl<C, A, M, G> Pixel for Hsl<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    type Alpha = M;
    type Gamma = G;
}

impl<C, A, M, G> Iterator for Hsl<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        Some(*self)
    }
}

impl<C, M, G> From<Hsl<C, Translucent<C>, M, G>> for Hsl<C, Opaque<C>, M, G>
where
    C: Channel,
    M: alpha::Mode,
    G: gamma::Mode,
{
    fn from(c: Hsl<C, Translucent<C>, M, G>) -> Self {
        Hsl::new(c.hue(), c.saturation(), c.lightness(), ())
    }
}

impl<C, M, G> From<Hsl<C, Opaque<C>, M, G>> for Hsl<C, Translucent<C>, M, G>
where
    C: Channel,
    M: alpha::Mode,
    G: gamma::Mode,
{
    fn from(c: Hsl<C, Opaque<C>, M, G>) -> Self {
        Hsl::new(c.hue(), c.saturation(), c.lightness(), ())
    }
}

impl<C, A, G> From<Hsl<C, A, Straight, G>> for Hsl<C, A, Premultiplied, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    G: gamma::Mode,
{
    fn from(c: Hsl<C, A, Straight, G>) -> Self {
        let hue = c.hue();
        let alpha = c.alpha();
        let saturation = Premultiplied::encode(c.saturation(), alpha);
        let lightness = Premultiplied::encode(c.lightness(), alpha);
        Hsl::new(hue, saturation, lightness, alpha)
    }
}

impl<C, A, G> From<Hsl<C, A, Premultiplied, G>> for Hsl<C, A, Straight, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    G: gamma::Mode,
{
    fn from(c: Hsl<C, A, Premultiplied, G>) -> Self {
        let hue = c.hue();
        let alpha = c.alpha();
        let saturation = Premultiplied::decode(c.saturation(), alpha);
        let lightness = Premultiplied::decode(c.lightness(), alpha);
        Hsl::new(hue, saturation, lightness, alpha)
    }
}

/// [Hsl](struct.Hsl.html) 8-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsl8 = Hsl<Ch8, Opaque<Ch8>, Straight, Linear>;
/// [Hsl](struct.Hsl.html) 16-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsl16 = Hsl<Ch16, Opaque<Ch16>, Straight, Linear>;
/// [Hsl](struct.Hsl.html) 32-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsl32 = Hsl<Ch32, Opaque<Ch32>, Straight, Linear>;

/// [Hsl](struct.Hsl.html) 8-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsla8 = Hsl<Ch8, Translucent<Ch8>, Straight, Linear>;
/// [Hsl](struct.Hsl.html) 16-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsla16 = Hsl<Ch16, Translucent<Ch16>, Straight, Linear>;
/// [Hsl](struct.Hsl.html) 32-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsla32 = Hsl<Ch32, Translucent<Ch32>, Straight, Linear>;

/// [Hsl](struct.Hsl.html) 8-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsla8p = Hsl<Ch8, Translucent<Ch8>, Premultiplied, Linear>;
/// [Hsl](struct.Hsl.html) 16-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsla16p = Hsl<Ch16, Translucent<Ch16>, Premultiplied, Linear>;
/// [Hsl](struct.Hsl.html) 32-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsla32p = Hsl<Ch32, Translucent<Ch32>, Premultiplied, Linear>;

#[cfg(test)]
mod test {
    use super::super::*;
    use super::*;

    #[test]
    fn check_sizes() {
        assert_eq!(std::mem::size_of::<Hsv8>(), 3);
        assert_eq!(std::mem::size_of::<Hsv16>(), 6);
        assert_eq!(std::mem::size_of::<Hsv32>(), 12);
        assert_eq!(std::mem::size_of::<Hsva8>(), 4);
        assert_eq!(std::mem::size_of::<Hsva16>(), 8);
        assert_eq!(std::mem::size_of::<Hsva32>(), 16);
        assert_eq!(std::mem::size_of::<Hsl8>(), 3);
        assert_eq!(std::mem::size_of::<Hsl16>(), 6);
        assert_eq!(std::mem::size_of::<Hsl32>(), 12);
        assert_eq!(std::mem::size_of::<Hsla8>(), 4);
        assert_eq!(std::mem::size_of::<Hsla16>(), 8);
        assert_eq!(std::mem::size_of::<Hsla32>(), 16);
    }

    #[test]
    fn hsv_to_rgb() {
        assert_eq!(
            Rgb8::new(255, 0, 0, ()),
            Hsv8::new(0, 255, 255, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(255, 255, 0, ()),
            Hsv32::new(60.0 / 360.0, 1.0, 1.0, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(0, 255, 0, ()),
            Hsv16::new(21845, 65535, 65535, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(0, 255, 255, ()),
            Hsv32::new(0.5, 1.0, 1.0, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(0, 0, 255, ()),
            Hsv32::new(240.0 / 360.0, 1.0, 1.0, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(255, 0, 255, ()),
            Hsv32::new(300.0 / 360.0, 1.0, 1.0, ()).convert(),
        );
    }

    #[test]
    fn hsv_to_rgb_unsat() {
        assert_eq!(
            Rgb8::new(255, 127, 127, ()),
            Hsv8::new(0, 128, 255, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(255, 255, 128, ()),
            Hsv32::new(60.0 / 360.0, 0.5, 1.0, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(127, 255, 127, ()),
            Hsv16::new(21845, 32768, 65535, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(128, 255, 255, ()),
            Hsv32::new(180.0 / 360.0, 0.5, 1.0, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(128, 128, 255, ()),
            Hsv32::new(240.0 / 360.0, 0.5, 1.0, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(255, 128, 255, ()),
            Hsv32::new(300.0 / 360.0, 0.5, 1.0, ()).convert(),
        );
    }

    #[test]
    fn hsv_to_rgb_dark() {
        assert_eq!(
            Rgb8::new(128, 0, 0, ()),
            Hsv8::new(0, 255, 128, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(128, 128, 0, ()),
            Hsv32::new(60.0 / 360.0, 1.0, 0.5, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(0, 128, 0, ()),
            Hsv16::new(21845, 65535, 32768, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(0, 128, 128, ()),
            Hsv32::new(180.0 / 360.0, 1.0, 0.5, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(0, 0, 128, ()),
            Hsv32::new(240.0 / 360.0, 1.0, 0.5, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(128, 0, 128, ()),
            Hsv32::new(300.0 / 360.0, 1.0, 0.5, ()).convert(),
        );
    }

    #[test]
    fn hsv_to_rgb_hue() {
        assert_eq!(
            Rgb8::new(255, 192, 0, ()),
            Hsv8::new(32, 255, 255, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(126, 255, 0, ()),
            Hsv8::new(64, 255, 255, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(0, 255, 66, ()),
            Hsv8::new(96, 255, 255, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(0, 60, 255, ()),
            Hsv8::new(160, 255, 255, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(132, 0, 255, ()),
            Hsv8::new(192, 255, 255, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(255, 0, 186, ()),
            Hsv8::new(224, 255, 255, ()).convert(),
        );
    }

    #[test]
    fn hsv_to_rgb_grays() {
        assert_eq!(
            Rgb8::new(255, 255, 255, ()),
            Hsv8::new(0, 0, 255, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(128, 128, 128, ()),
            Hsv8::new(100, 0, 128, ()).convert(),
        );
        assert_eq!(
            Hsv8::new(0, 0, 255, ()),
            Rgb8::new(255, 255, 255, ()).convert(),
        );
        assert_eq!(
            Hsv8::new(0, 0, 128, ()),
            Rgb8::new(128, 128, 128, ()).convert(),
        );
    }

    #[test]
    fn rgb_to_hsv() {
        assert_eq!(
            Hsv8::new(0, 255, 255, ()),
            Rgb8::new(255, 0, 0, ()).convert(),
        );
        assert_eq!(
            Hsv32::new(60.0 / 360.0, 1.0, 1.0, ()),
            Rgb8::new(255, 255, 0, ()).convert(),
        );
        assert_eq!(
            Hsv16::new(21845, 65535, 65535, ()),
            Rgb8::new(0, 255, 0, ()).convert(),
        );
        assert_eq!(
            Hsv32::new(0.5, 1.0, 1.0, ()),
            Rgb8::new(0, 255, 255, ()).convert(),
        );
        assert_eq!(
            Hsv32::new(240.0 / 360.0, 1.0, 1.0, ()),
            Rgb8::new(0, 0, 255, ()).convert(),
        );
        assert_eq!(
            Hsv32::new(300.0 / 360.0, 1.0, 1.0, ()),
            Rgb8::new(255, 0, 255, ()).convert(),
        );
    }

    #[test]
    fn rgb_to_hsv_unsat() {
        assert_eq!(
            Hsv8::new(0, 128, 255, ()),
            Rgb8::new(255, 127, 127, ()).convert(),
        );
        assert_eq!(
            Hsv8::new(42, 128, 255, ()),
            Rgb8::new(255, 255, 127, ()).convert(),
        );
        assert_eq!(
            Hsv8::new(85, 127, 255, ()),
            Rgb8::new(128, 255, 128, ()).convert(),
        );
        assert_eq!(
            Hsv8::new(128, 127, 255, ()),
            Rgb8::new(128, 255, 255, ()).convert()
        );
        assert_eq!(
            Hsv8::new(170, 127, 255, ()),
            Rgb8::new(128, 128, 255, ()).convert(),
        );
        assert_eq!(
            Hsv8::new(213, 127, 255, ()),
            Rgb8::new(255, 128, 255, ()).convert(),
        );
    }

    #[test]
    fn hsl_to_rgb() {
        assert_eq!(
            Rgb8::new(255, 1, 1, ()),
            Hsl8::new(0, 255, 128, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(255, 255, 0, ()),
            Hsl32::new(60.0 / 360.0, 1.0, 0.5, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(0, 255, 0, ()),
            Hsl16::new(21845, 65535, 32768, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(0, 255, 255, ()),
            Hsl32::new(0.5, 1.0, 0.5, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(0, 0, 255, ()),
            Hsl32::new(240.0 / 360.0, 1.0, 0.5, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(255, 0, 255, ()),
            Hsl32::new(300.0 / 360.0, 1.0, 0.5, ()).convert(),
        );
    }

    #[test]
    fn rgb_to_hsl() {
        assert_eq!(
            Hsl8::new(0, 255, 127, ()),
            Rgb8::new(255, 0, 0, ()).convert(),
        );
        assert_eq!(
            Hsl32::new(60.0 / 360.0, 1.0, 0.5, ()),
            Rgb8::new(255, 255, 0, ()).convert(),
        );
        assert_eq!(
            Hsl16::new(21845, 65535, 32767, ()),
            Rgb8::new(0, 255, 0, ()).convert(),
        );
        assert_eq!(
            Hsl32::new(0.5, 1.0, 0.5, ()),
            Rgb8::new(0, 255, 255, ()).convert(),
        );
        assert_eq!(
            Hsl32::new(240.0 / 360.0, 1.0, 0.5, ()),
            Rgb8::new(0, 0, 255, ()).convert(),
        );
        assert_eq!(
            Hsl32::new(300.0 / 360.0, 1.0, 0.5, ()),
            Rgb8::new(255, 0, 255, ()).convert(),
        );
    }
}
