use crate::app::{ActiveBlock, App};
use crate::event::Key;
use crate::handlers::common_key_events;
use crate::model::login::LoginState::NoActive;
use crate::model::login::{LoginForm, LoginInfo, LoginState};
use crate::IoEvent;

pub fn phone_input_handler(key: Key, app: &mut App) {
    match key {
        Key::Esc => {
            app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::PhoneBlock));
        }
        Key::Char(c) => {
            app.login_info.phone.push(c);
        }
        Key::Backspace => {
            app.login_info.phone.pop();
        }
        _ => {}
    }
}

pub fn password_input_handler(key: Key, app: &mut App) {
    match key {
        Key::Esc => {
            app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::PasswordBlock));
        }
        Key::Char(c) => {
            app.login_info.password.push(c);
        }
        Key::Backspace => {
            app.login_info.password.pop();
        }
        _ => {}
    }
}
pub fn login_button_handler(key: Key, app: &mut App) {
    let info = app.login_info.clone();
    match key {
        Key::Enter => match info.login_state {
            LoginState::Confirm => {
                login(info.phone, info.password, app);
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

fn login(phone: String, password: String, app: &mut App) {
    app.dispatch(IoEvent::Login(LoginForm { phone, password }));
}
