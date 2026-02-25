//! 序列帧动画播放器
//!
//! 提供基于固定时间步长的序列帧动画播放功能，支持：
//! - 在指定世界坐标播放动画
//! - 在指定 UI 元素位置播放动画
//! - 不同帧数的动画配置
//! - 动画完成回调

use bevy::prelude::*;
use std::collections::HashMap;

// ==================== 动画配置 ====================

/// 角色动作类型
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect)]
pub enum AnimationAction {
    /// 站立
    Stand,
    /// 行走
    Walk,
    /// 跑步
    Run,
    /// 攻击
    Attack,
    /// 施法
    Cast,
    /// 受伤
    Hurt,
    /// 死亡
    Death,
    /// 跳跃
    Jump,
    /// 上马
    Mount,
    /// 下马
    Dismount,
    /// 自定义动作
    Custom(u32),
}

/// 物品特效类型
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect)]
pub enum ItemEffectType {
    /// 物品在地面（旋转/闪烁）
    OnGround,
    /// 物品在背包（静态图标）
    InBag,
    /// 物品在装备栏（高亮/发光）
    OnEquip,
}

/// 物品特效动作代码（用于 AnimationAction::Custom）
pub mod item_effect_action {
    /// 物品在地面特效
    pub const ON_GROUND: u32 = 1000;
    /// 物品在背包中特效
    pub const IN_BAG: u32 = 1001;
    /// 物品在装备栏特效
    pub const ON_EQUIP: u32 = 1002;
}

impl ItemEffectType {
    /// 转换为 AnimationAction
    pub fn to_action(&self) -> AnimationAction {
        AnimationAction::Custom(match self {
            ItemEffectType::OnGround => item_effect_action::ON_GROUND,
            ItemEffectType::InBag => item_effect_action::IN_BAG,
            ItemEffectType::OnEquip => item_effect_action::ON_EQUIP,
        })
    }
}

/// 场景/UI 动画类型
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect)]
pub enum SceneAnimation {
    /// 登录界面 - 开门动画
    LoginDoor,
    /// 选择角色 - 角色展示动画
    CharacterSelect,
    /// 创建角色 - 角色预览动画
    CharacterCreate,
    /// 技能特效
    SkillEffect,
    /// 自定义场景动画
    Custom(u32),
}

/// 场景动画动作代码（用于 AnimationAction::Custom）
pub mod scene_animation_action {
    /// 登录开门动画
    pub const LOGIN_DOOR: u32 = 2000;
    /// 选择角色动画
    pub const CHARACTER_SELECT: u32 = 2001;
    /// 创建角色动画
    pub const CHARACTER_CREATE: u32 = 2002;
    /// 技能特效
    pub const SKILL_EFFECT: u32 = 2003;
}

impl SceneAnimation {
    /// 转换为 AnimationAction
    pub fn to_action(&self) -> AnimationAction {
        AnimationAction::Custom(match self {
            SceneAnimation::LoginDoor => scene_animation_action::LOGIN_DOOR,
            SceneAnimation::CharacterSelect => scene_animation_action::CHARACTER_SELECT,
            SceneAnimation::CharacterCreate => scene_animation_action::CHARACTER_CREATE,
            SceneAnimation::SkillEffect => scene_animation_action::SKILL_EFFECT,
            SceneAnimation::Custom(code) => *code,
        })
    }
}

/// 怪物/NPC 动画类型
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect)]
pub enum MonsterAnimation {
    /// 普通怪物 - 站立
    Stand,
    /// 普通怪物 - 行走
    Walk,
    /// 普通怪物 - 攻击
    Attack,
    /// 普通怪物 - 受伤
    Hurt,
    /// 普通怪物 - 死亡
    Death,
    /// BOSS 怪物 - 站立
    BossStand,
    /// BOSS 怪物 - 攻击
    BossAttack,
    /// NPC - 站立
    NpcStand,
    /// NPC - 交互
    NpcInteract,
    /// 自定义怪物动画
    Custom(u32),
}

/// 怪物动画动作代码（用于 AnimationAction::Custom）
pub mod monster_animation_action {
    /// 普通怪物站立
    pub const STAND: u32 = 3000;
    /// 普通怪物行走
    pub const WALK: u32 = 3001;
    /// 普通怪物攻击
    pub const ATTACK: u32 = 3002;
    /// 普通怪物受伤
    pub const HURT: u32 = 3003;
    /// 普通怪物死亡
    pub const DEATH: u32 = 3004;
    /// BOSS 站立
    pub const BOSS_STAND: u32 = 3010;
    /// BOSS 攻击
    pub const BOSS_ATTACK: u32 = 3011;
    /// NPC 站立
    pub const NPC_STAND: u32 = 3020;
    /// NPC 交互
    pub const NPC_INTERACT: u32 = 3021;
}

impl MonsterAnimation {
    /// 转换为 AnimationAction
    pub fn to_action(&self) -> AnimationAction {
        AnimationAction::Custom(match self {
            MonsterAnimation::Stand => monster_animation_action::STAND,
            MonsterAnimation::Walk => monster_animation_action::WALK,
            MonsterAnimation::Attack => monster_animation_action::ATTACK,
            MonsterAnimation::Hurt => monster_animation_action::HURT,
            MonsterAnimation::Death => monster_animation_action::DEATH,
            MonsterAnimation::BossStand => monster_animation_action::BOSS_STAND,
            MonsterAnimation::BossAttack => monster_animation_action::BOSS_ATTACK,
            MonsterAnimation::NpcStand => monster_animation_action::NPC_STAND,
            MonsterAnimation::NpcInteract => monster_animation_action::NPC_INTERACT,
            MonsterAnimation::Custom(code) => *code,
        })
    }
}

/// 动画方向（8方向）
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect, Default)]
pub enum AnimationDirection {
    #[default]
    Down = 0,
    DownLeft = 1,
    Left = 2,
    UpLeft = 3,
    Up = 4,
    UpRight = 5,
    Right = 6,
    DownRight = 7,
}

impl AnimationDirection {
    /// 从角度获取方向（0-360度）
    pub fn from_angle(angle: f32) -> Self {
        let normalized = ((angle % 360.0) + 360.0) % 360.0;
        let index = ((normalized + 22.5) / 45.0) as usize % 8;
        match index {
            0 => Self::Right,
            1 => Self::UpRight,
            2 => Self::Up,
            3 => Self::UpLeft,
            4 => Self::Left,
            5 => Self::DownLeft,
            6 => Self::Down,
            7 => Self::DownRight,
            _ => Self::Down,
        }
    }

    /// 获取方向索引（用于计算帧偏移）
    pub fn index(&self) -> usize {
        *self as usize
    }
}

/// 单个动画的配置
///
/// 对应原始 Mir2 的 `TActionInfo` 结构：
/// ```pascal
/// TActionInfo = packed record
///     start   :Word;  // 开始帧 -> start_index
///     frame   :Word;  // 帧数 -> frame_count
///     skip    :Word;  // 跳过的帧数 -> skip
///     ftime   :Word;  // 每帧延迟时间（毫秒）-> duration
///     usetick :Word;  // 用于行走动画
/// end;
/// ```
#[derive(Clone, Debug)]
pub struct AnimationClip {
    /// 动画名称
    pub name: String,
    /// 动作类型
    pub action: AnimationAction,
    /// 总帧数
    pub frame_count: usize,
    /// 动画总时长（秒）
    pub duration: f32,
    /// 是否循环
    pub looping: bool,
    /// 帧索引在图集中的起始位置
    pub start_index: usize,
    /// 帧间隔（每帧之间跳过的索引数，用于某些特殊动画）
    pub skip: usize,
    /// 方向数量（默认8方向）
    pub direction_count: usize,
    /// 每个方向的帧数
    pub frames_per_direction: usize,
    /// 帧偏移量（可选，用于不规则动画）
    pub frame_offsets: Option<Vec<Vec2>>,
}

impl AnimationClip {
    /// 创建新的动画剪辑
    pub fn new(
        name: impl Into<String>,
        action: AnimationAction,
        frame_count: usize,
        duration: f32,
        start_index: usize,
    ) -> Self {
        Self {
            name: name.into(),
            action,
            frame_count,
            duration,
            looping: true,
            start_index,
            skip: 0,
            direction_count: 8,
            frames_per_direction: frame_count,
            frame_offsets: None,
        }
    }

    /// 从 Mir2 的 TActionInfo 结构创建动画剪辑
    ///
    /// # 参数
    /// - `name`: 动画名称
    /// - `action`: 动作类型
    /// - `start`: 开始帧索引
    /// - `frame`: 帧数
    /// - `skip`: 帧间隔
    /// - `ftime`: 每帧延迟时间（毫秒）
    pub fn from_action_info(
        name: impl Into<String>,
        action: AnimationAction,
        start: usize,
        frame: usize,
        skip: usize,
        ftime: u32,
    ) -> Self {
        let duration = (frame as f32 * ftime as f32) / 1000.0;
        Self {
            name: name.into(),
            action,
            frame_count: frame,
            duration,
            looping: true,
            start_index: start,
            skip,
            direction_count: 8,
            frames_per_direction: frame,
            frame_offsets: None,
        }
    }

    /// 设置帧间隔
    pub fn with_skip(mut self, skip: usize) -> Self {
        self.skip = skip;
        self
    }

    /// 设置是否循环
    pub fn with_looping(mut self, looping: bool) -> Self {
        self.looping = looping;
        self
    }

    /// 设置方向数量
    pub fn with_directions(mut self, count: usize) -> Self {
        self.direction_count = count;
        self
    }

    /// 设置每个方向的帧数
    pub fn with_frames_per_direction(mut self, count: usize) -> Self {
        self.frames_per_direction = count;
        self
    }

    /// 设置帧偏移
    pub fn with_offsets(mut self, offsets: Vec<Vec2>) -> Self {
        self.frame_offsets = Some(offsets);
        self
    }

    /// 根据进度获取当前帧索引（不带方向）
    pub fn get_frame_at(&self, progress: f32) -> usize {
        let frame = (progress.clamp(0.0, 1.0) * self.frame_count as f32).floor() as usize;
        frame.min(self.frame_count.saturating_sub(1))
    }

    /// 根据进度和方向获取当前帧索引
    ///
    /// 计算公式：`start_index + direction_index * frames_per_direction + local_frame * (1 + skip)`
    pub fn get_frame_at_with_direction(
        &self,
        progress: f32,
        direction: AnimationDirection,
    ) -> usize {
        let local_frame = self.get_frame_at(progress);
        // 考虑 skip 字段：每帧实际占用的索引空间为 (1 + skip)
        let frame_stride = 1 + self.skip;
        self.start_index
            + direction.index() * self.frames_per_direction * frame_stride
            + local_frame * frame_stride
    }

    /// 获取每帧持续时间
    pub fn frame_duration(&self) -> f32 {
        if self.frame_count > 0 {
            self.duration / self.frame_count as f32
        } else {
            self.duration
        }
    }

    /// 检查动画是否完成
    pub fn is_finished(&self, progress: f32) -> bool {
        !self.looping && progress >= 1.0
    }
}

/// 动画库（包含多个动画剪辑）
#[derive(Clone, Resource)]
pub struct AnimationLibrary {
    /// 动画剪辑集合 (name -> clip)
    clips: HashMap<String, AnimationClip>,
    /// 默认动画名称
    default_animation: Option<String>,
}

impl Default for AnimationLibrary {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationLibrary {
    /// 创建新的动画库
    pub fn new() -> Self {
        Self {
            clips: HashMap::new(),
            default_animation: None,
        }
    }

    /// 添加动画剪辑
    pub fn add_clip(&mut self, clip: AnimationClip) -> &mut Self {
        let name = clip.name.clone();
        self.clips.insert(name.clone(), clip);
        if self.default_animation.is_none() {
            self.default_animation = Some(name);
        }
        self
    }

    /// 获取动画剪辑
    pub fn get_clip(&self, name: &str) -> Option<&AnimationClip> {
        self.clips.get(name)
    }

    /// 获取默认动画
    pub fn get_default_clip(&self) -> Option<&AnimationClip> {
        self.default_animation
            .as_ref()
            .and_then(|name| self.clips.get(name))
    }

    /// 设置默认动画
    pub fn set_default(&mut self, name: &str) -> &mut Self {
        if self.clips.contains_key(name) {
            self.default_animation = Some(name.to_string());
        }
        self
    }

    /// 获取所有动画名称
    pub fn animation_names(&self) -> Vec<&String> {
        self.clips.keys().collect()
    }
}

// ==================== 动画状态组件 ====================

/// 动画播放状态
#[derive(Component, Clone, Debug)]
pub struct SpriteAnimationState {
    /// 当前动画名称
    pub current_animation: String,
    /// 当前动画剪辑（缓存）
    clip: Option<AnimationClip>,
    /// 播放进度 0.0 ~ 1.0
    pub progress: f32,
    /// 当前方向
    pub direction: AnimationDirection,
    /// 播放速度倍率
    pub speed: f32,
    /// 是否暂停
    pub paused: bool,
    /// 是否完成（用于非循环动画）
    pub finished: bool,
    /// 帧纹理句柄列表
    pub frames: Vec<Handle<Image>>,
    /// 当前帧索引（渲染用）
    pub current_frame_index: usize,
}

impl SpriteAnimationState {
    /// 创建新的动画状态
    pub fn new(animation_name: impl Into<String>) -> Self {
        Self {
            current_animation: animation_name.into(),
            clip: None,
            progress: 0.0,
            direction: AnimationDirection::default(),
            speed: 1.0,
            paused: false,
            finished: false,
            frames: Vec::new(),
            current_frame_index: 0,
        }
    }

    /// 设置帧纹理
    pub fn with_frames(mut self, frames: Vec<Handle<Image>>) -> Self {
        self.frames = frames;
        self
    }

    /// 设置方向
    pub fn with_direction(mut self, direction: AnimationDirection) -> Self {
        self.direction = direction;
        self
    }

    /// 设置播放速度
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    /// 绑定动画剪辑
    pub fn bind_clip(&mut self, clip: &AnimationClip) {
        self.clip = Some(clip.clone());
        self.progress = 0.0;
        self.finished = false;
    }

    /// 切换动画
    pub fn play(&mut self, animation_name: &str, library: &AnimationLibrary) -> bool {
        if self.current_animation != animation_name {
            if let Some(clip) = library.get_clip(animation_name) {
                self.current_animation = animation_name.to_string();
                self.bind_clip(clip);
                return true;
            }
        }
        false
    }

    /// 更新动画进度
    pub fn update(&mut self, delta: f32) {
        if self.paused || self.finished || self.frames.is_empty() {
            return;
        }

        if let Some(ref clip) = self.clip {
            let adjusted_duration = clip.duration / self.speed;
            if adjusted_duration > 0.0 {
                let delta_progress = delta / adjusted_duration;
                self.progress += delta_progress;

                if self.progress >= 1.0 {
                    if clip.looping {
                        self.progress = self.progress % 1.0;
                    } else {
                        self.progress = 1.0;
                        self.finished = true;
                    }
                }
            }

            // 计算当前帧
            self.current_frame_index = self.get_current_frame_index();
        }
    }

    /// 获取当前帧索引
    fn get_current_frame_index(&self) -> usize {
        if self.frames.is_empty() {
            return 0;
        }

        if let Some(ref clip) = self.clip {
            clip.get_frame_at(self.progress)
        } else {
            (self.progress * self.frames.len() as f32).floor() as usize % self.frames.len()
        }
    }

    /// 获取当前帧纹理
    pub fn current_frame(&self) -> Option<Handle<Image>> {
        self.frames.get(self.current_frame_index).cloned()
    }

    /// 获取帧偏移
    pub fn get_frame_offset(&self) -> Vec2 {
        if let Some(ref clip) = self.clip {
            if let Some(ref offsets) = clip.frame_offsets {
                if let Some(&offset) = offsets.get(self.current_frame_index) {
                    return offset;
                }
            }
        }
        Vec2::ZERO
    }

    /// 重置动画
    pub fn reset(&mut self) {
        self.progress = 0.0;
        self.finished = false;
    }
}

// ==================== 动画播放器 ====================

/// 动画播放器命令
#[derive(Message, Clone, Debug)]
pub enum AnimationCommand {
    /// 在世界坐标播放动画
    PlayAtWorld {
        animation_name: String,
        position: Vec3,
        direction: AnimationDirection,
        speed: f32,
        looping: bool,
        frames: Vec<Handle<Image>>,
    },
    /// 跟随实体播放动画
    PlayAttached {
        animation_name: String,
        target: Entity,
        offset: Vec3,
        direction: AnimationDirection,
        speed: f32,
        looping: bool,
        frames: Vec<Handle<Image>>,
    },
    /// 在 UI 位置播放动画
    PlayAtUI {
        animation_name: String,
        ui_position: Vec2,
        direction: AnimationDirection,
        speed: f32,
        looping: bool,
        frames: Vec<Handle<Image>>,
    },
    /// 停止动画
    Stop,
    /// 暂停动画
    Pause,
    /// 恢复动画
    Resume,
}

/// 一次性动画播放器组件
#[derive(Component, Debug)]
pub struct OneShotAnimation {
    /// 动画状态
    pub state: SpriteAnimationState,
    /// 是否完成
    pub finished: bool,
    /// 动画完成后的回调标记
    pub on_complete: bool,
    /// 目标实体（如果是跟随模式）
    pub target_entity: Option<Entity>,
    /// 偏移量
    pub offset: Vec3,
    /// 是否是 UI 动画
    pub is_ui: bool,
}

impl OneShotAnimation {
    /// 在世界坐标创建一次性动画
    pub fn at_world(state: SpriteAnimationState) -> Self {
        Self {
            state,
            finished: false,
            on_complete: false,
            target_entity: None,
            offset: Vec3::ZERO,
            is_ui: false,
        }
    }

    /// 创建跟随实体的动画
    pub fn attached_to(state: SpriteAnimationState, target: Entity, offset: Vec3) -> Self {
        Self {
            state,
            finished: false,
            on_complete: false,
            target_entity: Some(target),
            offset,
            is_ui: false,
        }
    }

    /// 创建 UI 动画
    pub fn at_ui(state: SpriteAnimationState) -> Self {
        Self {
            state,
            finished: false,
            on_complete: false,
            target_entity: None,
            offset: Vec3::ZERO,
            is_ui: true,
        }
    }
}

// ==================== 动画事件 ====================

/// 动画事件
#[derive(Message, Clone, Debug)]
pub struct AnimationEvent {
    /// 触发动画的实体
    pub entity: Entity,
    /// 动画名称
    pub animation_name: String,
    /// 事件类型
    pub event_type: AnimationEventType,
}

/// 动画事件类型
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnimationEventType {
    /// 动画开始
    Started,
    /// 动画完成
    Finished,
    /// 动画循环
    Looped,
    /// 帧变化
    FrameChanged(usize),
}

// ==================== 系统实现 ====================

/// 固定时间步长动画更新系统
pub fn fixed_animation_update(
    time: Res<Time>,
    mut query: Query<(Entity, &mut SpriteAnimationState, &mut Sprite)>,
    mut events: MessageWriter<AnimationEvent>,
) {
    let delta = time.delta_secs();

    for (entity, mut state, mut sprite) in query.iter_mut() {
        let old_frame = state.current_frame_index;
        state.update(delta);

        // 更新精灵纹理
        if let Some(handle) = state.current_frame() {
            sprite.image = handle;
        }

        // 发送帧变化事件
        if old_frame != state.current_frame_index {
            events.write(AnimationEvent {
                entity,
                animation_name: state.current_animation.clone(),
                event_type: AnimationEventType::FrameChanged(state.current_frame_index),
            });
        }

        // 发送完成事件
        if state.finished && !state.paused {
            events.write(AnimationEvent {
                entity,
                animation_name: state.current_animation.clone(),
                event_type: AnimationEventType::Finished,
            });
        }
    }
}

/// 一次性动画更新系统
pub fn oneshot_animation_update(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut OneShotAnimation,
        &mut Transform,
        &mut Sprite,
        Option<&Visibility>,
    )>,
    target_query: Query<&Transform, Without<OneShotAnimation>>,
) {
    let delta = time.delta_secs();

    for (entity, mut anim, mut transform, mut sprite, _visibility) in query.iter_mut() {
        // 更新动画状态
        anim.state.update(delta);

        // 更新精灵纹理
        if let Some(handle) = anim.state.current_frame() {
            sprite.image = handle;
        }

        // 如果是跟随模式，更新位置
        if let Some(target) = anim.target_entity {
            if let Ok(target_transform) = target_query.get(target) {
                transform.translation = target_transform.translation + anim.offset;
            }
        }

        // 检查是否完成
        if anim.state.finished {
            anim.finished = true;
            anim.on_complete = true;

            // 如果完成且非循环，移除实体
            if !anim.state.paused {
                commands.entity(entity).despawn();
            }
        }
    }
}

/// 动画命令处理系统
pub fn animation_command_handler(
    mut commands: Commands,
    mut command_reader: MessageReader<AnimationCommand>,
    animation_library: Res<AnimationLibrary>,
) {
    for cmd in command_reader.read() {
        match cmd {
            AnimationCommand::PlayAtWorld {
                animation_name,
                position,
                direction,
                speed,
                looping,
                frames,
            } => {
                let mut state = SpriteAnimationState::new(animation_name.clone())
                    .with_direction(*direction)
                    .with_speed(*speed)
                    .with_frames(frames.clone());

                if let Some(clip) = animation_library.get_clip(animation_name) {
                    let mut clip = clip.clone();
                    clip.looping = *looping;
                    state.bind_clip(&clip);
                }

                commands.spawn((
                    Sprite::default(),
                    Transform::from_translation(*position),
                    Visibility::default(),
                    OneShotAnimation::at_world(state),
                ));
            }
            AnimationCommand::PlayAttached {
                animation_name,
                target,
                offset,
                direction,
                speed,
                looping,
                frames,
            } => {
                let mut state = SpriteAnimationState::new(animation_name.clone())
                    .with_direction(*direction)
                    .with_speed(*speed)
                    .with_frames(frames.clone());

                if let Some(clip) = animation_library.get_clip(animation_name) {
                    let mut clip = clip.clone();
                    clip.looping = *looping;
                    state.bind_clip(&clip);
                }

                commands.spawn((
                    Sprite::default(),
                    Transform::default(),
                    Visibility::default(),
                    OneShotAnimation::attached_to(state, *target, *offset),
                ));
            }
            AnimationCommand::PlayAtUI {
                animation_name,
                ui_position,
                direction,
                speed,
                looping,
                frames,
            } => {
                let mut state = SpriteAnimationState::new(animation_name.clone())
                    .with_direction(*direction)
                    .with_speed(*speed)
                    .with_frames(frames.clone());

                if let Some(clip) = animation_library.get_clip(animation_name) {
                    let mut clip = clip.clone();
                    clip.looping = *looping;
                    state.bind_clip(&clip);
                }

                commands.spawn((
                    Sprite::default(),
                    Transform::from_translation(Vec3::new(ui_position.x, ui_position.y, 0.0)),
                    Visibility::default(),
                    OneShotAnimation::at_ui(state),
                ));
            }
            _ => {}
        }
    }
}

// ==================== 辅助函数 ====================

/// 动画播放器 API
pub struct SpriteAnimationPlayer;

impl SpriteAnimationPlayer {
    /// 在指定世界坐标播放动画
    pub fn play_at_world(
        commands: &mut Commands,
        animation_name: &str,
        position: Vec3,
        frames: Vec<Handle<Image>>,
        direction: AnimationDirection,
        speed: f32,
        _looping: bool,
    ) -> Entity {
        let state = SpriteAnimationState::new(animation_name)
            .with_direction(direction)
            .with_speed(speed)
            .with_frames(frames);

        commands
            .spawn((
                Sprite::default(),
                Transform::from_translation(position),
                Visibility::default(),
                OneShotAnimation::at_world(state),
            ))
            .id()
    }

    /// 在实体位置播放动画（跟随）
    pub fn play_attached(
        commands: &mut Commands,
        animation_name: &str,
        target: Entity,
        offset: Vec3,
        frames: Vec<Handle<Image>>,
        direction: AnimationDirection,
        speed: f32,
        _looping: bool,
    ) -> Entity {
        let state = SpriteAnimationState::new(animation_name)
            .with_direction(direction)
            .with_speed(speed)
            .with_frames(frames);

        commands
            .spawn((
                Sprite::default(),
                Transform::default(),
                Visibility::default(),
                OneShotAnimation::attached_to(state, target, offset),
            ))
            .id()
    }

    /// 在 UI 位置播放动画
    pub fn play_at_ui(
        commands: &mut Commands,
        animation_name: &str,
        ui_position: Vec2,
        frames: Vec<Handle<Image>>,
        direction: AnimationDirection,
        speed: f32,
        _looping: bool,
    ) -> Entity {
        let state = SpriteAnimationState::new(animation_name)
            .with_direction(direction)
            .with_speed(speed)
            .with_frames(frames);

        commands
            .spawn((
                Sprite::default(),
                Transform::from_translation(Vec3::new(ui_position.x, ui_position.y, 0.0)),
                Visibility::default(),
                OneShotAnimation::at_ui(state),
            ))
            .id()
    }

    /// 停止动画实体
    pub fn stop(commands: &mut Commands, entity: Entity) {
        commands.entity(entity).despawn();
    }
}

// ==================== 预设动画配置 ====================

/// 传奇2角色动画配置
pub fn create_mir2_character_animations() -> AnimationLibrary {
    let mut library = AnimationLibrary::new();

    // 站立动画：4帧，800ms
    library.add_clip(
        AnimationClip::new("stand", AnimationAction::Stand, 4, 0.8, 0).with_frames_per_direction(4),
    );

    // 行走动画：8帧，600ms
    library.add_clip(
        AnimationClip::new("walk", AnimationAction::Walk, 8, 0.6, 32).with_frames_per_direction(8),
    );

    // 跑步动画：8帧，400ms
    library.add_clip(
        AnimationClip::new("run", AnimationAction::Run, 8, 0.4, 96).with_frames_per_direction(8),
    );

    // 攻击动画：10帧，500ms，不循环
    library.add_clip(
        AnimationClip::new("attack", AnimationAction::Attack, 10, 0.5, 160)
            .with_frames_per_direction(10)
            .with_looping(false),
    );

    // 施法动画：6帧，400ms
    library.add_clip(
        AnimationClip::new("cast", AnimationAction::Cast, 6, 0.4, 240).with_frames_per_direction(6),
    );

    // 受伤动画：3帧，200ms
    library.add_clip(
        AnimationClip::new("hurt", AnimationAction::Hurt, 3, 0.2, 288)
            .with_frames_per_direction(3)
            .with_looping(false),
    );

    // 死亡动画：4帧，800ms，不循环
    library.add_clip(
        AnimationClip::new("death", AnimationAction::Death, 4, 0.8, 312)
            .with_frames_per_direction(4)
            .with_looping(false),
    );

    library.set_default("stand");
    library
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_clip() {
        let clip = AnimationClip::new("test", AnimationAction::Stand, 4, 1.0, 0);

        assert_eq!(clip.get_frame_at(0.0), 0);
        assert_eq!(clip.get_frame_at(0.25), 1);
        assert_eq!(clip.get_frame_at(0.5), 2);
        assert_eq!(clip.get_frame_at(0.75), 3);
        assert_eq!(clip.get_frame_at(1.0), 3); // clamp
    }

    #[test]
    fn test_animation_clip_with_skip() {
        // 测试带 skip 的动画剪辑
        let clip = AnimationClip::new("test", AnimationAction::Stand, 4, 1.0, 0)
            .with_skip(2)
            .with_frames_per_direction(4);

        // skip=2 意味着每帧占用 3 个索引空间 (1 + 2)
        // 方向 Down (index=0):
        //   progress=0.0 -> local_frame=0 -> 0 + 0*3 + 0*3 = 0
        //   progress=0.25 -> local_frame=1 -> 0 + 0*3 + 1*3 = 3
        //   progress=0.5 -> local_frame=2 -> 0 + 0*3 + 2*3 = 6
        //   progress=0.75 -> local_frame=3 -> 0 + 0*3 + 3*3 = 9
        assert_eq!(
            clip.get_frame_at_with_direction(0.0, AnimationDirection::Down),
            0
        );
        assert_eq!(
            clip.get_frame_at_with_direction(0.25, AnimationDirection::Down),
            3
        );
        assert_eq!(
            clip.get_frame_at_with_direction(0.5, AnimationDirection::Down),
            6
        );
        assert_eq!(
            clip.get_frame_at_with_direction(0.75, AnimationDirection::Down),
            9
        );

        // 方向 Left (index=2):
        //   基础偏移 = 2 * 4 * 3 = 24
        //   progress=0.0 -> local_frame=0 -> 0 + 24 + 0 = 24
        //   progress=0.25 -> local_frame=1 -> 0 + 24 + 3 = 27
        assert_eq!(
            clip.get_frame_at_with_direction(0.0, AnimationDirection::Left),
            24
        );
        assert_eq!(
            clip.get_frame_at_with_direction(0.25, AnimationDirection::Left),
            27
        );
    }

    #[test]
    fn test_from_action_info() {
        // 模拟原始 Mir2 的 TActionInfo
        // start=64, frame=6, skip=0, ftime=100ms
        let clip = AnimationClip::from_action_info(
            "walk",
            AnimationAction::Walk,
            64,  // start
            6,   // frame
            0,   // skip
            100, // ftime (ms)
        );

        assert_eq!(clip.start_index, 64);
        assert_eq!(clip.frame_count, 6);
        assert_eq!(clip.skip, 0);
        // duration = frame * ftime / 1000 = 6 * 100 / 1000 = 0.6s
        assert!((clip.duration - 0.6).abs() < 0.001);
    }

    #[test]
    fn test_animation_direction() {
        assert_eq!(
            AnimationDirection::from_angle(0.0),
            AnimationDirection::Right
        );
        assert_eq!(
            AnimationDirection::from_angle(45.0),
            AnimationDirection::UpRight
        );
        assert_eq!(AnimationDirection::from_angle(90.0), AnimationDirection::Up);
        assert_eq!(
            AnimationDirection::from_angle(180.0),
            AnimationDirection::Left
        );
        assert_eq!(
            AnimationDirection::from_angle(270.0),
            AnimationDirection::Down
        );
    }

    #[test]
    fn test_animation_library() {
        let mut library = AnimationLibrary::new();
        library.add_clip(AnimationClip::new(
            "stand",
            AnimationAction::Stand,
            4,
            0.8,
            0,
        ));
        library.add_clip(AnimationClip::new(
            "walk",
            AnimationAction::Walk,
            8,
            0.6,
            32,
        ));

        assert!(library.get_clip("stand").is_some());
        assert!(library.get_clip("walk").is_some());
        assert!(library.get_clip("run").is_none());
    }
}
