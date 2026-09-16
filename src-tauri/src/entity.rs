use crate::get_game_data;
use crate::memory::GameProcess;
use crate::offsets::Offsets;
use glam::Vec3;

pub struct Entity {
    pub index: i32,
    pub controller: usize,
    pub pawn: usize,
    pub name: String,
    pub health: i32,
    pub team: i32,
    pub position: Vec3,
    pub head_position: Vec3,
    pub dormant: bool,
    pub is_visible: bool,
}

impl Entity {
    pub fn new(index: i32) -> Self {
        Self {
            index,
            controller: 0,
            pawn: 0,
            name: String::new(),
            health: 0,
            team: 0,
            position: Vec3::ZERO,
            head_position: Vec3::ZERO,
            dormant: true,
            is_visible: true,
        }
    }
    pub fn get_bone_pos(&self, game: &mut GameProcess, bone_id: usize) -> Option<Vec3> {
        if self.pawn == 0 {
            return None;
        }

        // 1. Ler o GameSceneNode
        let game_scene = game.read_ptr(self.pawn + Offsets::m_pGameSceneNode)?;
        if game_scene == 0 {
            return None;
        }

        // 2. Ler o ponteiro do Bone Array
        // No CS2, o endereço é: GameScene + m_modelState + m_skeletonInstance
        let skeleton_instance =
            game.read_ptr(game_scene + Offsets::m_modelState + Offsets::m_skeletonInstance)?;
        if skeleton_instance == 0 {
            return None;
        }

        // 3. Calcular o endereço do osso específico
        // Cada osso no CS2 ocupa exatamente 32 bytes (0x20)
        let bone_address = skeleton_instance + (bone_id * 0x20);

        // 4. Ler os dados (X, Y, Z)
        let x = game.read_float(bone_address)?;
        let y = game.read_float(bone_address + 0x4)?;
        let z = game.read_float(bone_address + 0x8)?;

        Some(Vec3::new(x, y, z))
    }

    pub fn get_bone_positions(
        &mut self,
        game: &mut GameProcess,
    ) -> std::collections::HashMap<usize, Vec3> {
        let mut bones = std::collections::HashMap::new();

        // Use o valor do Offsets como chave, para bater com o esp.rs
        let bone_ids = [
            Offsets::BONE_HEAD,
            Offsets::BONE_NECK_END,
            Offsets::BONE_LEFT_SHOULDER,
            Offsets::BONE_LEFT_ELBOW,
            Offsets::BONE_RIGHT_SHOULDER,
            Offsets::BONE_RIGHT_ELBOW,
            Offsets::HAND_L_BONE,
            Offsets::HAND_R_BONE,
            Offsets::BONE_PELVIS,
            Offsets::BONE_LEFT_HIP,
            Offsets::BONE_LEFT_KNEE,
            Offsets::BONE_LEFT_FOOT,
            Offsets::BONE_RIGHT_HIP,
            Offsets::BONE_RIGHT_KNEE,
            Offsets::BONE_RIGHT_FOOT,
        ];

        for &bone_id in &bone_ids {
            if let Some(pos) = self.get_bone_pos(game, bone_id) {
                bones.insert(bone_id, pos);
            }
        }

        bones
    }

    pub fn get_head_position(&self, game: &mut GameProcess) -> Option<Vec3> {
        self.get_bone_pos(game, Offsets::BONE_HEAD)
    }

    pub fn get_bone_pos_world(&self, game: &mut GameProcess, bone_id: usize) -> Option<Vec3> {
        self.get_bone_pos(game, bone_id)
    }

    pub fn update(
        &mut self,
        game: &mut GameProcess,
        client_base: usize,
        local_controller: usize,
    ) -> bool {
        let entity_list = match game.read_ptr(client_base + Offsets::dwEntityList) {
            Some(list) => list,
            None => return false,
        };

        let entry_index = (self.index & 0x7FFF) >> 9;
        let entry_ptr = entity_list + (8 * entry_index as usize) + 16;

        let list_entry = match game.read_ptr(entry_ptr) {
            Some(entry) => entry,
            None => return false,
        };

        self.controller = match game.read_ptr(list_entry + (112 * (self.index & 0x1FF) as usize)) {
            Some(ctrl) => ctrl,
            None => return false,
        };

        if self.controller == 0 {
            return false;
        }


        if let Some(name) = game.read_string(self.controller + Offsets::m_iszPlayerName, 64) {
            self.name = name;
        }

        let pawn_handle = match game.read_i32(self.controller + Offsets::m_hPawn) {
            Some(handle) => handle,
            None => return false,
        };

        let pawn_index = (pawn_handle & 0x7FFF) >> 9;
        let pawn_entry_ptr = entity_list + (8 * pawn_index as usize) + 16;

        let pawn_list_entry = match game.read_ptr(pawn_entry_ptr) {
            Some(entry) => entry,
            None => return false,
        };

        self.pawn = match game.read_ptr(pawn_list_entry + (112 * (pawn_handle & 0x1FF) as usize)) {
            Some(pawn) => pawn,
            None => return false,
        };

        if self.pawn == 0 {
            return false;
        }

        self.health = game.read_i32(self.pawn + Offsets::m_iHealth).unwrap_or(0);
        self.team = game.read_i32(self.pawn + Offsets::m_iTeamNum).unwrap_or(0);
        self.dormant = game
            .read_bool(self.pawn + Offsets::m_bDormant)
            .unwrap_or(true);

        if let Some(pos) = game.read_vec3(self.pawn + Offsets::m_vOldOrigin) {
            self.position = pos;
        }

        // Ler a posição da cabeça
        if let Some(head_pos) = self.get_head_position(game) {
            self.head_position = head_pos;
        } else {
            // Fallback: estimar altura baseada na distância
            self.head_position =
                Vec3::new(self.position.x, self.position.y, self.position.z + 70.0);
        }

        self.health > 0 && !self.dormant
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0 && !self.dormant
    }
}
