use ddsfile::{Dds, DxgiFormat};

use crate::ExtractError;

pub struct ExtractedImage {
    width: u32,
    height: u32,
    rgba: Vec<u8>,
    srgb: bool,
}

impl ExtractedImage {
    fn new(width: u32, height: u32, rgba: Vec<u8>, srgb: bool) -> Self {
        Self {
            width,
            height,
            rgba,
            srgb,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn rgba(self) -> Vec<u8> {
        self.rgba
    }

    pub fn srgb(&self) -> bool {
        self.srgb
    }
}

pub struct DdsDecoder;

impl DdsDecoder {
    pub fn decode(bytes: &[u8]) -> Result<ExtractedImage, ExtractError> {
        let dds = Dds::read(bytes)?;

        let width = dds.get_width();
        let height = dds.get_height();
        let data = dds.get_data(0)?;
        let format = dds.get_dxgi_format();

        let srgb = matches!(
            format,
            Some(DxgiFormat::BC1_UNorm_sRGB | DxgiFormat::BC3_UNorm_sRGB)
        );

        let rgba = match format {
            Some(DxgiFormat::BC1_UNorm | DxgiFormat::BC1_UNorm_sRGB) => {
                Self::decode_bc1(data, width, height)
            }
            Some(DxgiFormat::BC3_UNorm | DxgiFormat::BC3_UNorm_sRGB) => {
                Self::decode_bc3(data, width, height)
            }
            unsupported_format => return Err(ExtractError::UnsupportedFormat(unsupported_format)),
        };

        let extracted_image = ExtractedImage::new(width, height, rgba, srgb);
        Ok(extracted_image)
    }

    fn decode_bc1(data: &[u8], width: u32, height: u32) -> Vec<u8> {
        let stride = width * 4;
        let output_size = usize::try_from(stride * height).expect("image size fits usize");
        let mut rgba_pixels = vec![0; output_size];

        let blocks_x = width.div_ceil(4);
        let blocks_y = height.div_ceil(4);

        for by in 0..blocks_y {
            for bx in 0..blocks_x {
                let block_linear_index =
                    usize::try_from(by * blocks_x + bx).expect("block index fits usize");
                let block_index = block_linear_index * 8;
                let block = &data[block_index..block_index + 8];

                let pixel_column_start = bx * 4;
                let pixel_row_start = by * 4;

                if pixel_column_start >= width || pixel_row_start >= height {
                    continue;
                }

                let pixel_linear_index =
                    usize::try_from(pixel_row_start * width + pixel_column_start)
                        .expect("pixel index fits usize");
                let offset = pixel_linear_index * 4;
                let stride = usize::try_from(stride).expect("stride fits usize");
                bcdec_rs::bc1(block, &mut rgba_pixels[offset..], stride);
            }
        }

        rgba_pixels
    }

    fn decode_bc3(data: &[u8], width: u32, height: u32) -> Vec<u8> {
        let stride = width * 4;
        let output_size = usize::try_from(stride * height).expect("image size fits usize");
        let mut rgba_pixels: Vec<u8> = vec![0; output_size];

        let blocks_x = width.div_ceil(4);
        let blocks_y = height.div_ceil(4);

        let mut block_rgba: [u8; 4 * 4 * 4] = [0; 4 * 4 * 4];

        for by in 0..blocks_y {
            for bx in 0..blocks_x {
                let block_linear_index =
                    usize::try_from(by * blocks_x + bx).expect("block index fits usize");
                let block_index = block_linear_index * 16;
                let block = &data[block_index..block_index + 16];

                bcdec_rs::bc3(block, &mut block_rgba, 4 * 4);

                let start_x = bx * 4;
                let start_y = by * 4;

                for pixel_row in 0..4 {
                    let pixel_row_y = start_y + pixel_row;
                    if pixel_row_y >= height {
                        continue;
                    }

                    for pixel_column in 0..4 {
                        let pixel_column_x = start_x + pixel_column;
                        if pixel_column_x >= width {
                            continue;
                        }

                        let source_linear_index = usize::try_from(pixel_row * 4 + pixel_column)
                            .expect("block pixel index fits usize");
                        let src = source_linear_index * 4;
                        let dest_linear_index =
                            usize::try_from(pixel_row_y * width + pixel_column_x)
                                .expect("pixel index fits usize");
                        let dst = dest_linear_index * 4;

                        rgba_pixels[dst..dst + 4].copy_from_slice(&block_rgba[src..src + 4]);
                    }
                }
            }
        }

        rgba_pixels
    }
}
