mod chunk;

pub use chunk::*;

falcon_send_derive::falcon_send! {
    use falcon_core::network::packet::PacketEncode;
    use crate::{JoinGameSpec, ServerDifficultySpec};

    #[derive(PacketEncode)]
    #[falcon_packet(versions = {
        477, 480, 485, 490, 498 = 0x25;
    }, name = "join_game")]
    pub struct JoinGamePacket {
        entity_id: i32,
        game_mode: u8,
        dimension: i32,
        max_players: u8,
        #[max_length(16)]
        level_type: String,
        #[var_int]
        view_distance: i32,
        reduced_debug: bool,
    }

    impl From<JoinGameSpec> for JoinGamePacket {
        fn from(spec: JoinGameSpec) -> Self {
            JoinGamePacket {
                entity_id: spec.entity_id,
                game_mode: spec.game_mode as u8,
                dimension: spec.dimension,
                max_players: spec.max_players,
                level_type: spec.level_type,
                view_distance: spec.view_distance,
                reduced_debug: spec.reduced_debug,
            }
        }
    }

    #[derive(PacketEncode)]
    #[falcon_packet(versions = {
        477, 480, 485, 490, 498, 735, 736 = 0x0D;
        573, 575, 578 = 0x0E;
    }, name = "send_difficulty")]
    pub struct ServerDifficultyPacket {
        difficulty: u8,
        locked: bool,
    }

    impl From<ServerDifficultySpec> for ServerDifficultyPacket {
        fn from(spec: ServerDifficultySpec) -> Self {
            ServerDifficultyPacket {
                difficulty: spec.difficulty as u8,
                locked: spec.locked,
            }
        }
    }

    #[derive(PacketEncode)]
    #[falcon_packet(versions = {
        477, 480, 485, 490, 498, 735, 736 = 0x40;
        573, 575, 578 = 0x41;
    }, name = "update_viewpos")]
    pub struct UpdateViewPosition {
        #[var_int]
        chunk_x: i32,
        #[var_int]
        chunk_z: i32,
    }

    impl From<(i32, i32)> for UpdateViewPosition {
        fn from((chunk_x, chunk_z): (i32, i32)) -> Self {
            UpdateViewPosition {
                chunk_x,
                chunk_z
            }
        }
    }
}
