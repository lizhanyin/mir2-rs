//! 加载场景

use bevy::prelude::*;
use crate::core::SceneState;

/// 加载场景插件
pub struct LoadingScenePlugin;

impl Plugin for LoadingScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::Loading), setup_loading)
            .add_systems(OnExit(SceneState::Loading), cleanup_loading)
            .add_systems(Update, loading_system.run_if(in_state(SceneState::Loading)));
    }
}

/// 加载界面标记
#[derive(Component)]
struct LoadingScreen;

fn setup_loading(mut commands: Commands) {
    // 创建加载界面
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.0, 0.0, 0.0)),
        LoadingScreen,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("加载中..."),
            TextFont {
                font_size: 32.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 1.0)),
        ));
    });

    tracing::info!("进入加载场景");
}

fn cleanup_loading(mut commands: Commands, query: Query<Entity, With<LoadingScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_children();
        commands.entity(entity).try_despawn();
    }
}

fn loading_system(mut next_state: ResMut<NextState<SceneState>>) {
    // 加载完成后切换到登录场景
    // TODO: 实际的资源加载逻辑
    next_state.set(SceneState::Login);
}
