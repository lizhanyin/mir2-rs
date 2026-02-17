# Mir2-RS 架构设计

基于 Bevy 游戏引擎的《传奇2》客户端重构架构设计。

## 1. 技术栈

- **游戏引擎**: Bevy 0.15 (2D 模式)
- **UI 框架**: bevy_extended_ui
- **资源格式**: 自定义 formats/image 模块
- **网络模拟**: 本地模拟数据

## 2. 目录结构

```
mir2-rs/
├── Cargo.toml
├── src/
│   ├── main.rs              # 应用入口
│   ├── lib.rs               # 库入口
│   ├── error.rs             # 错误定义
│   │
│   ├── core/                # 核心系统
│   │   ├── mod.rs
│   │   ├── game_state.rs    # 游戏状态
│   │   ├── config.rs        # 配置管理
│   │   └── timer.rs         # 定时器系统
│   │
│   ├── resource/            # 资源管理 (原 formats/image)
│   │   ├── mod.rs
│   │   ├── formats/         # 格式解析 (移动自 src/formats)
│   │   ├── image/           # 图像处理 (移动自 src/image)
│   │   ├── loader.rs        # Bevy 资源加载器
│   │   └── cache.rs         # 资源缓存
│   │
│   ├── scene/               # 场景系统
│   │   ├── mod.rs
│   │   ├── login.rs         # 登录场景
│   │   ├── select_char.rs   # 选角色场景
│   │   ├── game.rs          # 游戏主场景
│   │   └── loading.rs       # 加载场景
│   │
│   ├── game/                # 游戏逻辑
│   │   ├── mod.rs
│   │   ├── actor/           # 角色系统
│   │   │   ├── mod.rs
│   │   │   ├── player.rs    # 玩家
│   │   │   ├── npc.rs       # NPC
│   │   │   ├── monster.rs   # 怪物
│   │   │   └── animation.rs # 动画
│   │   │
│   │   ├── map/             # 地图系统
│   │   │   ├── mod.rs
│   │   │   ├── tile.rs      # 地图块
│   │   │   └── pathfinding.rs # 寻路
│   │   │
│   │   ├── skill/           # 技能系统
│   │   │   ├── mod.rs
│   │   │   └── effect.rs    # 技能特效
│   │   │
│   │   └── item/            # 物品系统
│   │       ├── mod.rs
│   │       └── inventory.rs # 背包
│   │
│   ├── network/             # 网络模拟
│   │   ├── mod.rs
│   │   ├── protocol.rs      # 协议定义
│   │   ├── message.rs       # 消息结构
│   │   └── mock_server.rs   # 模拟服务器
│   │
│   ├── ui/                  # UI 系统
│   │   ├── mod.rs
│   │   ├── hud.rs           # 游戏HUD
│   │   ├── dialog.rs        # 对话框
│   │   ├── inventory.rs     # 背包UI
│   │   └── skill_bar.rs     # 技能栏
│   │
│   └── render/              # 渲染系统
│       ├── mod.rs
│       ├── sprite.rs        # 精灵渲染
│       └── effect.rs        # 特效渲染
│
└── assets/                  # 游戏资源 (符号链接到 E:\Game\Online\Legend of mir)
```

## 3. Bevy ECS 系统设计

### 3.1 核心资源 (Resource)

```rust
// 游戏状态
#[derive(Resource, Default)]
pub struct GameState {
    pub current_scene: SceneType,
    pub is_logged_in: bool,
    pub player_name: String,
}

// 资源管理器
#[derive(Resource)]
pub struct ResourceManager {
    pub libraries: HashMap<String, Arc<Mutex<LibraryLoader>>>,
}

// 配置
#[derive(Resource)]
pub struct GameConfig {
    pub resource_path: PathBuf,
    pub screen_width: f32,
    pub screen_height: f32,
}
```

### 3.2 核心组件 (Component)

```rust
// 角色组件
#[derive(Component)]
pub struct Actor {
    pub id: u32,
    pub name: String,
    pub actor_type: ActorType,
}

#[derive(Component)]
pub struct Transform2D {
    pub position: Vec2,
    pub direction: Direction,
}

// 地图块组件
#[derive(Component)]
pub struct MapTile {
    pub x: i32,
    pub y: i32,
    pub tile_type: TileType,
}

// 动画组件
#[derive(Component)]
pub struct Animation {
    pub frames: Vec<usize>,
    pub current_frame: usize,
    pub frame_time: f32,
    pub timer: f32,
}
```

### 3.3 核心系统 (System)

```rust
// 场景切换系统
fn scene_transition_system(
    current_state: Res<GameState>,
    mut next_state: ResMut<NextState<SceneType>>,
)

// 动画更新系统
fn animation_system(
    time: Res<Time>,
    mut query: Query<(&mut Animation, &mut TextureAtlas)>,
)

// 移动系统
fn movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity, &MoveTarget)>,
)

// 输入处理系统
fn input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut MoveTarget, With<Player>>,
)
```

## 4. 游戏流程

```
启动 → 加载资源 → 登录场景 → 选角色场景 → 游戏场景
         ↑                                      ↓
         └──────────── 返回登录 ←───────────────┘
```

## 5. 网络消息模拟

```rust
// 消息ID映射
pub fn get_mock_message(msg_id: u32) -> Option<MockMessage> {
    match msg_id {
        1001 => Some(MockMessage::LoginResponse { success: true }),
        1002 => Some(MockMessage::CharacterList { chars: vec![...] }),
        2001 => Some(MockMessage::PlayerInfo { ... }),
        // ... 更多消息
        _ => None,
    }
}
```

## 6. 资源加载策略

1. **启动阶段**: 加载必要资源（UI、登录界面）
2. **场景切换**: 预加载下一场景资源
3. **游戏运行**: 按需加载地图和角色资源
4. **资源缓存**: LRU 缓存常用资源

## 7. 渲染层级

```
Layer 0: 地面层 (地图块)
Layer 1: 物品层 (掉落物)
Layer 2: 角色层 (玩家、NPC、怪物)
Layer 3: 特效层 (技能特效)
Layer 4: UI层 (HUD、对话框)
```

## 8. 实现优先级

1. **Phase 1**: 基础框架
   - 错误处理模块
   - 资源加载插件
   - 基础场景系统

2. **Phase 2**: 登录流程
   - 登录界面
   - 角色选择
   - 网络模拟

3. **Phase 3**: 游戏核心
   - 地图渲染
   - 角色系统
   - 移动和动画

4. **Phase 4**: 完善功能
   - UI 系统
   - 技能系统
   - 物品系统
