use crate::dat::SurfacePixelFormat;
use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use num_traits::FromPrimitive;
use std::io::{Error, ErrorKind, Read};

use super::dat_file::DatFileRead;

#[cfg(feature = "dat-export")]
use image::{DynamicImage, ImageBuffer, RgbaImage};
#[cfg(feature = "dat-export")]
use std::{fs::File, io::BufWriter};

#[derive(Clone, Debug, PartialEq)]
pub struct Texture {
    pub unknown: i32, // This is sometimes 6? Seems used somehow.
    pub width: i32,
    pub height: i32,
    pub format: SurfacePixelFormat,
    pub length: i32,
    pub data: Vec<u8>,
    pub default_palette_id: Option<u32>,
}

impl DatFileRead for Texture {
    fn read<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let unknown = reader.read_i32::<LittleEndian>()?;
        let width = reader.read_i32::<LittleEndian>()?;
        let height = reader.read_i32::<LittleEndian>()?;

        let format_value = reader.read_i32::<LittleEndian>()?;
        let format = FromPrimitive::from_i32(format_value).ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Invalid pixel format: {}", format_value),
            )
        })?;
        let length = reader.read_i32::<LittleEndian>()?;

        // data
        let mut data = vec![0u8; length as usize];
        reader.read_exact(&mut data)?;

        // default_palette_id
        let mut palette_id_bytes = [0u8; 4];
        let default_palette_id = match format {
            SurfacePixelFormat::PFID_P8 | SurfacePixelFormat::PFID_INDEX16 => {
                match reader.read_exact(&mut palette_id_bytes) {
                    Ok(_) => {
                        let id = u32::from_le_bytes(palette_id_bytes);
                        Some(id)
                    }
                    Err(_) => None,
                }
            }
            _ => None,
        };

        Ok(Texture {
            unknown,
            width,
            height,
            format,
            length,
            data,
            default_palette_id,
        })
    }
}

impl Texture {
    /// export underlying file buffer to rgba-ordered Vec<u8>
    ///
    /// Normalizes input into [R,G,B,A] to simplify downstream code
    pub fn export(&self) -> Result<Vec<u8>, Error> {
        match self.format {
            SurfacePixelFormat::PFID_R8G8B8 => {
                // TODO: This is untested (PFID_A8R8G8B8 is tested)
                let result = self
                    .data
                    .chunks_exact(3)
                    .flat_map(|chunk| {
                        // [B,G,R] -> [R,G,B,255]
                        [chunk[2], chunk[1], chunk[0], 255]
                    })
                    .collect();

                Ok(result)
            }
            SurfacePixelFormat::PFID_A8R8G8B8 => {
                let result = self
                    .data
                    .chunks_exact(4)
                    .flat_map(|chunk| {
                        // [B,G,R,A] -> [R,G,B,A]
                        [chunk[2], chunk[1], chunk[0], chunk[3]]
                    })
                    .collect();

                Ok(result)
            }
            _ => Err(Error::new(
                ErrorKind::Unsupported,
                format!("Unsupported pixel format for export: {:?}", self.format),
            )),
        }
    }

    #[cfg(feature = "dat-export")]
    pub fn to_image(&self, scale: u32) -> Result<DynamicImage, Error> {
        let buf = self.export()?;
        let img: RgbaImage = ImageBuffer::from_raw(self.width as u32, self.height as u32, buf)
            .expect("Failed to create ImageBuffer from exported texture.");

        let mut dynamic_image = DynamicImage::ImageRgba8(img);

        if scale > 1 {
            dynamic_image = dynamic_image.resize(
                (self.width as u32) * scale,
                (self.height as u32) * scale,
                image::imageops::FilterType::Lanczos3,
            )
        }

        Ok(dynamic_image)
    }

    #[cfg(feature = "dat-export")]
    pub fn to_png(&self, path: &str, scale: u32) -> Result<(), Error> {
        let image = self.to_image(scale)?;
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        image
            .write_to(&mut writer, image::ImageFormat::Png)
            .expect("Failed to write image.");

        Ok(())
    }
}

#[cfg(feature = "dat-export")]
impl super::Exportable for Texture {
    fn export_to_path(&self, path: &str) -> Result<(), Error> {
        self.to_png(path, 1)
    }

    fn file_extension(&self) -> &str {
        "png"
    }
}
