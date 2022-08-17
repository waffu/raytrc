use crate::utility::vec3::Rgb;

pub fn write_color(out: &mut dyn std::io::Write, color: Rgb, samples_per_pixel: i32) {

    let mut r = color.x();
    let mut g = color.y();
    let mut b = color.z();

    let scale = 1.0 / samples_per_pixel as f32;

    r = f32::sqrt(scale * r);
    g = f32::sqrt(scale * g);
    b = f32::sqrt(scale * b);

    let ir = (256 as f32 * clamp(r, 0.0, 0.999)) as i32;
    let ig = (256 as f32 * clamp(g, 0.0, 0.999)) as i32;
    let ib = (256 as f32 * clamp(b, 0.0, 0.999)) as i32;

    let s = format!("{} {} {}\n", ir, ig, ib);
    out.write(s.as_bytes()).unwrap();
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min { return min; };
    if x > max { return max; };
    return x;

}