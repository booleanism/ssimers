use std::error::Error;

pub type Pixels = Vec<Vec<u8>>;

#[derive(PartialEq)]
pub struct PixelsNode {
    data: Option<Pixels>,
    width: usize,
    height: usize,
}

impl PixelsNode {
    pub fn new(pix: Pixels) -> Self {
        let w = pix.len();
        let h = pix[0].len();
        Self { data: Some(pix), width: w, height: h }
    }
}

impl<'a> Into<Image> for PixelsNode {
    fn into(self) -> Image {
        Image { pixels: self }
    }
}

#[derive(PartialEq)]
pub struct Image {
    pub pixels: PixelsNode,
}

impl<'a> Image {
    pub fn from_buf(buf: &'a Vec<u8>, width: usize, height: usize) -> Self {
        let pixels = image::load_from_memory(&buf)
            .unwrap()
            .resize_to_fill(width as u32, height as u32, image::imageops::FilterType::Gaussian)
            .to_luma8()
            .to_vec();

        let pixels = Image::into_2d(&pixels, width as u32, height as u32).unwrap();

        Self { pixels: PixelsNode::new(pixels) }
    }

    pub fn from_pixels_vec(&self, pix: &'a Vec<u8>) -> Self {
        let vec_2d =
            Image::into_2d(pix, self.pixels.width as u32, self.pixels.height as u32).unwrap();
        Self { pixels: PixelsNode::new(vec_2d) }
    }

    pub fn pixels(&'a self) -> Option<&'a Pixels> {
        self.pixels.data.as_ref()
    }

    pub fn sub_pixels(&self, win_size: usize) -> Result<Vec<PixelsNode>, Box<dyn Error>> {
        if win_size > self.pixels.width {
            panic!("windows size is bigger than size of actual pixels");
        }

        let mut subs = Vec::<PixelsNode>::new();
        for g in 0..self.pixels.width as usize {
            if g as usize + win_size > self.pixels.width as usize {
                break;
            }
            for h in 0..self.pixels.height as usize {
                if h as usize + win_size > self.pixels.height as usize {
                    break;
                }
                let mut sub = Vec::new();
                for i in g..g + win_size {
                    let mut col = Vec::new();
                    for j in h..h + win_size {
                        col.push(self.pixels.data.as_ref().unwrap()[i][j]);
                    }
                    sub.push(col);
                }
                subs.push(PixelsNode::new(sub));
            }
        }

        Ok(subs)
    }

    fn into_2d(a_vec: &'a Vec<u8>, width: u32, height: u32) -> Result<Pixels, Box<dyn Error>> {
        let mut y = Box::new(Vec::new());
        let mut xy = Vec::<Vec<u8>>::new();

        // flatten vec to 2d vec
        for i in 0..width {
            let c = i * height;
            for j in c..c + height {
                y.push(a_vec[j as usize]);
            }
            xy.push(y.as_ref().to_vec());
            y.clear();
        }

        Ok(xy)
    }

    pub fn flattened(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(self.pixels.data.as_ref().unwrap().iter().flat_map(|f| f.iter()).copied().collect())
    }

    pub fn length(&self) -> Result<f64, Box<dyn Error>> {
        let pix = self.pixels.data.as_ref().unwrap();
        let len = pix[0].len() * pix.len();

        Ok(len as f64)
    }

    pub fn sum_pixels(&self) -> Result<f64, Box<dyn Error>> {
        let pix_flat = self.flattened()?;

        let mut sum = 0f64;

        for i in pix_flat {
            sum += i as f64
        }

        Ok(sum)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     const BUF: [u8; 331] = [
//         255, 216, 255, 224, 0, 16, 74, 70, 73, 70, 0, 1, 2, 0, 0, 1, 0, 1, 0, 0, 255, 192, 0, 11,
//         8, 0, 3, 0, 3, 1, 1, 17, 0, 255, 219, 0, 67, 0, 8, 6, 6, 7, 6, 5, 8, 7, 7, 7, 9, 9, 8, 10,
//         12, 20, 13, 12, 11, 11, 12, 25, 18, 19, 15, 20, 29, 26, 31, 30, 29, 26, 28, 28, 32, 36, 46,
//         39, 32, 34, 44, 35, 28, 28, 40, 55, 41, 44, 48, 49, 52, 52, 52, 31, 39, 57, 61, 56, 50, 60,
//         46, 51, 52, 50, 255, 196, 0, 31, 0, 0, 1, 5, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1,
//         2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 255, 196, 0, 181, 16, 0, 2, 1, 3, 3, 2, 4, 3, 5, 5, 4, 4,
//         0, 0, 1, 125, 1, 2, 3, 0, 4, 17, 5, 18, 33, 49, 65, 6, 19, 81, 97, 7, 34, 113, 20, 50, 129,
//         145, 161, 8, 35, 66, 177, 193, 21, 82, 209, 240, 36, 51, 98, 114, 130, 9, 10, 22, 23, 24,
//         25, 26, 37, 38, 39, 40, 41, 42, 52, 53, 54, 55, 56, 57, 58, 67, 68, 69, 70, 71, 72, 73, 74,
//         83, 84, 85, 86, 87, 88, 89, 90, 99, 100, 101, 102, 103, 104, 105, 106, 115, 116, 117, 118,
//         119, 120, 121, 122, 131, 132, 133, 134, 135, 136, 137, 138, 146, 147, 148, 149, 150, 151,
//         152, 153, 154, 162, 163, 164, 165, 166, 167, 168, 169, 170, 178, 179, 180, 181, 182, 183,
//         184, 185, 186, 194, 195, 196, 197, 198, 199, 200, 201, 202, 210, 211, 212, 213, 214, 215,
//         216, 217, 218, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 241, 242, 243, 244, 245,
//         246, 247, 248, 249, 250, 255, 218, 0, 8, 1, 1, 0, 0, 63, 0, 74, 255, 217,
//     ];

//     struct Images {
//         img_x: Image,
//         img_y: Image,
//     }

//     fn init_buf<'a>(width: usize, height: usize) -> Images {
//         let buf = BUF.to_vec();
//         let img_x = Image::from_buf(&buf, width, height);
//         let img_y = Image::from_buf(&buf, width, height);

//         Images { img_x, img_y }
//     }

//     #[test]
//     fn from_buf() {
//         let w = 3;
//         let h = 3;
//         let expected = Image {
//             pixels: PixelsNode {
//                 data: Some(vec![vec![127, 127, 127], vec![127, 127, 127], vec![127, 127, 127]]),
//                 width: w,
//                 height: h,
//             },
//         };

//         let buf = BUF.to_vec();

//         let img = Image::from_buf(&buf, w, h);

//         assert!(img == expected);
//     }

//     #[test]
//     fn imgx_imgy_equals() {
//         let img = init_buf(3, 3);

//         assert!(img.img_x == img.img_y);
//     }

//     #[test]
//     fn length_x() {
//         let width = 3;
//         let height = 3;
//         let img = init_buf(width, height);

//         assert!(img.img_x.length().unwrap() == (width * height) as f64);
//     }

//     #[test]
//     fn length_y() {
//         let width = 3;
//         let height = 3;
//         let img = init_buf(width, height);

//         assert!(img.img_y.length().unwrap() == (width * height) as f64);
//     }
// }
