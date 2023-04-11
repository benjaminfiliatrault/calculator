use bevy::prelude::*;

use crate::setup::TokenText;

#[derive(Resource, Debug, Clone)]
pub struct TokenManager { 
    pub(crate) value: String,
    pub(crate) default: String,
}

impl FromWorld for TokenManager {
    fn from_world(_world: &mut World) -> Self {
        // You have full access to anything in the ECS from here.
        // For instance, you can mutate other resources:
        TokenManager { value: String::from(""), default: String::from("0") }
    }
}

pub fn token_system(mut query: Query<&mut Text, With<TokenText>>, token_state: ResMut<TokenManager>,) {
    for mut text in &mut query {
        // Update the value of the second section
        text.sections[0].value = token_state.value.to_owned();
    }
}