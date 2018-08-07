use ggez;
use ggez::audio::{SoundData, Source};
use libsmashbing::SoundId;

pub struct SoundRepo {
    bounce: Source,
    bounce_charge: Source,
}

impl SoundRepo {
    pub fn new(ctx: &mut ggez::Context) -> ggez::GameResult<SoundRepo> {
        macro_rules! load_sound_file {
            ($fname:tt) => {{
                const BYTES: &'static [u8] = include_bytes!($fname);
                let sound_data = SoundData::from_bytes(BYTES);
                Source::from_data(ctx, sound_data)?
            }};
        }

        Ok(SoundRepo {
            bounce: load_sound_file!("../../sounds/bounce.wav"),
            bounce_charge: load_sound_file!("../../sounds/bounce-charge.wav"),
        })
    }

    pub fn play(&self, id: &SoundId) -> ggez::GameResult<()> {
        let outcome = match id {
            SoundId::Bounce => self.bounce.play(),
            SoundId::BounceCharge => self.bounce_charge.play(),
        };
        outcome
    }
}
