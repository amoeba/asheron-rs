// Set of information related to a chess game move
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "Type")]
pub enum GameMoveData {
    #[serde(rename = "0x4")]
    Type4 {
        #[serde(rename = "PlayerId")]
        player_id: String,
        #[serde(rename = "Team")]
        team: String,
        #[serde(rename = "IdPieceToMove")]
        id_piece_to_move: String,
        #[serde(rename = "YGrid")]
        y_grid: String,
    },
    #[serde(rename = "0x5")]
    Type5 {
        #[serde(rename = "PlayerId")]
        player_id: String,
        #[serde(rename = "Team")]
        team: String,
        #[serde(rename = "IdPieceToMove")]
        id_piece_to_move: String,
        #[serde(rename = "YGrid")]
        y_grid: String,
        #[serde(rename = "XTo")]
        x_to: String,
        #[serde(rename = "YTo")]
        y_to: String,
    },
    #[serde(rename = "0x6")]
    Type6 {
        #[serde(rename = "PlayerId")]
        player_id: String,
        #[serde(rename = "Team")]
        team: String,
        #[serde(rename = "IdPieceToMove")]
        id_piece_to_move: String,
    },
}

