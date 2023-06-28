/// Linearly interpolates from a to b
///
/// Examples:
/// - `fraction == 0.0`: `a`
/// - `fraction == 0.5`: halfway between `a` and `b`
/// - `fraction == 0.75`: 25% `a`, 75% `b`
/// - `fraction == 1.0`: `b`
pub fn lerp(fraction: f32, a: f32, b: f32) -> f32 {
    assert!((0.0..=1.0).contains(&fraction));

    (1.0 - fraction) * a + fraction * b
}

fn dist_sq(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    (x1 - x2).powi(2) + (y1 - y2).powi(2)
}

fn rectangle_circle_intersection_slow(
    circle_center: (f32, f32),
    circle_radius: f32,
    rect_x1: f32,
    rect_y1: f32,
    rect_x2: f32,
    rect_y2: f32,
) -> f32 {
    // TODO this is a slow and inaccurate estimate... the exact math seems hard
    let mut samples_within_circle = 0;
    const SAMPLES_PER_DIM: usize = 100;
    for y_index in 0..SAMPLES_PER_DIM {
        for x_index in 0..SAMPLES_PER_DIM {
            let x = lerp(
                x_index as f32 / (SAMPLES_PER_DIM - 1) as f32,
                rect_x1,
                rect_x2,
            );
            let y = lerp(
                y_index as f32 / (SAMPLES_PER_DIM - 1) as f32,
                rect_y1,
                rect_y2,
            );

            if dist_sq(circle_center.0, circle_center.1, x, y) <= circle_radius.powi(2) {
                samples_within_circle += 1;
            }
        }
    }

    samples_within_circle as f32 / (SAMPLES_PER_DIM as f32).powi(2)
}

/// Calculates the area of intersection between a circle and a square
pub fn square_circle_intersection(
    circle_center: (f32, f32),
    circle_radius: f32,
    square_center: (f32, f32),
    square_side_len: f32,
) -> f32 {
    let dist_sq_between_centers = dist_sq(
        circle_center.0,
        circle_center.1,
        square_center.0,
        square_center.1,
    );

    // If the square is entirely contained within the circle, return the
    // square's area
    if dist_sq_between_centers
        <= (circle_radius - std::f32::consts::SQRT_2 * square_side_len / 2.0).powi(2)
    {
        return square_side_len.powi(2);
    }

    // If there is definitely no overlap at all, return 0
    if dist_sq_between_centers
        >= (circle_radius + std::f32::consts::SQRT_2 * square_side_len / 2.0).powi(2)
    {
        return 0.0;
    }

    // If the circle is entirely contained within the square, return the
    // circle's area
    if circle_center.0 - circle_radius >= square_center.0 - square_side_len / 2.0
        && circle_center.0 + circle_radius <= square_center.0 + square_side_len / 2.0
        && circle_center.1 - circle_radius >= square_center.1 - square_side_len / 2.0
        && circle_center.1 + circle_radius <= square_center.1 + square_side_len / 2.0
    {
        return std::f32::consts::PI * circle_radius.powi(2);
    }

    // Fall back on the full (but slow) computation
    rectangle_circle_intersection_slow(
        circle_center,
        circle_radius,
        square_center.0 - square_side_len / 2.0,
        square_center.1 - square_side_len / 2.0,
        square_center.0 + square_side_len / 2.0,
        square_center.1 + square_side_len / 2.0,
    )
}
