use std::collections::HashMap;

pub struct JwkEngine {
    /// For publicly listed keys.
    public: HashMap<String, String>,

    /// For confidential clients using HS256/384/512.
    secret: HashMap<(String, String), String>,
}

impl JwkEngine {}
