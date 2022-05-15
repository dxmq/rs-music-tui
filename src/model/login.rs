#[derive(Clone, Default, PartialEq, Debug)]
pub struct LoginInfo {
    pub phone_input: Vec<char>,
    pub phone_input_idx: usize,
    pub phone_input_cursor_position: u16,
    pub password_input: Vec<char>,
    pub password_input_idx: usize,
    pub password_input_cursor_position: u16,
    pub login_state: LoginState,
    pub cancel_login: bool,
    pub is_login_success: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub enum LoginState {
    NoActive,
    Confirm,
    Cancel,
}

impl Default for LoginState {
    fn default() -> Self {
        Self::NoActive
    }
}

#[derive(Clone, Debug)]
pub struct LoginForm {
    pub phone: String,
    pub password: String,
}
