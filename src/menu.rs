use crate::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Settings,
    InGame,
}

#[derive(Resource)]
pub struct GameSettings {
    pub motion_blur_enabled: bool,
    pub post_processing_enabled: bool,
    pub atmospheric_fog_enabled: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            motion_blur_enabled: true,
            post_processing_enabled: true, 
            atmospheric_fog_enabled: true, // Fog enabled by default for immersion
        }
    }
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_resource::<GameSettings>()
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu)
            .add_systems(Update, main_menu_system.run_if(in_state(GameState::MainMenu)))
            .add_systems(OnEnter(GameState::Settings), setup_settings_menu)
            .add_systems(OnExit(GameState::Settings), cleanup_settings_menu)
            .add_systems(Update, settings_menu_system.run_if(in_state(GameState::Settings)));
    }
}

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct SettingsMenuUI;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct SettingsButton;

#[derive(Component)]
pub struct ExitButton;

#[derive(Component)]
pub struct BackButton;

#[derive(Component)]
pub struct MotionBlurToggle;

#[derive(Component)]
pub struct MotionBlurText;

#[derive(Component)]
pub struct MotionBlurButton;

#[derive(Component)]
pub struct PostProcessToggle;

#[derive(Component)]
pub struct PostProcessText;

#[derive(Component)]
pub struct PostProcessButton;

#[derive(Component)]
pub struct AtmosphericFogToggle;

#[derive(Component)]
pub struct AtmosphericFogText;

#[derive(Component)]
pub struct AtmosphericFogButton;

fn setup_main_menu(mut commands: Commands) {
    // Spawn a camera for the menu
    commands.spawn((
        Camera2d,
        MainMenuUI,
    ));
    
    // Main menu UI
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.2)),
            MainMenuUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("BEVY VIBES"),
                TextFont {
                    font_size: 80.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
            ));

            // Play Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.5, 0.8)),
                    PlayButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("PLAY"),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Settings Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                    SettingsButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("SETTINGS"),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Exit Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.8, 0.2, 0.2)),
                    ExitButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("EXIT"),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn main_menu_system(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
    play_button_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    settings_button_query: Query<&Interaction, (Changed<Interaction>, With<SettingsButton>)>,
    exit_button_query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    // Handle button hover effects
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.7, 0.7, 0.7));
            }
            Interaction::None => {
                // Reset to original colors based on button type
                *color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
            }
            _ => {}
        }
    }

    // Handle Play button
    for interaction in play_button_query.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::InGame);
        }
    }

    // Handle Settings button
    for interaction in settings_button_query.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Settings);
        }
    }

    // Handle Exit button
    for interaction in exit_button_query.iter() {
        if *interaction == Interaction::Pressed {
            exit.write(AppExit::Success);
        }
    }
}

fn setup_settings_menu(mut commands: Commands, settings: Res<GameSettings>) {
    // Spawn a camera for the settings menu
    commands.spawn((
        Camera2d,
        SettingsMenuUI,
    ));
    
    // Settings menu UI
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.2)),
            SettingsMenuUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("SETTINGS"),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
            ));

            // Motion Blur Toggle
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(60.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(if settings.motion_blur_enabled {
                        Color::srgb(0.2, 0.8, 0.2)
                    } else {
                        Color::srgb(0.8, 0.2, 0.2)
                    }),
                    MotionBlurToggle,
                    MotionBlurButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new(format!(
                            "MOTION BLUR: {}",
                            if settings.motion_blur_enabled { "ON" } else { "OFF" }
                        )),
                        TextFont {
                            font_size: 25.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        MotionBlurText,
                    ));
                });

            // Post Processing Toggle
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(60.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(if settings.post_processing_enabled {
                        Color::srgb(0.2, 0.8, 0.2)
                    } else {
                        Color::srgb(0.8, 0.2, 0.2)
                    }),
                    PostProcessToggle,
                    PostProcessButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new(format!(
                            "POST PROCESSING: {}",
                            if settings.post_processing_enabled { "ON" } else { "OFF" }
                        )),
                        TextFont {
                            font_size: 25.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        PostProcessText,
                    ));
                });

            // Atmospheric Fog Toggle
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(60.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(if settings.atmospheric_fog_enabled {
                        Color::srgb(0.2, 0.8, 0.2)
                    } else {
                        Color::srgb(0.8, 0.2, 0.2)
                    }),
                    AtmosphericFogToggle,
                    AtmosphericFogButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new(format!(
                            "ATMOSPHERIC FOG: {}",
                            if settings.atmospheric_fog_enabled { "ON" } else { "OFF" }
                        )),
                        TextFont {
                            font_size: 25.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        AtmosphericFogText,
                    ));
                });

            // Back Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        margin: UiRect::top(Val::Px(50.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                    BackButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("BACK"),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

fn cleanup_settings_menu(mut commands: Commands, query: Query<Entity, With<SettingsMenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn settings_menu_system(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>, Without<MotionBlurButton>, Without<PostProcessButton>, Without<AtmosphericFogButton>)>,
    motion_blur_query: Query<&Interaction, (Changed<Interaction>, With<MotionBlurToggle>)>,
    post_process_query: Query<&Interaction, (Changed<Interaction>, With<PostProcessToggle>)>,
    atmospheric_fog_query: Query<&Interaction, (Changed<Interaction>, With<AtmosphericFogToggle>)>,
    back_button_query: Query<&Interaction, (Changed<Interaction>, With<BackButton>)>,
    mut motion_blur_button_query: Query<&mut BackgroundColor, (With<MotionBlurButton>, Without<PostProcessButton>, Without<AtmosphericFogButton>)>,
    mut post_process_button_query: Query<&mut BackgroundColor, (With<PostProcessButton>, Without<MotionBlurButton>, Without<AtmosphericFogButton>)>,
    mut atmospheric_fog_button_query: Query<&mut BackgroundColor, (With<AtmosphericFogButton>, Without<MotionBlurButton>, Without<PostProcessButton>)>,
    mut motion_blur_text_query: Query<&mut Text, (With<MotionBlurText>, Without<PostProcessText>, Without<AtmosphericFogText>)>,
    mut post_process_text_query: Query<&mut Text, (With<PostProcessText>, Without<MotionBlurText>, Without<AtmosphericFogText>)>,
    mut atmospheric_fog_text_query: Query<&mut Text, (With<AtmosphericFogText>, Without<MotionBlurText>, Without<PostProcessText>)>,
    mut settings: ResMut<GameSettings>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Handle button hover effects
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.7, 0.7, 0.7));
            }
            Interaction::None => {
                // Reset color based on button state
                *color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
            }
            _ => {}
        }
    }

    // Handle Motion Blur toggle
    for interaction in motion_blur_query.iter() {
        if *interaction == Interaction::Pressed {
            settings.motion_blur_enabled = !settings.motion_blur_enabled;
            
            // Update the button color directly
            if let Ok(mut button_color) = motion_blur_button_query.single_mut() {
                *button_color = BackgroundColor(if settings.motion_blur_enabled {
                    Color::srgb(0.2, 0.8, 0.2) // Green for ON
                } else {
                    Color::srgb(0.8, 0.2, 0.2) // Red for OFF
                });
            }
            
            // Update the button text directly
            if let Ok(mut text) = motion_blur_text_query.single_mut() {
                **text = format!(
                    "MOTION BLUR: {}",
                    if settings.motion_blur_enabled { "ON" } else { "OFF" }
                );
            }
        }
    }

    // Handle Post Processing toggle
    for interaction in post_process_query.iter() {
        if *interaction == Interaction::Pressed {
            settings.post_processing_enabled = !settings.post_processing_enabled;
            
            // Update the button color directly
            if let Ok(mut button_color) = post_process_button_query.single_mut() {
                *button_color = BackgroundColor(if settings.post_processing_enabled {
                    Color::srgb(0.2, 0.8, 0.2) // Green for ON
                } else {
                    Color::srgb(0.8, 0.2, 0.2) // Red for OFF
                });
            }
            
            // Update the button text directly
            if let Ok(mut text) = post_process_text_query.single_mut() {
                **text = format!(
                    "POST PROCESSING: {}",
                    if settings.post_processing_enabled { "ON" } else { "OFF" }
                );
            }
        }
    }

    // Handle Atmospheric Fog toggle
    for interaction in atmospheric_fog_query.iter() {
        if *interaction == Interaction::Pressed {
            settings.atmospheric_fog_enabled = !settings.atmospheric_fog_enabled;
            
            // Update the button color directly
            if let Ok(mut button_color) = atmospheric_fog_button_query.single_mut() {
                *button_color = BackgroundColor(if settings.atmospheric_fog_enabled {
                    Color::srgb(0.2, 0.8, 0.2) // Green for ON
                } else {
                    Color::srgb(0.8, 0.2, 0.2) // Red for OFF
                });
            }
            
            // Update the button text directly
            if let Ok(mut text) = atmospheric_fog_text_query.single_mut() {
                **text = format!(
                    "ATMOSPHERIC FOG: {}",
                    if settings.atmospheric_fog_enabled { "ON" } else { "OFF" }
                );
            }
        }
    }

    // Handle Back button
    for interaction in back_button_query.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::MainMenu);
        }
    }
} 