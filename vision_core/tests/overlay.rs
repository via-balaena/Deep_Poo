use vision_core::overlay::{draw_rect, normalize_box};

#[test]
fn normalize_and_draw_box() {
    let bbox = normalize_box([0.1, 0.2, 0.3, 0.4], (100, 200)).expect("bbox");
    assert_eq!(bbox, [10, 40, 30, 80]);

    let mut img = image::RgbaImage::new(40, 40);
    draw_rect(&mut img, [5, 5, 10, 10], image::Rgba([255, 0, 0, 255]), 2);
    // Expect the four corners to be colored.
    assert_eq!(img.get_pixel(5, 5), &image::Rgba([255, 0, 0, 255]));
    assert_eq!(img.get_pixel(10, 5), &image::Rgba([255, 0, 0, 255]));
    assert_eq!(img.get_pixel(5, 10), &image::Rgba([255, 0, 0, 255]));
    assert_eq!(img.get_pixel(10, 10), &image::Rgba([255, 0, 0, 255]));
}
