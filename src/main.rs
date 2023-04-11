use bevy::{
    prelude::*,
    utils::Duration,
    window::{PresentMode, WindowPlugin, WindowResolution, WindowMode},
    winit::WinitSettings
};

fn main() {
    App::new()
    // Power-saving reactive rendering for applications.
    .insert_resource(WinitSettings::desktop_app())
    // You can also customize update behavior with the fields of [`WinitConfig`]
    .insert_resource(WinitSettings {
        focused_mode: bevy::winit::UpdateMode::Continuous,
        unfocused_mode: bevy::winit::UpdateMode::ReactiveLowPower {
            max_wait: Duration::from_millis(100),
        },
        ..default()
    })
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            // Turn off vsync to maximize CPU/GPU usage
            present_mode: PresentMode::AutoNoVsync,
            title: ("Calculator").to_string(),
            resizable: false,
            transparent: true,
            resolution: WindowResolution::new(250.0, 300.0),
            ..default()
        }),
        ..default()
    }))
    .add_startup_system(setup)
    .add_system(button_system)
    .run();
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const FONT_PATH: &str = "fonts/Roboto/Regular.ttf";



const NUMBER_TOKENS: [char; 10] = ['7','8', '9', '4', '5', '6', '1', '2', '3', '0'];


struct ButtonToken<'a> {
    value: &'a str,
    color: Color
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, _) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
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


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Hello",
                    TextStyle {
                        font: asset_server.load(FONT_PATH),
                        font_size: 30.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });
        });

        // Spawn number buttons
        global.with_children(|parent| {
            for number_token in NUMBER_TOKENS {
                parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(55.0), Val::Px(50.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        number_token,
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