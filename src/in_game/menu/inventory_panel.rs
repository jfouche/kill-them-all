use crate::components::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct InventoryPanel;

fn add_inventory_panel(
    mut commands: Commands,
    panels: Query<Entity, Added<InventoryPanel>>,
    players: Query<(&Helmet, &BodyArmour, &Boots), With<Player>>,
    assets: Res<EquipmentAssets>,
) {
    let Ok((helmet, body_armour, boots)) = players.get_single() else {
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
                    let (texture, atlas) = assets.helmet(helmet);
                    p.spawn(inventory_box(pos, texture, atlas));
                    // amulet
                    p.spawn(empty_inventory_box(Vec2::new(142., 7.)));
                    // weapon
                    p.spawn(empty_inventory_box(Vec2::new(7., 74.)));
                    // body armour
                    let pos = Vec2::new(74., 74.);
                    let (texture, atlas) = assets.body_armour(body_armour);
                    p.spawn(inventory_box(pos, texture, atlas));
                    //
                    p.spawn(empty_inventory_box(Vec2::new(142., 74.)));
                    p.spawn(empty_inventory_box(Vec2::new(7., 142.)));
                    // Boots
                    let pos = Vec2::new(74., 142.);
                    let (texture, atlas) = assets.boots(boots);
                    p.spawn(inventory_box(pos, texture, atlas));
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
    app.add_systems(Update, add_inventory_panel);
}
