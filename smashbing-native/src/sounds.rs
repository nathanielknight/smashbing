use ggez;
use ggez::audio::{SoundData, Source};
use libsmashbing::SoundId;

pub struct SoundRepo {
    bounce: Source,
    bounce_charge: Source,
    impulse: Source,
    impulse_exhaust: Source,
    break1: Source,
    break2: Source,
    break3: Source,
    break4: Source,
    win: Source,
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
            impulse: load_sound_file!("../../sounds/impulse.wav"),
            impulse_exhaust: load_sound_file!("../../sounds/impulse-exhaust.wav"),
            break1: load_sound_file!("../../sounds/break1.wav"),
            break2: load_sound_file!("../../sounds/break2.wav"),
            break3: load_sound_file!("../../sounds/break3.wav"),
            break4: load_sound_file!("../../sounds/break4.wav"),
            win: load_sound_file!("../../sounds/win.wav"),
        })
    }

    pub fn play(&self, id: &SoundId) -> ggez::GameResult<()> {
        let outcome = match id {
            SoundId::Bounce => self.bounce.play(),
            SoundId::BounceCharge => self.bounce_charge.play(),
            SoundId::Impulse => self.impulse.play(),
            SoundId::ImpulseExhaust => self.impulse_exhaust.play(),
            SoundId::Break1 => self.break1.play(),
            SoundId::Break2 => self.break2.play(),
            SoundId::Break3 => self.break3.play(),
            SoundId::Break4 => self.break4.play(),
            SoundId::Win => self.win.play(),
        };
        outcome
    }
}
