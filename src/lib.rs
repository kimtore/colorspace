#![no_std]

/// Color manipulation library.
///
/// Allows conversion between RGB, XYZ and CIELUV color spaces,
/// as well as creation of gradients through the CIELUV color space.

#[cfg(any(test, feature = "std"))]
extern crate std;

#[cfg(not(any(test, feature = "std")))]
use num_traits::Float;

use core::fmt::Display;
use core::fmt::Formatter;

/// Represents a color in the sRGB color space.
///
/// Values in the range of 0.0..1.0.
///
/// * `r` is the amount of red,
/// * `g` is the amount of green,
/// * `b` is the amount of blue.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct RGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl RGB {
    pub const BLACK: RGB = RGB {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };
    pub const RED: RGB = RGB {
        r: 1.0,
        g: 0.0,
        b: 0.0,
    };
    pub const GREEN: RGB = RGB {
        r: 0.0,
        g: 1.0,
        b: 0.0,
    };
    pub const BLUE: RGB = RGB {
        r: 0.0,
        g: 0.0,
        b: 1.0,
    };
    pub const WHITE: RGB = RGB {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };
}

impl Display for RGB {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let r = self.r;
        let g = self.g;
        let b = self.b;
        write!(f, "RGB R={r:1.2}, G={g:1.2}, B={b:1.2}")
    }
}

impl From<XYZ> for RGB {
    fn from(xyz: XYZ) -> Self {
        // sYCC: Amendment 1 to IEC 61966-2-1:1999.
        // Higher conversion precision with seven decimals.
        let r = 3.2406255 * xyz.x - 1.5372080 * xyz.y - 0.4986286 * xyz.z;
        let g = -0.9689307 * xyz.x + 1.8758561 * xyz.y + 0.0415175 * xyz.z;
        let b = 0.0557101 * xyz.x - 0.2040211 * xyz.y + 1.0570959 * xyz.z;

        Self {
            r: linear_to_srgb(r).clamp(0.0, 1.0),
            g: linear_to_srgb(g).clamp(0.0, 1.0),
            b: linear_to_srgb(b).clamp(0.0, 1.0),
        }
    }
}

/// Conversions to and from CIELUV/RGB is done through the XYZ color space.
impl From<CIELUV> for RGB {
    fn from(cieluv: CIELUV) -> Self {
        XYZ::from(cieluv).into()
    }
}

/// Conversions to and from HCL/RGB is done via the CIELUV color space.
impl From<HCL> for RGB {
    fn from(hcl: HCL) -> Self {
        CIELUV::from(hcl).into()
    }
}

/// Represents a color using RGB and a white component.
///
/// Values in the range of 0.0..1.0.
///
/// * `r` is the amount of red,
/// * `g` is the amount of green,
/// * `b` is the amount of blue,
/// * `w` is the amount of white.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct RGBW {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub w: f32,
}

impl Display for RGBW {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let r = self.r;
        let g = self.g;
        let b = self.b;
        let w = self.w;
        write!(f, "RGBW R={r:1.2}, G={g:1.2}, B={b:1.2}, W={w:1.2}")
    }
}

/// For pure RGB values, we convert them directly into RGBW without adding any white.
impl From<RGB> for RGBW {
    fn from(rgb: RGB) -> Self {
        Self {
            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
            w: 0.0,
        }
    }
}

/// Direct conversion from XYZ to RGBW.
///
/// The white component is ignored when converting directly from XYZ.
/// You may use the CIELUV to RGBW conversion instead.
impl From<XYZ> for RGBW {
    fn from(xyz: XYZ) -> Self {
        RGB::from(xyz).into()
    }
}

/// Conversion from CIELUV to RGBW is a bit more tricky than with plain RGB,
/// because we have to derive a value for the white component.
///
/// Here, this is done by first performing the CIELUV->XYZ->RGB conversion,
/// and subsequently using the saturation value from CIELUV to determine
/// how much of the RGB and white components to use.
///
/// The white amount is defined as Y * (1.0 - saturation), whereas the color
/// RGB components are multiplied by the saturation value.
///
/// This calculation produces nice results on SK6812 LEDs with good saturation
/// for deep reds while avoiding whites that are a mixture of the RGB components.
impl From<CIELUV> for RGBW {
    fn from(cieluv: CIELUV) -> Self {
        // Color saturation from 0..1
        let saturation = cieluv.saturation();
        let whiteness = 1.0 - saturation;

        let xyz = XYZ::from(cieluv);

        // sYCC: Amendment 1 to IEC 61966-2-1:1999.
        // Higher conversion precision with seven decimals.
        let r = 3.2406255 * xyz.x - 1.5372080 * xyz.y - 0.4986286 * xyz.z;
        let g = -0.9689307 * xyz.x + 1.8758561 * xyz.y + 0.0415175 * xyz.z;
        let b = 0.0557101 * xyz.x - 0.2040211 * xyz.y + 1.0570959 * xyz.z;

        //let rgb_max = r.max(g).max(b);

        let r = r * saturation;
        let g = g * saturation;
        let b = b * saturation;
        let w = xyz.y * whiteness;

        Self {
            r: linear_to_srgb(r).clamp(0.0, 1.0),
            g: linear_to_srgb(g).clamp(0.0, 1.0),
            b: linear_to_srgb(b).clamp(0.0, 1.0),
            w: linear_to_srgb(w).clamp(0.0, 1.0),
        }
    }
}

/// Conversions from HCL to RGBW is done through CIELUV, then the XYZ color space.
impl From<HCL> for RGBW {
    fn from(hcl: HCL) -> Self {
        CIELUV::from(hcl).into()
    }
}

/// CIE 1931 XYZ color space, derived from CIE RGB in an effort to simplify the math.
/// This color space defines the relationship between the visible spectrum
/// and the visual sensation of specific colors by human color vision.
///
/// Values in the range of 0.0..1.0.
///
/// * `x` is a mix of all three RGB curves chosen to be nonnegative,
/// * `y` is the luminance, and
/// * `z` is quasi-equal to blue (from CIE RGB).
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct XYZ {
    x: f32,
    y: f32,
    z: f32,
}

// Constants for D65 white point
const X_REF: f32 = 95.047;
const Y_REF: f32 = 100.0;
const Z_REF: f32 = 108.883;

// XYZ/LUV conversion
const K: f32 = 24389.0 / 27.0;
const E: f32 = 216.0 / 24389.0;
const U_PRIME_REF: f32 = 4.0 * X_REF / (X_REF + 15.0 * Y_REF + 3.0 * Z_REF);
const V_PRIME_REF: f32 = 9.0 * Y_REF / (X_REF + 15.0 * Y_REF + 3.0 * Z_REF);

impl XYZ {
    #[inline]
    fn u_prime(&self) -> f32 {
        4.0 * self.x / (self.x + 15.0 * self.y + 3.0 * self.z)
    }

    #[inline]
    fn v_prime(&self) -> f32 {
        9.0 * self.y / (self.x + 15.0 * self.y + 3.0 * self.z)
    }

    #[inline]
    fn y_ref(&self) -> f32 {
        self.y / Y_REF
    }
}

impl Display for XYZ {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        write!(f, "CIEXYZ X={x:1.2}, Y={y:1.2}, Z={z:1.2}")
    }
}

impl From<RGB> for XYZ {
    fn from(rgb: RGB) -> Self {
        let r = srgb_to_linear(rgb.r);
        let g = srgb_to_linear(rgb.g);
        let b = srgb_to_linear(rgb.b);

        // Based on sRGB Working Space Matrix
        // http://www.brucelindbloom.com/Eqn_RGB_XYZ_Matrix.html
        Self {
            x: r * 0.4124564 + g * 0.3575761 + b * 0.1804375,
            y: r * 0.2126729 + g * 0.7151522 + b * 0.0721750,
            z: r * 0.0193339 + g * 0.1191920 + b * 0.9503041,
        }
    }
}

impl From<CIELUV> for XYZ {
    fn from(cieluv: CIELUV) -> Self {
        if cieluv.l == 0.0 {
            return XYZ {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }

        let u_prime = cieluv.u / (13.0 * cieluv.l) + 0.19783000664283;
        let v_prime = cieluv.v / (13.0 * cieluv.l) + 0.46831999493879;

        let y = if cieluv.l > 8.0 {
            Y_REF * ((cieluv.l + 16.0) / 116.0).powi(3)
        } else {
            Y_REF * cieluv.l / 903.3
        };

        let x = y * 9.0 * u_prime / (4.0 * v_prime);
        let z = y * (12.0 - 3.0 * u_prime - 20.0 * v_prime) / (4.0 * v_prime);

        XYZ { x, y, z }
    }
}

/// Represents a color using the CIE 1976 L*, u*, v* color space.
///
/// * `l` is the luminance, with values nominally within `0.0..1.0`, but usually `-10.0..15.0`,
/// * `u` is the horizontal axis (green/red), with values approximately `-1.34..2.24`, and
/// * `v` is the vertical axis (blue/yellow), with values approximately `-1.40..1.22`.
///
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct CIELUV {
    l: f32,
    u: f32,
    v: f32,
}

impl CIELUV {
    /// Interpolate between two colors based on a parameter `t` (0.0 to 1.0).
    /// `t = 0.0` returns the start color, `t = 1.0` returns the end color.
    /// Any value in between is derived using linear interpolation in the
    /// CIELUV color space.
    pub fn interpolate(&self, end: &Self, t: f32) -> Self {
        Self {
            l: lerp(self.l, end.l, t),
            u: lerp(self.u, end.u, t),
            v: lerp(self.v, end.v, t),
        }
    }

    pub fn chroma(&self) -> f32 {
        (self.u.powi(2) + self.v.powi(2)).sqrt()
    }

    pub fn saturation(&self) -> f32 {
        if self.l <= 0.0 {
            return 0.0;
        }
        self.chroma() / self.l
    }
}

impl Display for CIELUV {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let l = self.l;
        let u = self.u;
        let v = self.v;
        write!(f, "CIELUV L*={l:1.2}, u*={u:1.2}, v*={v:1.2}")
    }
}

impl From<XYZ> for CIELUV {
    // Verified here: http://www.brucelindbloom.com/index.html?Eqn_XYZ_to_Luv.html
    // Introduced constants due to http://www.brucelindbloom.com/LContinuity.html
    fn from(xyz: XYZ) -> Self {
        if xyz.x == 0.0 && xyz.y == 0.0 && xyz.z == 0.0 {
            return Self {
                l: 0.0,
                u: 0.0,
                v: 0.0,
            };
        }

        let u_prime = xyz.u_prime();
        let v_prime = xyz.v_prime();
        let y_ref = xyz.y_ref();

        let l = if y_ref > E {
            116.0 * y_ref.powf(1.0 / 3.0) - 16.0
        } else {
            K * y_ref
        };

        Self {
            l,
            u: 13.0 * l * (u_prime - U_PRIME_REF),
            v: 13.0 * l * (v_prime - V_PRIME_REF),
        }
    }
}

/// Conversions to CIELUV from RGB is done through the XYZ color space.
impl From<RGB> for CIELUV {
    fn from(rgb: RGB) -> Self {
        XYZ::from(rgb).into()
    }
}

/// Conversions to HCL from RGB is done by first converting to CIELUV, then converting to HCL.
impl From<HCL> for CIELUV {
    fn from(hcl: HCL) -> Self {
        let h_rad = hcl.h.to_radians(); // Convert hue to radians
        let u = hcl.c * h_rad.cos();
        let v = hcl.c * h_rad.sin();
        CIELUV { l: hcl.l, u, v }
    }
}

/// CIELCh, also known as HCL or CIELUVch, is a cylindrical representation of the CIELUV color space.
///
/// * `h` is the hue, expressed as an angle and ranging from `0.0..360.0`,
/// * `c` is the chromaticity, ranging from `0.0..1.0`, and
/// * `l` is the luminance, ranging from `0.0..1.0`.
///
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct HCL {
    pub h: f32,
    pub c: f32,
    pub l: f32,
}

/// Helper function to perform linear interpolation
#[inline]
pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + t * (end - start)
}

const GAMMA: f32 = 2.4;

/// Convert sRGB to linear RGB (inverse sRGB companding)
/// Verified here: http://www.brucelindbloom.com/index.html?Eqn_RGB_to_XYZ.html
#[inline]
fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(GAMMA)
    }
}

/// Convert linear RGB to sRGB
/// Verified here: http://www.brucelindbloom.com/index.html?Eqn_XYZ_to_RGB.html
#[inline]
fn linear_to_srgb(c: f32) -> f32 {
    if c <= 0.0031308 {
        12.92 * c
    } else {
        1.055 * c.powf(1.0 / GAMMA) - 0.055
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::println;

    fn round(x: f32) -> f32 {
        (x * 100.0).round() / 100.0
    }

    /// Equal to the second decimal.
    fn approximately_equal(actual: RGBW, expected: RGBW) {
        assert_eq!(
            round(actual.r),
            round(expected.r),
            "found {actual}, expected {expected}"
        );
        assert_eq!(
            round(actual.g),
            round(expected.g),
            "found {actual}, expected {expected}"
        );
        assert_eq!(
            round(actual.b),
            round(expected.b),
            "found {actual}, expected {expected}"
        );
        assert_eq!(
            round(actual.w),
            round(expected.w),
            "found {actual}, expected {expected}"
        );
    }

    fn assert_clean_cieluv_conversion(rgb: RGB) {
        let expected = RGBW::from(rgb);
        let cieluv = CIELUV::from(rgb);
        approximately_equal(cieluv.into(), expected.into());
    }

    /// Red, green, blue, yellow and magenta convert cleanly to CIELUV and back.
    #[test]
    fn test_saturated_clean_conversion() {
        assert_clean_cieluv_conversion(RGB {
            r: 1.0,
            g: 0.0,
            b: 0.0,
        });
        assert_clean_cieluv_conversion(RGB {
            r: 0.0,
            g: 1.0,
            b: 0.0,
        });
        assert_clean_cieluv_conversion(RGB {
            r: 0.0,
            g: 0.0,
            b: 1.0,
        });
        assert_clean_cieluv_conversion(RGB {
            r: 1.0,
            g: 0.0,
            b: 1.0,
        });
        assert_clean_cieluv_conversion(RGB {
            r: 1.0,
            g: 1.0,
            b: 0.0,
        });

        // Teal is too light to convert cleanly, it will contain a white component.
        //assert_clean_cieluv_conversion(RGB { r: 0.0, g: 1.0, b: 1.0 });
    }

    fn print_gradient_as_rgbw(a: impl Into<CIELUV>, b: impl Into<CIELUV>, steps: usize) {
        let a = a.into();
        let b = b.into();

        println!("Start.........: {a}");
        println!("End...........: {b}");

        for i in 0..=steps {
            let i = i as f32 / steps as f32;
            let cieluv = CIELUV::interpolate(&a, &b, i);
            let rgbw = RGBW::from(cieluv);
            let l = cieluv.l;
            let sat = cieluv.saturation();
            let c = cieluv.chroma();
            println!("Gradient {i:1.03}: L*={l:1.03}, sat={sat:1.03}, chroma={c:1.03} // {rgbw}");
        }
    }

    #[test]
    fn test_rgbw_red_green() {
        print_gradient_as_rgbw(RGB::RED, RGB::GREEN, 100);
    }

    #[test]
    fn test_rgbw_black_white() {
        print_gradient_as_rgbw(RGB::BLACK, RGB::WHITE, 100);
    }

    #[test]
    fn test_rgbw_green_magenta() {
        let magenta = CIELUV::from(RGB {
            r: 1.0,
            g: 0.0,
            b: 1.0,
        });
        print_gradient_as_rgbw(RGB::GREEN, magenta, 100);
    }
}