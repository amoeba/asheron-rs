// Binary readers for common types

#[allow(unused_imports)]
use std::io::Read;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;

impl ObjectId {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self(read_u32(reader)?))
    }
}

impl crate::readers::ACDataType for ObjectId {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        ObjectId::read(reader)
    }
}

impl WindowPropertyType1000007F {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_j = read_u32(reader)?;
        let value_j = read_u64(reader)?;

        Ok(Self {
            unknown_j,
            value_j,
        })
    }
}

impl crate::readers::ACDataType for WindowPropertyType1000007F {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        WindowPropertyType1000007F::read(reader)
    }
}

impl WindowPropertyType10000086 {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_i = read_u32(reader)?;
        let value_i = read_u32(reader)?;

        Ok(Self {
            unknown_i,
            value_i,
        })
    }
}

impl crate::readers::ACDataType for WindowPropertyType10000086 {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        WindowPropertyType10000086::read(reader)
    }
}

impl WindowPropertyType10000087 {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_h = read_u32(reader)?;
        let value_h = read_u32(reader)?;

        Ok(Self {
            unknown_h,
            value_h,
        })
    }
}

impl crate::readers::ACDataType for WindowPropertyType10000087 {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        WindowPropertyType10000087::read(reader)
    }
}

impl WindowPropertyType10000088 {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_f = read_u32(reader)?;
        let value_f = read_u32(reader)?;

        Ok(Self {
            unknown_f,
            value_f,
        })
    }
}

impl crate::readers::ACDataType for WindowPropertyType10000088 {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        WindowPropertyType10000088::read(reader)
    }
}

impl WindowPropertyType10000089 {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_e = read_u32(reader)?;
        let value_e = read_u32(reader)?;

        Ok(Self {
            unknown_e,
            value_e,
        })
    }
}

impl crate::readers::ACDataType for WindowPropertyType10000089 {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        WindowPropertyType10000089::read(reader)
    }
}

impl WindowPropertyType1000008A {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_d = read_u32(reader)?;
        let value_d = read_u8(reader)?;

        Ok(Self {
            unknown_d,
            value_d,
        })
    }
}

impl crate::readers::ACDataType for WindowPropertyType1000008A {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        WindowPropertyType1000008A::read(reader)
    }
}

impl WindowPropertyType1000008D {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_c = read_u32(reader)?;
        let title_source = WindowPropertyType1000008DTitleSourceVariant::read(reader)?;

impl WindowPropertyType1000008DTitleSourceVariant {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let title_source = read_u8(reader)?;

        match title_source {
            0x00 => {
                let string_id = read_u32(reader)?;
                let file_id = read_u32(reader)?;
                return Ok(Self::Type0 {
                    string_id,
                    file_id,
                });
            },
            0x01 => {
                let value_a = read_wstring(reader).map(WString)?;
                return Ok(Self::Type1 {
                    value_a,
                });
            },
            _ => Err("Unknown nested switch value".into()),
        }
    }
}



        Ok(Self {
            unknown_c,
            title_source,
        })
    }
}

impl crate::readers::ACDataType for WindowPropertyType1000008D {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        WindowPropertyType1000008D::read(reader)
    }
}

impl WindowProperty {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let key_a = read_u32(reader)?;

        match key_a {
            0x1000007F => {
                let variant_struct = WindowPropertyType1000007F::read(reader)?;
                Ok(Self::Type1000007F(variant_struct))
            },
            0x10000086 => {
                let variant_struct = WindowPropertyType10000086::read(reader)?;
                Ok(Self::Type10000086(variant_struct))
            },
            0x10000087 => {
                let variant_struct = WindowPropertyType10000087::read(reader)?;
                Ok(Self::Type10000087(variant_struct))
            },
            0x10000088 => {
                let variant_struct = WindowPropertyType10000088::read(reader)?;
                Ok(Self::Type10000088(variant_struct))
            },
            0x10000089 => {
                let variant_struct = WindowPropertyType10000089::read(reader)?;
                Ok(Self::Type10000089(variant_struct))
            },
            0x1000008A => {
                let variant_struct = WindowPropertyType1000008A::read(reader)?;
                Ok(Self::Type1000008A(variant_struct))
            },
            0x1000008D => {
                let variant_struct = WindowPropertyType1000008D::read(reader)?;
                Ok(Self::Type1000008D(variant_struct))
            },
            _ => Err(format!("Unknown {} value: {:?}", "key_a", key_a).into()),
        }
    }
}

impl crate::readers::ACDataType for WindowProperty {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        WindowProperty::read(reader)
    }
}

impl WindowOptionType1000008B {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_b = read_u8(reader)?;
        let property_count = read_u8(reader)?;
        let properties = read_vec::<WindowProperty>(reader, property_count as usize)?;

        Ok(Self {
            unknown_b,
            property_count,
            properties,
        })
    }
}

impl crate::readers::ACDataType for WindowOptionType1000008B {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        WindowOptionType1000008B::read(reader)
    }
}

impl WindowOption {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let type_a = read_u32(reader)?;

        match type_a {
            0x1000008B => {
                let variant_struct = WindowOptionType1000008B::read(reader)?;
                Ok(Self::Type1000008B(variant_struct))
            },
            _ => Err(format!("Unknown {} value: {:?}", "type_a", type_a).into()),
        }
    }
}

impl crate::readers::ACDataType for WindowOption {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        WindowOption::read(reader)
    }
}

impl OptionPropertyType10000080 {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_l = read_u32(reader)?;
        let inactive_opacity = read_f32(reader)?;

        Ok(Self {
            unknown_l,
            inactive_opacity,
        })
    }
}

impl crate::readers::ACDataType for OptionPropertyType10000080 {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        OptionPropertyType10000080::read(reader)
    }
}

impl OptionPropertyType10000081 {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_k = read_u32(reader)?;
        let active_opacity = read_f32(reader)?;

        Ok(Self {
            unknown_k,
            active_opacity,
        })
    }
}

impl crate::readers::ACDataType for OptionPropertyType10000081 {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        OptionPropertyType10000081::read(reader)
    }
}

impl OptionPropertyType1000008C {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_a = read_u32(reader)?;
        let window_options = read_packable_list::<WindowOption>(reader)?;

        Ok(Self {
            unknown_a,
            window_options,
        })
    }
}

impl crate::readers::ACDataType for OptionPropertyType1000008C {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        OptionPropertyType1000008C::read(reader)
    }
}

impl OptionProperty {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let type_ = read_u32(reader)?;

        match type_ {
            0x10000080 => {
                let variant_struct = OptionPropertyType10000080::read(reader)?;
                Ok(Self::Type10000080(variant_struct))
            },
            0x10000081 => {
                let variant_struct = OptionPropertyType10000081::read(reader)?;
                Ok(Self::Type10000081(variant_struct))
            },
            0x1000008C => {
                let variant_struct = OptionPropertyType1000008C::read(reader)?;
                Ok(Self::Type1000008C(variant_struct))
            },
            _ => Err(format!("Unknown {} value: {:?}", "type_", type_).into()),
        }
    }
}

impl crate::readers::ACDataType for OptionProperty {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        OptionProperty::read(reader)
    }
}

