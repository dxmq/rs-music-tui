extern crate rodio;
extern crate tempfile;
extern crate tokio;

mod fetch;
mod track;

use self::tempfile::NamedTempFile;
use crate::player::fetch::fetch_data;
use crate::player::track::Track;
use futures::channel::oneshot;
use log::debug;
use std::sync::mpsc::Sender;
use std::{fs, thread, time};

#[allow(unused)]
pub enum PlayerCommand {
    Play,
    Pause,
    Stop,
    PlayPause,
    Seek(i32),
    Next,
    Previous,
    Load(String),
    Position(i32, u64),
    Metadata(MetaInfo, Sender<String>),
}

#[allow(unused)]
pub enum MetaInfo {
    Volume,
    Shuffle,
    Position,
    LoopStatus,
    Status,
    Info,
}

pub struct Nplayer {
    pub player: Player,
}

impl Nplayer {
    pub fn new() -> Nplayer {
        let mplayer = Player::new();
        debug!("init player");
        Nplayer { player: mplayer }
    }

    pub fn play_url(&mut self, url: &str) {
        self.player.load(url.to_owned(), true);
    }

    pub fn is_playing(&mut self) -> bool {
        self.player.status()
    }

    pub fn pause(&mut self) {
        self.player.pause()
    }

    pub fn play(&mut self) {
        self.player.play()
    }

    #[allow(unused)]
    pub fn stop(&self) {
        self.player.stop()
    }

    pub fn get_position(&self) -> Option<u64> {
        self.player
            .current
            .clone()
            .map(|current| current.elapsed().as_millis() as u64)
    }

    #[allow(unused)]
    pub fn get_duration(&self) -> Option<u64> {
        match self.player.current.clone() {
            Some(current) => Some(current.duration.as_millis() as u64),
            None => None,
        }
    }

    #[allow(unused)]
    pub fn seek_forwards(&mut self) {
        // let next_duration = self.get_position().unwrap() + 3000;
        // self.player.seek(ClockTime::from_mseconds(next_duration))
    }

    #[allow(unused)]
    pub fn seek_backwards(&mut self) {
        // let song_progress_ms = self.get_position().unwrap();
        // let next_duration = if song_progress_ms < 3000 {
        // 0
        // } else {
        // song_progress_ms - 3000
        // };
        // self.player.seek(ClockTime::from_mseconds(next_duration))
    }

    #[allow(unused)]
    pub fn seek(&mut self, offset: i32) {
        let next_duration = self.get_position().unwrap() as i32 + (offset * 1000);
        // self.player
        // .seek(ClockTime::from_mseconds(next_duration as u64))
    }

    #[allow(unused)]
    pub fn position(&mut self, position: u64) {
        // self.player.seek(ClockTime::from_mseconds(position))
    }

    #[allow(unused)]
    pub fn increase_volume(&mut self) {
        let current = self.player.get_volume();
        let volume = if current < 9.9 {
            current + 0.1_f32
        } else {
            10.0_f32
        };
        self.player.set_volume(volume);
    }

    #[allow(unused)]
    pub fn decrease_volume(&mut self) {
        let current = self.player.get_volume();
        let volume = if current > 0.1 {
            current - 0.1_f32
        } else {
            0.0_f32
        };
        self.player.set_volume(volume);
    }

    #[allow(unused)]
    pub fn get_volume(&self) -> f32 {
        self.player.sink.volume()
    }
}

#[allow(unused)]
pub enum PlayerState {
    Stopped,
    Paused {},
    Playing {},
    EndOfTrack { url: String },
    Invalid,
}

pub struct Player {
    pub state: PlayerState,
    pub current: Option<Track>,
    pub sink: rodio::Sink,
    pub stream: rodio::OutputStream,
    pub stream_handle: rodio::OutputStreamHandle,
}

impl Player {
    pub fn new() -> Player {
        let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&stream_handle).unwrap();
        Player {
            state: PlayerState::Stopped,
            current: None,
            sink,
            stream,
            stream_handle,
        }
    }

    pub fn load(&mut self, url: String, start_playing: bool) {
        match &self.current {
            Some(track) => {
                fs::remove_file(track.file()).ok();
                self.start();
            }
            None => {}
        }

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_string_lossy().to_string();
        let (ptx, mut prx) = oneshot::channel::<String>();

        thread::spawn(move || {
            fetch_data(&url.to_owned(), temp_file, ptx).expect("error thread task");
        });
        if start_playing {
            loop {
                if let Ok(p) = prx.try_recv() {
                    if p.is_some() {
                        if let Ok(track) = Track::load(path) {
                            let mut track = track;
                            self.load_track(track.clone(), start_playing);
                            track.resume();
                            self.current = Some(track);
                            self.state = PlayerState::Playing {};
                        }
                        break;
                    }
                }
                let t = time::Duration::from_millis(250);
                thread::sleep(t);
            }
        }
    }

    pub fn load_track(&mut self, track: Track, playing: bool) {
        if playing {
            let f = std::fs::File::open(&track.file).unwrap();
            let source = rodio::Decoder::new(std::io::BufReader::new(f)).unwrap();

            self.sink.play();
            self.sink.append(source);
        }
    }

    pub fn start(&mut self) {
        let vol = self.sink.volume();
        self.sink.stop();
        self.sink = rodio::Sink::try_new(&self.stream_handle).unwrap();
        self.set_volume(vol);
    }

    pub fn play(&mut self) {
        self.sink.play();
        self.state = PlayerState::Playing {};
        self.current = self.current.take().map(|mut s| {
            s.resume();
            s
        });
    }

    pub fn pause(&mut self) {
        self.sink.pause();
        self.state = PlayerState::Paused {};
        self.current = self.current.take().map(|mut s| {
            s.stop();
            s
        });
    }

    pub fn stop(&self) {
        self.sink.stop()
    }

    #[allow(unused)]
    pub fn seek(&self, position_ms: u32) {}

    pub fn status(&self) -> bool {
        self.state.is_playing()
    }

    #[allow(unused)]
    pub fn get_volume(&self) -> f32 {
        self.sink.volume()
    }

    pub fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume)
    }
}

// drop player
impl Drop for Player {
    fn drop(&mut self) {
        debug!("Shutting down player thread ...");
    }
}

impl PlayerState {
    fn is_playing(&self) -> bool {
        use self::PlayerState::*;
        match *self {
            Stopped | EndOfTrack { .. } | Paused { .. } => false,
            Playing { .. } => true,
            Invalid => panic!("invalid state"),
        }
    }
}
