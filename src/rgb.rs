use crate::vec3::Vec3;

pub type Rgb = Vec3;

pub fn write_color(out: &mut dyn std::io::Write, color: Rgb) {
    let ir = (255.999 * color.x()) as i32;
    let ig = (255.999 * color.y()) as i32;
    let ib = (255.999 * color.z()) as i32;

    let s = format!("{} {} {}\n", ir, ig, ib);
    out.write(s.as_bytes()).unwrap();
}