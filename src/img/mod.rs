pub mod functions;
pub mod types;

#[cfg(test)]
mod test {
    use super::{functions::*, types::*};
    use image::{
        self, error::ImageResult, imageops::FilterType, save_buffer_with_format, ColorType,
        GenericImageView, ImageFormat,
    };
    use std::sync::Arc;

    #[test]
    fn img_test_1_edge_detection() -> ImageResult<()> {
        let source = image::open("misc/assets/skull.jpg")?;

        let receiver = apply_filters3x3_parallel(
            source,
            &[
                Arc::new(EdgeDetect::Prewitt(Mode::Vertical)),
                Arc::new(EdgeDetect::Prewitt(Mode::Horizontal)),
                Arc::new(EdgeDetect::Robert(Mode::Vertical)),
                Arc::new(EdgeDetect::Robert(Mode::Horizontal)),
                Arc::new(EdgeDetect::Sobel(Mode::Vertical)),
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
    fn img_test_2_cover() -> ImageResult<()> {
        let base = image::open("misc/assets/wall.jpg")?;
        let mut cover = image::open("misc/assets/rust.jpg")?;
        let (b_width, b_height) = base.dimensions();
        cover = cover.resize(b_width, b_height, FilterType::Triangle);
        let (width, height) = get_min_dim(base.dimensions(), cover.dimensions());

        let receiver = mix_two_images_parallel(
            cover,
            base,
            &[
                Arc::new(LightCover::Screen),
                Arc::new(LightCover::LinearDodge),
                Arc::new(DimCover::Multiply),
                Arc::new(DimCover::LinearBurn),
            ],
        );

        for (img_buff, mix_rule_name) in receiver {
            let path = format!("misc/test_output/img_test_3_{}.jpg", mix_rule_name);
            save_buffer_with_format(
                path,
                &img_buff,
                width,
                height,
                ColorType::Rgb8,
                ImageFormat::Jpeg,
            )?;
        }

        Ok(())
    }
}
