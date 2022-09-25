use bevy::prelude::{Handle, HandleUntyped, Image};

pub struct WinSize {
    pub w: f32,
    pub h: f32
}

pub struct GameTextures {
    pub wall: Handle<Image>
}

pub struct AssetsLoading(pub Vec<HandleUntyped>);
