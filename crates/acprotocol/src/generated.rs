// Set of information related to a chess game move
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GameMoveData {
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "PlayerId")]
    player_id: String,
    #[serde(rename = "Team")]
    team: String,
    #[serde(rename = "IdPieceToMove")]
    id_piece_to_move: String,
    #[serde(rename = "YGrid")]
    ygrid: String,
    #[serde(rename = "IdPieceToMove")]
    id_piece_to_move: String,
    #[serde(rename = "YGrid")]
    ygrid: String,
    #[serde(rename = "XTo")]
    xto: String,
    #[serde(rename = "YTo")]
    yto: String,
    #[serde(rename = "IdPieceToMove")]
    id_piece_to_move: String
}

