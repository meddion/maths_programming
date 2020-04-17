use std::cmp::min;

pub trait MixRule: Sync + Send {
    fn mix(&self, c1: u8, c2: u8) -> u8;
    fn name(&self) -> &'static str;
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

    fn name(&self) -> &'static str {
        match self {
            Self::Multiply => "multiply",
            Self::LinearBurn => "linear_burn",
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

    fn name(&self) -> &'static str {
        match self {
            Self::Screen => "screen",
            Self::LinearDodge => "linear_dodge",
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
