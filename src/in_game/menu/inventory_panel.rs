use crate::components::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct InventoryPanel;

#[derive(Component)]
struct AffixesText;

fn add_inventory_panel(
    mut commands: Commands,
    panels: Query<Entity, Added<InventoryPanel>>,
    players: Query<&Equipments, With<Player>>,
    assets: Res<EquipmentAssets>,
) {
    let Ok(equipments) = players.get_single() else {
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
                    let (texture, atlas) = assets.helmet(&equipments.helmet);
                    p.spawn(inventory_box(pos, texture, atlas))
                        .insert(Equipment::Helmet(equipments.helmet.clone()));
                    // amulet
                    p.spawn(empty_inventory_box(Vec2::new(142., 7.)));
                    // weapon
                    p.spawn(empty_inventory_box(Vec2::new(7., 74.)));
                    // body armour
                    let pos = Vec2::new(74., 74.);
                    let (texture, atlas) = assets.body_armour(&equipments.body_armour);
                    p.spawn(inventory_box(pos, texture, atlas))
                        .insert(Equipment::BodyArmour(equipments.body_armour.clone()));
                    //
                    p.spawn(empty_inventory_box(Vec2::new(142., 74.)));
                    p.spawn(empty_inventory_box(Vec2::new(7., 142.)));
                    // Boots
                    let pos = Vec2::new(74., 142.);
                    let (texture, atlas) = assets.boots(&equipments.boots);
                    p.spawn(inventory_box(pos, texture, atlas))
                        .insert(Equipment::Boots(equipments.boots.clone()));
                });
                panel.spawn(item_affixes_panel());
            });
    }
}

fn hover_equipment(
    equipments: Query<(&Equipment, &Interaction)>,
    mut texts: Query<&mut Text, With<AffixesText>>,
) {
    let Ok(mut text) = texts.get_single_mut() else {
        return;
    };
    text.sections[0].value = "".into();
    for (equipment, interaction) in &equipments {
        if interaction == &Interaction::Hovered {
            text.sections[0].value = equipment.to_string();
        }
    }
}

fn main_panel_bundle() -> impl Bundle {
    (
        Name::new("InventoryPanel"),
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(5.)),
                ..Default::default()
            },
            ..Default::default()
        },
    )
}

fn items_panel_bundle() -> impl Bundle {
    (
        Name::new("Items"),
        NodeBundle {
            style: Style {
                width: Val::Px(200.),
                height: Val::Px(200.),
                ..Default::default()
            },
            background_color: Srgba::rgb_u8(40, 40, 40).into(),
            ..Default::default()
        },
    )
}

fn item_affixes_panel() -> impl Bundle {
    (
        AffixesText,
        Name::new("Affixes"),
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 16.,
                ..Default::default()
            },
        )
        .with_style(Style {
            margin: UiRect::all(Val::Px(5.)),
            ..Default::default()
        }),
    )
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
        Interaction::None,
    )
}

pub fn inventory_panel_plugin(app: &mut App) {
    app.add_systems(Update, (add_inventory_panel, hover_equipment));
}
