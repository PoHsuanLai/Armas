#[cfg(target_arch = "wasm32")]
pub mod url {
    use web_sys::window;

    /// Get current hash from URL (e.g., "#/basic/button" -> "/basic/button")
    pub fn get_hash() -> Option<String> {
        window()?
            .location()
            .hash()
            .ok()
            .and_then(|hash| {
                if hash.starts_with('#') {
                    Some(hash[1..].to_string())
                } else {
                    Some(hash)
                }
            })
    }

    /// Set URL hash without triggering page reload
    pub fn set_hash(path: &str) {
        if let Some(window) = window() {
            let location = window.location();
            let hash = if path.is_empty() {
                String::new()
            } else {
                format!("#{}", path)
            };
            let _ = location.set_hash(&hash);
        }
    }

    /// Parse route from hash (e.g., "/basic/button" -> ("basic", "button"))
    pub fn parse_route(hash: &str) -> Option<(String, String)> {
        let path = hash.trim_start_matches('/');
        let parts: Vec<&str> = path.split('/').collect();

        if parts.len() == 2 {
            Some((parts[0].to_string(), parts[1].to_string()))
        } else {
            None
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub mod url {
    // Native stubs (no-op)
    pub fn get_hash() -> Option<String> {
        None
    }
    pub fn set_hash(_path: &str) {}
    pub fn parse_route(_hash: &str) -> Option<(String, String)> {
        None
    }
}
