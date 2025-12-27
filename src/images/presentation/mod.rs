use anyhow::{anyhow, Result};
use image::{DynamicImage, GenericImageView, ImageBuffer, ImageResult, Rgb};

trait TupleU32Ext {
    fn to_f64(&self) -> (f64, f64);
}

// TODO: あんま使ってないので、とりあえずワーニングブロック
#[allow(dead_code)]
trait TupleF64Ext {
    fn to_u32(&self) -> (u32, u32);
}

impl TupleU32Ext for (u32, u32) {
    fn to_f64(&self) -> (f64, f64) {
        (self.0 as f64, self.1 as f64)
    }
}

impl TupleF64Ext for (f64, f64) {
    fn to_u32(&self) -> (u32, u32) {
        (self.0 as u32, self.1 as u32)
    }
}

#[derive(Debug)]
pub struct PaperSize {
    pub x: u32,
    pub y: u32,
}

pub const A2: PaperSize = PaperSize { x: 420, y: 594 };
pub const A3: PaperSize = PaperSize { x: 297, y: 420 };
pub const A4: PaperSize = PaperSize { x: 210, y: 297 };

// f64::sqrt()
const PAPER_RATE: f64 = 1.4142135623730951;

pub fn create(w: u32, h: u32, path: &str) -> ImageResult<()> {
    let mut img = ImageBuffer::new(w, h);

    // set color to each pixel
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let rate = 255 * (x * y) / (w * h);

        let r = rate as u8;
        let g = rate as u8;
        let b = rate as u8;

        *pixel = Rgb([r, g, b]);
    }
    img.save(path)
}

pub fn get_info(path: &str) -> Result<DynamicImage> {
    let img = image::open(path).map_err(|e| anyhow!(format!("{}", e)))?;
    Ok(img)
}

pub fn fit_rate(src: &str, dst: &str) -> Result<()> {
    let img: DynamicImage = image::open(src)?;

    let (w, h) = img.dimensions().to_f64();

    // 元のサイズを拡縮しない形でセット
    let is_horizontal = w > h;
    let fit_size = get_size_fit_rate(is_horizontal, PAPER_RATE, w, h);

    let cropped = crop_center(&img, fit_size.0 as u32, fit_size.1 as u32);
    cropped.save(dst)?;

    Ok(())
}

// width, height を 超過分は削る方式で、rateに合わせて返却
pub fn get_size_fit_rate(is_horizontal: bool, rate: f64, width: f64, height: f64) -> (f64, f64) {
    // 長い方向を基準に割合取得
    let src_rate = if is_horizontal {
        width / height
    } else {
        height / width
    };

    // 元のサイズを拡縮しない形でセット
    if is_horizontal {
        if src_rate > rate {
            (height * rate, height)
        } else {
            (width, width / rate)
        }
    } else {
        if src_rate > rate {
            (width, width * rate)
        } else {
            (height / rate, height)
        }
    }
}

pub fn crop_center(img: &DynamicImage, width: u32, height: u32) -> DynamicImage {
    let (w, h) = img.dimensions();

    let (_w, _x) = if w > width {
        (width, ((w - width) as f64 / 2 as f64) as u32)
    } else {
        // 実際のサイズを超えた指定なので、フル指定
        (w, 0)
    };

    let (_h, _y) = if h > height {
        (height, ((h - height) as f64 / 2 as f64) as u32)
    } else {
        // 実際のサイズを超えた指定なので、フル指定
        (h, 0)
    };

    img.crop_imm(_x, _y, _w, _h)
}
