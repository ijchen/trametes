#[derive(Debug)]
pub struct PixelBuffer {
    pub pixels: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl Default for PixelBuffer {
    fn default() -> Self {
        let width = 800;
        let height = 600;

        const WHITE: [u8; 4] = [255, 255, 255, 255];

        let pixels: Vec<u8> = std::iter::repeat(WHITE)
            .take(width * height)
            .flatten()
            .collect();

        Self {
            pixels,
            width,
            height,
        }
    }
}

impl PixelBuffer {
    /// Returns an iterator over the pixels in a rectangular subsection of this
    /// PixelBuffer. The iterator yields ((r, g, b, a), (col, row))
    // TODO can these lifetime annotations be inferred?
    pub fn iter_block_mut<'a>(
        &'a mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> impl Iterator<
        Item = (
            (&'a mut u8, &'a mut u8, &'a mut u8, &'a mut u8),
            (usize, usize),
        ),
    > {
        let image_width = self.width;
        let image_height = self.height;
        // TODO not pay a runtime cost to sneak around the borrow checker
        // (will probably need `unsafe`)
        let mut mut_refs: Vec<Option<&'a mut u8>> = self.pixels.iter_mut().map(Some).collect();

        (y..y + height)
            .flat_map(move |row| (x..x + width).map(move |col| (col, row)))
            .filter_map(move |(col, row)| {
                if col < image_width && row < image_height {
                    let base_index = (row * image_width + col) * 4;
                    let pixel_bytes = (
                        #[allow(clippy::identity_op)] // ok but this looks nicer
                        mut_refs[base_index + 0].take().unwrap(),
                        mut_refs[base_index + 1].take().unwrap(),
                        mut_refs[base_index + 2].take().unwrap(),
                        mut_refs[base_index + 3].take().unwrap(),
                    );
                    Some((pixel_bytes, (col, row)))
                } else {
                    None
                }
            })
    }
}
