use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Text};
use tui::widgets::{Block, Borders, Gauge, List, ListItem, ListState, Paragraph, Row, Table, Wrap};
use tui::Frame;

use crate::app::{ActiveBlock, App, RouteId, LIBRARY_OPTIONS};
use crate::cli::clap::BANNER;
use crate::model::enums::{PlayingItem, RepeatState};
use crate::ui::help::get_help_docs;
use crate::util;
use crate::util::{
    create_artist_string, display_track_progress, get_color, get_track_progress_percentage,
    SMALL_TERMINAL_WIDTH,
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
                    // Constraint::Min(1),
                    // Constraint::Length(6),
                ]
                .as_ref(),
            )
            .split(f.size());

        draw_input_and_help_box(f, app, parent_layout[0])
    }
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
        .title(Span::styled("playbar!", Style::default().fg(Color::Blue)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue));
    f.render_widget(playbar, chunks[0]);

    if let Some(current_playback_context) = &app.current_playback_context {
        if let Some(track_item) = &current_playback_context.item {
            let play_title = if current_playback_context.is_playing {
                "Playing"
            } else {
                "Paused"
            };

            let shuffle_text = if current_playback_context.shuffle_state {
                "On"
            } else {
                "Off"
            };

            let repeat_text = match current_playback_context.repeat_state {
                RepeatState::Off => "Off",
                RepeatState::Track => "Track",
                RepeatState::Context => "All",
            };

            let title = format!(
                "{:-7} ({} | Shuffle: {:-3} | Repeat: {:-5} | Volume: {:-2}%)",
                play_title,
                current_playback_context.device.name,
                shuffle_text,
                repeat_text,
                current_playback_context.device.volume_percent
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
                PlayingItem::Track(track) => (
                    track.id.to_owned().unwrap_or_else(|| "".to_string()),
                    track.name.to_owned(),
                    track.duration_ms,
                ),
                PlayingItem::Episode(episode) => (
                    episode.id.to_owned(),
                    episode.name.to_owned(),
                    episode.duration_ms,
                ),
            };

            let track_name = if app.liked_song_ids_set.contains(&item_id) {
                format!("{}{}", &app.user_config.padded_liked_icon(), name)
            } else {
                name
            };

            let play_bar_text = match track_item {
                PlayingItem::Track(track) => create_artist_string(&track.artists),
                PlayingItem::Episode(episode) => {
                    format!("{} - {}", episode.name, episode.show.name)
                }
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
        RouteId::Search => {
            // draw_search_results(f, app, chunks[1]);
        }
        RouteId::Home => {
            draw_home(f, app, chunks[1]);
        }
    }
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
        // draw_library_block(f, app, chunks[0]);
        // draw_playlist_block(f, app, chunks[1]);
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
        Some(p) => p.items.iter().map(|item| item.name.to_owned()).collect(),
        None => vec![],
    };

    draw_selectable_list(
        f,
        app,
        layout_chunk,
        "Playlists",
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
        "Library",
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

    let mut current_route = app.get_current_route();

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
