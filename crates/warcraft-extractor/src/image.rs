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

/// Decodes the classic BLP1 texture format some command button icons still
/// ship in (patch 3.0.0 campaign items exist only as 64x64 BLP1, with no DDS
/// counterpart). Supports the paletted variant (`compression = 1`) with 0, 1,
/// 4 or 8 alpha bits; JPEG-compressed BLP1 is rejected.
///
/// Layout: `BLP1` magic, then little-endian `u32`s for compression, alpha
/// bits, width, height, picture type and subtype, sixteen mipmap offsets and
/// sixteen mipmap sizes, then a 256-entry BGRA palette. The palette's fourth
/// byte is unused; transparency lives in a separate alpha array that follows
/// the palette indices of each mipmap.
pub struct BlpDecoder;

impl BlpDecoder {
    const MAGIC: &'static [u8] = b"BLP1";
    const COMPRESSION_OFFSET: usize = 4;
    const ALPHA_BITS_OFFSET: usize = 8;
    const WIDTH_OFFSET: usize = 12;
    const HEIGHT_OFFSET: usize = 16;
    const FIRST_MIPMAP_OFFSET_OFFSET: usize = 28;
    const PALETTE_OFFSET: usize = 156;
    const PALETTE_ENTRY_COUNT: usize = 256;
    const PALETTED_COMPRESSION: u32 = 1;

    pub fn decode(bytes: &[u8]) -> Result<ExtractedImage, ExtractError> {
        if !bytes.starts_with(Self::MAGIC) {
            return Err(ExtractError::UnsupportedBlp);
        }
        let compression = Self::little_endian_u32(bytes, Self::COMPRESSION_OFFSET)?;
        if compression != Self::PALETTED_COMPRESSION {
            return Err(ExtractError::UnsupportedBlp);
        }
        let alpha_bits = Self::little_endian_u32(bytes, Self::ALPHA_BITS_OFFSET)?;
        let width = Self::little_endian_u32(bytes, Self::WIDTH_OFFSET)?;
        let height = Self::little_endian_u32(bytes, Self::HEIGHT_OFFSET)?;
        let mipmap_offset_value = Self::little_endian_u32(bytes, Self::FIRST_MIPMAP_OFFSET_OFFSET)?;
        let mipmap_offset =
            usize::try_from(mipmap_offset_value).map_err(|_| ExtractError::UnsupportedBlp)?;
        let pixel_count_value = width
            .checked_mul(height)
            .ok_or(ExtractError::UnsupportedBlp)?;
        let pixel_count =
            usize::try_from(pixel_count_value).map_err(|_| ExtractError::UnsupportedBlp)?;

        let palette_length = Self::PALETTE_ENTRY_COUNT * 4;
        let palette_end = Self::PALETTE_OFFSET + palette_length;
        let palette = bytes
            .get(Self::PALETTE_OFFSET..palette_end)
            .ok_or(ExtractError::UnsupportedBlp)?;
        let indices_end = mipmap_offset + pixel_count;
        let indices = bytes
            .get(mipmap_offset..indices_end)
            .ok_or(ExtractError::UnsupportedBlp)?;
        let alpha_values = Self::alpha_values(bytes, indices_end, pixel_count, alpha_bits)?;

        let mut rgba_pixels: Vec<u8> = Vec::with_capacity(pixel_count * 4);
        for (palette_index, alpha) in indices.iter().zip(alpha_values) {
            let palette_position = usize::from(*palette_index);
            let entry_start = palette_position * 4;
            let blue = palette[entry_start];
            let green = palette[entry_start + 1];
            let red = palette[entry_start + 2];
            let rgba_pixel: [u8; 4] = [red, green, blue, alpha];
            rgba_pixels.extend_from_slice(&rgba_pixel);
        }

        let extracted_image = ExtractedImage::new(width, height, rgba_pixels, false);
        Ok(extracted_image)
    }

    fn little_endian_u32(bytes: &[u8], offset: usize) -> Result<u32, ExtractError> {
        let end = offset + 4;
        let field = bytes.get(offset..end).ok_or(ExtractError::UnsupportedBlp)?;
        let field_bytes: [u8; 4] = field.try_into().map_err(|_| ExtractError::UnsupportedBlp)?;
        Ok(u32::from_le_bytes(field_bytes))
    }

    /// One alpha value (0..=255) per pixel, read from the alpha array that
    /// follows the palette indices. Packed alpha is stored least significant
    /// bits first.
    fn alpha_values(
        bytes: &[u8],
        alpha_offset: usize,
        pixel_count: usize,
        alpha_bits: u32,
    ) -> Result<Vec<u8>, ExtractError> {
        let bits_per_pixel = match alpha_bits {
            0 => {
                let opaque: Vec<u8> = vec![u8::MAX; pixel_count];
                return Ok(opaque);
            }
            1 | 4 | 8 => usize::try_from(alpha_bits).map_err(|_| ExtractError::UnsupportedBlp)?,
            _ => return Err(ExtractError::UnsupportedBlp),
        };
        let alpha_length = (pixel_count * bits_per_pixel).div_ceil(8);
        let alpha_end = alpha_offset + alpha_length;
        let packed_alpha = bytes
            .get(alpha_offset..alpha_end)
            .ok_or(ExtractError::UnsupportedBlp)?;
        let maximum_value: u16 = (1 << bits_per_pixel) - 1;
        let mut alpha_values: Vec<u8> = Vec::with_capacity(pixel_count);
        for pixel_index in 0..pixel_count {
            let bit_index = pixel_index * bits_per_pixel;
            let packed_byte = packed_alpha[bit_index / 8];
            let shift = bit_index % 8;
            let shifted_byte = packed_byte >> shift;
            let raw_value = u16::from(shifted_byte) & maximum_value;
            let full_scale = u16::from(u8::MAX);
            let scaled_value = raw_value * full_scale / maximum_value;
            let alpha = u8::try_from(scaled_value).map_err(|_| ExtractError::UnsupportedBlp)?;
            alpha_values.push(alpha);
        }
        Ok(alpha_values)
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
