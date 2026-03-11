use crate::response::ApiResult;
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::phc::SaltString,
};

/// 使用 argon2 对密码进行加密。
///
/// # 功能描述
/// 该函数接收一个原始密码字符串，并使用 argon2 算法对其进行加密处理，返回加密后的密码字符串。
/// 此过程是不可逆的，意味着你不能从加密后的密码中解密出原始密码。
///
/// # 参数
/// - `password`: 需要进行加密的原始密码。必须是非空字符串。
///
/// # 返回值
/// 如果加密成功，则返回包含加密后密码字符串的 `ApiResult<String>`。
/// 如果加密过程中出现错误（例如，内部算法执行失败），则返回相应的错误信息。
///
/// # 示例
/// ```
/// let hashed_password = encode_password("123456").unwrap();
/// println!("Hashed password: {}", hashed_password);
/// ```
pub fn encode_password(password: &str) -> ApiResult<String> {
    // 生成随机 22 位 salt
    // let salt = SaltString::try_from_rng(&mut OsRng).unwrap();
    let salt = SaltString::generate();
    // let salt = SaltString::from_rng(&mut OsRng);
    // 使用默认参数生成 hash 密码
    let argon2 = Argon2::default();
    // 生成 hash 密码
    let password_hash = argon2
        .hash_password_with_salt(password.as_bytes(), salt.as_bytes())?
        .to_string();
    // 返回 hash 后的结果 97 位
    Ok(password_hash)
}

/// 验证原始密码与加密后的密码是否匹配。
///
/// # 功能描述
/// 该函数接收一个原始密码和一个使用 argon2 算法加密后的密码哈希，并验证原始密码与加密后的密码哈希是否匹配。
/// 此过程是安全的，确保了只有知道正确密码的用户才能通过验证。
///
/// # 参数
/// - `password`: 需要验证的原始密码。必须是非空字符串。
/// - `password_hash`: 已经加密过的密码哈希，用于对比验证。必须是非空字符串且应该是之前通过 `encode_password` 或类似方法生成的。
///
/// # 返回值
/// 如果原始密码与加密后的密码哈希匹配，则返回 `Ok(true)`；
/// 如果不匹配，则返回 `Ok(false)`；
/// 如果在验证过程中出现错误（例如，内部算法执行失败），则返回相应的错误信息。
///
/// # 示例
/// ```
/// let hashed_password = encode_password("my_secure_password").unwrap();
/// let is_valid = verify_password("my_secure_password", &hashed_password).unwrap();
/// assert!(is_valid);
/// ```
pub fn verify_password(password: &str, password_hash: &str) -> ApiResult<bool> {
    // 使用默认参数验证密码
    let argon2 = Argon2::default();
    // 解析 hash 密码
    let parsed_hash = PasswordHash::new(password_hash)?;
    // 比对密码是否一样
    let verify_result = argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();
    // 返回验证的结果
    Ok(verify_result)
}
