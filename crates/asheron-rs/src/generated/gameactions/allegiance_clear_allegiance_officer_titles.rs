use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::writers::ACWriter;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::writers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;
#[cfg(feature = "tracing")]
#[allow(unused_imports)]
use tracing::{span, Level};

// Clears the allegiance officer titles
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_ClearAllegianceOfficerTitles")]
pub struct AllegianceClearAllegianceOfficerTitles {}

impl AllegianceClearAllegianceOfficerTitles {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for AllegianceClearAllegianceOfficerTitles {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceClearAllegianceOfficerTitles::read(reader)
    }
}

impl AllegianceClearAllegianceOfficerTitles {
    pub fn write(&self, _writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

impl crate::writers::ACWritable for AllegianceClearAllegianceOfficerTitles {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        AllegianceClearAllegianceOfficerTitles::write(self, writer)
    }
}

