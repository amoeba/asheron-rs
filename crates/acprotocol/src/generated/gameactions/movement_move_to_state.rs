use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;
#[cfg(feature = "tracing")]
#[allow(unused_imports)]
use tracing::{span, Level};

// Move to state data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_MoveToState")]
pub struct MovementMoveToState {
    #[serde(rename = "MoveToState")]
    pub move_to_state: MoveToStatePack,
}

impl crate::readers::ACDataType for MovementMoveToState {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "MovementMoveToState").entered();

        #[cfg(feature = "tracing")]
        let _field_span_move_to_state = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MoveToState", position = pos).entered()
        };
        let move_to_state = MoveToStatePack::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_move_to_state);

        Ok(Self {
            move_to_state,
        })
    }
}

