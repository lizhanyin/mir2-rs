//! 模拟服务器
//!
//! 提供模拟的网络数据响应，用于本地开发测试

use super::message::*;
use super::protocol::MessageId;
use std::collections::HashMap;

/// 模拟服务器
pub struct MockServer {
    /// 模拟角色数据
    characters: HashMap<String, CharacterInfo>,
    /// 模拟物品数据
    items: HashMap<u32, ItemInfo>,
}

impl Default for MockServer {
    fn default() -> Self {
        Self::new()
    }
}

impl MockServer {
    /// 创建新的模拟服务器
    pub fn new() -> Self {
        let mut server = Self {
            characters: HashMap::new(),
            items: HashMap::new(),
        };
        server.init_mock_data();
        server
    }

    /// 初始化模拟数据
    fn init_mock_data(&mut self) {
        // 添加模拟角色
        self.characters.insert(
            "战士一号".to_string(),
            CharacterInfo {
                name: "战士一号".to_string(),
                class: 0,
                gender: 0,
                level: 35,
                map: "比奇省".to_string(),
            },
        );
        self.characters.insert(
            "法师小号".to_string(),
            CharacterInfo {
                name: "法师小号".to_string(),
                class: 1,
                gender: 1,
                level: 22,
                map: "盟重土城".to_string(),
            },
        );
        self.characters.insert(
            "道士高人".to_string(),
            CharacterInfo {
                name: "道士高人".to_string(),
                class: 2,
                gender: 0,
                level: 40,
                map: "蜈蚣洞".to_string(),
            },
        );

        // 添加模拟物品
        self.items.insert(
            1,
            ItemInfo {
                id: 1,
                name: "木剑".to_string(),
                count: 1,
                item_type: 0,
            },
        );
        self.items.insert(
            2,
            ItemInfo {
                id: 2,
                name: "金创药(小)".to_string(),
                count: 10,
                item_type: 1,
            },
        );
        self.items.insert(
            3,
            ItemInfo {
                id: 3,
                name: "魔法药(小)".to_string(),
                count: 10,
                item_type: 1,
            },
        );
    }

    /// 根据消息ID获取模拟响应
    pub fn get_response(&self, msg_id: MessageId) -> Option<MockResponse> {
        match msg_id {
            MessageId::CM_IDPASSWORD => Some(MockResponse::Login(LoginResponse {
                success: true,
                user_id: 1001,
                error_msg: String::new(),
            })),
            MessageId::CM_QUERYCHR => Some(MockResponse::CharacterList(CharacterListResponse {
                characters: self.characters.values().cloned().collect(),
            })),
            MessageId::CM_SELCHR => Some(MockResponse::EnterGame(EnterGameResponse {
                success: true,
                char_name: "战士一号".to_string(),
                pos_x: 330,
                pos_y: 280,
                map_name: "比奇省".to_string(),
                hp: 500,
                max_hp: 500,
                mp: 100,
                max_mp: 100,
                exp: 100000,
                max_exp: 150000,
            })),
            MessageId::CM_CLIENTBEGIN => Some(MockResponse::Message(MessageResponse {
                msg_id: msg_id.0,
                success: true,
                message: "欢迎来到传奇世界!".to_string(),
            })),
            _ => None,
        }
    }

    /// 获取模拟物品
    pub fn get_item(&self, id: u32) -> Option<&ItemInfo> {
        self.items.get(&id)
    }

    /// 获取所有角色
    pub fn get_characters(&self) -> Vec<&CharacterInfo> {
        self.characters.values().collect()
    }
}

/// 模拟响应类型
#[derive(Debug, Clone)]
pub enum MockResponse {
    Login(LoginResponse),
    CharacterList(CharacterListResponse),
    EnterGame(EnterGameResponse),
    Message(MessageResponse),
    Chat(ChatMessage),
}

impl MockResponse {
    /// 转换为JSON字符串
    pub fn to_json(&self) -> String {
        match self {
            MockResponse::Login(r) => serde_json::to_string(r).unwrap_or_default(),
            MockResponse::CharacterList(r) => serde_json::to_string(r).unwrap_or_default(),
            MockResponse::EnterGame(r) => serde_json::to_string(r).unwrap_or_default(),
            MockResponse::Message(r) => serde_json::to_string(r).unwrap_or_default(),
            MockResponse::Chat(r) => serde_json::to_string(r).unwrap_or_default(),
        }
    }
}

/// 根据消息ID获取模拟数据的便捷函数
pub fn get_mock_message(msg_id: u32) -> Option<MockResponse> {
    let server = MockServer::new();
    server.get_response(MessageId(msg_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_server() {
        let server = MockServer::new();

        // 测试登录响应
        let response = server.get_response(MessageId::CM_IDPASSWORD);
        assert!(response.is_some());
        if let Some(MockResponse::Login(login)) = response {
            assert!(login.success);
            assert_eq!(login.user_id, 1001);
        }

        // 测试角色列表
        let response = server.get_response(MessageId::CM_QUERYCHR);
        assert!(response.is_some());
        if let Some(MockResponse::CharacterList(list)) = response {
            assert_eq!(list.characters.len(), 3);
        }
    }
}
