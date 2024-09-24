use crate::components::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct InventoryPanel;

#[derive(Component)]
struct AffixesText;

trait BaseAndAffixesLabels {
    fn base_label(&self) -> String;
    fn affixes_labels(&self) -> Vec<String>;
}

impl BaseAndAffixesLabels for Equipment {
    fn base_label(&self) -> String {
        match self {
            Equipment::Helmet(helmet) => helmet.base_label(),
            Equipment::BodyArmour(body_armour) => body_armour.base_label(),
            Equipment::Boots(boots) => boots.base_label(),
        }
    }

    fn affixes_labels(&self) -> Vec<String> {
        match self {
            Equipment::Helmet(helmet) => helmet.affixes_labels(),
            Equipment::BodyArmour(body_armour) => body_armour.affixes_labels(),
            Equipment::Boots(boots) => boots.affixes_labels(),
        }
    }
}

impl BaseAndAffixesLabels for Helmet {
    fn base_label(&self) -> String {
        match self {
            Helmet::None => "".into(),
            Helmet::Normal(helmet) => format!("Helmet +{} armour", helmet.armor),
            Helmet::Magic(helmet) => format!("Helmet +{} armour", helmet.base.armor),
        }
    }

    fn affixes_labels(&self) -> Vec<String> {
        match *self {
            Helmet::None => Vec::new(),
            Helmet::Normal(_helmet) => Vec::new(),
            Helmet::Magic(helmet) => vec![format!("Item adds +{} life", helmet.life)],
        }
    }
}

impl BaseAndAffixesLabels for BodyArmour {
    fn base_label(&self) -> String {
        match self {
            BodyArmour::None => "".into(),
            BodyArmour::Normal(body_armour) => format!("Body armour +{} armour", body_armour.armor),
            BodyArmour::Magic(body_armour) => {
                format!("Body armour +{} armour", body_armour.base.armor)
            }
        }
    }

    fn affixes_labels(&self) -> Vec<String> {
        match *self {
            BodyArmour::None => Vec::new(),
            BodyArmour::Normal(_body_armour) => Vec::new(),
            BodyArmour::Magic(body_armour) => vec![format!("Item adds +{} life", body_armour.life)],
        }
    }
}

impl BaseAndAffixesLabels for Boots {
    fn base_label(&self) -> String {
        match self {
            Boots::None => "".into(),
            Boots::Normal(boots) => format!("Boots +{} armour", boots.armor),
            Boots::Magic(boots) => format!("Boots +{} armour", boots.base.armor),
        }
    }

    fn affixes_labels(&self) -> Vec<String> {
        match *self {
            Boots::None => Vec::new(),
            Boots::Normal(_boots) => Vec::new(),
            Boots::Magic(boots) => vec![format!("Item adds +{} life", boots.life)],
        }
    }
}

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
                    p.spawn(inventory_box(pos, texture, atlas))
                        .insert(Equipment::Helmet(helmet.clone()));
                    // amulet
                    p.spawn(empty_inventory_box(Vec2::new(142., 7.)));
                    // weapon
                    p.spawn(empty_inventory_box(Vec2::new(7., 74.)));
                    // body armour
                    let pos = Vec2::new(74., 74.);
                    let (texture, atlas) = assets.body_armour(body_armour);
                    p.spawn(inventory_box(pos, texture, atlas))
                        .insert(Equipment::BodyArmour(body_armour.clone()));
                    //
                    p.spawn(empty_inventory_box(Vec2::new(142., 74.)));
                    p.spawn(empty_inventory_box(Vec2::new(7., 142.)));
                    // Boots
                    let pos = Vec2::new(74., 142.);
                    let (texture, atlas) = assets.boots(boots);
                    p.spawn(inventory_box(pos, texture, atlas))
                        .insert(Equipment::Boots(boots.clone()));
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
            let mut infos = String::new();
            infos.push_str(&equipment.base_label());
            for affix in equipment.affixes_labels() {
                infos.push('\n');
                infos.push_str(&affix);
            }
            text.sections[0].value = infos;
        }
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
    let _ = NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Auto,
            ..Default::default()
        },
        background_color: Color::srgb(0.1, 0.1, 0.5).into(),
        ..Default::default()
    };

    (
        AffixesText,
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 16.,
                ..Default::default()
            },
        ),
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
