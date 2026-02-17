//! 登录场景

use bevy::prelude::*;
use crate::core::{GameState, SceneState};

/// 登录场景插件
pub struct LoginScenePlugin;

impl Plugin for LoginScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::Login), setup_login)
            .add_systems(OnExit(SceneState::Login), cleanup_login)
            .add_systems(Update, login_system.run_if(in_state(SceneState::Login)));
    }
}

/// 登录界面标记
#[derive(Component)]
struct LoginScreen;

/// 登录按钮
#[derive(Component)]
struct LoginButton;

fn setup_login(mut commands: Commands) {
    // 创建登录界面
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.2)),
        LoginScreen,
    )).with_children(|parent| {
        // 登录框
        parent.spawn((
            Node {
                width: Val::Px(300.0),
                height: Val::Px(200.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(10.0),
                padding: UiRect::all(Val::Px(20.0)),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.3)),
            BorderColor::all(Color::srgb(0.4, 0.4, 0.5)),
        )).with_children(|login_box| {
            // 标题
            login_box.spawn((
                Text::new("传奇世界"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.8, 0.0)),
            ));

            // 提示文本
            login_box.spawn((
                Text::new("点击登录进入游戏"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));

            // 登录按钮
            login_box.spawn((
                Button,
                Node {
                    width: Val::Px(120.0),
                    height: Val::Px(40.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    border_radius: BorderRadius::all(Val::Px(5.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.3, 0.5, 0.3)),
                BorderColor::all(Color::srgb(0.4, 0.6, 0.4)),
                LoginButton,
            )).with_children(|btn| {
                btn.spawn((
                    Text::new("登 录"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 1.0, 1.0)),
                ));
            });
        });
    });

    tracing::info!("进入登录场景");
}

fn cleanup_login(mut commands: Commands, query: Query<Entity, With<LoginScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_children();
        commands.entity(entity).try_despawn();
    }
}

fn login_system(
    mut next_state: ResMut<NextState<SceneState>>,
    mut game_state: ResMut<GameState>,
    query: Query<&Interaction, (Changed<Interaction>, With<LoginButton>)>,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            // 模拟登录成功
            game_state.is_logged_in = true;
            game_state.player_name = "TestPlayer".to_string();
            game_state.player_id = 1001;

            tracing::info!("登录成功，切换到选角色场景");
            next_state.set(SceneState::SelectChar);
        }
    }
}
