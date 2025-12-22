use crate::cli_helper::parse_opcode_filter;
use crate::network::RawMessage;

use super::output::{format_parsed_messages, format_raw_messages};
use super::types::{DirectionFilter, OutputFormat, SortField};

/// Filter, sort, and output messages based on provided criteria
#[allow(clippy::too_many_arguments)]
pub fn output_messages(
    messages: &[RawMessage],
    id: Option<u32>,
    filter_type: Option<&str>,
    filter_opcode: Option<&str>,
    direction: Option<DirectionFilter>,
    sort: SortField,
    reverse: bool,
    limit: Option<usize>,
    output: OutputFormat,
    raw: bool,
) {
    // Parse opcode filter if provided
    let opcode_filter: Option<u32> = filter_opcode.and_then(|s| parse_opcode_filter(s).ok());

    let mut filtered: Vec<&RawMessage> = messages
        .iter()
        .filter(|m| {
            if let Some(msg_id) = id
                && m.id != msg_id
            {
                return false;
            }
            if let Some(ft) = filter_type
                && !m.message_type.to_lowercase().contains(&ft.to_lowercase())
            {
                return false;
            }
            if let Some(oc) = opcode_filter
                && m.opcode != oc
            {
                return false;
            }
            if let Some(d) = direction {
                match d {
                    DirectionFilter::Send => {
                        if m.direction != "Send" {
                            return false;
                        }
                    }
                    DirectionFilter::Recv => {
                        if m.direction != "Recv" {
                            return false;
                        }
                    }
                }
            }
            true
        })
        .collect();

    filtered.sort_by(|a, b| {
        let cmp = match sort {
            SortField::Id => a.id.cmp(&b.id),
            SortField::Type => a.message_type.cmp(&b.message_type),
            SortField::Direction => a.direction.cmp(&b.direction),
        };
        if reverse { cmp.reverse() } else { cmp }
    });

    if let Some(lim) = limit {
        filtered.truncate(lim);
    }

    if raw {
        format_raw_messages(filtered, output);
    } else {
        format_parsed_messages(filtered, output);
    }
}
