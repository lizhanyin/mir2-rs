//! 游戏主场景

use bevy::prelude::*;
use crate::core::{GameConfig, GameState, SceneState};

/// 游戏场景插件
pub struct GameScenePlugin;

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::Game), setup_game)
            .add_systems(OnExit(SceneState::Game), cleanup_game)
            .add_systems(Update, (
                game_input_system,
                camera_follow_system,
            ).run_if(in_state(SceneState::Game)));
    }
}

/// 游戏场景标记
#[derive(Component)]
struct GameWorld;

/// 玩家实体标记
#[derive(Component)]
struct Player;

/// 主相机
#[derive(Component)]
struct MainCamera;

fn setup_game(
    mut commands: Commands,
    game_state: Res<GameState>,
    config: Res<GameConfig>,
) {
    // 创建相机
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 1000.0),
        MainCamera,
    ));

    // 创建游戏世界容器
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.0, 0.0, 0.05)),
        GameWorld,
    )).with_children(|world| {
        // 游戏界面 HUD
        world.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(60.0),
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
        )).with_children(|hud| {
            hud.spawn((
                Text::new(format!("地图: {} | 玩家: {}", game_state.current_map, game_state.player_name)),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
            ));
        });
    });

    // 创建玩家实体
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.8, 0.2),
            custom_size: Some(Vec2::new(32.0, 48.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player,
    ));

    tracing::info!("进入游戏场景 - 地图: {}", game_state.current_map);
    tracing::debug!("资源路径: {:?}", config.resource_path);
}

fn cleanup_game(mut commands: Commands, query: Query<Entity, With<GameWorld>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_children();
        commands.entity(entity).try_despawn();
    }
}

fn game_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let speed = 200.0;

    // 使用 iter_mut() 来安全地处理可能不存在的情况
    let mut iter = query.iter_mut();
    let Some(mut transform) = iter.next() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if direction != Vec2::ZERO {
        direction = direction.normalize();
        transform.translation.x += direction.x * speed * time.delta_secs();
        transform.translation.y += direction.y * speed * time.delta_secs();
    }
}

fn camera_follow_system(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    let mut player_iter = player_query.iter();
    let Some(player_transform) = player_iter.next() else {
        return;
    };

    let mut camera_iter = camera_query.iter_mut();
    let Some(mut camera_transform) = camera_iter.next() else {
        return;
    };

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}
