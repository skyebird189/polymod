use std::time::Duration;

use polymod::{self, track::{Track, Pattern}, track_player::{TrackPlayer}, Note, PianoKey};
use sdl2::audio::{AudioSpecDesired, AudioCallback};

struct Audio<'a> {
    player: &'a mut TrackPlayer<'a>
}

impl<'a> AudioCallback for Audio<'a> {
    type Channel = f32;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        for x in out.iter_mut() {
            *x = self.player.advance();
        }
    }
}

#[test]
pub fn test_track() {
    let track = Track::from_it("/home/ollie/Music/Modules/Created/TestPlayback.it").unwrap();

    let mut player = TrackPlayer::new(&track);
    
    let sdl = sdl2::init().unwrap();
    let audio = sdl.audio().unwrap();

    let desired_spec = AudioSpecDesired {
        freq: Some(polymod::track_player::SAMPLE_RATE),
        channels: Some(2),
        samples: Some(8192)
    };

    let device = audio.open_playback(None, &desired_spec, |_| {
        Audio {
            player: &mut player
        }
    }).unwrap();

    device.resume();

    //std::thread::sleep(Duration::from_secs((((length as i32) / 4 / rate) - 1) as u64));
    loop {
        std::thread::sleep(Duration::from_secs(5));
    }
}