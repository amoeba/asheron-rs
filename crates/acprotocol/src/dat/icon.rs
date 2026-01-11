// Effect IDs reference:
// { "", 0x060011C5 }, // transparent
// { "none", 0x060011C5 }, // transparent
// { "magical", 0x060011CA },
// { "posioned", 0x060011C6 },
// { "boosthealth", 0x06001B05 },
// { "boostmana", 0x060011CA },
// { "booststamina", 0x06001B06 },
// { "fire", 0x06001B2E },
// { "lightning", 0x06001B2D },
// { "frost", 0x06001B2F },
// { "acid", 0x06001B2C },
// { "bludgeoning", 0x060033C3 },
// { "slashing", 0x060033C2 },
// { "piercing", 0x060033C4 },
// { "reversed", 0x06004C3E } // for spells

#[cfg(feature = "dat-export")]
use std::{
    fs::File,
    io::{BufWriter, Cursor},
};

#[cfg(feature = "dat-export")]
use image::{DynamicImage, ImageBuffer, Pixel, Rgba, RgbaImage};

#[cfg(feature = "dat-export")]
use crate::dat::file_types::texture::Texture;

#[cfg(feature = "dat-export")]
#[derive(Debug, Clone, Default)]
pub struct IconExportOptions {
    pub convert_white_to_black: bool,
}

#[cfg(feature = "dat-export")]
#[derive(Debug)]
pub struct Icon {
    pub width: u32,
    pub height: u32,
    pub scale: u32,
    pub base: Texture,
    pub underlay: Option<Texture>,
    pub overlay: Option<Texture>,
    pub overlay2: Option<Texture>,
    pub effect: Option<Texture>,
}

#[cfg(feature = "dat-export")]
impl Icon {
    /// Convert white pixels to black in the given image (game rendering
    /// behavior)
    ///
    /// Without running this, icons have a white outline around the outline of
    /// whatever shape and, ingame, that same area has a black outline. I don't
    /// have any proof of what the client is doing so this I do this since it
    /// appears to get the right result.
    fn convert_white_to_black(image: &mut RgbaImage) {
        for pixel in image.pixels_mut() {
            if pixel[0] == 255 && pixel[1] == 255 && pixel[2] == 255 {
                pixel[0] = 0;
                pixel[1] = 0;
                pixel[2] = 0;
            }
        }
    }

    pub fn blend(
        &self,
        options: &IconExportOptions,
    ) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, std::io::Error> {
        // TODO: Remove clones

        let mut texture_stack: Vec<Texture> = vec![];

        if self.underlay.is_some() {
            texture_stack.push(self.underlay.clone().unwrap());
        }

        texture_stack.push(self.base.clone());

        if self.overlay.is_some() {
            texture_stack.push(self.overlay.clone().unwrap());
        }

        if self.overlay2.is_some() {
            texture_stack.push(self.overlay2.clone().unwrap());
        }

        if self.effect.is_some() {
            texture_stack.push(self.effect.clone().unwrap());
        }

        println!("Blending {} texture(s)", texture_stack.len());

        // We now have a Vec<Texture> with at least one element so we create
        // our final ImageBuffer from the first layer and blend in the rest
        let base_buf = texture_stack[0].export()?;
        let mut blended_image: RgbaImage = ImageBuffer::from_raw(self.width, self.height, base_buf)
            .expect("Failed to create ImageBuffer");

        if options.convert_white_to_black {
            Self::convert_white_to_black(&mut blended_image);
        }

        if texture_stack.len() == 1 {
            println!("Early return since we only have one layer.");
            return Ok(blended_image);
        }

        // Write any remaining textures in the stack
        for next_layer in texture_stack.iter().skip(1) {
            let next_layer_buf = next_layer.export()?;
            let mut next_layer_img: RgbaImage =
                ImageBuffer::from_raw(self.width, self.height, next_layer_buf)
                    .expect("Failed to create ImageBuffer");

            if options.convert_white_to_black {
                Self::convert_white_to_black(&mut next_layer_img);
            }

            for x in 0..self.width {
                for y in 0..self.height {
                    let target_pixel = blended_image.get_pixel_mut(x, y);
                    let new_pixel = next_layer_img.get_pixel(x, y);
                    target_pixel.blend(new_pixel);
                }
            }
        }

        Ok(blended_image)
    }

    pub fn export(&self) -> Result<Vec<u8>, std::io::Error> {
        self.export_with_options(&IconExportOptions::default())
    }

    pub fn export_with_options(
        &self,
        options: &IconExportOptions,
    ) -> Result<Vec<u8>, std::io::Error> {
        let blended = self.blend(options)?;

        let image = DynamicImage::ImageRgba8(blended).resize(
            self.width * self.scale,
            self.height * self.scale,
            image::imageops::FilterType::Lanczos3,
        );

        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);
        let _ = image.write_to(&mut cursor, image::ImageFormat::Png);

        Ok(buffer)
    }

    pub fn export_to_file(&self, path: &str) -> Result<(), std::io::Error> {
        let blended = self.blend(&IconExportOptions::default())?;

        let image = DynamicImage::ImageRgba8(blended).resize(
            self.width * self.scale,
            self.height * self.scale,
            image::imageops::FilterType::Lanczos3,
        );

        // Write to disk
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        image
            .write_to(&mut writer, image::ImageFormat::Png)
            .expect("Failed to write image to disk.");

        Ok(())
    }
}
