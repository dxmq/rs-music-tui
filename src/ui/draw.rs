use pad::{Alignment as PadAlignment, PadStr};
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::canvas::{Canvas, Map, MapResolution};
use tui::widgets::{
    Block, Borders, Clear, Gauge, List, ListItem, ListState, Paragraph, Row, Table, Wrap,
};
use tui::Frame;

use crate::app::{ActiveBlock, App, RouteId, LIBRARY_OPTIONS};
use crate::cli::clap::BANNER;
use crate::handlers::search::SearchResultBlock;
use crate::model::album::AlbumUi;
use crate::model::artist::ArtistBlock;
use crate::model::enums::RepeatState;
use crate::model::login::LoginState;
use crate::model::table::{ColumnId, TableHeader, TableHeaderItem, TableId, TableItem};
use crate::ui::help::get_help_docs;
use crate::util;
use crate::util::{
    create_artist_string, display_track_progress, get_artist_highlight_state, get_color,
    get_percentage_width, get_search_results_highlight_state, get_track_progress_percentage,
    millis_to_minutes2, BASIC_VIEW_HEIGHT, SMALL_TERMINAL_WIDTH,
};

pub fn draw_main_layout<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let margin = util::get_main_layout_margin(app);
    if app.size.width >= SMALL_TERMINAL_WIDTH && !app.user_config.behavior.enforce_wide_search_bar {
        let parent_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(6)].as_ref())
            .margin(margin)
            .split(f.size());
        draw_routes(f, app, parent_layout[0]);

        draw_playbar(f, app, parent_layout[1]);
    } else {
        let parent_layout = Layout::default()
            .margin(margin)
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(6),
                ]
                .as_ref(),
            )
            .split(f.size());

        draw_input_and_help_box(f, app, parent_layout[0]);

        draw_routes(f, app, parent_layout[1]);

        draw_playbar(f, app, parent_layout[2]);
    }

    // Possibly draw confirm dialog
    draw_dialog(f, app);
}

pub fn draw_login_page<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::PhoneBlock,
        current_route.hovered_block == ActiveBlock::PhoneBlock,
    );
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .margin(10)
        .split(f.size());
    let text = vec![Spans::from(Span::styled(
        "??????",
        Style::default().add_modifier(Modifier::BOLD),
    ))];
    let text = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);

    f.render_widget(text, chunks[0]);

    let input_string: String = app.login_info.phone.clone();

    let line = Text::from((&input_string).as_str());
    let phone_input = Paragraph::new(line).block(
        Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(
                "?????????",
                get_color(highlight_state, app.user_config.theme),
            ))
            .border_style(get_color(highlight_state, app.user_config.theme)),
    );
    f.render_widget(phone_input, chunks[1]);

    let password_input_string: String = app.login_info.password.clone();
    let password_line = Text::from((&password_input_string).as_str());
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::PasswordBlock,
        current_route.hovered_block == ActiveBlock::PasswordBlock,
    );
    let password_input = Paragraph::new(password_line).block(
        Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(
                "??????",
                get_color(highlight_state, app.user_config.theme),
            ))
            .border_style(get_color(highlight_state, app.user_config.theme)),
    );
    f.render_widget(password_input, chunks[2]);

    let hchunks = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(3)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
        .split(chunks[3]);

    let ok_text = Span::raw("??????");

    let ok = Paragraph::new(ok_text)
        .style(match &app.login_info.login_state {
            LoginState::NoActive | LoginState::Cancel => {
                Style::default().fg(app.user_config.theme.inactive)
            }
            LoginState::Confirm => Style::default().fg(app.user_config.theme.active),
        })
        .alignment(Alignment::Center);

    f.render_widget(ok, hchunks[0]);

    let cancel_text = Span::raw("??????");
    let cancel = Paragraph::new(cancel_text)
        .style(match &app.login_info.login_state {
            LoginState::NoActive => Style::default().fg(app.user_config.theme.inactive),
            LoginState::Confirm => Style::default().fg(app.user_config.theme.inactive),
            LoginState::Cancel => Style::default().fg(app.user_config.theme.active),
        })
        .alignment(Alignment::Center);

    f.render_widget(cancel, hchunks[1]);
}

pub fn draw_dialog<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    if let ActiveBlock::Dialog(_) = app.get_current_route().active_block {
        if let Some(dialog) = app.dialog.as_ref() {
            let bounds = f.size();
            // maybe do this better
            let width = std::cmp::min(bounds.width - 2, 45);
            let height = 8;
            let left = (bounds.width - width) / 2;
            let top = bounds.height / 4;

            let rect = Rect::new(left, top, width, height);

            f.render_widget(Clear, rect);

            // ??????????????????
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.user_config.theme.inactive));

            f.render_widget(block, rect);

            let vchunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref())
                .split(rect);

            let text = vec![
                Spans::from(Span::raw(dialog.tips.clone())),
                Spans::from(Span::styled(
                    &dialog.item_name,
                    Style::default().add_modifier(Modifier::BOLD),
                )),
                Spans::from(Span::raw("???")),
            ];
            let text = Paragraph::new(text)
                .wrap(Wrap { trim: true })
                .alignment(Alignment::Center);

            // ???????????????
            f.render_widget(text, vchunks[0]);

            let hchunks = Layout::default()
                .direction(Direction::Horizontal)
                .horizontal_margin(3)
                .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
                .split(vchunks[1]);
            let ok_text = Span::raw("??????");
            let ok = Paragraph::new(ok_text)
                .style(Style::default().fg(if dialog.confirm {
                    app.user_config.theme.active
                } else {
                    app.user_config.theme.inactive
                }))
                .alignment(Alignment::Center);

            // Ok radio
            f.render_widget(ok, hchunks[0]);

            let cancel_text = Span::raw("??????");
            let cancel = Paragraph::new(cancel_text)
                .style(Style::default().fg(if dialog.confirm {
                    app.user_config.theme.inactive
                } else {
                    app.user_config.theme.active
                }))
                .alignment(Alignment::Center);

            // Cancel radio
            f.render_widget(cancel, hchunks[1]);
        }
    }
}

pub fn draw_basic_view<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    // If space is negative, do nothing because the widget would not fit
    if let Some(s) = app.size.height.checked_sub(BASIC_VIEW_HEIGHT) {
        let space = s / 2;
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(space),
                    Constraint::Length(BASIC_VIEW_HEIGHT),
                    Constraint::Length(space),
                ]
                .as_ref(),
            )
            .split(f.size());

        draw_playbar(f, app, chunks[1]);
    }
}
pub fn draw_error_screen<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(5)
        .split(f.size());

    let playing_text = vec![
        Spans::from(vec![
            Span::raw("Api response: "),
            Span::styled(
                &app.api_error,
                Style::default().fg(app.user_config.theme.error_text),
            ),
        ]),
        Spans::from(Span::styled(
            "\nPress <Esc> to return",
            Style::default().fg(app.user_config.theme.inactive),
        )),
    ];

    let playing_paragraph = Paragraph::new(playing_text)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(app.user_config.theme.error_text))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    "Error",
                    Style::default().fg(app.user_config.theme.error_border),
                ))
                .border_style(Style::default().fg(app.user_config.theme.error_border)),
        );
    f.render_widget(playing_paragraph, chunks[0]);
}

pub fn draw_playbar<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(layout_chunk);

    let playbar = Block::default()
        .title(Span::styled("", Style::default().fg(Color::Reset)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Reset));
    f.render_widget(playbar, chunks[0]);

    if let Some(current_playback_context) = &app.current_playback_context {
        if let Some(track_item) = &current_playback_context.item {
            let play_title = if current_playback_context.is_playing {
                "Playing"
            } else {
                "Paused"
            };
            let play_state_text = match current_playback_context.repeat_state {
                RepeatState::Off => "=",
                RepeatState::Track => "O1",
                RepeatState::Context => "O",
                RepeatState::Shuffle => "X",
            };
            let title = format!(
                "{:-7} {:-1}% {:-1} ",
                play_title,
                (app.volume * 100f32).ceil(), // current_playback_context.device.volume_percent
                play_state_text,
            );

            let current_route = app.get_current_route();
            let highlight_state = (
                current_route.active_block == ActiveBlock::PlayBar,
                current_route.hovered_block == ActiveBlock::PlayBar,
            );

            let title_block = Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    &title,
                    get_color(highlight_state, app.user_config.theme),
                ))
                .border_style(get_color(highlight_state, app.user_config.theme));

            f.render_widget(title_block, layout_chunk);

            let (item_id, duration_ms, name) = (
                track_item.id,
                track_item.duration as u32,
                track_item.name.clone(),
            );
            let track_name = if app.liked_track_ids_set.contains(&item_id) {
                format!("{}{}", &app.user_config.padded_liked_icon(), name)
            } else {
                name
            };

            let play_bar_text = create_artist_string(&track_item.artists);

            let content = format!("{} | {}", track_name, play_bar_text);

            let mut lines: Text = Text::from(Span::styled(
                "",
                Style::default().fg(app.user_config.theme.playbar_text),
            ));
            if app.is_show_playbar_lyric {
                let lyric_index = app.lyric_index;
                let lyrics = &app.lyric;
                let mut lyric_line = "";
                if let Some(context) = &app.current_playback_context {
                    if context.is_playing {
                        if let Some(lyrics) = lyrics {
                            if let Some(lyric) = lyrics.get(lyric_index) {
                                lyric_line = &lyric.lyric;
                            }
                        }
                    }
                }
                lines = Text::from(Span::styled(
                    lyric_line,
                    Style::default().fg(app.user_config.theme.playbar_text),
                ));
            }

            let artist = Paragraph::new(lines)
                .style(Style::default().fg(app.user_config.theme.playbar_text))
                .alignment(Alignment::Center)
                .block(
                    Block::default().title(Span::styled(
                        &content,
                        Style::default()
                            .fg(app.user_config.theme.selected)
                            .add_modifier(Modifier::BOLD),
                    )),
                );
            f.render_widget(artist, chunks[0]);

            let progress_ms = match app.seek_ms {
                Some(seek_ms) => seek_ms,
                None => app.song_progress_ms,
            };

            let perc = get_track_progress_percentage(progress_ms, duration_ms);

            let song_progress_label = display_track_progress(progress_ms, duration_ms);
            let modifier = if app.user_config.behavior.enable_text_emphasis {
                Modifier::ITALIC | Modifier::BOLD
            } else {
                Modifier::empty()
            };
            let song_progress = Gauge::default()
                .gauge_style(
                    Style::default()
                        .fg(app.user_config.theme.playbar_progress)
                        .bg(app.user_config.theme.playbar_background)
                        .add_modifier(modifier),
                )
                .percent(perc)
                .label(Span::styled(
                    &song_progress_label,
                    Style::default().fg(app.user_config.theme.playbar_progress_text),
                ));
            f.render_widget(song_progress, chunks[2]);
        }
    }
}

pub fn draw_routes<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(layout_chunk);

    draw_user_block(f, app, chunks[0]);

    let current_route = app.get_current_route();
    match current_route.id {
        RouteId::Search => {
            draw_search_results(f, app, chunks[1]);
        }
        RouteId::TrackTable => {
            draw_song_table(f, app, chunks[1]);
        }
        RouteId::Home => {
            draw_home(f, app, chunks[1]);
        }
        RouteId::Lyric => {
            // draw playing lyric ui
            draw_lyric(f, app, chunks[1]);
        }
        RouteId::Artists => {
            draw_artist_table(f, app, chunks[1]);
        }
        RouteId::ArtistDetail => {
            draw_artist_detail_table(f, app, chunks[1]);
        }
        RouteId::AlbumTracks => {
            draw_album_detail_table(f, app, chunks[1]);
        }
        RouteId::PhoneBlock => {}
        RouteId::PasswordBlock => {}
        RouteId::LoginButton => {}
        RouteId::Error => {}
        RouteId::BasicView => {}
        RouteId::Dialog => {}
    }
}

pub fn draw_album_detail_table<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let header = TableHeader {
        id: TableId::Album,
        items: vec![
            TableHeaderItem {
                id: ColumnId::Liked,
                text: "",
                width: 2,
            },
            TableHeaderItem {
                id: ColumnId::Title,
                text: "??????",
                width: get_percentage_width(layout_chunk.width, 2.0 / 5.0) - 5,
            },
            TableHeaderItem {
                text: "??????",
                width: get_percentage_width(layout_chunk.width, 2.0 / 5.0),
                ..Default::default()
            },
            TableHeaderItem {
                text: "??????",
                width: get_percentage_width(layout_chunk.width, 1.0 / 5.0),
                ..Default::default()
            },
        ],
    };

    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::AlbumTracks,
        current_route.hovered_block == ActiveBlock::AlbumTracks,
    );
    let album_ui = app.album_detail.as_ref().map(|selected_album| AlbumUi {
        items: selected_album
            .tracks
            .iter()
            .map(|item| TableItem {
                id: item.id,
                fee: 0,
                format: vec![
                    "".to_string(),
                    item.name.to_owned(),
                    create_artist_string(&item.artists),
                    millis_to_minutes2(item.duration),
                ],
            })
            .collect::<Vec<TableItem>>(),
        title: format!(
            "{} | {}",
            create_artist_string(&[selected_album.album.artist.clone()]),
            selected_album
                .album
                .name
                .clone()
                .unwrap_or_else(|| "".to_string()),
        ),
        selected_index: selected_album.selected_track_index,
    });

    if let Some(album_ui) = album_ui {
        draw_table(
            f,
            app,
            layout_chunk,
            (&album_ui.title, &header),
            &album_ui.items,
            album_ui.selected_index,
            highlight_state,
        );
    };
}
pub fn draw_artist_detail_table<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(layout_chunk);

    if let Some(artist_detail) = &app.artist_detail {
        let tracks = artist_detail
            .tracks
            .iter()
            .map(|track| {
                let mut song_name = "".to_string();
                if let Some(context) = &app.current_playback_context {
                    let mut track_id = 0;
                    if let Some(track) = &context.item {
                        track_id = track.id
                    };
                    if track_id == track.id {
                        song_name.push_str("??? ");
                    }
                };
                if app.liked_track_ids_set.contains(&track.id) {
                    song_name += &app.user_config.padded_liked_icon();
                }
                song_name.push_str(&track.name);
                song_name
            })
            .collect::<Vec<String>>();

        draw_selectable_list(
            f,
            app,
            chunks[0],
            &format!("{} - ??????", &artist_detail.artist_name),
            &tracks,
            get_artist_highlight_state(app, ArtistBlock::Tracks),
            Some(artist_detail.selected_track_index),
        );

        let albums = &artist_detail
            .albums
            .iter()
            .map(|item| {
                let mut album_artist = String::new();
                album_artist.push_str(&format!(
                    "{} - {}",
                    item.name.clone().unwrap(),
                    create_artist_string(&[item.artist.clone()]),
                ));
                album_artist
            })
            .collect::<Vec<String>>();

        draw_selectable_list(
            f,
            app,
            chunks[1],
            "??????",
            albums,
            get_artist_highlight_state(app, ArtistBlock::Albums),
            Some(artist_detail.selected_album_index),
        );

        let simi_artists = artist_detail
            .simi_artists
            .iter()
            .map(|item| {
                let mut artist = String::new();
                if app.artist_sub_ids_set.contains(&item.id) {
                    artist.push_str(&app.user_config.padded_liked_icon());
                }
                artist.push_str(&item.name.clone().unwrap());
                artist
            })
            .collect::<Vec<String>>();

        draw_selectable_list(
            f,
            app,
            chunks[2],
            "????????????",
            &simi_artists,
            get_artist_highlight_state(app, ArtistBlock::SimiArtists),
            Some(artist_detail.selected_simi_artist_index),
        );
    }
}

pub fn draw_artist_table<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let header = TableHeader {
        id: TableId::Artist,
        items: vec![TableHeaderItem {
            text: "??????",
            width: get_percentage_width(layout_chunk.width, 1.0),
            ..Default::default()
        }],
    };

    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::Artists,
        current_route.hovered_block == ActiveBlock::Artists,
    );

    let items: Vec<TableItem> = app
        .artists
        .iter()
        .map(|item| TableItem {
            id: item.id,
            fee: 0,
            format: vec![item.name.clone().unwrap_or("".to_string())],
        })
        .collect();

    draw_table(
        f,
        app,
        layout_chunk,
        ("", &header),
        &items,
        app.artists_selected_index,
        highlight_state,
    );
}

pub fn draw_search_results<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(layout_chunk);

    // ???????????????block
    {
        let song_artist_block = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[0]);

        let currently_playing_id: usize = app
            .current_playback_context
            .clone()
            .and_then(|context| context.item.map(|item| item.id))
            .unwrap_or(0);

        let songs = match &app.search_results.tracks {
            Some(tracks) => tracks
                .iter()
                .map(|item| {
                    let mut song_name = "".to_string();
                    let id = item.clone().id;
                    if currently_playing_id == id {
                        song_name += "??? "
                    }
                    if app.liked_track_ids_set.contains(&id) {
                        song_name += &app.user_config.padded_liked_icon();
                    }

                    song_name += &item.name;
                    song_name += &format!(" - {}", &create_artist_string(&item.artists));
                    song_name
                })
                .collect(),
            None => vec![],
        };

        draw_selectable_list(
            f,
            app,
            song_artist_block[0],
            "??????",
            &songs,
            get_search_results_highlight_state(app, SearchResultBlock::TrackSearch),
            app.search_results.selected_tracks_index,
        );

        let artists = match &app.search_results.artists {
            Some(artists) => artists
                .iter()
                .map(|item| {
                    let mut artist = String::new();
                    if app.artist_sub_ids_set.contains(&item.id) {
                        artist.push_str(&app.user_config.padded_liked_icon());
                    }
                    let artist_name = item.name.clone().unwrap();
                    artist.push_str(&artist_name);
                    artist
                })
                .collect(),
            None => vec![],
        };

        draw_selectable_list(
            f,
            app,
            song_artist_block[1],
            "??????",
            &artists,
            get_search_results_highlight_state(app, SearchResultBlock::ArtistSearch),
            app.search_results.selected_artists_index,
        );
    }

    // ???????????????block
    {
        let albums_playlist_block = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[1]);

        let albums = match &app.search_results.albums {
            Some(albums) => albums
                .iter()
                .map(|item| {
                    let mut album_artist = String::new();
                    // if let Some(album_id) = &item.id {
                    //     if app.saved_album_ids_set.contains(&album_id.to_owned()) {
                    //         album_artist.push_str(&app.user_config.padded_liked_icon());
                    //     }
                    // }
                    let artists = vec![item.artist.clone()];
                    album_artist.push_str(&format!(
                        "{} - {}",
                        item.clone().name.unwrap(),
                        create_artist_string(&artists),
                    ));
                    album_artist
                })
                .collect(),
            None => vec![],
        };

        draw_selectable_list(
            f,
            app,
            albums_playlist_block[0],
            "??????",
            &albums,
            get_search_results_highlight_state(app, SearchResultBlock::AlbumSearch),
            app.search_results.selected_album_index,
        );

        let sub_playlists = &app.sub_playlists.clone().unwrap();
        let sub_ids = sub_playlists
            .iter()
            .map(|item| item.id)
            .collect::<Vec<usize>>();
        let playlists = match &app.search_results.playlists {
            Some(playlists) => playlists
                .iter()
                .map(|item| {
                    let mut playlist_str = "".to_string();
                    if sub_ids.contains(&item.id) {
                        playlist_str += &app.user_config.padded_liked_icon();
                    }
                    playlist_str += &item.name;
                    playlist_str
                })
                .collect(),
            None => vec![],
        };

        draw_selectable_list(
            f,
            app,
            albums_playlist_block[1],
            "??????",
            &playlists,
            get_search_results_highlight_state(app, SearchResultBlock::PlaylistSearch),
            app.search_results.selected_playlists_index,
        );
    }
}

pub fn draw_map<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::Lyric,
        current_route.hovered_block == ActiveBlock::Lyric,
    );
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
                .title("")
                .style(Style::default().fg(Color::LightYellow))
                .border_style(get_color(highlight_state, app.user_config.theme)),
        )
        .paint(|ctx| {
            ctx.draw(&Map {
                color: Color::LightYellow,
                resolution: MapResolution::High,
            });
        })
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0]);
    f.render_widget(canvas, layout_chunk);
}

pub fn draw_lyric<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let lyric_items = match &app.lyric {
        Some(l) => l
            .iter()
            .map(|item| {
                vec![item
                    .lyric
                    .pad_to_width_with_alignment(10, PadAlignment::Left)
                    .pad_to_width_with_alignment(30, PadAlignment::Middle)]
            })
            .collect(),
        None => vec![],
    };
    // ????????????????????????????????????????????????
    if lyric_items.is_empty() {
        draw_map(f, app, layout_chunk);
        return;
    }
    // ??????????????????map???????????????????????????????????????65%??????map???35%????????????
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)].as_ref())
        .split(layout_chunk);
    // ??????map
    draw_map(f, app, chunks[0]);

    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::Lyric,
        current_route.hovered_block == ActiveBlock::Lyric,
    );
    let selected_index = app.lyric_index;

    let interval = (layout_chunk.height / 2) as usize;
    let (row_items, margin) = if !lyric_items.is_empty() {
        let count = (layout_chunk.height - 4) as usize;
        let total = lyric_items.len();
        if selected_index >= count - interval && total > count {
            if selected_index >= total - interval {
                let margin = total - count;
                (&lyric_items[margin..], margin)
            } else {
                let margin = selected_index + interval - count;
                (&lyric_items[margin..], margin)
            }
        } else {
            (lyric_items.as_ref(), 0_usize)
        }
    } else {
        (lyric_items.as_ref(), 0_usize)
    };

    let header = TableHeader {
        id: TableId::Lyric,
        items: vec![TableHeaderItem {
            id: ColumnId::Title,
            text: "",
            width: get_percentage_width(layout_chunk.width, 0.5),
        }],
    };
    let selected_style = Style::default().fg(Color::Rgb(18, 150, 136));
    let rows = row_items.iter().enumerate().map(|(i, item)| {
        let mut style = Style::default().fg(Color::Blue); // default styling
        if i == selected_index - margin {
            style = selected_style;
        }
        Row::new(item.clone()).style(style)
    });

    let widths = header
        .items
        .iter()
        .map(|h| Constraint::Length(h.width))
        .collect::<Vec<Constraint>>();

    let header = Row::new(header.items.iter().map(|h| h.text))
        .style(Style::default().fg(app.user_config.theme.header));
    // ????????????table
    let table = Table::new(rows)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::RIGHT | Borders::TOP | Borders::BOTTOM)
                .style(Style::default().fg(Color::Blue))
                .border_style(get_color(highlight_state, app.user_config.theme)),
        )
        .style(Style::default().fg(Color::Blue))
        .column_spacing(1)
        .widths(&widths);
    f.render_widget(table, chunks[1]);
}

pub fn draw_song_table<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let header = TableHeader {
        id: TableId::Song,
        items: vec![
            TableHeaderItem {
                id: ColumnId::Liked,
                text: "",
                width: 2,
            },
            TableHeaderItem {
                id: ColumnId::Title,
                text: "??????",
                width: get_percentage_width(layout_chunk.width, 0.3),
            },
            TableHeaderItem {
                text: "??????",
                width: get_percentage_width(layout_chunk.width, 0.3),
                ..Default::default()
            },
            TableHeaderItem {
                text: "??????",
                width: get_percentage_width(layout_chunk.width, 0.3),
                ..Default::default()
            },
            TableHeaderItem {
                text: "??????",
                width: get_percentage_width(layout_chunk.width, 0.1),
                ..Default::default()
            },
        ],
    };

    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::TrackTable,
        current_route.hovered_block == ActiveBlock::TrackTable,
    );
    let items = app
        .track_table
        .tracks
        .iter()
        .map(|item| TableItem {
            id: item.id,
            fee: item.fee,
            format: vec![
                "".to_string(),
                item.name.to_owned(),
                create_artist_string(&item.artists),
                item.album.name.to_owned().unwrap(),
                millis_to_minutes2(item.duration),
            ],
        })
        .collect::<Vec<TableItem>>();
    // let items = vec![];
    draw_table(
        f,
        app,
        layout_chunk,
        (&app.title, &header),
        &items,
        app.track_table.selected_index,
        highlight_state,
    )
}

pub fn draw_home<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Length(93)].as_ref())
        .margin(2)
        .split(layout_chunk);

    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::Home,
        current_route.hovered_block == ActiveBlock::Home,
    );
    let welcome = Block::default()
        .title(Span::styled(
            "??????!",
            get_color(highlight_state, app.user_config.theme),
        ))
        .borders(Borders::ALL)
        .border_style(get_color(highlight_state, app.user_config.theme));
    f.render_widget(welcome, layout_chunk);

    let mut top_text = Text::from(BANNER);
    top_text.patch_style(Style::default().fg(app.user_config.theme.banner));

    let top_text = Paragraph::new(top_text)
        .style(Style::default().fg(app.user_config.theme.text))
        .alignment(Alignment::Center)
        .block(Block::default());
    f.render_widget(top_text, chunks[0]);

    let mut usage: Vec<Spans> = vec![Spans::from(Span::styled(
        "",
        Style::default().fg(Color::Yellow),
    ))];

    let docs = get_help_docs(&app.user_config.keys);
    for x in &docs {
        usage.push(Spans::from(vec![
            Span::styled(
                x.get(0)
                    .unwrap()
                    .pad_to_width_with_alignment(58, pad::Alignment::Left),
                Style::default().fg(Color::Magenta),
            ),
            Span::styled(
                x.get(1)
                    .unwrap()
                    .pad_to_width_with_alignment(30, pad::Alignment::Left),
                Style::default().fg(Color::Yellow),
            ),
        ]))
    }

    f.render_widget(
        Paragraph::new(usage)
            .style(Style::default().add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .scroll((app.home_scroll, 0)),
        chunks[1],
    );
}

pub fn draw_user_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    // Check for width to make a responsive layout
    if app.size.width >= SMALL_TERMINAL_WIDTH && !app.user_config.behavior.enforce_wide_search_bar {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Percentage(30),
                    Constraint::Percentage(70),
                ]
                .as_ref(),
            )
            .split(layout_chunk);

        // Search input and help
        draw_input_and_help_box(f, app, chunks[0]);
        draw_library_block(f, app, chunks[1]);
        draw_playlist_block(f, app, chunks[2]);
    } else {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(layout_chunk);

        draw_library_block(f, app, chunks[0]);
        draw_playlist_block(f, app, chunks[1]);
    }
}

pub fn draw_playlist_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(layout_chunk);

    let current_route = app.get_current_route();
    let my_playlists_highlight_state = (
        current_route.active_block == ActiveBlock::MyPlaylists,
        current_route.hovered_block == ActiveBlock::MyPlaylists,
    );
    let subscribed_playlists_highlight_state = (
        current_route.active_block == ActiveBlock::SubscribedPlaylists,
        current_route.hovered_block == ActiveBlock::SubscribedPlaylists,
    );
    let my_playlist_items = match &app.playlists {
        Some(list) => list.iter().map(|item| item.name.to_owned()).collect(),
        None => vec![],
    };
    let subscribed_playlist_items = match &app.sub_playlists {
        Some(list) => list.iter().map(|item| item.name.to_owned()).collect(),
        None => vec![],
    };

    draw_selectable_list(
        f,
        app,
        chunks[0],
        "????????????",
        &my_playlist_items,
        my_playlists_highlight_state,
        app.selected_playlist_index,
    );

    draw_selectable_list(
        f,
        app,
        chunks[1],
        "????????????",
        &subscribed_playlist_items,
        subscribed_playlists_highlight_state,
        app.selected_sub_playlist_index,
    );
}
pub fn draw_library_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::Library,
        current_route.hovered_block == ActiveBlock::Library,
    );

    draw_selectable_list(
        f,
        app,
        layout_chunk,
        "????????????",
        &LIBRARY_OPTIONS,
        highlight_state,
        Some(app.library.selected_index),
    );
}

pub fn draw_selectable_list<B, S>(
    f: &mut Frame<B>,
    app: &App,
    layout_chunk: Rect,
    title: &str,
    items: &[S],
    highlight_state: (bool, bool),
    selected_index: Option<usize>,
) where
    B: Backend,
    S: AsRef<str>,
{
    let mut state = ListState::default();
    state.select(selected_index);

    let list_items: Vec<ListItem> = items
        .iter()
        .map(|item| ListItem::new(Span::raw(item.as_ref())))
        .collect();

    let list = List::new(list_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    title,
                    get_color(highlight_state, app.user_config.theme),
                ))
                .border_style(get_color(highlight_state, app.user_config.theme)),
        )
        .style(Style::default().fg(app.user_config.theme.text))
        .highlight_style(
            get_color(highlight_state, app.user_config.theme).add_modifier(Modifier::BOLD),
        );
    f.render_stateful_widget(list, layout_chunk, &mut state);
}

pub fn draw_help_menu<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(2)
        .split(f.size());

    let format_row =
        |r: Vec<String>| -> Vec<String> { vec![format!("{:50}{:40}{:20}", r[0], r[1], r[2])] };
    let header = ["??????", "??????", "??????"];
    let header = format_row(header.iter().map(|s| s.to_string()).collect());

    let help_docs = get_help_docs(&app.user_config.keys);
    let help_docs = help_docs
        .into_iter()
        .map(format_row)
        .collect::<Vec<Vec<String>>>();
    let help_menu_style = Style::default().fg(app.user_config.theme.text);

    let help_docs = &help_docs[app.help_menu_offset as usize..];

    let rows = help_docs
        .iter()
        .map(|item| Row::new(item.clone()).style(help_menu_style));

    let help_menu = Table::new(rows)
        .header(Row::new(header))
        .column_spacing(2)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(help_menu_style)
                .title(Span::styled(
                    "Help (press <Esc> to go back)",
                    help_menu_style,
                ))
                .border_style(help_menu_style),
        )
        .style(help_menu_style)
        .widths(&[Constraint::Percentage(100)]);

    f.render_widget(help_menu, chunks[0]);
}

pub fn draw_input_and_help_box<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            if app.size.width >= SMALL_TERMINAL_WIDTH
                && !app.user_config.behavior.enforce_wide_search_bar
            {
                [Constraint::Percentage(65), Constraint::Percentage(35)].as_ref()
            } else {
                [Constraint::Percentage(90), Constraint::Percentage(10)].as_ref()
            },
        )
        .split(layout_chunk);

    let current_route = app.get_current_route();

    let highlight_state = (
        current_route.active_block == ActiveBlock::Input,
        current_route.hovered_block == ActiveBlock::Input,
    );

    let input_string: String = app.input.iter().collect();
    let line = Text::from((&input_string).as_str());
    let input = Paragraph::new(line).block(
        Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(
                "??????",
                get_color(highlight_state, app.user_config.theme),
            ))
            .border_style(get_color(highlight_state, app.user_config.theme)),
    );
    f.render_widget(input, chunks[0]);

    let show_loading = app.is_loading && app.user_config.behavior.show_loading_indicator;
    let help_block_text = if show_loading {
        (app.user_config.theme.hint, "Loading...")
    } else {
        (app.user_config.theme.inactive, "Type ?")
    };
    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled("Help", Style::default().fg(help_block_text.0)))
        .border_style(Style::default().fg(help_block_text.0));

    let line = Text::from(help_block_text.1);
    let help = Paragraph::new(line)
        .block(block)
        .style(Style::default().fg(help_block_text.0));
    f.render_widget(help, chunks[1]);
}

fn draw_table<B>(
    f: &mut Frame<B>,
    app: &App,
    layout_chunk: Rect,
    table_layout: (&str, &TableHeader), // (title, header colums)
    items: &[TableItem], // The nested vector must have the same length as the `header_columns`
    selected_index: usize,
    highlight_state: (bool, bool),
) where
    B: Backend,
{
    let selected_style =
        get_color(highlight_state, app.user_config.theme).add_modifier(Modifier::BOLD);

    let track_playing_index = app.current_playback_context.to_owned().and_then(|ctx| {
        ctx.item
            .and_then(|track_item| items.iter().position(|item| item.id == track_item.id))
    });

    let (title, header) = table_layout;

    // Make sure that the selected item is visible on the page. Need to add some rows of padding
    // to chunk height for header and header space to get a true table height
    let padding = 5;
    let offset = layout_chunk
        .height
        .checked_sub(padding)
        .and_then(|height| selected_index.checked_sub(height as usize))
        .unwrap_or(0);

    let rows = items.iter().skip(offset).enumerate().map(|(i, item)| {
        let mut formatted_row = item.format.clone();
        let mut style = Style::default().fg(app.user_config.theme.text); // default styling

        // if table displays songs
        match header.id {
            TableId::Song | TableId::RecentlyPlayed | TableId::Album => {
                if item.fee == 1 {
                    style = Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD);
                }
                // First check if the song should be highlighted because it is currently playing
                if let Some(title_idx) = header.get_index(ColumnId::Title) {
                    if let Some(track_playing_offset_index) =
                        track_playing_index.and_then(|idx| idx.checked_sub(offset))
                    {
                        if i == track_playing_offset_index {
                            formatted_row[title_idx] = format!("??? {}", &formatted_row[title_idx]);
                            style = Style::default()
                                .fg(app.user_config.theme.active)
                                .add_modifier(Modifier::BOLD);
                        }
                    }
                }

                // Show this the liked icon if the song is liked
                if let Some(liked_idx) = header.get_index(ColumnId::Liked) {
                    if app.liked_track_ids_set.contains(&item.id) {
                        formatted_row[liked_idx] = app.user_config.padded_liked_icon();
                    }
                }
            }
            _ => {}
        }

        // Next check if the item is under selection.
        if Some(i) == selected_index.checked_sub(offset) {
            style = selected_style;
        }

        // Return row styled data
        Row::new(formatted_row).style(style)
    });

    let widths = header
        .items
        .iter()
        .map(|h| Constraint::Length(h.width))
        .collect::<Vec<Constraint>>();

    let table = Table::new(rows)
        .header(
            Row::new(header.items.iter().map(|h| h.text))
                .style(Style::default().fg(app.user_config.theme.header)),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(app.user_config.theme.text))
                .title(Span::styled(
                    title,
                    get_color(highlight_state, app.user_config.theme),
                ))
                .border_style(get_color(highlight_state, app.user_config.theme)),
        )
        .style(Style::default().fg(app.user_config.theme.text))
        .widths(&widths);
    f.render_widget(table, layout_chunk);
}
