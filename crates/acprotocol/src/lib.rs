mod generated {
    include!("generated/mod.rs");
}

pub use generated::enums;
pub use generated::messages;
pub use generated::gameactions;
pub use generated::gameevents;
pub use generated::types;

pub mod readers;
pub mod network;

// WIP: this is what I want to generate for a type that uses a switch like
// GameMoveData. This code is just testing that out.
//
// <type name="GameMoveData" text="Set of information related to a chess game move">
// 	<field type="int" name="Type" text="Type of move" />
// 	<field type="ObjectId" name="PlayerId" text="Player making the move" />
// 	<field type="int" name="Team" text="Team making this move" />
// 	<switch name="Type">
// 		<case value="0x4">
// 			<field type="int" name="IdPieceToMove" text="Id of piece being moved" />
// 			<field type="int" name="YGrid" />
// 		</case>
// 		<case value="0x5">
// 			<field type="int" name="IdPieceToMove" text="Id of piece being moved" />
// 			<field type="int" name="YGrid" />
// 			<field type="int" name="XTo" text="x position to move the piece" />
// 			<field type="int" name="YTo" text="y position to move the piece" />
// 		</case>
// 		<case value="0x6">
// 			<field type="int" name="IdPieceToMove" text="Id of piece being moved" />
// 		</case>
// 	</switch>
// </type>

// type ObjectId = i32;

// #[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
// #[serde(tag = "Type")]
// pub enum GameMoveDataEx {
//     #[serde(rename = "0x4")]
//     Type4 {
//         player_id: ObjectId,
//         team: i32,
//         id_piece_to_move: i32,
//         y_grid: i32,
//     },
//     #[serde(rename = "0x5")]
//     Type5 {
//         player_id: ObjectId,
//         team: i32,
//         id_piece_to_move: i32,
//         y_grid: i32,
//         x_to: i32,
//         y_to: i32,
//     },
//     #[serde(rename = "0x6")]
//     Type6 {
//         player_id: ObjectId,
//         team: i32,
//         id_piece_to_move: i32,
//     },
// }

// fn read_game_move_data(reader: &mut impl Read) -> Result<GameMoveDataEx, Box<dyn Error>> {
//     let move_type = read_i32(reader)?;
//     let player_id = read_object_id(reader)?;
//     let team = read_i32(reader)?;

//     let result = match move_type {
//         0x4 => GameMoveDataEx::Type4 {
//             player_id,
//             team,
//             id_piece_to_move: read_i32(reader)?,
//             y_grid: read_i32(reader)?,
//         },
//         0x5 => GameMoveDataEx::Type5 {
//             player_id,
//             team,
//             id_piece_to_move: read_i32(reader)?,
//             y_grid: read_i32(reader)?,
//             x_to: read_i32(reader)?,
//             y_to: read_i32(reader)?,
//         },
//         0x6 => GameMoveDataEx::Type6 {
//             player_id,
//             team,
//             id_piece_to_move: read_i32(reader)?,
//         },
//         _ => panic!("Unknown move type: {:#x}", move_type),
//     };

//     Ok(result)
// }

// fn read_object_id(reader: &mut impl Read) -> Result<ObjectId, Box<dyn Error>> {
//     todo!()
// }

// fn read_i32(reader: &mut impl Read) -> Result<i32, Box<dyn Error>> {
//     todo!()
// }
