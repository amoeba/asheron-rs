use crate::generated::enums::{ItemType, UiEffects};

/// Get UI effect texture ID from UiEffects
///
/// Based on Lifestoned IconEffects mapping
pub fn get_ui_effect_texture_id(ui_effect: UiEffects) -> u32 {
    match ui_effect {
        UiEffects::UNDEF => 0x060011C5,         // 100667845 - transparent
        UiEffects::MAGICAL => 0x060011CA,       // 100667850
        UiEffects::POISONED => 0x060011C6,      // 100667846
        UiEffects::BOOST_HEALTH => 0x06001B05,  // 100670213
        UiEffects::BOOST_MANA => 0x060011CA,    // 100667850 - Same as Magical
        UiEffects::BOOST_STAMINA => 0x06001B06, // 100670214
        UiEffects::FIRE => 0x06001B2E,          // 100670254
        UiEffects::LIGHTNING => 0x06001B2D,     // 100670253
        UiEffects::FROST => 0x06001B2F,         // 100670255
        UiEffects::ACID => 0x06001B2C,          // 100670252
        UiEffects::BLUDGEONING => 0x060033C3,   // 100676547
        UiEffects::SLASHING => 0x060033C2,      // 100676546
        UiEffects::PIERCING => 0x060033C4,      // 100676548
        UiEffects::NETHER => 0x060011C5, // Lifestoned has this marked as missing, using default
        // Default for multi-flag or unknown
        _ => 0x060011C5, // 100667845
    }
}

/// Get background texture ID from ItemType
///
/// Based on Lifestoned.Lib GetDefaultUnderlayFromItemType
pub fn get_background_from_item_type(item_type: ItemType) -> u32 {
    // Single-bit item types get specific backgrounds
    match item_type {
        ItemType::MELEE_WEAPON => 0x060011CB,
        ItemType::ARMOR => 0x060011CF,
        ItemType::CLOTHING => 0x060011F3,
        ItemType::JEWELRY => 0x060011D5,
        ItemType::CREATURE => 0x060011D1,
        ItemType::FOOD => 0x060011CC,
        ItemType::MONEY => 0x060011F4,
        ItemType::MISSILE_WEAPON => 0x060011D2,
        ItemType::CONTAINER => 0x060011CE,
        ItemType::MISC => 0x060011D0,
        ItemType::GEM => 0x060011D3,
        ItemType::SPELL_COMPONENTS => 0x060011CD,
        ItemType::SERVICE => 0x06005E23,
        // Default for unknown or multi-flag types
        _ => 0x060011D4,
    }
}

/// Returns a random single-bit ItemType
pub fn random_item_type() -> ItemType {
    use rand::seq::SliceRandom;
    let types = [
        ItemType::MELEE_WEAPON,
        ItemType::ARMOR,
        ItemType::CLOTHING,
        ItemType::JEWELRY,
        ItemType::CREATURE,
        ItemType::FOOD,
        ItemType::MONEY,
        ItemType::MISC,
        ItemType::MISSILE_WEAPON,
        ItemType::CONTAINER,
        ItemType::GEM,
        ItemType::SPELL_COMPONENTS,
    ];
    *types.choose(&mut rand::thread_rng()).unwrap()
}

/// Parses an item type string (case-insensitive name or numeric ID)
/// Returns the corresponding ItemType bitflag
/// Special value "random" returns a random ItemType
pub fn parse_item_type(s: &str) -> Result<ItemType, String> {
    // Handle "random" special case
    if s.eq_ignore_ascii_case("random") {
        return Ok(random_item_type());
    }

    // Try parsing as a number first (hex with 0x prefix, or decimal)
    if let Ok(value) = parse_texture_id(s) {
        return Ok(ItemType::from_bits_retain(value));
    }

    // Parse as case-insensitive name
    match s.to_uppercase().as_str() {
        "MELEE_WEAPON" | "MELEEWEAPON" | "MELEE" => Ok(ItemType::MELEE_WEAPON),
        "ARMOR" => Ok(ItemType::ARMOR),
        "CLOTHING" => Ok(ItemType::CLOTHING),
        "JEWELRY" => Ok(ItemType::JEWELRY),
        "CREATURE" => Ok(ItemType::CREATURE),
        "FOOD" => Ok(ItemType::FOOD),
        "MONEY" => Ok(ItemType::MONEY),
        "MISC" | "USELESS" => Ok(ItemType::MISC),
        "MISSILE_WEAPON" | "MISSILEWEAPON" | "MISSILE" => Ok(ItemType::MISSILE_WEAPON),
        "CONTAINER" => Ok(ItemType::CONTAINER),
        "GEM" => Ok(ItemType::GEM),
        "SPELL_COMPONENTS" | "SPELLCOMPONENTS" => Ok(ItemType::SPELL_COMPONENTS),
        "KEY" => Ok(ItemType::KEY),
        "CASTER" => Ok(ItemType::CASTER),
        "PORTAL" => Ok(ItemType::PORTAL),
        "PROMISSORY_NOTE" | "PROMISSORYNOTE" => Ok(ItemType::PROMISSORY_NOTE),
        "MANA_STONE" | "MANASTONE" => Ok(ItemType::MANA_STONE),
        "SERVICE" => Ok(ItemType::SERVICE),
        _ => Err(format!("Unknown item type: {}", s)),
    }
}

/// Returns a random single-bit UiEffects
pub fn random_ui_effect() -> UiEffects {
    use rand::seq::SliceRandom;
    let effects = [
        UiEffects::MAGICAL,
        UiEffects::POISONED,
        UiEffects::BOOST_HEALTH,
        UiEffects::BOOST_MANA,
        UiEffects::BOOST_STAMINA,
        UiEffects::FIRE,
        UiEffects::LIGHTNING,
        UiEffects::FROST,
        UiEffects::ACID,
        UiEffects::BLUDGEONING,
        UiEffects::SLASHING,
        UiEffects::PIERCING,
        UiEffects::NETHER,
    ];
    *effects.choose(&mut rand::thread_rng()).unwrap()
}

/// Parses a UI effect string (case-insensitive name or numeric ID)
/// Returns the corresponding UiEffects bitflag
/// Special value "random" returns a random UiEffects
pub fn parse_ui_effect(s: &str) -> Result<UiEffects, String> {
    // Handle "random" special case
    if s.eq_ignore_ascii_case("random") {
        return Ok(random_ui_effect());
    }

    // Try parsing as a number first (hex with 0x prefix, or decimal)
    if let Ok(value) = parse_texture_id(s) {
        return Ok(UiEffects::from_bits_retain(value));
    }

    // Parse as case-insensitive effect name
    match s.to_uppercase().as_str() {
        "" | "NONE" | "TRANSPARENT" | "UNDEF" => Ok(UiEffects::UNDEF),
        "MAGICAL" => Ok(UiEffects::MAGICAL),
        "POISONED" | "POISON" | "POSIONED" => Ok(UiEffects::POISONED),
        "BOOSTHEALTH" | "BOOST_HEALTH" => Ok(UiEffects::BOOST_HEALTH),
        "BOOSTMANA" | "BOOST_MANA" => Ok(UiEffects::BOOST_MANA),
        "BOOSTSTAMINA" | "BOOST_STAMINA" => Ok(UiEffects::BOOST_STAMINA),
        "FIRE" => Ok(UiEffects::FIRE),
        "DEFAULT" => Ok(UiEffects::FIRE | UiEffects::MAGICAL), // 0x21 = Fire | Magical
        "LIGHTNING" | "ELECTRIC" => Ok(UiEffects::LIGHTNING),
        "FROST" | "COLD" => Ok(UiEffects::FROST),
        "ACID" => Ok(UiEffects::ACID),
        "BLUDGEONING" | "BLUDGEON" => Ok(UiEffects::BLUDGEONING),
        "SLASHING" | "SLASH" => Ok(UiEffects::SLASHING),
        "PIERCING" | "PIERCE" => Ok(UiEffects::PIERCING),
        "NETHER" => Ok(UiEffects::NETHER),
        "REVERSED" => Ok(UiEffects::from_bits_retain(0x06004C3E)), // For spells
        _ => Err(format!("Unknown UI effect: {}", s)),
    }
}

/// Helper to parse texture IDs in various formats (0x prefix hex, short hex, or decimal)
fn parse_texture_id(s: &str) -> Result<u32, String> {
    if s.starts_with("0x") || s.starts_with("0X") {
        // Parse as hex with 0x prefix
        u32::from_str_radix(&s[2..], 16).map_err(|e| format!("Invalid hex ID: {}", e))
    } else if s.len() <= 8 && s.chars().all(|c| c.is_ascii_hexdigit()) {
        // Try as hex (short form like "0321" or full like "060011CB")
        u32::from_str_radix(s, 16).map_err(|e| format!("Invalid hex ID: {}", e))
    } else {
        // Try as decimal
        s.parse::<u32>()
            .map_err(|e| format!("Invalid decimal ID: {}", e))
    }
}

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
#[derive(Debug)]
pub struct Icon {
    pub width: u32,
    pub height: u32,
    pub scale: u32,
    pub background: Option<Texture>,
    pub underlay: Option<Texture>,
    pub icon: Texture,
    pub overlay: Option<Texture>,
    pub effect: Option<Texture>,
}

#[cfg(feature = "dat-export")]
impl Icon {
    /// Blends an effect texture with an icon by replacing pure white pixels
    /// with the effect pixels. This matches the reference implementation's
    /// BlendEffect method.
    ///
    /// Reference: Lifestoned.Lib/ImageBuilder.cs::BlendEffect
    fn blend_effect(icon_image: &RgbaImage, effect_image: &RgbaImage) -> RgbaImage {
        let mut result = RgbaImage::new(icon_image.width(), icon_image.height());

        // Initialize result as transparent
        for pixel in result.pixels_mut() {
            *pixel = Rgba([0, 0, 0, 0]);
        }

        for y in 0..icon_image.height() {
            for x in 0..icon_image.width() {
                let icon_pixel = icon_image.get_pixel(x, y);

                // If icon pixel has any alpha, copy it to result
                if icon_pixel[3] > 0 {
                    result.put_pixel(x, y, *icon_pixel);
                }

                // If icon pixel is fully opaque white, replace with effect pixel
                if icon_pixel[3] == 255
                    && icon_pixel[0] == 255
                    && icon_pixel[1] == 255
                    && icon_pixel[2] == 255
                {
                    result.put_pixel(x, y, *effect_image.get_pixel(x, y));
                }
            }
        }

        result
    }

    pub fn blend(&self) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, std::io::Error> {
        let icon_buf = self.icon.export()?;
        let icon_img: RgbaImage = ImageBuffer::from_raw(self.width, self.height, icon_buf)
            .expect("Failed to create ImageBuffer from icon");

        // If no effect, return the raw icon texture (no compositing)
        let Some(ref effect) = self.effect else {
            println!("Exporting raw icon (no compositing)");
            return Ok(icon_img);
        };

        // Layer order (matching Lifestoned.Lib/ImageBuilder.cs):
        // 1. Background (from item type)
        // 2. Underlay
        // 3. Icon + UI effect (blended together using blend_effect)
        // 4. Overlay

        let mut layer_stack: Vec<RgbaImage> = vec![];

        // 1. Add background layer if present
        if let Some(ref background) = self.background {
            let bg_buf = background.export()?;
            let bg_img: RgbaImage = ImageBuffer::from_raw(self.width, self.height, bg_buf)
                .expect("Failed to create ImageBuffer from background");
            layer_stack.push(bg_img);
        }

        // 2. Add underlay if present
        if let Some(ref underlay) = self.underlay {
            let underlay_buf = underlay.export()?;
            let underlay_img: RgbaImage =
                ImageBuffer::from_raw(self.width, self.height, underlay_buf)
                    .expect("Failed to create ImageBuffer from underlay");
            layer_stack.push(underlay_img);
        }

        // 3. Add icon blended with effect (white pixels replaced with effect pixels)
        let effect_buf = effect.export()?;
        let effect_img: RgbaImage = ImageBuffer::from_raw(self.width, self.height, effect_buf)
            .expect("Failed to create ImageBuffer from effect");
        layer_stack.push(Self::blend_effect(&icon_img, &effect_img));

        // 4. Add overlay if present
        if let Some(ref overlay) = self.overlay {
            let overlay_buf = overlay.export()?;
            let overlay_img: RgbaImage =
                ImageBuffer::from_raw(self.width, self.height, overlay_buf)
                    .expect("Failed to create ImageBuffer from overlay");
            layer_stack.push(overlay_img);
        }

        println!("Blending {} layer(s)", layer_stack.len());

        // Start with black background (matching Lifestoned behavior)
        // This is used when compositing layers together
        let mut result = RgbaImage::new(self.width, self.height);
        for pixel in result.pixels_mut() {
            *pixel = Rgba([0, 0, 0, 255]);
        }

        // Blend all layers using standard alpha blending
        for layer in layer_stack.iter() {
            for x in 0..self.width {
                for y in 0..self.height {
                    let target_pixel = result.get_pixel_mut(x, y);
                    let source_pixel = layer.get_pixel(x, y);
                    target_pixel.blend(source_pixel);
                }
            }
        }

        Ok(result)
    }

    /// Renders the icon to a scaled DynamicImage
    fn render(&self) -> Result<DynamicImage, std::io::Error> {
        let blended = self.blend()?;
        Ok(DynamicImage::ImageRgba8(blended).resize(
            self.width * self.scale,
            self.height * self.scale,
            image::imageops::FilterType::Lanczos3,
        ))
    }

    pub fn export(&self) -> Result<Vec<u8>, std::io::Error> {
        let image = self.render()?;
        let mut buffer = Vec::new();
        image
            .write_to(&mut Cursor::new(&mut buffer), image::ImageFormat::Png)
            .expect("Failed to write PNG to buffer");
        Ok(buffer)
    }

    pub fn export_to_file(&self, path: &str) -> Result<(), std::io::Error> {
        let image = self.render()?;
        let file = File::create(path)?;
        image
            .write_to(&mut BufWriter::new(file), image::ImageFormat::Png)
            .expect("Failed to write PNG to file");
        Ok(())
    }
}

#[cfg(feature = "dat-export")]
/// Builder for creating Icon instances with a fluent API
/// Uses game types instead of raw texture IDs
///
/// # Example
/// ```no_run
/// use asheron_rs::dat::{DatDatabase, IconBuilder};
/// use asheron_rs::dat::reader::FileRangeReader;
/// use asheron_rs::enums::{ItemType, UiEffects};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let file = tokio::fs::File::open("client_portal.dat").await?;
/// # let mut reader = FileRangeReader::new(file);
/// let dat = DatDatabase::read_async(&mut reader).await?;
///
/// let icon = IconBuilder::new(0x06001234)
///     .with_item_type(ItemType::FOOD)
///     .with_ui_effect(UiEffects::FIRE)
///     .with_scale(2)
///     .build(&dat, "client_portal.dat")
///     .await?;
///
/// icon.export_to_file("icon.png")?;
/// # Ok(())
/// # }
/// ```
pub struct IconBuilder {
    icon_id: u32,
    scale: u32,
    item_type: Option<ItemType>,
    background_id: Option<u32>,
    underlay_id: Option<u32>,
    overlay_id: Option<u32>,
    ui_effect: Option<UiEffects>,
}

#[cfg(feature = "dat-export")]
impl IconBuilder {
    /// Create a new IconBuilder with default scale of 1
    pub fn new(icon_id: u32) -> Self {
        Self {
            icon_id,
            scale: 1,
            item_type: None,
            background_id: None,
            underlay_id: None,
            overlay_id: None,
            ui_effect: None,
        }
    }

    /// Set the scale factor (1-10)
    pub fn with_scale(mut self, scale: u32) -> Self {
        self.scale = scale.clamp(1, 10);
        self
    }

    /// Set the item type to automatically determine the background
    pub fn with_item_type(mut self, item_type: ItemType) -> Self {
        self.item_type = Some(item_type);
        self
    }

    /// Set a custom background texture ID (overrides item_type background)
    pub fn with_background(mut self, background_id: u32) -> Self {
        self.background_id = Some(background_id);
        self
    }

    /// Set the underlay texture ID
    pub fn with_underlay(mut self, underlay_id: u32) -> Self {
        self.underlay_id = Some(underlay_id);
        self
    }

    /// Set the overlay texture ID
    pub fn with_overlay(mut self, overlay_id: u32) -> Self {
        self.overlay_id = Some(overlay_id);
        self
    }

    /// Set the UI effect
    pub fn with_ui_effect(mut self, ui_effect: UiEffects) -> Self {
        self.ui_effect = Some(ui_effect);
        self
    }

    /// Build the Icon by loading all textures from the DAT database
    #[cfg(all(feature = "dat-tokio", not(target_arch = "wasm32")))]
    pub async fn build(
        &self,
        dat: &crate::dat::DatDatabase,
        dat_file_path: &str,
    ) -> Result<Icon, Box<dyn std::error::Error>> {
        use crate::dat::{DatFile, find_file_by_id};
        use std::io::Cursor;
        use tokio::io::{AsyncReadExt, AsyncSeekExt};

        // Helper to load a texture by ID
        async fn load_texture_by_id(
            dat: &crate::dat::DatDatabase,
            dat_file_path: &str,
            texture_id: u32,
        ) -> Result<Texture, Box<dyn std::error::Error>> {
            let texture_id_str = format!("{:08X}", texture_id);
            let found_file = find_file_by_id(dat, &texture_id_str).await?;

            let mut file = tokio::fs::File::open(dat_file_path).await?;
            file.seek(std::io::SeekFrom::Start(found_file.file_offset as u64))
                .await?;

            let mut buf = vec![0u8; found_file.file_size as usize];
            file.read_exact(&mut buf).await?;

            let mut buf_reader = Cursor::new(buf);
            let outer_file: DatFile<Texture> = DatFile::read(&mut buf_reader)?;
            Ok(outer_file.inner)
        }

        // Load the main icon texture
        let icon_texture = load_texture_by_id(dat, dat_file_path, self.icon_id).await?;
        let width = icon_texture.width as u32;
        let height = icon_texture.height as u32;

        // Determine background
        let background = if let Some(bg_id) = self.background_id {
            Some(load_texture_by_id(dat, dat_file_path, bg_id).await?)
        } else if let Some(item_type) = self.item_type {
            let bg_id = get_background_from_item_type(item_type);
            Some(load_texture_by_id(dat, dat_file_path, bg_id).await?)
        } else {
            None
        };

        // Load underlay if specified
        let underlay = if let Some(underlay_id) = self.underlay_id {
            Some(load_texture_by_id(dat, dat_file_path, underlay_id).await?)
        } else {
            None
        };

        // Load overlay if specified
        let overlay = if let Some(overlay_id) = self.overlay_id {
            Some(load_texture_by_id(dat, dat_file_path, overlay_id).await?)
        } else {
            None
        };

        // Load UI effect only if specified
        // White pixels in the icon are replaced with effect pixels via blend_effect
        let effect = if let Some(ui_effect_flags) = self.ui_effect {
            let effect_id = get_ui_effect_texture_id(ui_effect_flags);
            Some(load_texture_by_id(dat, dat_file_path, effect_id).await?)
        } else {
            None
        };

        Ok(Icon {
            width,
            height,
            scale: self.scale,
            background,
            underlay,
            icon: icon_texture,
            overlay,
            effect,
        })
    }
}
