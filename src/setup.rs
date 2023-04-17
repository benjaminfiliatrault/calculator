

use bevy::prelude::*;
use crate::config::*;

struct ButtonToken<'a> {
    value: &'a str,
    color: Color,
    width: Val,
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
pub struct TokenText;

const BUTTONS: [ButtonToken; 16] = [
    ButtonToken {
        value: "AC",
        color: Color::Rgba { red: 0.0, green: 0.4, blue: 0.2, alpha: 1.0 },
        width: Val::Percent(100.0)
    },
    ButtonToken { 
        value: "7",
        color: NORMAL_BUTTON, 
        width: Val::Px(FLEX_4)
    },
    ButtonToken { 
        value: "8",
        color: NORMAL_BUTTON, 
        width: Val::Px(FLEX_4)
    },
    ButtonToken { 
        value: "9",
        color: NORMAL_BUTTON, 
        width: Val::Px(FLEX_4)
    },
    ButtonToken { 
        value: "+",
        color: NORMAL_BUTTON, 
        width: Val::Px(FLEX_4)
    },
    ButtonToken { 
        value: "4",
        color: NORMAL_BUTTON, 
        width: Val::Px(FLEX_4)
    },
    ButtonToken { 
        value: "5",
        color: NORMAL_BUTTON, 
        width: Val::Px(FLEX_4)
    },
    ButtonToken { 
        value: "6",
        color: NORMAL_BUTTON, 
        width: Val::Px(FLEX_4)
    },
    ButtonToken { 
        value: "-",
        color: NORMAL_BUTTON, 
        width: Val::Px(FLEX_4)
    },
    ButtonToken { 
        value: "1",
        color: NORMAL_BUTTON, 
        width: Val::Px(FLEX_4)
    },
    ButtonToken { 
        value: "2",
        color: NORMAL_BUTTON, 
        width: Val::Px(FLEX_4)
    },
    ButtonToken { 
        value: "3",
        color: NORMAL_BUTTON, 
        width: Val::Px(FLEX_4)
    },
    ButtonToken { 
        value: "*",
        color: NORMAL_BUTTON, 
        width: Val::Px(FLEX_4)
    },
    ButtonToken {
        value: "0",
        color: NORMAL_BUTTON,
        width: Val::Px(FLEX_4 * 3.)
    },
    ButtonToken { 
        value: "/",
        color: NORMAL_BUTTON, 
        width: Val::Px(FLEX_4)
    },
    ButtonToken {
        value: "=",
        color: NORMAL_BUTTON,
        width: Val::Percent(100.0)
    },
];

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
commands.spawn(Camera2dBundle::default());

let mut global = commands
    .spawn(NodeBundle {
        background_color: BackgroundColor(Color::NONE),
        style: Style {
            // size: Size::width(Val::Percent(100.0)),
            // justify_content: JustifyContent::SpaceAround,
            display: Display::Flex,
            flex_wrap: FlexWrap::Wrap,
            ..default()
        },
        ..default()
    });

    // The current value
    global.with_children(|parent| {
        parent.spawn(NodeBundle {
            background_color: BackgroundColor(Color::NONE),
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                display: Display::Flex,
                flex_wrap: FlexWrap::Wrap,
                gap: Size::new(Val::Px(0.0), Val::Px(0.0)),
                margin: UiRect {
                    left: Val::Px(0.0),
                    right: Val::Px(0.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                }, 
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((TextBundle::from_section(
                "0",
                TextStyle {
                    font: asset_server.load(FONT_PATH),
                    font_size: 30.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ), TokenText));
        });
    });

    // Spawn number buttons
    global.with_children(|parent| {
        for button in BUTTONS {
            parent
            .spawn(ButtonBundle {
                style: Style {
                    size: Size::new(button.width, Val::Px(50.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    margin: UiRect {
                        left: Val::Px(0.0),
                        right: Val::Px(0.0),
                        top: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                    }, 
                    ..default()
                },
                background_color: button.color.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    button.value,
                    TextStyle {
                        font: asset_server.load(FONT_PATH),
                        font_size: 30.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });
        }
    });
}
