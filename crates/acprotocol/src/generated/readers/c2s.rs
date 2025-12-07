// Binary readers for c2s types

#[allow(unused_imports)]
use std::io::Read;
#[allow(unused_imports)]
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::types::*;
use crate::types::c2s::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;

impl CharacterPlayerOptionChangedEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let option = CharacterOptions1::try_from(read_u32(reader)?)?;
        let value = read_bool(reader)?;

        Ok(Self {
            option,
            value,
        })
    }
}

impl crate::readers::ACDataType for CharacterPlayerOptionChangedEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterPlayerOptionChangedEvent::read(reader)
    }
}

impl CombatTargetedMeleeAttack {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let height = AttackHeight::try_from(read_u32(reader)?)?;
        let power = read_f32(reader)?;

        Ok(Self {
            object_id,
            height,
            power,
        })
    }
}

impl crate::readers::ACDataType for CombatTargetedMeleeAttack {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CombatTargetedMeleeAttack::read(reader)
    }
}

impl CombatTargetedMissileAttack {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let height = AttackHeight::try_from(read_u32(reader)?)?;
        let accuracy = read_f32(reader)?;

        Ok(Self {
            object_id,
            height,
            accuracy,
        })
    }
}

impl crate::readers::ACDataType for CombatTargetedMissileAttack {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CombatTargetedMissileAttack::read(reader)
    }
}

impl CommunicationSetAFKMode {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let afk = read_bool(reader)?;

        Ok(Self {
            afk,
        })
    }
}

impl crate::readers::ACDataType for CommunicationSetAFKMode {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationSetAFKMode::read(reader)
    }
}

impl CommunicationSetAFKMessage {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;

        Ok(Self {
            message,
        })
    }
}

impl crate::readers::ACDataType for CommunicationSetAFKMessage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationSetAFKMessage::read(reader)
    }
}

impl CommunicationTalk {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;

        Ok(Self {
            message,
        })
    }
}

impl crate::readers::ACDataType for CommunicationTalk {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationTalk::read(reader)
    }
}

impl SocialRemoveFriend {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for SocialRemoveFriend {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SocialRemoveFriend::read(reader)
    }
}

impl SocialAddFriend {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character_name = read_string(reader)?;

        Ok(Self {
            character_name,
        })
    }
}

impl crate::readers::ACDataType for SocialAddFriend {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SocialAddFriend::read(reader)
    }
}

impl InventoryPutItemInContainer {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let container_id = ObjectId::read(reader)?;
        let slot_index = read_u32(reader)?;

        Ok(Self {
            object_id,
            container_id,
            slot_index,
        })
    }
}

impl crate::readers::ACDataType for InventoryPutItemInContainer {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        InventoryPutItemInContainer::read(reader)
    }
}

impl InventoryGetAndWieldItem {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let slot = EquipMask::try_from(read_u32(reader)?)?;

        Ok(Self {
            object_id,
            slot,
        })
    }
}

impl crate::readers::ACDataType for InventoryGetAndWieldItem {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        InventoryGetAndWieldItem::read(reader)
    }
}

impl InventoryDropItem {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for InventoryDropItem {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        InventoryDropItem::read(reader)
    }
}

impl AllegianceSwearAllegiance {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for AllegianceSwearAllegiance {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceSwearAllegiance::read(reader)
    }
}

impl AllegianceBreakAllegiance {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for AllegianceBreakAllegiance {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceBreakAllegiance::read(reader)
    }
}

impl AllegianceUpdateRequest {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let on = read_bool(reader)?;

        Ok(Self {
            on,
        })
    }
}

impl crate::readers::ACDataType for AllegianceUpdateRequest {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceUpdateRequest::read(reader)
    }
}

impl SocialClearFriends {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for SocialClearFriends {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SocialClearFriends::read(reader)
    }
}

impl CharacterTeleToPKLArena {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for CharacterTeleToPKLArena {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterTeleToPKLArena::read(reader)
    }
}

impl CharacterTeleToPKArena {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for CharacterTeleToPKArena {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterTeleToPKArena::read(reader)
    }
}

impl SocialSetDisplayCharacterTitle {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let title_id = read_u32(reader)?;

        Ok(Self {
            title_id,
        })
    }
}

impl crate::readers::ACDataType for SocialSetDisplayCharacterTitle {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SocialSetDisplayCharacterTitle::read(reader)
    }
}

impl AllegianceQueryAllegianceName {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for AllegianceQueryAllegianceName {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceQueryAllegianceName::read(reader)
    }
}

impl AllegianceClearAllegianceName {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for AllegianceClearAllegianceName {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceClearAllegianceName::read(reader)
    }
}

impl CommunicationTalkDirect {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;
        let target_id = ObjectId::read(reader)?;

        Ok(Self {
            message,
            target_id,
        })
    }
}

impl crate::readers::ACDataType for CommunicationTalkDirect {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationTalkDirect::read(reader)
    }
}

impl AllegianceSetAllegianceName {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let name = read_string(reader)?;

        Ok(Self {
            name,
        })
    }
}

impl crate::readers::ACDataType for AllegianceSetAllegianceName {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceSetAllegianceName::read(reader)
    }
}

impl InventoryUseWithTargetEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let target_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
            target_id,
        })
    }
}

impl crate::readers::ACDataType for InventoryUseWithTargetEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        InventoryUseWithTargetEvent::read(reader)
    }
}

impl InventoryUseEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for InventoryUseEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        InventoryUseEvent::read(reader)
    }
}

impl AllegianceSetAllegianceOfficer {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character_name = read_string(reader)?;
        let level = AllegianceOfficerLevel::try_from(read_u32(reader)?)?;

        Ok(Self {
            character_name,
            level,
        })
    }
}

impl crate::readers::ACDataType for AllegianceSetAllegianceOfficer {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceSetAllegianceOfficer::read(reader)
    }
}

impl AllegianceSetAllegianceOfficerTitle {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let level = AllegianceOfficerLevel::try_from(read_u32(reader)?)?;
        let title = read_string(reader)?;

        Ok(Self {
            level,
            title,
        })
    }
}

impl crate::readers::ACDataType for AllegianceSetAllegianceOfficerTitle {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceSetAllegianceOfficerTitle::read(reader)
    }
}

impl AllegianceListAllegianceOfficerTitles {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for AllegianceListAllegianceOfficerTitles {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceListAllegianceOfficerTitles::read(reader)
    }
}

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

impl AllegianceDoAllegianceLockAction {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let action = AllegianceLockAction::try_from(read_u32(reader)?)?;

        Ok(Self {
            action,
        })
    }
}

impl crate::readers::ACDataType for AllegianceDoAllegianceLockAction {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceDoAllegianceLockAction::read(reader)
    }
}

impl AllegianceSetAllegianceApprovedVassal {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character_name = read_string(reader)?;

        Ok(Self {
            character_name,
        })
    }
}

impl crate::readers::ACDataType for AllegianceSetAllegianceApprovedVassal {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceSetAllegianceApprovedVassal::read(reader)
    }
}

impl AllegianceAllegianceChatGag {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character_name = read_string(reader)?;
        let on = read_bool(reader)?;

        Ok(Self {
            character_name,
            on,
        })
    }
}

impl crate::readers::ACDataType for AllegianceAllegianceChatGag {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceAllegianceChatGag::read(reader)
    }
}

impl AllegianceDoAllegianceHouseAction {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let action = AllegianceHouseAction::try_from(read_u32(reader)?)?;

        Ok(Self {
            action,
        })
    }
}

impl crate::readers::ACDataType for AllegianceDoAllegianceHouseAction {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceDoAllegianceHouseAction::read(reader)
    }
}

impl TrainTrainAttribute2nd {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let type_ = VitalId::try_from(read_u32(reader)?)?;
        let experience = read_u32(reader)?;

        Ok(Self {
            type_,
            experience,
        })
    }
}

impl crate::readers::ACDataType for TrainTrainAttribute2nd {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TrainTrainAttribute2nd::read(reader)
    }
}

impl TrainTrainAttribute {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let type_ = AttributeId::try_from(read_u32(reader)?)?;
        let experience = read_u32(reader)?;

        Ok(Self {
            type_,
            experience,
        })
    }
}

impl crate::readers::ACDataType for TrainTrainAttribute {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TrainTrainAttribute::read(reader)
    }
}

impl TrainTrainSkill {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let skill = SkillId::try_from(read_i32(reader)?)?;
        let experience = read_u32(reader)?;

        Ok(Self {
            skill,
            experience,
        })
    }
}

impl crate::readers::ACDataType for TrainTrainSkill {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TrainTrainSkill::read(reader)
    }
}

impl TrainTrainSkillAdvancementClass {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let skill = SkillId::try_from(read_i32(reader)?)?;
        let credits = read_u32(reader)?;

        Ok(Self {
            skill,
            credits,
        })
    }
}

impl crate::readers::ACDataType for TrainTrainSkillAdvancementClass {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TrainTrainSkillAdvancementClass::read(reader)
    }
}

impl MagicCastUntargetedSpell {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let spell_id = LayeredSpellId::read(reader)?;

        Ok(Self {
            spell_id,
        })
    }
}

impl crate::readers::ACDataType for MagicCastUntargetedSpell {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MagicCastUntargetedSpell::read(reader)
    }
}

impl MagicCastTargetedSpell {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let spell_id = LayeredSpellId::read(reader)?;

        Ok(Self {
            object_id,
            spell_id,
        })
    }
}

impl crate::readers::ACDataType for MagicCastTargetedSpell {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MagicCastTargetedSpell::read(reader)
    }
}

impl CombatChangeCombatMode {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let mode = CombatMode::try_from(read_u32(reader)?)?;

        Ok(Self {
            mode,
        })
    }
}

impl crate::readers::ACDataType for CombatChangeCombatMode {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CombatChangeCombatMode::read(reader)
    }
}

impl InventoryStackableMerge {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let target_id = ObjectId::read(reader)?;
        let amount = read_u32(reader)?;

        Ok(Self {
            object_id,
            target_id,
            amount,
        })
    }
}

impl crate::readers::ACDataType for InventoryStackableMerge {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        InventoryStackableMerge::read(reader)
    }
}

impl InventoryStackableSplitToContainer {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let container_id = ObjectId::read(reader)?;
        let slot_index = read_u32(reader)?;
        let amount = read_u32(reader)?;

        Ok(Self {
            object_id,
            container_id,
            slot_index,
            amount,
        })
    }
}

impl crate::readers::ACDataType for InventoryStackableSplitToContainer {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        InventoryStackableSplitToContainer::read(reader)
    }
}

impl InventoryStackableSplitTo3D {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let amount = read_u32(reader)?;

        Ok(Self {
            object_id,
            amount,
        })
    }
}

impl crate::readers::ACDataType for InventoryStackableSplitTo3D {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        InventoryStackableSplitTo3D::read(reader)
    }
}

impl CommunicationModifyCharacterSquelch {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let add = read_bool(reader)?;
        let object_id = ObjectId::read(reader)?;
        let character_name = read_string(reader)?;
        let type_ = ChatFragmentType::try_from(read_u32(reader)?)?;

        Ok(Self {
            add,
            object_id,
            character_name,
            type_,
        })
    }
}

impl crate::readers::ACDataType for CommunicationModifyCharacterSquelch {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationModifyCharacterSquelch::read(reader)
    }
}

impl CommunicationModifyAccountSquelch {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let add = read_bool(reader)?;
        let character_name = read_string(reader)?;

        Ok(Self {
            add,
            character_name,
        })
    }
}

impl crate::readers::ACDataType for CommunicationModifyAccountSquelch {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationModifyAccountSquelch::read(reader)
    }
}

impl CommunicationModifyGlobalSquelch {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let add = read_bool(reader)?;
        let type_ = ChatFragmentType::try_from(read_u32(reader)?)?;

        Ok(Self {
            add,
            type_,
        })
    }
}

impl crate::readers::ACDataType for CommunicationModifyGlobalSquelch {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationModifyGlobalSquelch::read(reader)
    }
}

impl CommunicationTalkDirectByName {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;
        let target_name = read_string(reader)?;

        Ok(Self {
            message,
            target_name,
        })
    }
}

impl crate::readers::ACDataType for CommunicationTalkDirectByName {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationTalkDirectByName::read(reader)
    }
}

impl VendorBuy {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let items = read_packable_list::<ItemProfile>(reader)?;
        let alternate_currency_id = read_u32(reader)?;

        Ok(Self {
            object_id,
            items,
            alternate_currency_id,
        })
    }
}

impl crate::readers::ACDataType for VendorBuy {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        VendorBuy::read(reader)
    }
}

impl VendorSell {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let items = read_packable_list::<ItemProfile>(reader)?;

        Ok(Self {
            object_id,
            items,
        })
    }
}

impl crate::readers::ACDataType for VendorSell {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        VendorSell::read(reader)
    }
}

impl CharacterTeleToLifestone {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for CharacterTeleToLifestone {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterTeleToLifestone::read(reader)
    }
}

impl CharacterLoginCompleteNotification {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for CharacterLoginCompleteNotification {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterLoginCompleteNotification::read(reader)
    }
}

impl FellowshipCreate {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let name = read_string(reader)?;
        let share_xp = read_bool(reader)?;

        Ok(Self {
            name,
            share_xp,
        })
    }
}

impl crate::readers::ACDataType for FellowshipCreate {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        FellowshipCreate::read(reader)
    }
}

impl FellowshipQuit {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let disband = read_bool(reader)?;

        Ok(Self {
            disband,
        })
    }
}

impl crate::readers::ACDataType for FellowshipQuit {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        FellowshipQuit::read(reader)
    }
}

impl FellowshipDismiss {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for FellowshipDismiss {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        FellowshipDismiss::read(reader)
    }
}

impl FellowshipRecruit {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for FellowshipRecruit {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        FellowshipRecruit::read(reader)
    }
}

impl FellowshipUpdateRequest {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let on = read_bool(reader)?;

        Ok(Self {
            on,
        })
    }
}

impl crate::readers::ACDataType for FellowshipUpdateRequest {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        FellowshipUpdateRequest::read(reader)
    }
}

impl WritingBookAddPage {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for WritingBookAddPage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        WritingBookAddPage::read(reader)
    }
}

impl WritingBookModifyPage {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let page_num = read_i32(reader)?;
        let page_text = read_string(reader)?;

        Ok(Self {
            object_id,
            page_num,
            page_text,
        })
    }
}

impl crate::readers::ACDataType for WritingBookModifyPage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        WritingBookModifyPage::read(reader)
    }
}

impl WritingBookData {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for WritingBookData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        WritingBookData::read(reader)
    }
}

impl WritingBookDeletePage {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let page_num = read_i32(reader)?;

        Ok(Self {
            object_id,
            page_num,
        })
    }
}

impl crate::readers::ACDataType for WritingBookDeletePage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        WritingBookDeletePage::read(reader)
    }
}

impl WritingBookPageData {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let page_num = read_i32(reader)?;

        Ok(Self {
            object_id,
            page_num,
        })
    }
}

impl crate::readers::ACDataType for WritingBookPageData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        WritingBookPageData::read(reader)
    }
}

impl WritingSetInscription {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let inscription = read_string(reader)?;

        Ok(Self {
            object_id,
            inscription,
        })
    }
}

impl crate::readers::ACDataType for WritingSetInscription {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        WritingSetInscription::read(reader)
    }
}

impl ItemAppraise {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for ItemAppraise {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemAppraise::read(reader)
    }
}

impl InventoryGiveObjectRequest {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let target_id = ObjectId::read(reader)?;
        let object_id = ObjectId::read(reader)?;
        let amount = read_u32(reader)?;

        Ok(Self {
            target_id,
            object_id,
            amount,
        })
    }
}

impl crate::readers::ACDataType for InventoryGiveObjectRequest {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        InventoryGiveObjectRequest::read(reader)
    }
}

impl AdvocateTeleport {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = read_string(reader)?;
        let destination = Position::read(reader)?;

        Ok(Self {
            object_id,
            destination,
        })
    }
}

impl crate::readers::ACDataType for AdvocateTeleport {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AdvocateTeleport::read(reader)
    }
}

impl CharacterAbuseLogRequest {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character = read_string(reader)?;
        let status = read_u32(reader)?;
        let complaint = read_string(reader)?;

        Ok(Self {
            character,
            status,
            complaint,
        })
    }
}

impl crate::readers::ACDataType for CharacterAbuseLogRequest {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterAbuseLogRequest::read(reader)
    }
}

impl CommunicationAddToChannel {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = Channel::try_from(read_u32(reader)?)?;

        Ok(Self {
            channel,
        })
    }
}

impl crate::readers::ACDataType for CommunicationAddToChannel {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationAddToChannel::read(reader)
    }
}

impl CommunicationRemoveFromChannel {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = Channel::try_from(read_u32(reader)?)?;

        Ok(Self {
            channel,
        })
    }
}

impl crate::readers::ACDataType for CommunicationRemoveFromChannel {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationRemoveFromChannel::read(reader)
    }
}

impl CommunicationChannelBroadcast {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = Channel::try_from(read_u32(reader)?)?;
        let sender_name = read_string(reader)?;
        let message = read_string(reader)?;

        Ok(Self {
            channel,
            sender_name,
            message,
        })
    }
}

impl crate::readers::ACDataType for CommunicationChannelBroadcast {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationChannelBroadcast::read(reader)
    }
}

impl CommunicationChannelList {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = Channel::try_from(read_u32(reader)?)?;

        Ok(Self {
            channel,
        })
    }
}

impl crate::readers::ACDataType for CommunicationChannelList {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationChannelList::read(reader)
    }
}

impl CommunicationChannelIndex {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for CommunicationChannelIndex {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationChannelIndex::read(reader)
    }
}

impl InventoryNoLongerViewingContents {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for InventoryNoLongerViewingContents {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        InventoryNoLongerViewingContents::read(reader)
    }
}

impl InventoryStackableSplitToWield {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let slot = EquipMask::try_from(read_u32(reader)?)?;
        let amount = read_i32(reader)?;

        Ok(Self {
            object_id,
            slot,
            amount,
        })
    }
}

impl crate::readers::ACDataType for InventoryStackableSplitToWield {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        InventoryStackableSplitToWield::read(reader)
    }
}

impl CharacterAddShortCut {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let shortcut = ShortCutData::read(reader)?;

        Ok(Self {
            shortcut,
        })
    }
}

impl crate::readers::ACDataType for CharacterAddShortCut {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterAddShortCut::read(reader)
    }
}

impl CharacterRemoveShortCut {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let index = read_u32(reader)?;

        Ok(Self {
            index,
        })
    }
}

impl crate::readers::ACDataType for CharacterRemoveShortCut {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterRemoveShortCut::read(reader)
    }
}

impl CharacterCharacterOptionsEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let options = PlayerModule::read(reader)?;

        Ok(Self {
            options,
        })
    }
}

impl crate::readers::ACDataType for CharacterCharacterOptionsEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterCharacterOptionsEvent::read(reader)
    }
}

impl MagicRemoveSpell {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let spell_id = LayeredSpellId::read(reader)?;

        Ok(Self {
            spell_id,
        })
    }
}

impl crate::readers::ACDataType for MagicRemoveSpell {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MagicRemoveSpell::read(reader)
    }
}

impl CombatCancelAttack {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for CombatCancelAttack {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CombatCancelAttack::read(reader)
    }
}

impl CombatQueryHealth {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for CombatQueryHealth {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CombatQueryHealth::read(reader)
    }
}

impl CharacterQueryAge {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for CharacterQueryAge {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterQueryAge::read(reader)
    }
}

impl CharacterQueryBirth {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for CharacterQueryBirth {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterQueryBirth::read(reader)
    }
}

impl CommunicationEmote {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;

        Ok(Self {
            message,
        })
    }
}

impl crate::readers::ACDataType for CommunicationEmote {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationEmote::read(reader)
    }
}

impl CommunicationSoulEmote {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;

        Ok(Self {
            message,
        })
    }
}

impl crate::readers::ACDataType for CommunicationSoulEmote {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationSoulEmote::read(reader)
    }
}

impl CharacterAddSpellFavorite {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let spell_id = LayeredSpellId::read(reader)?;
        let index = read_u32(reader)?;
        let spell_bar = read_u32(reader)?;

        Ok(Self {
            spell_id,
            index,
            spell_bar,
        })
    }
}

impl crate::readers::ACDataType for CharacterAddSpellFavorite {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterAddSpellFavorite::read(reader)
    }
}

impl CharacterRemoveSpellFavorite {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let spell_id = LayeredSpellId::read(reader)?;
        let spell_bar = read_u32(reader)?;

        Ok(Self {
            spell_id,
            spell_bar,
        })
    }
}

impl crate::readers::ACDataType for CharacterRemoveSpellFavorite {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterRemoveSpellFavorite::read(reader)
    }
}

impl CharacterRequestPing {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for CharacterRequestPing {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterRequestPing::read(reader)
    }
}

impl TradeOpenTradeNegotiations {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for TradeOpenTradeNegotiations {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TradeOpenTradeNegotiations::read(reader)
    }
}

impl TradeCloseTradeNegotiations {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for TradeCloseTradeNegotiations {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TradeCloseTradeNegotiations::read(reader)
    }
}

impl TradeAddToTrade {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let slot_index = read_u32(reader)?;

        Ok(Self {
            object_id,
            slot_index,
        })
    }
}

impl crate::readers::ACDataType for TradeAddToTrade {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TradeAddToTrade::read(reader)
    }
}

impl TradeAcceptTrade {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = Trade::read(reader)?;

        Ok(Self {
            contents,
        })
    }
}

impl crate::readers::ACDataType for TradeAcceptTrade {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TradeAcceptTrade::read(reader)
    }
}

impl TradeDeclineTrade {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for TradeDeclineTrade {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TradeDeclineTrade::read(reader)
    }
}

impl TradeResetTrade {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for TradeResetTrade {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TradeResetTrade::read(reader)
    }
}

impl CharacterClearPlayerConsentList {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for CharacterClearPlayerConsentList {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterClearPlayerConsentList::read(reader)
    }
}

impl CharacterDisplayPlayerConsentList {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for CharacterDisplayPlayerConsentList {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterDisplayPlayerConsentList::read(reader)
    }
}

impl CharacterRemoveFromPlayerConsentList {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let target_name = read_string(reader)?;

        Ok(Self {
            target_name,
        })
    }
}

impl crate::readers::ACDataType for CharacterRemoveFromPlayerConsentList {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterRemoveFromPlayerConsentList::read(reader)
    }
}

impl CharacterAddPlayerPermission {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let target_name = read_string(reader)?;

        Ok(Self {
            target_name,
        })
    }
}

impl crate::readers::ACDataType for CharacterAddPlayerPermission {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterAddPlayerPermission::read(reader)
    }
}

impl HouseBuyHouse {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let items = read_packable_list::<ObjectId>(reader)?;

        Ok(Self {
            object_id,
            items,
        })
    }
}

impl crate::readers::ACDataType for HouseBuyHouse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseBuyHouse::read(reader)
    }
}

impl HouseQueryHouse {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for HouseQueryHouse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseQueryHouse::read(reader)
    }
}

impl HouseAbandonHouse {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for HouseAbandonHouse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseAbandonHouse::read(reader)
    }
}

impl CharacterRemovePlayerPermission {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let target_name = read_string(reader)?;

        Ok(Self {
            target_name,
        })
    }
}

impl crate::readers::ACDataType for CharacterRemovePlayerPermission {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterRemovePlayerPermission::read(reader)
    }
}

impl HouseRentHouse {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let items = read_packable_list::<ObjectId>(reader)?;

        Ok(Self {
            object_id,
            items,
        })
    }
}

impl crate::readers::ACDataType for HouseRentHouse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseRentHouse::read(reader)
    }
}

impl CharacterSetDesiredComponentLevel {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let wcid = read_u32(reader)?;
        let amount = read_u32(reader)?;

        Ok(Self {
            wcid,
            amount,
        })
    }
}

impl crate::readers::ACDataType for CharacterSetDesiredComponentLevel {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterSetDesiredComponentLevel::read(reader)
    }
}

impl HouseAddPermanentGuest {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let guest_name = read_string(reader)?;

        Ok(Self {
            guest_name,
        })
    }
}

impl crate::readers::ACDataType for HouseAddPermanentGuest {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseAddPermanentGuest::read(reader)
    }
}

impl HouseRemovePermanentGuest {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let guest_name = read_string(reader)?;

        Ok(Self {
            guest_name,
        })
    }
}

impl crate::readers::ACDataType for HouseRemovePermanentGuest {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseRemovePermanentGuest::read(reader)
    }
}

impl HouseSetOpenHouseStatus {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let open_house = read_bool(reader)?;

        Ok(Self {
            open_house,
        })
    }
}

impl crate::readers::ACDataType for HouseSetOpenHouseStatus {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseSetOpenHouseStatus::read(reader)
    }
}

impl HouseChangeStoragePermission {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let guest_name = read_string(reader)?;
        let has_permission = read_bool(reader)?;

        Ok(Self {
            guest_name,
            has_permission,
        })
    }
}

impl crate::readers::ACDataType for HouseChangeStoragePermission {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseChangeStoragePermission::read(reader)
    }
}

impl HouseBootSpecificHouseGuest {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let guest_name = read_string(reader)?;

        Ok(Self {
            guest_name,
        })
    }
}

impl crate::readers::ACDataType for HouseBootSpecificHouseGuest {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseBootSpecificHouseGuest::read(reader)
    }
}

impl HouseRemoveAllStoragePermission {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for HouseRemoveAllStoragePermission {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseRemoveAllStoragePermission::read(reader)
    }
}

impl HouseRequestFullGuestList {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for HouseRequestFullGuestList {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseRequestFullGuestList::read(reader)
    }
}

impl AllegianceSetMotd {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;

        Ok(Self {
            message,
        })
    }
}

impl crate::readers::ACDataType for AllegianceSetMotd {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceSetMotd::read(reader)
    }
}

impl AllegianceQueryMotd {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for AllegianceQueryMotd {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceQueryMotd::read(reader)
    }
}

impl AllegianceClearMotd {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for AllegianceClearMotd {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceClearMotd::read(reader)
    }
}

impl HouseQueryLord {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for HouseQueryLord {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseQueryLord::read(reader)
    }
}

impl HouseAddAllStoragePermission {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for HouseAddAllStoragePermission {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseAddAllStoragePermission::read(reader)
    }
}

impl HouseRemoveAllPermanentGuests {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for HouseRemoveAllPermanentGuests {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseRemoveAllPermanentGuests::read(reader)
    }
}

impl HouseBootEveryone {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for HouseBootEveryone {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseBootEveryone::read(reader)
    }
}

impl HouseTeleToHouse {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for HouseTeleToHouse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseTeleToHouse::read(reader)
    }
}

impl ItemQueryItemMana {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for ItemQueryItemMana {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemQueryItemMana::read(reader)
    }
}

impl HouseSetHooksVisibility {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let visible = read_bool(reader)?;

        Ok(Self {
            visible,
        })
    }
}

impl crate::readers::ACDataType for HouseSetHooksVisibility {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseSetHooksVisibility::read(reader)
    }
}

impl HouseModifyAllegianceGuestPermission {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let add = read_bool(reader)?;

        Ok(Self {
            add,
        })
    }
}

impl crate::readers::ACDataType for HouseModifyAllegianceGuestPermission {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseModifyAllegianceGuestPermission::read(reader)
    }
}

impl HouseModifyAllegianceStoragePermission {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let add = read_bool(reader)?;

        Ok(Self {
            add,
        })
    }
}

impl crate::readers::ACDataType for HouseModifyAllegianceStoragePermission {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseModifyAllegianceStoragePermission::read(reader)
    }
}

impl GameJoin {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let game_id = read_u32(reader)?;
        let team = read_u32(reader)?;

        Ok(Self {
            game_id,
            team,
        })
    }
}

impl crate::readers::ACDataType for GameJoin {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GameJoin::read(reader)
    }
}

impl GameQuit {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for GameQuit {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GameQuit::read(reader)
    }
}

impl GameMove {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let x_from = read_i32(reader)?;
        let y_from = read_i32(reader)?;
        let x_to = read_i32(reader)?;
        let y_to = read_i32(reader)?;

        Ok(Self {
            x_from,
            y_from,
            x_to,
            y_to,
        })
    }
}

impl crate::readers::ACDataType for GameMove {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GameMove::read(reader)
    }
}

impl GameMovePass {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for GameMovePass {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GameMovePass::read(reader)
    }
}

impl GameStalemate {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let on = read_bool(reader)?;

        Ok(Self {
            on,
        })
    }
}

impl crate::readers::ACDataType for GameStalemate {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GameStalemate::read(reader)
    }
}

impl HouseListAvailableHouses {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let type_ = HouseType::try_from(read_u32(reader)?)?;

        Ok(Self {
            type_,
        })
    }
}

impl crate::readers::ACDataType for HouseListAvailableHouses {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseListAvailableHouses::read(reader)
    }
}

impl CharacterConfirmationResponse {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let type_ = ConfirmationType::try_from(read_u32(reader)?)?;
        let context = read_u32(reader)?;
        let accepted = read_bool(reader)?;

        Ok(Self {
            type_,
            context,
            accepted,
        })
    }
}

impl crate::readers::ACDataType for CharacterConfirmationResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterConfirmationResponse::read(reader)
    }
}

impl AllegianceBreakAllegianceBoot {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let bootee_name = read_string(reader)?;
        let account_boot = read_bool(reader)?;

        Ok(Self {
            bootee_name,
            account_boot,
        })
    }
}

impl crate::readers::ACDataType for AllegianceBreakAllegianceBoot {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceBreakAllegianceBoot::read(reader)
    }
}

impl HouseTeleToMansion {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for HouseTeleToMansion {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseTeleToMansion::read(reader)
    }
}

impl CharacterSuicide {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for CharacterSuicide {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterSuicide::read(reader)
    }
}

impl AllegianceAllegianceInfoRequest {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let target_name = read_string(reader)?;

        Ok(Self {
            target_name,
        })
    }
}

impl crate::readers::ACDataType for AllegianceAllegianceInfoRequest {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceAllegianceInfoRequest::read(reader)
    }
}

impl InventoryCreateTinkeringTool {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let tool_id = ObjectId::read(reader)?;
        let items = read_packable_list::<ObjectId>(reader)?;

        Ok(Self {
            tool_id,
            items,
        })
    }
}

impl crate::readers::ACDataType for InventoryCreateTinkeringTool {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        InventoryCreateTinkeringTool::read(reader)
    }
}

impl CharacterSpellbookFilterEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let options = SpellBookFilterOptions::try_from(read_u32(reader)?)?;

        Ok(Self {
            options,
        })
    }
}

impl crate::readers::ACDataType for CharacterSpellbookFilterEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterSpellbookFilterEvent::read(reader)
    }
}

impl CharacterTeleToMarketplace {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for CharacterTeleToMarketplace {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterTeleToMarketplace::read(reader)
    }
}

impl CharacterEnterPKLite {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for CharacterEnterPKLite {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterEnterPKLite::read(reader)
    }
}

impl FellowshipAssignNewLeader {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for FellowshipAssignNewLeader {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        FellowshipAssignNewLeader::read(reader)
    }
}

impl FellowshipChangeFellowOpeness {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let open = read_bool(reader)?;

        Ok(Self {
            open,
        })
    }
}

impl crate::readers::ACDataType for FellowshipChangeFellowOpeness {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        FellowshipChangeFellowOpeness::read(reader)
    }
}

impl AllegianceAllegianceChatBoot {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character_name = read_string(reader)?;
        let reason = read_string(reader)?;

        Ok(Self {
            character_name,
            reason,
        })
    }
}

impl crate::readers::ACDataType for AllegianceAllegianceChatBoot {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceAllegianceChatBoot::read(reader)
    }
}

impl AllegianceAddAllegianceBan {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character_name = read_string(reader)?;

        Ok(Self {
            character_name,
        })
    }
}

impl crate::readers::ACDataType for AllegianceAddAllegianceBan {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceAddAllegianceBan::read(reader)
    }
}

impl AllegianceRemoveAllegianceBan {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character_name = read_string(reader)?;

        Ok(Self {
            character_name,
        })
    }
}

impl crate::readers::ACDataType for AllegianceRemoveAllegianceBan {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceRemoveAllegianceBan::read(reader)
    }
}

impl AllegianceListAllegianceBans {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for AllegianceListAllegianceBans {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceListAllegianceBans::read(reader)
    }
}

impl AllegianceRemoveAllegianceOfficer {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character_name = read_string(reader)?;

        Ok(Self {
            character_name,
        })
    }
}

impl crate::readers::ACDataType for AllegianceRemoveAllegianceOfficer {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceRemoveAllegianceOfficer::read(reader)
    }
}

impl AllegianceListAllegianceOfficers {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for AllegianceListAllegianceOfficers {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceListAllegianceOfficers::read(reader)
    }
}

impl AllegianceClearAllegianceOfficers {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for AllegianceClearAllegianceOfficers {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceClearAllegianceOfficers::read(reader)
    }
}

impl AllegianceRecallAllegianceHometown {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for AllegianceRecallAllegianceHometown {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceRecallAllegianceHometown::read(reader)
    }
}

impl AdminQueryPluginListResponse {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let context = read_u32(reader)?;
        let plugin_list = read_string(reader)?;

        Ok(Self {
            context,
            plugin_list,
        })
    }
}

impl crate::readers::ACDataType for AdminQueryPluginListResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AdminQueryPluginListResponse::read(reader)
    }
}

impl AdminQueryPluginResponse {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let context = read_u32(reader)?;
        let success = read_bool(reader)?;
        let plugin_name = read_string(reader)?;
        let plugin_author = read_string(reader)?;
        let plugin_email = read_string(reader)?;
        let plugin_webpage = read_string(reader)?;

        Ok(Self {
            context,
            success,
            plugin_name,
            plugin_author,
            plugin_email,
            plugin_webpage,
        })
    }
}

impl crate::readers::ACDataType for AdminQueryPluginResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AdminQueryPluginResponse::read(reader)
    }
}

impl CharacterFinishBarber {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let base_palette = DataId::read(reader)?;
        let head_object = DataId::read(reader)?;
        let head_texture = DataId::read(reader)?;
        let default_head_texture = DataId::read(reader)?;
        let eyes_texture = DataId::read(reader)?;
        let default_eyes_texture = DataId::read(reader)?;
        let nose_texture = DataId::read(reader)?;
        let default_nose_texture = DataId::read(reader)?;
        let mouth_texture = DataId::read(reader)?;
        let default_mouth_texture = DataId::read(reader)?;
        let skin_palette = DataId::read(reader)?;
        let hair_palette = DataId::read(reader)?;
        let eyes_palette = DataId::read(reader)?;
        let setup_id = DataId::read(reader)?;
        let option1 = read_i32(reader)?;
        let option2 = read_i32(reader)?;

        Ok(Self {
            base_palette,
            head_object,
            head_texture,
            default_head_texture,
            eyes_texture,
            default_eyes_texture,
            nose_texture,
            default_nose_texture,
            mouth_texture,
            default_mouth_texture,
            skin_palette,
            hair_palette,
            eyes_palette,
            setup_id,
            option1,
            option2,
        })
    }
}

impl crate::readers::ACDataType for CharacterFinishBarber {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterFinishBarber::read(reader)
    }
}

impl SocialAbandonContract {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let contract_id = ContractId::try_from(read_u32(reader)?)?;

        Ok(Self {
            contract_id,
        })
    }
}

impl crate::readers::ACDataType for SocialAbandonContract {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SocialAbandonContract::read(reader)
    }
}

impl MovementJump {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let jump = JumpPack::read(reader)?;

        Ok(Self {
            jump,
        })
    }
}

impl crate::readers::ACDataType for MovementJump {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MovementJump::read(reader)
    }
}

impl MovementMoveToState {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let move_to_state = MoveToStatePack::read(reader)?;

        Ok(Self {
            move_to_state,
        })
    }
}

impl crate::readers::ACDataType for MovementMoveToState {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MovementMoveToState::read(reader)
    }
}

impl MovementDoMovementCommand {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let motion = read_u32(reader)?;
        let speed = read_f32(reader)?;
        let hold_key = HoldKey::try_from(read_u32(reader)?)?;

        Ok(Self {
            motion,
            speed,
            hold_key,
        })
    }
}

impl crate::readers::ACDataType for MovementDoMovementCommand {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MovementDoMovementCommand::read(reader)
    }
}

impl MovementStopMovementCommand {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let motion = read_u32(reader)?;
        let hold_key = HoldKey::try_from(read_u32(reader)?)?;

        Ok(Self {
            motion,
            hold_key,
        })
    }
}

impl crate::readers::ACDataType for MovementStopMovementCommand {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MovementStopMovementCommand::read(reader)
    }
}

impl MovementAutonomyLevel {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let autonomy_level = read_u32(reader)?;
        let __alignment_marker___align_dword = align_dword(reader)?;

        Ok(Self {
            autonomy_level,
        })
    }
}

impl crate::readers::ACDataType for MovementAutonomyLevel {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MovementAutonomyLevel::read(reader)
    }
}

impl MovementAutonomousPosition {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let position = AutonomousPositionPack::read(reader)?;

        Ok(Self {
            position,
        })
    }
}

impl crate::readers::ACDataType for MovementAutonomousPosition {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MovementAutonomousPosition::read(reader)
    }
}

impl MovementJumpNonAutonomous {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let extent = read_f32(reader)?;

        Ok(Self {
            extent,
        })
    }
}

impl crate::readers::ACDataType for MovementJumpNonAutonomous {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MovementJumpNonAutonomous::read(reader)
    }
}

impl LoginLogOffCharacter {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for LoginLogOffCharacter {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        LoginLogOffCharacter::read(reader)
    }
}

impl CharacterCharacterDelete {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let account = read_string(reader)?;
        let slot = read_i32(reader)?;

        Ok(Self {
            account,
            slot,
        })
    }
}

impl crate::readers::ACDataType for CharacterCharacterDelete {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterCharacterDelete::read(reader)
    }
}

impl CharacterSendCharGenResult {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let account = read_string(reader)?;
        let result = CharGenResult::read(reader)?;

        Ok(Self {
            account,
            result,
        })
    }
}

impl crate::readers::ACDataType for CharacterSendCharGenResult {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterSendCharGenResult::read(reader)
    }
}

impl LoginSendEnterWorld {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character_id = ObjectId::read(reader)?;
        let account = read_string(reader)?;

        Ok(Self {
            character_id,
            account,
        })
    }
}

impl crate::readers::ACDataType for LoginSendEnterWorld {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        LoginSendEnterWorld::read(reader)
    }
}

impl ObjectSendForceObjdesc {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for ObjectSendForceObjdesc {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ObjectSendForceObjdesc::read(reader)
    }
}

impl LoginSendEnterWorldRequest {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for LoginSendEnterWorldRequest {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        LoginSendEnterWorldRequest::read(reader)
    }
}

impl AdminSendAdminGetServerVersion {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for AdminSendAdminGetServerVersion {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AdminSendAdminGetServerVersion::read(reader)
    }
}

impl SocialSendFriendsCommand {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let command = read_u32(reader)?;
        let player = read_string(reader)?;

        Ok(Self {
            command,
            player,
        })
    }
}

impl crate::readers::ACDataType for SocialSendFriendsCommand {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SocialSendFriendsCommand::read(reader)
    }
}

impl AdminSendAdminRestoreCharacter {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let restored_char_name = read_string(reader)?;
        let account_to_restore_to = read_string(reader)?;

        Ok(Self {
            object_id,
            restored_char_name,
            account_to_restore_to,
        })
    }
}

impl crate::readers::ACDataType for AdminSendAdminRestoreCharacter {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AdminSendAdminRestoreCharacter::read(reader)
    }
}

impl CommunicationTurbineChat {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let mmessage_size = read_u32(reader)?;
        let type_ = TurbineChatType::try_from(read_u32(reader)?)?;
        let blob_dispatch_type = read_u32(reader)?;
        let target_type = read_i32(reader)?;
        let target_id = read_i32(reader)?;
        let transport_type = read_i32(reader)?;
        let transport_id = read_i32(reader)?;
        let cookie = read_i32(reader)?;
        let payload_size = read_u32(reader)?;

        match type_ {
            _ => Err(format!("Unknown {} value: {:?}", "type_", type_).into()),
        }
    }
}

impl crate::readers::ACDataType for CommunicationTurbineChat {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationTurbineChat::read(reader)
    }
}

impl DDDRequestDataMessage {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let resource_type = read_u32(reader)?;
        let resource_id = DataId::read(reader)?;

        Ok(Self {
            resource_type,
            resource_id,
        })
    }
}

impl crate::readers::ACDataType for DDDRequestDataMessage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        DDDRequestDataMessage::read(reader)
    }
}

impl DDDInterrogationResponseMessage {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let language = read_u32(reader)?;
        let files = read_packable_list::<i64>(reader)?;

        Ok(Self {
            language,
            files,
        })
    }
}

impl crate::readers::ACDataType for DDDInterrogationResponseMessage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        DDDInterrogationResponseMessage::read(reader)
    }
}

impl DDDEndDDDMessage {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for DDDEndDDDMessage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        DDDEndDDDMessage::read(reader)
    }
}

impl DDDOnEndDDD {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for DDDOnEndDDD {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        DDDOnEndDDD::read(reader)
    }
}

