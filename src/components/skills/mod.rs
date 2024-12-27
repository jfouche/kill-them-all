mod death_aura;
mod fireball;
mod mine;
mod shuriken;

pub use death_aura::*;
pub use fireball::*;
pub use mine::*;
pub use shuriken::*;

use bevy::prelude::*;

#[derive(Resource)]
pub struct SkillAssets {
    texture: Handle<Image>,
    atlas_layout: Handle<TextureAtlasLayout>,
}

impl FromWorld for SkillAssets {
    fn from_world(world: &mut World) -> Self {
        SkillAssets {
            texture: world.load_asset(
                "items/Kyrise's 16x16 RPG Icon Pack - V1.3/spritesheet/spritesheet_48x48.png",
            ),
            atlas_layout: world.add_asset(TextureAtlasLayout::from_grid(
                UVec2::new(48, 48),
                16,
                22,
                None,
                None,
            )),
        }
    }
}

impl SkillAssets {
    pub fn image(&self) -> Handle<Image> {
        self.texture.clone()
    }

    pub fn texture_atlas(&self, index: usize) -> TextureAtlas {
        TextureAtlas {
            layout: self.atlas_layout.clone(),
            index,
        }
    }

    pub fn image_node(&self, index: usize) -> ImageNode {
        ImageNode::from_atlas_image(self.image(), self.texture_atlas(index))
    }
}

#[derive(Component, Default)]
pub struct Skill;

pub trait SkillUI {
    fn title() -> String;
    fn label() -> String;
    fn tile_index() -> usize;
}
