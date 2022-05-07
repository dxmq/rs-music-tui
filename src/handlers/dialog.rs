use crate::app::ActiveBlock;
use crate::event::Key;
use crate::model::context::DialogContext;
use crate::model::dialog::Dialog as OtherDialog;
use crate::{App, IoEvent};

pub fn handler(key: Key, app: &mut App) {
    if let Some(dialog) = app.dialog.clone() {
        match key {
            Key::Enter => {
                if let Some(route) = app.pop_navigation_stack() {
                    if dialog.confirm {
                        if let ActiveBlock::Dialog(context) = route.active_block {
                            match context {
                                DialogContext::Playlist => {}
                                DialogContext::SubPlaylist => handle_sub_playlist_dialog(app),
                                DialogContext::PlaylistSearch => {}
                            }
                        }
                    }
                }
            }
            Key::Char('q') => {
                app.pop_navigation_stack();
            }
            Key::Right | Key::Left => {
                let dg = OtherDialog {
                    confirm: !app.dialog.clone().unwrap().confirm,
                    ..app.dialog.clone().unwrap()
                };
                app.dialog = Some(dg);
            }
            _ => {}
        }
    }
}

fn handle_sub_playlist_dialog(app: &mut App) {
    if let (Some(playlists), Some(selected_index)) =
        (&app.sub_playlists, app.selected_sub_playlist_index)
    {
        let selected_playlist = &playlists[selected_index];
        let selected_id = selected_playlist.id;
        app.dispatch(IoEvent::ToggleSubscribePlaylist(selected_id))
    }
}
