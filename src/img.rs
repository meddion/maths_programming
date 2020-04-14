use image::{self, DynamicImage, GenericImage, GenericImageView, Rgba};
use std::{
    cmp::min,
    sync::{mpsc, Arc},
    thread,
};

pub trait MixRule {
    fn mix(&self, c1: u8, c2: u8) -> u8;
}

pub enum DimCover {
    Multiply,
    LinearBurn,
}

impl MixRule for DimCover {
    fn mix(&self, c1: u8, c2: u8) -> u8 {
        match self {
            Self::Multiply => ((c1 as i32 * c2 as i32) / 255) as u8,
            Self::LinearBurn => map((c1 as i32 + c2 as i32) - 255, -255, 255, 0, 255) as u8,
        }
    }
}

pub enum LightCover {
    Screen,
    LinearDodge,
}

impl MixRule for LightCover {
    fn mix(&self, c1: u8, c2: u8) -> u8 {
        match self {
            Self::Screen => map(
                255 - ((255 - c1 as i32) * (255 - c2 as i32)),
                255 - (255 * 255),
                255,
                0,
                255,
            ) as u8,
            Self::LinearDodge => min(c1 as u32 + c2 as u32, 255) as u8,
        }
    }
}

// map function is a same as in Processing
fn map(val: i32, in_min: i32, in_max: i32, out_min: i32, out_max: i32) -> i32 {
    (val - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

// The Sync marker trait indicates that it is safe for the type
// implementing Sync to be referenced from multiple threads
// In other words, any type T is Sync if &T (a reference to T) is Send,
// meaning the reference can be sent safely to another thread.
/// Filter is an abstraction over different types of a kernel filter.
pub trait Filter: Sync + Send {
    /// get_matrix returns a 3×3 kernel which is then convolved
    /// with an image to calculate approximations of the derivatives
    fn get_matrix(&self) -> &[f32];
    /// name returns the name of filter
    fn name(&self) -> &'static str;
}

// Mode is used to specify the resulted 3x3 matrix from EdgeDetect::get_matrix.
pub enum Mode {
    Vertical,
    Horizontal,
}

// EdgeDetect is a set of some popular edge detection strategies
pub enum EdgeDetect {
    Robert(Mode),
    Prewitt(Mode),
    Sobel(Mode),
}

impl Filter for EdgeDetect {
    fn get_matrix(&self) -> &[f32] {
        match self {
            Self::Robert(mode) => match mode {
                Mode::Vertical => &[-1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
                Mode::Horizontal => &[0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            },
            Self::Prewitt(mode) => match mode {
                Mode::Vertical => &[-1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
                Mode::Horizontal => &[-1.0, 0.0, 1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0],
            },
            Self::Sobel(mode) => match mode {
                Mode::Vertical => &[-1.0, -2.0, -1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0],
                Mode::Horizontal => &[-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0],
            },
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Robert(mode) => match mode {
                Mode::Vertical => "robert_vertical",
                Mode::Horizontal => "robert_horizontal",
            },
            Self::Prewitt(mode) => match mode {
                Mode::Vertical => "prewitt_vertical",
                Mode::Horizontal => "prewitt_horizontal",
            },
            Self::Sobel(mode) => match mode {
                Mode::Vertical => "sobel_vertical",
                Mode::Horizontal => "sobel_horizontal",
            },
        }
    }
}

pub fn mix_two_images(
    mut base: DynamicImage,
    cover: &DynamicImage,
    mix_rule: impl MixRule,
) -> DynamicImage {
    let (b_width, b_height) = base.dimensions();
    let (c_width, c_height) = cover.dimensions();

    for x in 0..min(b_width, c_width) {
        for y in 0..min(b_height, c_height) {
            let foreign_p = base.get_pixel(x, y);
            let mother_p = cover.get_pixel(x, y);

            base.put_pixel(
                x,
                y,
                Rgba::<u8>([
                    mix_rule.mix(mother_p[0], foreign_p[0]),
                    mix_rule.mix(mother_p[1], foreign_p[1]),
                    mix_rule.mix(mother_p[2], foreign_p[2]),
                    mix_rule.mix(mother_p[3], foreign_p[3]),
                ]),
            );
        }
    }
    base
}

/// apply_filters_parallel takes an image and then applies each filter in filters
/// to it in a separate thread, after a computation inside the thread is over it sends
/// a resulting image over the chanel for further processing (e.g. save on disk)
pub fn apply_filters_parallel(
    source: DynamicImage,
    sender: mpsc::Sender<(DynamicImage, &'static str)>,
    filters: &[Arc<dyn Filter>],
) {
    for filter in filters {
        let s_chan = mpsc::Sender::clone(&sender);
        let new_img = source.clone();
        let filter = Arc::clone(&filter);
        thread::spawn(move || {
            s_chan
                .send((new_img.filter3x3(filter.get_matrix()), filter.name()))
                .unwrap();
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use image::{error::ImageResult, imageops::FilterType};

    #[test]
    fn img_test_1_light_cover() -> ImageResult<()> {
        let mut base = image::open("misc/assets/wall.jpg")?;
        let mut cover = image::open("misc/assets/rust.jpg")?;

        let (b_width, b_height) = base.dimensions();
        cover = cover.resize(b_width, b_height, FilterType::Triangle);

        base = mix_two_images(base, &cover, LightCover::Screen);
        base.crop(0, 0, b_width / 2, b_height)
            .save("misc/test_output/img_test_1.jpg")?;
        Ok(())
    }

    #[test]
    fn img_test_2_edge_detection() -> ImageResult<()> {
        let source = image::open("misc/assets/skull.jpg")?;
        let (sender, receiver) = mpsc::channel();

        apply_filters_parallel(
            source,
            sender,
            &[
                //Arc::new(EdgeDetect::Prewitt(Mode::Vertical)),
                Arc::new(EdgeDetect::Prewitt(Mode::Horizontal)),
                // Arc::new(EdgeDetect::Robert(Mode::Vertical)),
                Arc::new(EdgeDetect::Robert(Mode::Horizontal)),
                //Arc::new(EdgeDetect::Sobel(Mode::Vertical)),
                Arc::new(EdgeDetect::Sobel(Mode::Horizontal)),
            ],
        );

        for (new_image, filter_name) in receiver {
            let path = format!("misc/test_output/img_test_2_{}.jpg", filter_name);
            new_image.save(path)?;
        }
        Ok(())
    }
    #[test]
    fn img_test_3_dim_cover() -> ImageResult<()> {
        let mut base = image::open("misc/assets/wall.jpg")?;
        let mut cover = image::open("misc/assets/skull.jpg")?;

        let (b_width, b_height) = base.dimensions();
        cover = cover.resize(b_width, b_height, FilterType::Triangle);

        base = mix_two_images(base, &cover, DimCover::Multiply);
        base.save("misc/test_output/img_test_3_multiply.jpg")?;
        base = mix_two_images(base, &cover, DimCover::LinearBurn);
        base.save("misc/test_output/img_test_3_linear_burn.jpg")?;

        Ok(())
    }
}
