use crate::middlewares::auth::identity::Identity;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Principal {
    pub id: i32,
    pub username: String,
    pub identity: Identity,
}

/// 手动实现 Debug trait
impl std::fmt::Debug for Principal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Principal")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("identity", &self.identity.as_str())
            .finish()
    }
}
