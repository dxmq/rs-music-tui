use std::fs;
use std::thread;
use std::time;

use futures::channel::oneshot;
use log::debug;
use tempfile::NamedTempFile;

use super::fetch::fetch_data;
use super::track::Track;

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
                match prx.try_recv() {
                    Ok(p) => match p {
                        Some(_) => {
                            match Track::load(path) {
                                Ok(track) => {
                                    let mut track = track;
                                    self.load_track(track.clone(), start_playing);
                                    track.resume();
                                    self.current = Some(track);
                                    self.state = PlayerState::Playing {};
                                }
                                Err(_) => {}
                            }
                            break;
                        }
                        None => {}
                    },
                    Err(_) => {}
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
        self.current = self.current.take().and_then(|mut s| {
            s.resume();
            Some(s)
        });
    }

    pub fn pause(&mut self) {
        self.sink.pause();
        self.state = PlayerState::Paused {};
        self.current = self.current.take().and_then(|mut s| {
            s.stop();
            Some(s)
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
