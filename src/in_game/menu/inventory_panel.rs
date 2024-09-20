use crate::components::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct InventoryPanel;

#[derive(Resource)]
struct InventoryAssets {
    texture: Handle<Image>,
    texture_atlas_layout: Handle<TextureAtlasLayout>,
}

impl InventoryAssets {
    fn helmet(&self) -> (Handle<Image>, TextureAtlas) {
        (
            self.texture.clone(),
            TextureAtlas {
                layout: self.texture_atlas_layout.clone(),
                index: 182,
            },
        )
    }
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_atlas_layout = texture_atlases.add(TextureAtlasLayout::from_grid(
        UVec2::new(48, 48),
        16,
        22,
        None,
        None,
    ));

    let texture = asset_server
        .load("items/Kyrise's 16x16 RPG Icon Pack - V1.3/spritesheet/spritesheet_48x48.png");

    let assets = InventoryAssets {
        texture,
        texture_atlas_layout,
    };
    commands.insert_resource(assets);
}

fn add_inventory_panel(
    mut commands: Commands,
    panels: Query<Entity, Added<InventoryPanel>>,
    players: Query<&Helmet, With<Player>>,
    assets: Res<InventoryAssets>,
) {
    let Ok(helmet) = players.get_single() else {
        return;
    };
    for entity in &panels {
        commands
            .entity(entity)
            .insert(main_panel_bundle())
            .with_children(|panel| {
                panel.spawn(items_panel_bundle()).with_children(|p| {
                    // helmet
                    let pos = Vec2::new(74., 7.);
                    match helmet {
                        Helmet::None => {
                            p.spawn(empty_inventory_box(pos));
                        }
                        Helmet::NormalHelmet(_helmet) => {
                            let helmet = assets.helmet();
                            p.spawn(inventory_box(pos, helmet.0, helmet.1));
                        }
                    }
                    p.spawn(empty_inventory_box(Vec2::new(142., 7.)));
                    p.spawn(empty_inventory_box(Vec2::new(7., 74.)));
                    p.spawn(empty_inventory_box(Vec2::new(74., 74.)));
                    p.spawn(empty_inventory_box(Vec2::new(142., 74.)));
                    p.spawn(empty_inventory_box(Vec2::new(7., 142.)));
                    p.spawn(empty_inventory_box(Vec2::new(74., 142.)));
                });
                panel.spawn(item_affixes_panel());
            });
    }
}

fn main_panel_bundle() -> impl Bundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            width: Val::Px(214.),
            height: Val::Px(350.),
            ..Default::default()
        },
        // background_color: Color::srgb_u8(0.5, 0.1, 0.1, 1.).into(),
        ..Default::default()
    }
}

fn items_panel_bundle() -> impl Bundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Px(280.),
            ..Default::default()
        },
        background_color: Srgba::rgb_u8(40, 40, 40).into(),
        ..Default::default()
    }
}

fn item_affixes_panel() -> impl Bundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Auto,
            ..Default::default()
        },
        background_color: Color::srgb(0.1, 0.1, 0.5).into(),
        ..Default::default()
    }
}

fn empty_inventory_box(pos: Vec2) -> impl Bundle {
    NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            left: Val::Px(pos.x),
            top: Val::Px(pos.y),
            width: Val::Px(48.),
            height: Val::Px(48.),
            ..Default::default()
        },
        background_color: Srgba::rgb_u8(70, 70, 70).into(),
        ..Default::default()
    }
}

fn inventory_box(pos: Vec2, texture: Handle<Image>, atlas: TextureAtlas) -> impl Bundle {
    (
        ImageBundle {
            image: UiImage::new(texture),
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(pos.x),
                top: Val::Px(pos.y),
                width: Val::Px(48.),
                height: Val::Px(48.),
                ..Default::default()
            },
            background_color: Srgba::rgb_u8(70, 70, 70).into(),
            ..Default::default()
        },
        atlas,
    )
}

pub fn inventory_panel_plugin(app: &mut App) {
    app.add_systems(Startup, load_assets)
        .add_systems(Update, add_inventory_panel);
}
