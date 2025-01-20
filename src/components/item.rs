use bevy::prelude::*;

#[derive(Component, Default)]
#[require(ItemLevel)]
pub struct Item;

#[derive(Component, Default, Deref, Reflect)]
pub struct ItemLevel(u16);
