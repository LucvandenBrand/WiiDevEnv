use alloc::sync::Arc;
use ogc_rs::{mp3player::MP3Player, prelude::Asnd};
use super::ogg_player;

/**
 * General audio player interface, hiding the specific audio format players.
 */
pub struct AudioPlayer {
    ogg_player: OGGPlayer,
    mp3_player: MP3Player
}

impl AudioPlayer {
    pub fn new() -> AudioPlayer {
        let asnd = Asnd::init();
        return AudioPlayer { 
            ogg_player: OGGPlayer::new(asnd),
            mp3_player: MP3Player::new(asnd) 
        };
    }

    /**
     * Play a given sound.
     */
    pub fn play(&mut self, audio: Audio) {
        match audio.get_format() {
            AudioFormat::OGG => self.ogg_player.play(audio),
            AudioFormat::MP3 => self.mp3_player.play_buffer(audio.get_buffer())
        }
    }

    /**
     * Stop the given sound.
     */
    pub fn stop(&mut self, audio: Audio) {
        match audio.get_format() {
            AudioFormat::OGG => self.ogg_player.stop(audio),
            AudioFormat::MP3 => self.mp3_player.stop() // MP3 player doesn't mix, so stop all.
        }
    }

    /**
     * Set the global audio volume.
     */
    pub fn set_volume(&mut self, volume: u32) {
        self.ogg_player.set_volume(volume);
        self.mp3_player.volume(volume);
    }
}