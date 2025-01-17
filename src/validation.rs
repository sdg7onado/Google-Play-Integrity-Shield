pub fn is_valid_integritytoken(integrity_token: Option<String>) -> bool {
    !is_option_string_empty(integrity_token)
}

pub fn is_valid_appid(app_id: Option<String>) -> bool {
    !is_option_string_empty(app_id)
}

pub fn is_valid_sessionid(session_id: Option<String>) -> bool {
    !is_option_string_empty(session_id)
}

fn is_option_string_empty(value: Option<String>) -> bool {
    match value {
        Some(s) => s.trim().is_empty(),
        None => true,
    }
}
