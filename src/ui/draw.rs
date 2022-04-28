use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::canvas::Canvas;
use tui::widgets::{
    Block, Borders, Clear, Gauge, List, ListItem, ListState, Paragraph, Row, Table, Wrap,
};
use tui::Frame;

use crate::app::{ActiveBlock, App, RouteId, LIBRARY_OPTIONS};
use crate::cli::clap::BANNER;
use crate::model::enums::{PlayingItem, RepeatState};
use crate::model::table::{ColumnId, TableHeader, TableHeaderItem, TableId, TableItem};
use crate::ui::help::get_help_docs;
use crate::util;
use crate::util::{
    create_artist_string, display_track_progress, get_color, get_percentage_width,
    get_track_progress_percentage, millis_to_minutes2, BASIC_VIEW_HEIGHT, SMALL_TERMINAL_WIDTH,
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

        // Search input and help
        draw_input_and_help_box(f, app, parent_layout[0]);

        // Nested main block with potential routes
        draw_routes(f, app, parent_layout[1]);

        // Currently playing
        draw_playbar(f, app, parent_layout[2]);
    }

    // Possibly draw confirm dialog
    draw_dialog(f, app);
}

pub fn draw_dialog<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    if let ActiveBlock::Dialog(_) = app.get_current_route().active_block {
        if let Some(playlist) = app.dialog.as_ref() {
            let bounds = f.size();
            // maybe do this better
            let width = std::cmp::min(bounds.width - 2, 45);
            let height = 8;
            let left = (bounds.width - width) / 2;
            let top = bounds.height / 4;

            let rect = Rect::new(left, top, width, height);

            f.render_widget(Clear, rect);

            // 对话框边框线
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.user_config.theme.inactive));

            f.render_widget(block, rect);

            let vchunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref())
                .split(rect);

            // suggestion: possibly put this as part of
            // app.dialog, but would have to introduce lifetime
            let text = vec![
                Spans::from(Span::raw("Are you sure you want to delete the playlist: ")),
                Spans::from(Span::styled(
                    playlist.as_str(),
                    Style::default().add_modifier(Modifier::BOLD),
                )),
                Spans::from(Span::raw("?")),
            ];
            let text = Paragraph::new(text)
                .wrap(Wrap { trim: true })
                .alignment(Alignment::Center);

            // 对话框标题
            f.render_widget(text, vchunks[0]);

            let hchunks = Layout::default()
                .direction(Direction::Horizontal)
                .horizontal_margin(3)
                .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
                .split(vchunks[1]);
            let ok_text = Span::raw("Ok");
            let ok = Paragraph::new(ok_text)
                .style(Style::default().fg(if app.confirm {
                    app.user_config.theme.hovered
                } else {
                    app.user_config.theme.inactive
                }))
                .alignment(Alignment::Center);

            // Ok radio
            f.render_widget(ok, hchunks[0]);

            let cancel_text = Span::raw("Cancel");
            let cancel = Paragraph::new(cancel_text)
                .style(Style::default().fg(if app.confirm {
                    app.user_config.theme.inactive
                } else {
                    app.user_config.theme.hovered
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
        .title(Span::styled("", Style::default().fg(Color::LightRed)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::LightRed));
    f.render_widget(playbar, chunks[0]);

    if let Some(current_playback_context) = &app.current_playback_context {
        if let Some(track_item) = &current_playback_context.item {
            let play_title = if current_playback_context.is_playing {
                "Playing"
            } else {
                "Paused"
            };

            // let shuffle_text = if current_playback_context.shuffle_state {
            //     "On"
            // } else {
            //     "Off"
            // };

            let play_state_text = match current_playback_context.repeat_state {
                RepeatState::Off => "顺序播放",
                RepeatState::Track => "单曲循环",
                RepeatState::Context => "列表循环",
                RepeatState::Shuffle => "随机播放",
            };

            // let title = format!(
            //     "{:-7} (Shuffle: {:-3} | Repeat: {:-5} Volume: {:-1}%)",
            //     play_title,
            //     // current_playback_context.device.name,
            //     shuffle_text,
            //     repeat_text,
            //     (app.volume * 100f32).ceil() // current_playback_context.device.volume_percent
            // );
            let title = format!(
                "{:-7} ({:-3} Volume: {:-1}%)",
                play_title,
                play_state_text,
                (app.volume * 100f32).ceil() // current_playback_context.device.volume_percent
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

            let (item_id, name, duration_ms) = match track_item {
                PlayingItem::Track(track) => {
                    let duration = track.duration as u32;
                    (track.id, track.name.to_owned(), duration)
                }
            };

            let track_name = if app.liked_track_ids_set.contains(&item_id) {
                format!("{}{}", &app.user_config.padded_liked_icon(), name)
            } else {
                name
            };

            let play_bar_text = match track_item {
                PlayingItem::Track(track) => create_artist_string(&track.artists),
            };

            let lines = Text::from(Span::styled(
                play_bar_text,
                Style::default().fg(app.user_config.theme.playbar_text),
            ));

            let artist = Paragraph::new(lines)
                .style(Style::default().fg(app.user_config.theme.playbar_text))
                .block(
                    Block::default().title(Span::styled(
                        &track_name,
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
        // RouteId::Search => {
        //     draw_search_results(f, app, chunks[1]);
        // }
        RouteId::TrackTable => {
            draw_song_table(f, app, chunks[1]);
        }
        RouteId::Home => {
            draw_home(f, app, chunks[1]);
        }
        RouteId::MadeForYou => {
            // draw_made_for_you(f, app, chunks[1]);
        }
        RouteId::RecentlyPlayed => {
            draw_recently_played_table(f, app, chunks[1]);
        }
        RouteId::Lyric => {
            // draw playing lyric ui
            draw_lyric(f, app, chunks[1]);
        }
        RouteId::Error => {} // This is handled as a "full screen" route in main.rs
        RouteId::BasicView => {} // This is handled as a "full screen" route in main.rs
        RouteId::Dialog => {} // This is handled in the draw_dialog function in mod.rs
        _ => {}
    }
}

pub fn draw_lyric<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::Lyric,
        current_route.hovered_block == ActiveBlock::Lyric,
    );

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        // .margin(2)
        .split(layout_chunk);

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
                .title("Playing")
                .border_style(get_color(highlight_state, app.user_config.theme)),
        )
        .paint(|ctx| {
            ctx.draw(&app.playing_circle);
        })
        .x_bounds([-90.0, 90.0])
        .y_bounds([-90.0, 90.0]);
    f.render_widget(canvas, chunks[0]);

    let lyric_items = match &app.lyric {
        Some(l) => l.iter().map(|item| vec![item.lyric.to_owned()]).collect(),
        None => vec![],
    };
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
            (lyric_items.as_ref(), 0 as usize)
        }
    } else {
        (lyric_items.as_ref(), 0 as usize)
    };

    let header = TableHeader {
        id: TableId::Lyric,
        items: vec![TableHeaderItem {
            id: ColumnId::Title,
            text: "",
            width: get_percentage_width(layout_chunk.width, 0.5),
        }],
    };
    let selected_style = get_color(highlight_state, app.user_config.theme);
    let rows = row_items.iter().enumerate().map(|(i, item)| {
        let mut style = Style::default().fg(Color::White); // default styling
        if i == selected_index - margin {
            style = selected_style;
        }
        // Return row styled data
        Row::new(item.clone()).style(style)
    });
    // let items = app
    //     .track_table
    //     .tracks
    //     .iter()
    //     .map(|item| TableItem {
    //         id: item.id,
    //         format: vec![
    //             "".to_string(),
    //             item.name.to_owned(),
    //             create_artist_string(&item.artists),
    //             item.album.name.to_owned().unwrap(),
    //             millis_to_minutes2(item.duration),
    //         ],
    //     })
    //     .collect::<Vec<TableItem>>();
    // let rows = items.iter().skip(offset).enumerate().map(|(i, item)| {
    //     let mut formatted_row = item.format.clone();
    //     let mut style = Style::default().fg(app.user_config.theme.text); // default styling
    //                                                                      // Return row styled data
    //     Row::new(formatted_row).style(style)
    // });

    let widths = header
        .items
        .iter()
        .map(|h| Constraint::Length(h.width))
        .collect::<Vec<tui::layout::Constraint>>();

    let header = Row::new(header.items.iter().map(|h| h.text))
        .style(Style::default().fg(app.user_config.theme.header));
    let table = Table::new(rows)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::RIGHT | Borders::TOP | Borders::BOTTOM)
                .style(Style::default().fg(Color::White))
                .border_style(get_color(highlight_state, app.user_config.theme)),
        )
        .style(Style::default().fg(Color::White))
        .widths(&widths);
    f.render_widget(table, chunks[1]);
}

pub fn draw_recently_played_table<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let header = TableHeader {
        id: TableId::RecentlyPlayed,
        items: vec![
            TableHeaderItem {
                id: ColumnId::Liked,
                text: "",
                width: 2,
            },
            TableHeaderItem {
                id: ColumnId::Title,
                text: "标题",
                width: get_percentage_width(layout_chunk.width, 2.0 / 5.0) - 2,
            },
            TableHeaderItem {
                text: "歌手",
                width: get_percentage_width(layout_chunk.width, 2.0 / 5.0),
                ..Default::default()
            },
            TableHeaderItem {
                text: "时间",
                width: get_percentage_width(layout_chunk.width, 1.0 / 5.0),
                ..Default::default()
            },
        ],
    };

    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::RecentlyPlayed,
        current_route.hovered_block == ActiveBlock::RecentlyPlayed,
    );

    if !app.recently_played.tracks.is_empty() {
        let items = app
            .recently_played
            .tracks
            .iter()
            .map(|item| TableItem {
                id: item.id,
                format: vec![
                    "".to_string(),
                    item.name.to_owned(),
                    create_artist_string(&item.artists),
                    millis_to_minutes2(item.duration),
                ],
            })
            .collect::<Vec<TableItem>>();
        // let items = vec![];
        draw_table(
            f,
            app,
            layout_chunk,
            ("最近播放", &header),
            &items,
            app.recently_played.selected_index,
            highlight_state,
        )
    };
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
                text: "标题",
                width: get_percentage_width(layout_chunk.width, 0.3),
            },
            TableHeaderItem {
                text: "歌手",
                width: get_percentage_width(layout_chunk.width, 0.3),
                ..Default::default()
            },
            TableHeaderItem {
                text: "专辑",
                width: get_percentage_width(layout_chunk.width, 0.3),
                ..Default::default()
            },
            TableHeaderItem {
                text: "时间",
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
            "Welcome!",
            get_color(highlight_state, app.user_config.theme),
        ))
        .borders(Borders::ALL)
        .border_style(get_color(highlight_state, app.user_config.theme));
    f.render_widget(welcome, layout_chunk);

    let changelog = include_str!("../../CHANGELOG.md").to_string();

    // If debug mode show the "Unreleased" header. Otherwise it is a release so there should be no
    // unreleased features
    let clean_changelog = if cfg!(debug_assertions) {
        changelog
    } else {
        changelog.replace("\n## [Unreleased]\n", "")
    };

    // Banner text with correct styling
    let mut top_text = Text::from(BANNER);
    top_text.patch_style(Style::default().fg(app.user_config.theme.banner));

    let bottom_text_raw = format!(
        "{}{}",
        "\nPlease report any bugs or missing features to xxxxx\n\n", clean_changelog
    );
    let bottom_text = Text::from(bottom_text_raw.as_str());

    // Contains the banner
    let top_text = Paragraph::new(top_text)
        .style(Style::default().fg(app.user_config.theme.text))
        .block(Block::default());
    f.render_widget(top_text, chunks[0]);

    // CHANGELOG
    let bottom_text = Paragraph::new(bottom_text)
        .style(Style::default().fg(app.user_config.theme.text))
        .block(Block::default())
        .wrap(Wrap { trim: false })
        .scroll((app.home_scroll, 0));
    f.render_widget(bottom_text, chunks[1]);
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

        // Search input and help
        draw_input_and_help_box(f, app, chunks[0]);
        draw_library_block(f, app, chunks[0]);
        draw_playlist_block(f, app, chunks[1]);
    }
}

pub fn draw_playlist_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::MyPlaylists,
        current_route.hovered_block == ActiveBlock::MyPlaylists,
    );
    let playlist_items = match &app.playlists {
        Some(list) => list.iter().map(|item| item.name.to_owned()).collect(),
        None => vec![],
    };

    draw_selectable_list(
        f,
        app,
        layout_chunk,
        "歌单",
        &playlist_items,
        highlight_state,
        app.selected_playlist_index,
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
        "目录",
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
    S: std::convert::AsRef<str>,
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
    let header = ["描述", "动作", "内容"];
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
                "搜索",
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
        ctx.item.and_then(|item| match item {
            PlayingItem::Track(track) => items.iter().position(|item| {
                track.id == item.id
                // track.id.to_string().to_owned()
                // .map(|id| id == item.id)
                // .unwrap_or(false)
            }),
        })
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
                // First check if the song should be highlighted because it is currently playing
                if let Some(title_idx) = header.get_index(ColumnId::Title) {
                    if let Some(track_playing_offset_index) =
                        track_playing_index.and_then(|idx| idx.checked_sub(offset))
                    {
                        if i == track_playing_offset_index {
                            formatted_row[title_idx] = format!("▶ {}", &formatted_row[title_idx]);
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
            TableId::PodcastEpisodes => {
                if let Some(name_idx) = header.get_index(ColumnId::Title) {
                    if let Some(track_playing_offset_index) =
                        track_playing_index.and_then(|idx| idx.checked_sub(offset))
                    {
                        if i == track_playing_offset_index {
                            formatted_row[name_idx] = format!("▶ {}", &formatted_row[name_idx]);
                            style = Style::default()
                                .fg(app.user_config.theme.active)
                                .add_modifier(Modifier::BOLD);
                        }
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
        .collect::<Vec<tui::layout::Constraint>>();

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

#[test]
fn test() {
    let f = 79.99999_f32;
    println!("{}", f.ceil());
    // assert_eq!(f.ceil(), 80);
    // assert_eq!(g.floor(), 3.0);
    // assert_eq!(h.floor(), -4.0);
}
