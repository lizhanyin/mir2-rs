//! 网络协议定义

use serde::{Deserialize, Serialize};

/// 消息ID定义
/// 基于原项目的 NetComs.pas 中的协议
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MessageId(pub u32);

// ==================== 客户端消息 (CM_*) ====================

impl MessageId {
    /// 登录请求
    pub const CM_IDPASSWORD: MessageId = MessageId(100);
    /// 请求新账号
    pub const CM_NEWID: MessageId = MessageId(101);
    /// 修改密码
    pub const CM_CHANGEPASSWORD: MessageId = MessageId(102);
    /// 修改密码确认
    pub const CM_CHANGEPASSWORDACK: MessageId = MessageId(103);
    /// 请求更新
    pub const CM_QUERYBASIC: MessageId = MessageId(104);
    /// 请求详细信息
    pub const CM_QUERYBASICACK: MessageId = MessageId(105);
    /// 查询角色
    pub const CM_QUERYCHR: MessageId = MessageId(110);
    /// 删除角色
    pub const CM_DELCHR: MessageId = MessageId(111);
    /// 选择角色
    pub const CM_SELCHR: MessageId = MessageId(112);
    /// 创建角色
    pub const CM_NEWCHR: MessageId = MessageId(113);
    /// 客户端开始
    pub const CM_CLIENTBEGIN: MessageId = MessageId(120);
    /// 客户端退出
    pub const CM_CLIENTEXIT: MessageId = MessageId(121);
    /// 客户端挂机
    pub const CM_CLIENTIDLE: MessageId = MessageId(122);

    // 游戏操作消息
    /// 走路
    pub const CM_WALK: MessageId = MessageId(200);
    /// 跑步
    pub const CM_RUN: MessageId = MessageId(201);
    /// 攻击
    pub const CM_HIT: MessageId = MessageId(202);
    /// 跑步攻击
    pub const CM_RUNHIT: MessageId = MessageId(203);
    /// 重击
    pub const CM_HEAVYHIT: MessageId = MessageId(204);
    /// 大击
    pub const CM_BIGHIT: MessageId = MessageId(205);
    /// 法术攻击
    pub const CM_SPELL: MessageId = MessageId(210);
    /// 使用物品
    pub const CM_EAT: MessageId = MessageId(220);
    /// 捡物品
    pub const CM_PICKUP_ITEM: MessageId = MessageId(221);
    /// 丢物品
    pub const CM_DROP_ITEM: MessageId = MessageId(222);
    /// 说话
    pub const CM_SAY: MessageId = MessageId(230);
    /// 点击NPC
    pub const CM_CLICKNPC: MessageId = MessageId(240);
    /// 买物品
    pub const CM_BUYITEM: MessageId = MessageId(241);
    /// 卖物品
    pub const CM_SELLITEM: MessageId = MessageId(242);
    /// 关闭NPC
    pub const CM_CLOSENPC: MessageId = MessageId(243);
}

// ==================== 服务器消息 (SM_*) ====================

impl MessageId {
    /// 登录成功
    pub const SM_NEWID_SUCCESS: MessageId = MessageId(500);
    /// 新建ID失败
    pub const SM_NEWID_FAIL: MessageId = MessageId(501);
    /// 登录成功
    pub const SM_LOGON_SUCCESS: MessageId = MessageId(502);
    /// 密码不匹配
    pub const SM_PASSWORD_FAIL: MessageId = MessageId(503);
    /// ID不存在
    pub const SM_NEWID_FAIL2: MessageId = MessageId(504);
    /// 服务器忙
    pub const SM_SERVERFULL: MessageId = MessageId(505);
    /// 开始创建角色
    pub const SM_STARTCREATECHAR: MessageId = MessageId(510);
    /// 创建角色成功
    pub const SM_CREATECHAR_SUCCESS: MessageId = MessageId(511);
    /// 创建角色失败
    pub const SM_CREATECHAR_FAIL: MessageId = MessageId(512);
    /// 角色名称重复
    pub const SM_CHARNAMEEXISTED: MessageId = MessageId(513);
    /// 查询角色响应
    pub const SM_QUERYCHR: MessageId = MessageId(520);
    /// 删除角色成功
    pub const SM_DELCHR_SUCCESS: MessageId = MessageId(521);
    /// 删除角色失败
    pub const SM_DELCHR_FAIL: MessageId = MessageId(522);
    /// 开始游戏
    pub const SM_STARTPLAY: MessageId = MessageId(530);
    /// 开始失败
    pub const SM_STARTFAIL: MessageId = MessageId(531);
    /// 用户信息
    pub const SM_USERNAME: MessageId = MessageId(532);
    /// 窗口标题
    pub const SM_WINDOWTITLE: MessageId = MessageId(533);

    // 游戏状态消息
    /// 地图信息
    pub const SM_MAPDESCRIPTION: MessageId = MessageId(600);
    /// 游戏信息
    pub const SM_GAMEGOLD: MessageId = MessageId(601);
    /// 组选项
    pub const SM_GROUPOPTION: MessageId = MessageId(602);
    /// 玩家更新
    pub const SM_PLAYERSHIFT: MessageId = MessageId(603);
    /// 创建对象
    pub const SM_CREATEOBJECT: MessageId = MessageId(604);
    /// 删除对象
    pub const SM_DELETEOBJECT: MessageId = MessageId(605);
    /// 移动失败
    pub const SM_MOVEFAIL: MessageId = MessageId(606);
    /// 对象动作
    pub const SM_OBJECTACTION: MessageId = MessageId(607);
    /// 对象走
    pub const SM_OBJECTWALK: MessageId = MessageId(608);
    /// 对象跑
    pub const SM_OBJECTRUN: MessageId = MessageId(609);
    /// 出现物品
    pub const SM_ITEMSHOW: MessageId = MessageId(610);
    /// 消失物品
    pub const SM_ITEMHIDE: MessageId = MessageId(611);
    /// 捡物品
    pub const SM_ITEMUPDATE: MessageId = MessageId(612);
    /// 增加物品
    pub const SM_ADDITEM: MessageId = MessageId(613);
    /// 更新物品
    pub const SM_UPDATEITEM: MessageId = MessageId(614);
    /// 删除物品
    pub const SM_DELITEM: MessageId = MessageId(615);
    /// 聊天消息
    pub const SM_SENDMSG: MessageId = MessageId(616);
    /// 系统消息
    pub const SM_SYSMSG: MessageId = MessageId(617);
}

/// 协议版本
pub const PROTOCOL_VERSION: u32 = 100;

/// 包头结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketHeader {
    /// 包大小
    pub size: u32,
    /// 数据大小
    pub data_size: u32,
    /// 校验和
    pub checksum: u32,
    /// 消息ID
    pub msg_id: u32,
    /// 标志
    pub flags: u32,
}

impl Default for PacketHeader {
    fn default() -> Self {
        Self {
            size: 0,
            data_size: 0,
            checksum: 0,
            msg_id: 0,
            flags: 0,
        }
    }
}
