pub struct Offsets;

impl Offsets {
    // client.dll - Atualize com cs2-dumper
    pub const dwLocalPlayerPawn: usize = 0x2341698;
    pub const dwEntityList: usize = 0x24E76A0;
    pub const dwViewMatrix: usize = 0x2346B30;
    pub const dwViewAngles: usize = 0x23568C8;
    pub const m_hPlayerPawn: usize = 0x90C;
    // Pawn
    pub const m_iHealth: usize = 0x34C;
    pub const m_iTeamNum: usize = 0x3EB;
    pub const m_vOldOrigin: usize = 0x1390;
    pub const m_skeletonInstance: usize = 0x80;
    pub const m_bDormant: usize = 0x103;
    pub const m_entitySpottedState: usize = 0x1170;
    pub const m_bIsScoped: usize = 0x1C50;
    pub const m_flFlashDuration: usize = 0x1400;
    pub const m_pGameSceneNode: usize = 0x330;
    pub const m_modelState: usize = 0x150;
    pub const m_iIDEntIndex: usize = 0x33FC;
    pub const m_bSpottedByMask: usize = 0xC;
    pub const m_bSpotted: usize = 0x8;
    pub const m_hController: usize = 0x13A8;

    // Controller
    pub const m_hPawn: usize = 0x6BC;
    pub const m_iszPlayerName: usize = 0x6F4;

    //Bones
    pub const HAND_L_BONE: usize = 15;
    pub const HAND_R_BONE: usize = 11;
    pub const BONE_HEAD: usize = 25; // cabeça
    pub const BONE_NECK_END: usize = 12; // fim pescoço
    pub const BONE_LEFT_SHOULDER: usize = 13; // ombro esquerdo
    pub const BONE_LEFT_ELBOW: usize = 14; // cotovelo esquerdo
    pub const BONE_RIGHT_SHOULDER: usize = 9; // ombro direito
    pub const BONE_RIGHT_ELBOW: usize = 10; // cotovelo direito
    pub const BONE_PELVIS: usize = 2; // quadril
    pub const BONE_LEFT_HIP: usize = 20; // anca esquerda
    pub const BONE_LEFT_KNEE: usize = 21; // joelho esquerdo
    pub const BONE_LEFT_FOOT: usize = 22; // pé esquerdo
    pub const BONE_RIGHT_HIP: usize = 17; // anca direita
    pub const BONE_RIGHT_KNEE: usize = 18; // joelho direito
    pub const BONE_RIGHT_FOOT: usize = 19;
    

    //Skinchanger
    pub const m_hMyWeapons: usize = 0x48;
    pub const m_nFallbackPaintKit: usize = 0x1658;
    pub const m_flFallbackWear: usize = 0x1660;
    pub const m_nFallbackSeed: usize = 0x165C;
    pub const m_iItemDefinitionIndex: usize = 0x1BA;
}
