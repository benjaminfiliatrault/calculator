use bevy::prelude::*;
use crate::{config::*, token_manager::*};

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut token_state: ResMut<TokenManager>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let text = text_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Clicked => {
                // token_state.value.push_str(text.sections[0].value.as_str());
                handle_interaction(&mut token_state, text.sections[0].value.as_str());
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}


fn handle_interaction(mut token_state: &mut TokenManager, btn_token: &str) {
    if btn_token == "AC" {
        token_state.value = "0".to_string();
        return;
    }

    // We don't want to push stuff
    // after 0 if that's the only value
    if token_state.value == "0" {
        // TODO: handle exceptions like operators and decimal pointer
        token_state.value = btn_token.to_string();
        return;
    }


    token_state.value.push_str(btn_token);
}
