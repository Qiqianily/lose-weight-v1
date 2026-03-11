use std::sync::LazyLock;

use crate::middlewares::auth::{identity::Identity, principal::Principal};

/// define the default secret
const DEFAULT_SECRET: &str = "WK2953aOagwo2SwUye";
/// define the static JWT for global
static DEFAULT_JWT: LazyLock<JWT> = LazyLock::new(JWT::default);

const EXPIRATION_TIME: u64 = 60 * 60 * 24 * 30;

/// get default global JWT pointer
pub fn get_default_jwt() -> &'static JWT {
    &DEFAULT_JWT
}

/// JWT payload info
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Claims {
    jti: String, // jwt id
    sub: String, // jwt subject user "123:Qiqianily:3:vip",
    aud: String, // audience the jwt receiver
    iss: String, // issuer signatory
    iat: u64,    // issued at
    exp: u64,    // expiration time
}

/// JWT generated and authenticated config
#[derive(Debug)]
pub struct JwtConfig {
    pub secret: std::borrow::Cow<'static, str>, // the secret key
    pub expiration: std::time::Duration,        // expiration time
    pub audience: String,                       // receiver
    pub issuer: String,                         // issuer
}

/// JwtConfig default impl
impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: std::borrow::Cow::Borrowed(DEFAULT_SECRET),
            expiration: std::time::Duration::from_secs(EXPIRATION_TIME),
            audience: "audience".to_string(),
            issuer: "issuer".to_string(),
        }
    }
}
/// JwtAuth generation and authenticated more infos
pub struct JWT {
    encoding_key: jsonwebtoken::EncodingKey, // encoding secret
    decoding_key: jsonwebtoken::DecodingKey, // decoding secret
    header: jsonwebtoken::Header,            // header
    validation: jsonwebtoken::Validation,    // validation rules
    expiration: std::time::Duration,         // expiration time
    audience: String,                        // receiver
    issuer: String,                          // issuer
}

/// JwtAuth new encode and decode methods
impl JWT {
    pub fn new(config: JwtConfig) -> Self {
        let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
        validation.set_audience(&[&config.audience]);
        validation.set_issuer(&[&config.issuer]);
        validation.set_required_spec_claims(&["jti", "sub", "aud", "iss", "iat", "exp"]);
        let secret = config.secret.as_bytes();
        Self {
            encoding_key: jsonwebtoken::EncodingKey::from_secret(secret),
            decoding_key: jsonwebtoken::DecodingKey::from_secret(secret),
            header: jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
            validation,
            expiration: config.expiration,
            audience: config.audience,
            issuer: config.issuer,
        }
    }
    /// encode
    pub fn encode(&self, principal: Principal) -> anyhow::Result<String> {
        // get the current timestamp use jsonwebtoken method
        let current_timestamp = jsonwebtoken::get_current_timestamp();
        // create the claims use principal
        let claims = Claims {
            jti: xid::new().to_string(),
            sub: format!(
                "{}:{}:{}",
                principal.id,
                principal.username,
                principal.identity.as_str()
            ),
            aud: self.audience.clone(),
            iss: self.issuer.clone(),
            iat: current_timestamp,
            exp: current_timestamp.saturating_add(self.expiration.as_secs()),
        };
        // return the encoding result
        Ok(jsonwebtoken::encode(
            &self.header,
            &claims,
            &self.encoding_key,
        )?)
    }

    /// decode token
    ///
    /// # 功能描述
    /// 该函数接收一个 token 字符串引用，对其进行解码出里面的信息 Principal，并判断这个 token 是否过期。
    ///
    /// # 参数
    /// - `token`： token 字符串引用
    ///
    /// # 返回值
    /// 返回 Principal `anyhow::Result<Principal>`
    pub fn decode(&self, token: &str) -> anyhow::Result<Principal> {
        // decoded if had not erred returned the claims
        let claims: Claims =
            jsonwebtoken::decode(token, &self.decoding_key, &self.validation)?.claims;
        // split the claims part
        let mut parts = claims.sub.splitn(3, ":");
        let id_str = parts.next().ok_or_else(|| anyhow::anyhow!("no id"))?;
        let id: i32 = id_str.parse()?;
        let username = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("no name"))?
            .to_string();
        let identity = Identity::from_str(parts.next().unwrap());
        // get principal info
        let principal = Principal {
            id,
            username,
            identity,
        };
        Ok(principal)
    }
}

/// Default for JwtAuth
impl Default for JWT {
    fn default() -> Self {
        Self::new(JwtConfig::default())
    }
}
