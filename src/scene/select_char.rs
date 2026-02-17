//! 选角色场景

use bevy::prelude::*;
use crate::core::{GameState, SceneState};

/// 选角色场景插件
pub struct SelectCharScenePlugin;

impl Plugin for SelectCharScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::SelectChar), setup_select_char)
            .add_systems(OnExit(SceneState::SelectChar), cleanup_select_char)
            .add_systems(Update, select_char_system.run_if(in_state(SceneState::SelectChar)));
    }
}

/// 选角界面标记
#[derive(Component)]
struct SelectCharScreen;

/// 开始游戏按钮
#[derive(Component)]
struct StartGameButton;

fn setup_select_char(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.05, 0.05, 0.1)),
        SelectCharScreen,
    )).with_children(|parent| {
        parent.spawn((
            Node {
                width: Val::Px(400.0),
                height: Val::Px(300.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(20.0),
                padding: UiRect::all(Val::Px(20.0)),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.2)),
            BorderColor::all(Color::srgb(0.3, 0.3, 0.4)),
        )).with_children(|char_box| {
            // 标题
            char_box.spawn((
                Text::new("选择角色"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.8, 0.0)),
            ));

            // 角色信息
            char_box.spawn((
                Text::new("角色: 战士 (Lv.35)"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));

            // 开始游戏按钮
            char_box.spawn((
                Button,
                Node {
                    width: Val::Px(150.0),
                    height: Val::Px(45.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    border_radius: BorderRadius::all(Val::Px(5.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.4, 0.2, 0.1)),
                BorderColor::all(Color::srgb(0.6, 0.3, 0.2)),
                StartGameButton,
            )).with_children(|btn| {
                btn.spawn((
                    Text::new("进入游戏"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 1.0, 1.0)),
                ));
            });
        });
    });

    tracing::info!("进入选角色场景");
}

fn cleanup_select_char(mut commands: Commands, query: Query<Entity, With<SelectCharScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_children();
        commands.entity(entity).try_despawn();
    }
}

fn select_char_system(
    mut next_state: ResMut<NextState<SceneState>>,
    mut game_state: ResMut<GameState>,
    query: Query<&Interaction, (Changed<Interaction>, With<StartGameButton>)>,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            game_state.current_map = "比奇省".to_string();
            tracing::info!("选择角色完成，进入游戏: {}", game_state.current_map);
            next_state.set(SceneState::Game);
        }
    }
}
