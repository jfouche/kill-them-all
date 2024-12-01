use crate::components::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct InventoryPanel;

pub fn inventory_panel() -> impl Bundle {
    (
        InventoryPanel,
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

#[derive(Component)]
struct AffixesText;

#[derive(Component)]
struct EquipmentsPanel;

trait EquipmentPos {
    fn pos() -> Vec2;
}

// weapon : Vec2::new(7., 74.)
// ?      : Vec2::new(142., 74.)
// ?      : Vec2::new(7., 142.)

impl EquipmentPos for Amulet {
    fn pos() -> Vec2 {
        Vec2::new(142., 7.)
    }
}

impl EquipmentPos for BodyArmour {
    fn pos() -> Vec2 {
        Vec2::new(74., 74.)
    }
}

impl EquipmentPos for Boots {
    fn pos() -> Vec2 {
        Vec2::new(74., 142.)
    }
}

impl EquipmentPos for Helmet {
    fn pos() -> Vec2 {
        Vec2::new(74., 7.)
    }
}

fn show_equipment<T>(
    mut commands: Commands,
    panels: Query<Entity, Added<EquipmentsPanel>>,
    equipments: Query<(&TileIndex, &AffixesLabels, &Parent), With<T>>,
    players: Query<Entity, With<Player>>,
    assets: Res<EquipmentAssets>,
) where
    T: Component + EquipmentPos,
{
    let Ok(player) = players.get_single() else {
        return;
    };
    for panel in &panels {
        for (tile_index, label, parent) in &equipments {
            if **parent == player {
                commands.entity(panel).with_children(|p| {
                    let pos = T::pos();
                    let texture = assets.texture();
                    let atlas = assets.atlas(**tile_index);
                    p.spawn((inventory_box(pos, texture, atlas), label.clone()));
                });
            }
        }
    }
}

fn hover_equipment(
    equipments: Query<(&AffixesLabels, &Interaction)>,
    mut texts: Query<&mut Text, With<AffixesText>>,
) {
    if let Ok(mut text) = texts.get_single_mut() {
        text.sections[0].value = "".into();
        for (label, interaction) in &equipments {
            if interaction == &Interaction::Hovered {
                text.sections[0].value = label.to_string();
            }
        }
    };
}

fn equipments_panel_bundle() -> impl Bundle {
    (
        EquipmentsPanel,
        Name::new("Equipments Panel"),
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

// fn empty_inventory_box(pos: Vec2) -> impl Bundle {
//     NodeBundle {
//         style: Style {
//             position_type: PositionType::Absolute,
//             left: Val::Px(pos.x),
//             top: Val::Px(pos.y),
//             width: Val::Px(48.),
//             height: Val::Px(48.),
//             ..Default::default()
//         },
//         background_color: Srgba::rgb_u8(70, 70, 70).into(),
//         ..Default::default()
//     }
// }

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

fn setup_hooks(world: &mut World) {
    world.register_component_hooks::<InventoryPanel>().on_add(
        |mut world, entity, _component_id| {
            world.commands().entity(entity).with_children(|panel| {
                panel.spawn(equipments_panel_bundle());
                panel.spawn(item_affixes_panel());
            });
        },
    );
}

pub fn inventory_panel_plugin(app: &mut App) {
    app.add_systems(Startup, setup_hooks).add_systems(
        Update,
        (
            show_equipment::<Amulet>,
            show_equipment::<BodyArmour>,
            show_equipment::<Boots>,
            show_equipment::<Helmet>,
            hover_equipment,
        ),
    );
}
