extern crate rodio;
extern crate tempfile;
extern crate tokio;

use std::fs::File;
use std::sync::mpsc::Sender;
use std::time::Duration;
use std::{fs, thread, time};

use anyhow::Result;
use futures::channel::oneshot;
use log::debug;
use rodio::decoder::DecoderError;
use rodio::{Decoder, Sink, Source};

use crate::player::fetch::fetch_data;
use crate::player::track::Track;

mod fetch;
mod track;

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

    pub fn play_url(&mut self, url: &str) -> Result<()> {
        self.player.load(url.to_owned(), true)
    }

    pub fn is_playing(&mut self) -> bool {
        self.player.status()
    }

    pub fn pause(&mut self) {
        self.player.pause()
    }

    #[allow(unused)]
    pub fn play(&mut self) {
        self.player.play()
    }

    #[allow(unused)]
    pub fn stop(&self) {
        self.player.stop()
    }

    #[allow(unused)]
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
    pub fn seek(&mut self, next_duration: Duration) {
        self.player.seek(next_duration)
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

    pub fn load(&mut self, url: String, start_playing: bool) -> Result<()> {
        match &self.current {
            Some(track) => {
                fs::remove_file(track.file()).ok();
                self.start();
            }
            None => {}
        }

        let (ptx, mut prx) = oneshot::channel::<String>();

        thread::spawn(move || {
            fetch_data(&url.to_owned(), ptx).expect("error thread task");
        });
        if start_playing {
            loop {
                if let Ok(p) = prx.try_recv() {
                    if p.is_some() {
                        let track = Track::load(p.unwrap().clone())?;
                        let mut track = track;
                        self.load_track(track.clone(), start_playing)?;
                        track.resume();
                        self.current = Some(track);
                        self.state = PlayerState::Playing {};
                        break;
                    }
                }
                let t = time::Duration::from_millis(500);
                thread::sleep(t);
            }
        }
        Ok(())
    }

    pub fn load_track(&mut self, track: Track, playing: bool) -> Result<()> {
        if playing {
            let f = std::fs::File::open(&track.file)?;
            let source = rodio::Decoder::new(std::io::BufReader::new(f))?;

            self.sink.play();
            self.sink.append(source);
        }
        Ok(())
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
    pub fn seek(&mut self, position_ms: Duration) {
        if let Some(track) = &self.current {
            let path = &track.file;
            match get_audio_source(path) {
                Ok(source) => {
                    self.new_sink().unwrap();
                    self.sink.append(source.skip_duration(position_ms));
                    // self.sink.play();
                }
                Err(err) => {
                    println!("{}", err);
                }
            };
        }
    }

    fn new_sink(&mut self) -> Result<()> {
        self.sink = Sink::try_new(&self.stream_handle)?;
        Ok(())
    }

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

pub fn get_audio_source(path: &str) -> Result<Decoder<File>, DecoderError> {
    let file = File::open(path).unwrap();
    Decoder::new(file)
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
