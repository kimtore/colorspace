# colorspace

Floating point color conversion on embedded devices, targeting SK6812 RGBW LEDs.

Supports RGB, RGBW, XYZ, CIELuv, and LCh color spaces.

You can use this library with `#![no_std]`.

## Usage

```bash
cargo add --git https://github.com/kimtore/colorspace
```

This example shows how to create a gradient with 100 samples
between blue and green in the CIELUV color space, then
convert them to RGBW:

```rust
use colorspace::*;

fn main() {
    let blue: CIELUV = RGB { r: 0.0, g: 0.0, b: 1.0 }.into();
    let green: CIELUV = RGB { r: 0.0, g: 1.0, b: 0.0 }.into();

    for i in 0..100 {
        let color = blue.interpolate(&green, i as f32 / 100.0);
        let rgbw = RGBW::from(color);
        println!("{color} converted to {rgbw}");
    }
}
```

## Why this library

There are some excellent libraries out there, such as [scarlet](https://github.com/nicholas-miklaucic/scarlet).
However they did not seem to have `no_std` support, which is a primary goal of this library.

## Further reading

* https://cscheid.github.io/lux/demos/hcl/hcl.html
* https://cscheid.net/2012/02/16/hcl-color-space-blues.html
* https://observablehq.com/@mbostock/luv-and-hcl
* https://hackaday.com/2018/03/30/color-spaces-the-model-at-the-end-of-the-rainbow/
* https://howaboutanorange.com/blog/2011/08/10/color_interpolation/
* https://en.wikipedia.org/wiki/SRGB
* https://en.wikipedia.org/wiki/Colorfulness
* https://gist.github.com/Myndex/47c793f8a054041bd2b52caa7ad5271c
* https://blog.saikoled.com/post/44677718712/how-to-convert-from-hsi-to-rgb-white
* https://ccom.unh.edu/vislab/tools/color_sequence_editor/