use image::{self, DynamicImage, GenericImageView};
use rayon::prelude::*;
use std::{
    cmp::min,
    sync::{mpsc, Arc, RwLock},
};

use crate::img::types::*;

pub type ImageSender<T> = mpsc::Sender<(T, &'static str)>;
pub type ImageReceiver<T> = mpsc::Receiver<(T, &'static str)>;

pub fn get_min_dim(dim1: (u32, u32), dim2: (u32, u32)) -> (u32, u32) {
    (min(dim1.0, dim2.0), min(dim1.1, dim2.1))
}

// mix_two_images_parallel achieves what it says by
// utilizing channels and rayon (concurrency lib)
pub fn mix_two_images_parallel(
    base: DynamicImage,
    cover: DynamicImage,
    mix_rules: &[Arc<dyn MixRule>],
) -> ImageReceiver<Vec<u8>> {
    let (width, height) = get_min_dim(base.dimensions(), cover.dimensions());
    let base = Arc::new(RwLock::new(base));
    let cover = Arc::new(RwLock::new(cover));
    let size = 3 * width as usize * height as usize;

    let (sender, receiver) = mpsc::channel();
    mix_rules
        .par_iter()
        .for_each_with((sender, base, cover), |(s, b, c), mix_rule| {
            let buf = (0..size)
                .into_par_iter()
                .map(|i| {
                    let x = (i / 3) as u32 % width;
                    let y = (i / 3) as u32 / width;
                    let mother_p = b.read().unwrap().get_pixel(x, y)[i % 3];
                    let foreign_p = c.read().unwrap().get_pixel(x, y)[i % 3];
                    mix_rule.mix(mother_p, foreign_p)
                })
                .collect();

            s.send((buf, mix_rule.name())).unwrap()
        });
    receiver
}

/// apply_filters3x3_parallel takes an image and then applies each filter in filters
/// to it in a separate thread, after a computation inside the thread is over it sends
/// a resulting image over the chanel for further processing (e.g. save on disk)
pub fn apply_filters3x3_parallel(
    source: DynamicImage,
    filters: &[Arc<dyn Filter>],
) -> ImageReceiver<DynamicImage> {
    let (sender, receiver) = mpsc::channel();
    filters
        .par_iter()
        .for_each_with((sender, source), |(s, img), filter| {
            s.send((img.filter3x3(filter.get_matrix()), filter.name()))
                .unwrap();
        });
    receiver
}
