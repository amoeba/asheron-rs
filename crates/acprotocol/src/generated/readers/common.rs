// Binary readers for common types

#[allow(unused_imports)]
use std::io::Read;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;

impl Emote {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let type_ = EmoteType::try_from(read_u32(reader)?)?;
        let delay = read_f32(reader)?;
        let extent = read_f32(reader)?;

        match type_ {
            EmoteType::ActEmoteType | EmoteType::SayEmoteType | EmoteType::TellEmoteType | EmoteType::TextDirectEmoteType | EmoteType::WorldBroadcastEmoteType | EmoteType::LocalBroadcastEmoteType | EmoteType::DirectBroadcastEmoteType | EmoteType::UpdateQuestEmoteType | EmoteType::InqQuestEmoteType | EmoteType::StampQuestEmoteType | EmoteType::StartEventEmoteType | EmoteType::StopEventEmoteType | EmoteType::BLogEmoteType | EmoteType::AdminSpamEmoteType | EmoteType::EraseQuestEmoteType | EmoteType::InqEventEmoteType | EmoteType::InqFellowQuestEmoteType | EmoteType::UpdateFellowQuestEmoteType | EmoteType::StampFellowQuestEmoteType | EmoteType::TellFellowEmoteType | EmoteType::FellowBroadcastEmoteType | EmoteType::GotoEmoteType | EmoteType::PopUpEmoteType | EmoteType::UpdateMyQuestEmoteType | EmoteType::InqMyQuestEmoteType | EmoteType::StampMyQuestEmoteType | EmoteType::EraseMyQuestEmoteType | EmoteType::LocalSignalEmoteType | EmoteType::InqContractsFullEmoteType => {
                let message = read_string(reader)?;

                Ok(Self::Type1 {
                    delay,
                    extent,
                    message,
                })
            }
            EmoteType::AwardXPEmoteType | EmoteType::AwardNoShareXPEmoteType => {
                let amount64 = read_u64(reader)?;
                let hero_xp64 = read_u64(reader)?;

                Ok(Self::Type2 {
                    delay,
                    extent,
                    amount64,
                    hero_xp64,
                })
            }
            EmoteType::GiveEmoteType | EmoteType::TakeItemsEmoteType => {
                let c_profile = CreationProfile::read(reader)?;

                Ok(Self::Type3 {
                    delay,
                    extent,
                    c_profile,
                })
            }
            EmoteType::MoveHomeEmoteType | EmoteType::MoveEmoteType | EmoteType::TurnEmoteType | EmoteType::MoveToPosEmoteType => {
                let frame = Frame::read(reader)?;

                Ok(Self::Type4 {
                    delay,
                    extent,
                    frame,
                })
            }
            EmoteType::MotionEmoteType | EmoteType::ForceMotionEmoteType => {
                let motion = read_u32(reader)?;

                Ok(Self::Type5 {
                    delay,
                    extent,
                    motion,
                })
            }
            EmoteType::PhysScriptEmoteType => {
                let physics_script = read_u32(reader)?;

                Ok(Self::Type7 {
                    delay,
                    extent,
                    physics_script,
                })
            }
            EmoteType::SoundEmoteType => {
                let sound = read_u32(reader)?;

                Ok(Self::Type9 {
                    delay,
                    extent,
                    sound,
                })
            }
            EmoteType::CastSpellEmoteType | EmoteType::CastSpellInstantEmoteType | EmoteType::TeachSpellEmoteType | EmoteType::PetCastSpellOnOwnerEmoteType => {
                let spell_id = read_u32(reader)?;

                Ok(Self::TypeE {
                    delay,
                    extent,
                    spell_id,
                })
            }
            EmoteType::AwardSkillXPEmoteType | EmoteType::AwardSkillPointsEmoteType => {
                let amount = read_u32(reader)?;
                let stat = read_u32(reader)?;

                Ok(Self::Type1C {
                    delay,
                    extent,
                    amount,
                    stat,
                })
            }
            EmoteType::InqQuestSolvesEmoteType | EmoteType::InqFellowNumEmoteType | EmoteType::InqNumCharacterTitlesEmoteType | EmoteType::InqMyQuestSolvesEmoteType => {
                let message = read_string(reader)?;
                let min = read_u32(reader)?;
                let max = read_u32(reader)?;

                Ok(Self::Type1E {
                    delay,
                    extent,
                    message,
                    min,
                    max,
                })
            }
            EmoteType::DecrementQuestEmoteType | EmoteType::IncrementQuestEmoteType | EmoteType::SetQuestCompletionsEmoteType | EmoteType::DecrementMyQuestEmoteType | EmoteType::IncrementMyQuestEmoteType | EmoteType::SetMyQuestCompletionsEmoteType | EmoteType::InqPackSpaceEmoteType | EmoteType::InqQuestBitsOnEmoteType | EmoteType::InqQuestBitsOffEmoteType | EmoteType::InqMyQuestBitsOnEmoteType | EmoteType::InqMyQuestBitsOffEmoteType | EmoteType::SetQuestBitsOnEmoteType | EmoteType::SetQuestBitsOffEmoteType | EmoteType::SetMyQuestBitsOnEmoteType | EmoteType::SetMyQuestBitsOffEmoteType => {
                let message = read_string(reader)?;
                let amount = read_u32(reader)?;

                Ok(Self::Type20 {
                    delay,
                    extent,
                    message,
                    amount,
                })
            }
            EmoteType::AddCharacterTitleEmoteType | EmoteType::AwardTrainingCreditsEmoteType | EmoteType::InflictVitaePenaltyEmoteType | EmoteType::RemoveVitaePenaltyEmoteType | EmoteType::SetAltRacialSkillsEmoteType | EmoteType::AddContractEmoteType | EmoteType::RemoveContractEmoteType => {
                let amount = read_u32(reader)?;

                Ok(Self::Type22 {
                    delay,
                    extent,
                    amount,
                })
            }
            EmoteType::InqBoolStatEmoteType | EmoteType::InqSkillTrainedEmoteType | EmoteType::InqSkillSpecializedEmoteType => {
                let message = read_string(reader)?;
                let stat = read_u32(reader)?;

                Ok(Self::Type23 {
                    delay,
                    extent,
                    message,
                    stat,
                })
            }
            EmoteType::InqIntStatEmoteType | EmoteType::InqAttributeStatEmoteType | EmoteType::InqRawAttributeStatEmoteType | EmoteType::InqSecondaryAttributeStatEmoteType | EmoteType::InqRawSecondaryAttributeStatEmoteType | EmoteType::InqSkillStatEmoteType | EmoteType::InqRawSkillStatEmoteType => {
                let message = read_string(reader)?;
                let min = read_u32(reader)?;
                let max = read_u32(reader)?;
                let stat = read_u32(reader)?;

                Ok(Self::Type24 {
                    delay,
                    extent,
                    message,
                    min,
                    max,
                    stat,
                })
            }
            EmoteType::InqFloatStatEmoteType => {
                let message = read_string(reader)?;
                let f_min = read_f64(reader)?;
                let f_max = read_f64(reader)?;
                let stat = read_u32(reader)?;

                Ok(Self::Type25 {
                    delay,
                    extent,
                    message,
                    f_min,
                    f_max,
                    stat,
                })
            }
            EmoteType::InqStringStatEmoteType | EmoteType::InqYesNoEmoteType => {
                let message = read_string(reader)?;
                let test_string = read_string(reader)?;
                let stat = read_u32(reader)?;

                Ok(Self::Type26 {
                    delay,
                    extent,
                    message,
                    test_string,
                    stat,
                })
            }
            EmoteType::AwardLevelProportionalXPEmoteType => {
                let percent = read_f64(reader)?;
                let min64 = read_u64(reader)?;
                let max64 = read_u64(reader)?;

                Ok(Self::Type31 {
                    delay,
                    extent,
                    percent,
                    min64,
                    max64,
                })
            }
            EmoteType::AwardLevelProportionalSkillXPEmoteType => {
                let stat = read_u32(reader)?;
                let percent = read_f64(reader)?;
                let min = read_u32(reader)?;
                let max = read_u32(reader)?;
                let display = read_bool(reader)?;

                Ok(Self::Type32 {
                    delay,
                    extent,
                    stat,
                    percent,
                    min,
                    max,
                    display,
                })
            }
            EmoteType::SetIntStatEmoteType | EmoteType::IncrementIntStatEmoteType | EmoteType::DecrementIntStatEmoteType | EmoteType::SetBoolStatEmoteType => {
                let stat = read_u32(reader)?;
                let amount = read_u32(reader)?;

                Ok(Self::Type35 {
                    delay,
                    extent,
                    stat,
                    amount,
                })
            }
            EmoteType::CreateTreasureEmoteType => {
                let wealth_rating = read_i32(reader)?;
                let treasure_class = read_i32(reader)?;
                let treasure_type = read_i32(reader)?;

                Ok(Self::Type38 {
                    delay,
                    extent,
                    wealth_rating,
                    treasure_class,
                    treasure_type,
                })
            }
            EmoteType::SetSanctuaryPositionEmoteType | EmoteType::TeleportTargetEmoteType | EmoteType::TeleportSelfEmoteType => {
                let position = Position::read(reader)?;

                Ok(Self::Type3F {
                    delay,
                    extent,
                    position,
                })
            }
            EmoteType::InqOwnsItemsEmoteType => {
                let msg = read_string(reader)?;
                let c_profile = CreationProfile::read(reader)?;

                Ok(Self::Type4C {
                    delay,
                    extent,
                    msg,
                    c_profile,
                })
            }
            EmoteType::UntrainSkillEmoteType | EmoteType::SetInt64StatEmoteType => {
                let stat = read_u32(reader)?;

                Ok(Self::Type6E {
                    delay,
                    extent,
                    stat,
                })
            }
            EmoteType::SpendLuminanceEmoteType | EmoteType::AwardLuminanceEmoteType => {
                let amount64 = read_u64(reader)?;

                Ok(Self::Type70 {
                    delay,
                    extent,
                    amount64,
                })
            }
            EmoteType::InqInt64StatEmoteType => {
                let message = read_string(reader)?;
                let min64 = read_u64(reader)?;
                let max64 = read_u64(reader)?;
                let stat = read_u32(reader)?;

                Ok(Self::Type72 {
                    delay,
                    extent,
                    message,
                    min64,
                    max64,
                    stat,
                })
            }
            EmoteType::SetFloatStatEmoteType => {
                let stat = read_u32(reader)?;
                let percent = read_f64(reader)?;

                Ok(Self::Type76 {
                    delay,
                    extent,
                    stat,
                    percent,
                })
            }
            _ => Err(format!("Unknown Type value: {:#x}", type_ as u32).into())
        }
    }
}

impl crate::readers::ACDataType for Emote {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        Emote::read(reader)
    }
}

