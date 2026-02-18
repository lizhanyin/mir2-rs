//! 登录成功动画场景
//!
//! 播放登录成功后的动画 (index 24-32)
//! 动画播放完成后自动切换到角色选择场景

use bevy::prelude::*;

use crate::core::{GameConfig, SceneState};
use crate::resource::{load_bevy_images_by_indices_from_path, FrameAnimation};

/// 登录成功动画场景插件
pub struct LoginSuccessScenePlugin;

impl Plugin for LoginSuccessScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::LoginSuccess), setup_login_success)
            .add_systems(OnExit(SceneState::LoginSuccess), cleanup_login_success)
            .add_systems(
                Update,
                (login_success_animation_system, check_animation_complete)
                    .run_if(in_state(SceneState::LoginSuccess)),
            );
    }
}

/// 登录成功场景标记
#[derive(Component)]
struct LoginSuccessScreen;

/// 登录成功背景标记
#[derive(Component)]
struct LoginSuccessBackground;

/// 登录成功动画精灵标记
#[derive(Component)]
struct LoginSuccessAnimationSprite;

/// 登录成功动画帧索引
const LOGIN_SUCCESS_ANIMATION_INDICES: &[usize] = &[24, 25, 26, 27, 28, 29, 30, 31, 32];

/// 登录成功背景图片索引 (复用登录背景)
const LOGIN_SUCCESS_BG_INDEX: usize = 22;

/// 库名称
const LIBRARY_NAME: &str = "data/ChrSel";

/// 动画播放次数后切换场景
const ANIMATION_PLAY_COUNT: usize = 1;

/// 动画播放计数器资源
#[derive(Resource, Default)]
struct AnimationPlayCounter {
    count: usize,
}

fn setup_login_success(
    mut commands: Commands,
    config: Res<GameConfig>,
    mut images: ResMut<Assets<Image>>,
) {
    tracing::info!("进入登录成功动画场景");

    // 加载背景图片 (复用登录背景 index=22)
    if let Ok(bg_images) = load_bevy_images_by_indices_from_path(
        &config.resource_path,
        LIBRARY_NAME,
        &[LOGIN_SUCCESS_BG_INDEX],
    ) {
        if let Some(Some(bevy_image)) = bg_images.into_iter().next() {
            let image_handle = images.add(bevy_image);

            // 生成背景精灵
            commands.spawn((
                Sprite::from_image(image_handle),
                Transform::from_xyz(0.0, 0.0, -1.0),
                LoginSuccessBackground,
            ));
        }
    }

    // 加载登录成功动画帧 (index 24-32)
    if let Ok(animation_images) = load_bevy_images_by_indices_from_path(
        &config.resource_path,
        LIBRARY_NAME,
        LOGIN_SUCCESS_ANIMATION_INDICES,
    ) {
        let animation_frames: Vec<Handle<Image>> = animation_images
            .into_iter()
            .filter_map(|img| img.map(|i| images.add(i)))
            .collect();

        if !animation_frames.is_empty() {
            tracing::info!("登录成功动画帧加载成功: {} 帧", animation_frames.len());

            // 创建动画资源 (10 FPS)
            let animation = FrameAnimation::new(animation_frames.clone(), 10.0);
            commands.insert_resource(animation);

            // 创建动画精灵
            let first_frame = animation_frames[0].clone();
            commands.spawn((
                Sprite::from_image(first_frame),
                Transform::from_xyz(0.0, 0.0, 0.0),
                LoginSuccessAnimationSprite,
            ));
        }
    }

    // 初始化动画播放计数器
    commands.insert_resource(AnimationPlayCounter::default());

    // 标记场景已加载
    commands.spawn(LoginSuccessScreen);
}

/// 登录成功动画系统
fn login_success_animation_system(
    time: Res<Time>,
    mut animation: Option<ResMut<FrameAnimation>>,
    mut query: Query<&mut Sprite, With<LoginSuccessAnimationSprite>>,
    mut counter: ResMut<AnimationPlayCounter>,
) {
    if let Some(ref mut anim) = animation {
        let prev_frame = anim.current_frame;
        anim.update(time.delta_secs());

        // 检测动画是否完成一轮 (从最后一帧回到第一帧)
        if prev_frame > 0 && anim.current_frame == 0 {
            counter.count += 1;
            tracing::debug!("动画播放完成一轮，当前计数: {}", counter.count);
        }

        // 更新动画精灵的纹理
        if let Some(current_handle) = anim.current_frame_handle() {
            for mut sprite in query.iter_mut() {
                sprite.image = current_handle.clone();
            }
        }
    }
}

/// 检查动画是否播放完成
fn check_animation_complete(
    counter: Res<AnimationPlayCounter>,
    mut next_state: ResMut<NextState<SceneState>>,
) {
    if counter.count >= ANIMATION_PLAY_COUNT {
        tracing::info!("登录成功动画播放完成，切换到角色选择场景");
        next_state.set(SceneState::SelectChar);
    }
}

fn cleanup_login_success(
    mut commands: Commands,
    query: Query<Entity, With<LoginSuccessScreen>>,
    bg_query: Query<Entity, With<LoginSuccessBackground>>,
    anim_query: Query<Entity, With<LoginSuccessAnimationSprite>>,
) {
    // 移除场景标记实体
    for entity in query.iter() {
        commands.entity(entity).try_despawn();
    }

    // 移除背景精灵
    for entity in bg_query.iter() {
        commands.entity(entity).try_despawn();
    }

    // 移除动画精灵
    for entity in anim_query.iter() {
        commands.entity(entity).try_despawn();
    }

    // 移除动画资源
    commands.remove_resource::<FrameAnimation>();
    commands.remove_resource::<AnimationPlayCounter>();

    tracing::info!("退出登录成功动画场景");
}
