use sqlx::{
    Decode, Encode,
    encode::IsNull,
    error::BoxDynError,
    postgres::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueRef},
    prelude::Type,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Identity {
    Guest,
    Member,
    Vip,
    Admin,
}
impl std::fmt::Display for Identity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.as_str())
    }
}
// 为 Identity 实现 sqlx::Type，指定 PostgreSQL 枚举类型名称
impl Type<sqlx::Postgres> for Identity {
    fn type_info() -> PgTypeInfo {
        // 指定数据库中的枚举类型名称
        PgTypeInfo::with_name("user_level")
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        *ty == Self::type_info() || *ty == PgTypeInfo::with_name("TEXT")
    }
}
// 实现从数据库值到 Identity 的转换
impl<'r> Decode<'r, sqlx::Postgres> for Identity {
    fn decode(value: PgValueRef<'r>) -> anyhow::Result<Self, BoxDynError> {
        // 方法1: 通过字符串解码
        let str_value = match value.format() {
            sqlx::postgres::PgValueFormat::Binary => {
                // 对于二进制格式，需要转换为字符串
                let bytes = value.as_bytes()?;
                std::str::from_utf8(bytes)?
            }
            sqlx::postgres::PgValueFormat::Text => {
                // 对于文本格式，可以直接获取字符串
                value.as_str()?
            }
        };

        match str_value {
            "guest" => Ok(Identity::Guest),
            "member" => Ok(Identity::Member),
            "vip" => Ok(Identity::Vip),
            "admin" => Ok(Identity::Admin),
            _ => Err(format!("Invalid identity value from DB: {}", str_value).into()),
        }
    }
}

// 实现从 Identity 到数据库值的转换
impl Encode<'_, sqlx::Postgres> for Identity {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> anyhow::Result<IsNull, BoxDynError> {
        let str_value = match self {
            Identity::Guest => "guest",
            Identity::Member => "member",
            Identity::Vip => "vip",
            Identity::Admin => "admin",
        };

        // 调用字符串的 encode 方法，它返回 Result<IsNull, BoxDynError>
        <&str as Encode<sqlx::Postgres>>::encode(str_value, buf)
    }

    fn size_hint(&self) -> usize {
        // 返回最大可能的字符串长度
        10
    }
}

// 将 Identity 作为数组参数传递
impl PgHasArrayType for Identity {
    fn array_type_info() -> PgTypeInfo {
        // PostgreSQL 枚举数组的类型名为 "_" + 枚举类型名
        PgTypeInfo::with_name("_user_level")
    }
}
impl Identity {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Identity::Guest => "guest",
            Identity::Member => "member",
            Identity::Vip => "vip",
            Identity::Admin => "admin",
        }
    }
    pub(crate) fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "member" => Identity::Member,
            "vip" => Identity::Vip,
            "admin" => Identity::Admin,
            _ => Identity::Guest,
        }
    }
}

/// 实现自定义的序列化 trait
impl serde::Serialize for Identity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

/// 实现自定义的反序列化 trait
impl<'de> serde::Deserialize<'de> for Identity {
    fn deserialize<D>(deserializer: D) -> anyhow::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Identity::from_str(&s))
    }
}
