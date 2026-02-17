//! 网络消息定义

use serde::{Deserialize, Serialize};

/// 通用消息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResponse {
    /// 消息ID
    pub msg_id: u32,
    /// 是否成功
    pub success: bool,
    /// 消息内容
    pub message: String,
}

/// 登录请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
}

/// 登录响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    /// 是否成功
    pub success: bool,
    /// 用户ID
    pub user_id: u32,
    /// 错误消息
    pub error_msg: String,
}

/// 角色信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterInfo {
    /// 角色名称
    pub name: String,
    /// 职业 (0:战士, 1:法师, 2:道士)
    pub class: u8,
    /// 性别 (0:男, 1:女)
    pub gender: u8,
    /// 等级
    pub level: u16,
    /// 当前地图
    pub map: String,
}

/// 角色列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterListResponse {
    /// 角色列表
    pub characters: Vec<CharacterInfo>,
}

/// 选择角色请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectCharacterRequest {
    /// 角色名称
    pub char_name: String,
}

/// 进入游戏响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterGameResponse {
    /// 是否成功
    pub success: bool,
    /// 角色名称
    pub char_name: String,
    /// 当前X坐标
    pub pos_x: i16,
    /// 当前Y坐标
    pub pos_y: i16,
    /// 地图名称
    pub map_name: String,
    /// HP
    pub hp: u32,
    /// 最大HP
    pub max_hp: u32,
    /// MP
    pub mp: u32,
    /// 最大MP
    pub max_mp: u32,
    /// 经验值
    pub exp: u64,
    /// 最大经验值
    pub max_exp: u64,
}

/// 玩家状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    /// 对象ID
    pub object_id: u32,
    /// 名称
    pub name: String,
    /// X坐标
    pub x: i16,
    /// Y坐标: i16,
    /// 方向 (0-7)
    pub direction: u8,
    /// 动作
    pub action: u8,
}

/// 物品信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemInfo {
    /// 物品ID
    pub id: u32,
    /// 名称
    pub name: String,
    /// 数量
    pub count: u16,
    /// 物品类型
    pub item_type: u8,
}

/// NPC对话
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcDialog {
    /// NPC名称
    pub npc_name: String,
    /// 对话内容
    pub content: String,
    /// 选项列表
    pub options: Vec<String>,
}

/// 聊天消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// 发送者
    pub sender: String,
    /// 消息内容
    pub content: String,
    /// 频道类型
    pub channel: ChatChannel,
}

/// 聊天频道
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ChatChannel {
    /// 普通
    Normal,
    /// 私聊
    Private,
    /// 组队
    Team,
    /// 公会
    Guild,
    /// 系统
    System,
    /// 世界
    World,
}
