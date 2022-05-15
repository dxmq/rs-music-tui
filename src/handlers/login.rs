use crate::app::{ActiveBlock, App};
use crate::event::Key;
use crate::handlers::common_key_events;
use crate::model::login::LoginState::NoActive;
use crate::model::login::{LoginForm, LoginInfo, LoginState};
use crate::IoEvent;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

pub fn username_input_handler(key: Key, app: &mut App) {
    match key {
        Key::Ctrl('k') => {
            app.login_info
                .phone_input
                .drain(app.login_info.phone_input_idx..app.login_info.phone_input.len());
        }
        Key::Ctrl('u') => {
            app.login_info
                .phone_input
                .drain(..app.login_info.phone_input_idx);
            app.login_info.phone_input_idx = 0;
            app.login_info.phone_input_cursor_position = 0;
        }
        Key::Ctrl('l') => {
            app.input = vec![];
            app.login_info.phone_input_idx = 0;
            app.login_info.phone_input_cursor_position = 0;
        }
        Key::Ctrl('w') => {
            if app.login_info.phone_input_cursor_position == 0 {
                return;
            }
            let word_end = match app.input[..app.login_info.phone_input_idx]
                .iter()
                .rposition(|&x| x != ' ')
            {
                Some(index) => index + 1,
                None => 0,
            };
            let word_start = match app.input[..word_end].iter().rposition(|&x| x == ' ') {
                Some(index) => index + 1,
                None => 0,
            };
            let deleted: String = app.input[word_start..app.login_info.phone_input_idx]
                .iter()
                .collect();
            let deleted_len: u16 = UnicodeWidthStr::width(deleted.as_str()).try_into().unwrap();
            app.login_info
                .phone_input
                .drain(word_start..app.login_info.phone_input_idx);
            app.login_info.phone_input_idx = word_start;
            app.login_info.phone_input_cursor_position -= deleted_len;
        }
        Key::End | Key::Ctrl('e') => {
            app.login_info.phone_input_idx = app.login_info.phone_input.len();
            let input_string: String = app.login_info.phone_input.iter().collect();
            app.login_info.phone_input_cursor_position =
                UnicodeWidthStr::width(input_string.as_str())
                    .try_into()
                    .unwrap();
        }
        Key::Home | Key::Ctrl('a') => {
            app.login_info.phone_input_idx = 0;
            app.login_info.phone_input_cursor_position = 0;
        }
        Key::Left | Key::Ctrl('b') => {
            if !app.login_info.phone_input.is_empty() && app.login_info.phone_input_idx > 0 {
                let last_c = app.input[app.login_info.phone_input_idx - 1];
                app.login_info.phone_input_idx -= 1;
                app.login_info.phone_input_cursor_position -= compute_character_width(last_c);
            }
        }
        Key::Right | Key::Ctrl('f') => {
            if app.login_info.phone_input_idx < app.login_info.phone_input.len() {
                let next_c = app.input[app.login_info.phone_input_idx];
                app.login_info.phone_input_idx += 1;
                app.login_info.phone_input_cursor_position += compute_character_width(next_c);
            }
        }
        Key::Esc => {
            app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::PhoneBlock));
        }
        Key::Char(c) => {
            app.login_info
                .phone_input
                .insert(app.login_info.phone_input_idx, c);
            app.login_info.phone_input_idx += 1;
            app.login_info.phone_input_cursor_position += compute_character_width(c);
        }
        Key::Backspace | Key::Ctrl('h') => {
            if !app.login_info.phone_input.is_empty() && app.login_info.phone_input_idx > 0 {
                let last_c = app
                    .login_info
                    .phone_input
                    .remove(app.login_info.phone_input_idx - 1);
                app.login_info.phone_input_idx -= 1;
                app.login_info.phone_input_cursor_position -= compute_character_width(last_c);
            }
        }
        Key::Delete | Key::Ctrl('d') => {
            if !app.login_info.phone_input.is_empty()
                && app.login_info.phone_input_idx < app.login_info.phone_input.len()
            {
                app.login_info
                    .phone_input
                    .remove(app.login_info.phone_input_idx);
            }
        }
        _ => {}
    }
}

pub fn password_input_handler(key: Key, app: &mut App) {
    match key {
        Key::Ctrl('k') => {
            app.login_info
                .password_input
                .drain(app.login_info.password_input_idx..app.login_info.password_input.len());
        }
        Key::Ctrl('u') => {
            app.login_info
                .password_input
                .drain(..app.login_info.password_input_idx);
            app.login_info.password_input_idx = 0;
            app.login_info.password_input_cursor_position = 0;
        }
        Key::Ctrl('l') => {
            app.input = vec![];
            app.login_info.password_input_idx = 0;
            app.login_info.password_input_cursor_position = 0;
        }
        Key::Ctrl('w') => {
            if app.login_info.password_input_cursor_position == 0 {
                return;
            }
            let word_end = match app.input[..app.login_info.password_input_idx]
                .iter()
                .rposition(|&x| x != ' ')
            {
                Some(index) => index + 1,
                None => 0,
            };
            let word_start = match app.input[..word_end].iter().rposition(|&x| x == ' ') {
                Some(index) => index + 1,
                None => 0,
            };
            let deleted: String = app.input[word_start..app.login_info.password_input_idx]
                .iter()
                .collect();
            let deleted_len: u16 = UnicodeWidthStr::width(deleted.as_str()).try_into().unwrap();
            app.login_info
                .password_input
                .drain(word_start..app.login_info.password_input_idx);
            app.login_info.password_input_idx = word_start;
            app.login_info.password_input_cursor_position -= deleted_len;
        }
        Key::End | Key::Ctrl('e') => {
            app.login_info.password_input_idx = app.login_info.password_input.len();
            let input_string: String = app.login_info.password_input.iter().collect();
            app.login_info.password_input_cursor_position =
                UnicodeWidthStr::width(input_string.as_str())
                    .try_into()
                    .unwrap();
        }
        Key::Home | Key::Ctrl('a') => {
            app.login_info.password_input_idx = 0;
            app.login_info.password_input_cursor_position = 0;
        }
        Key::Left | Key::Ctrl('b') => {
            if !app.login_info.password_input.is_empty() && app.login_info.password_input_idx > 0 {
                let last_c = app.input[app.login_info.password_input_idx - 1];
                app.login_info.password_input_idx -= 1;
                app.login_info.password_input_cursor_position -= compute_character_width(last_c);
            }
        }
        Key::Right | Key::Ctrl('f') => {
            if app.login_info.password_input_idx < app.login_info.password_input.len() {
                let next_c = app.input[app.login_info.password_input_idx];
                app.login_info.password_input_idx += 1;
                app.login_info.password_input_cursor_position += compute_character_width(next_c);
            }
        }
        Key::Esc => {
            app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::PasswordBlock));
        }
        Key::Char(c) => {
            app.login_info
                .password_input
                .insert(app.login_info.password_input_idx, c);
            app.login_info.password_input_idx += 1;
            app.login_info.password_input_cursor_position += compute_character_width(c);
        }
        Key::Backspace | Key::Ctrl('h') => {
            if !app.login_info.password_input.is_empty() && app.login_info.password_input_idx > 0 {
                let last_c = app
                    .login_info
                    .password_input
                    .remove(app.login_info.password_input_idx - 1);
                app.login_info.password_input_idx -= 1;
                app.login_info.password_input_cursor_position -= compute_character_width(last_c);
            }
        }
        Key::Delete | Key::Ctrl('d') => {
            if !app.login_info.password_input.is_empty()
                && app.login_info.password_input_idx < app.login_info.password_input.len()
            {
                app.login_info
                    .password_input
                    .remove(app.login_info.password_input_idx);
            }
        }
        _ => {}
    }
}
pub fn login_button_handler(key: Key, app: &mut App) {
    let info = app.login_info.clone();
    match key {
        Key::Enter => match info.login_state {
            LoginState::Confirm => {
                let username: String = info.phone_input.iter().collect();
                let password: String = info.password_input.iter().collect();
                login(username, password, app);
            }
            LoginState::Cancel => {
                app.login_info.cancel_login = true;
            }
            _ => {}
        },
        Key::Char('q') => {
            app.pop_navigation_stack();
        }
        k if common_key_events::up_event(k) => {
            app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::PasswordBlock));
            app.login_info.login_state = NoActive;
        }
        Key::Right | Key::Left => {
            let login_info = LoginInfo {
                login_state: match info.login_state {
                    LoginState::Confirm | NoActive => LoginState::Cancel,
                    LoginState::Cancel => LoginState::Confirm,
                },
                ..info
            };
            app.login_info = login_info;
        }
        _ => {}
    }
}

fn compute_character_width(character: char) -> u16 {
    UnicodeWidthChar::width(character)
        .unwrap()
        .try_into()
        .unwrap()
}

fn login(username: String, password: String, app: &mut App) {
    app.dispatch(IoEvent::Login(LoginForm {
        phone: username,
        password,
    }));
}
