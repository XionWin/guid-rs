use super::*;

pub fn hsv2rgb(hsv: &HSV) -> RGB {
    let p2 = match hsv {
        hsv if hsv.v <= 0.5f32 => hsv.v * (1f32 + hsv.s),
        _ => hsv.v + hsv.s - hsv.v * hsv.s,
    };

    let p1 = 2f32 * hsv.v - p2;
    let (r, g, b) = match hsv.s {
        s if s == 0f32 => (hsv.v, hsv.v, hsv.v),
        _ => (qqh_to_rgb(p1, p2, hsv.h + 120f32), qqh_to_rgb(p1, p2, hsv.h), qqh_to_rgb(p1, p2, hsv.h - 120f32))
    };
    RGB::new((r * 255f32) as _, (g * 255f32) as _, (b * 255f32) as _)
}

pub fn qqh_to_rgb(q1: f32, q2: f32, mut hue: f32) -> f32 {
    hue += match hue {
        h if h > 360f32 => -360f32,
        h if h < 0f32 => 360f32,
        _ => 0f32,
    };

    match hue {
        h if h < 60f32 => q1 + (q2 - q1) * hue / 60f32,
        h if h < 180f32 => q2,
        h if h < 240f32 => q1 + (q2 - q1) * (240f32 - hue) / 60f32,
        _ => q1,
    }
}