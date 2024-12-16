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
        let hue = cieluv.hue();
        assert!(hue >= 0.0);
        assert!(hue <= 360.0);
        println!("Gradient {i:1.03}: L*={l:1.03}, sat={sat:1.03}, chroma={c:1.03}, hue={hue:1.01} // {rgbw}");
    }
}

#[test]
fn test_rgbw_red_green() {
    println!("==> Gradient from RED to GREEN");
    print_gradient_as_rgbw(RGB::RED, RGB::GREEN, 100);
}

#[test]
fn test_rgbw_green_blue() {
    println!("==> Gradient from GREEN to BLUE");
    print_gradient_as_rgbw(RGB::GREEN, RGB::BLUE, 100);
}

#[test]
fn test_rgbw_blue_red() {
    println!("==> Gradient from BLUE to RED");
    print_gradient_as_rgbw(RGB::BLUE, RGB::RED, 100);
}

#[test]
fn test_rgbw_black_white() {
    println!("==> Gradient from BLACK to WHITE");
    print_gradient_as_rgbw(RGB::BLACK, RGB::WHITE, 100);
}

#[test]
fn test_rgbw_green_magenta() {
    println!("==> Gradient from GREEN to MAGENTA");
    let magenta = CIELUV::from(RGB {
        r: 1.0,
        g: 0.0,
        b: 1.0,
    });
    print_gradient_as_rgbw(RGB::GREEN, magenta, 100);
}