use ggez;
use ggez::audio::{SoundData, Source};
use libsmashbing::SoundId;

const BOUNCE: &'static [u8] = include_bytes!("../../sounds/bounce.wav");
const BOUNCE_CHARGE: &'static [u8] = include_bytes!("../../sounds/bounce-charge.wav");

pub struct SoundRepo {
    bounce: Source,
    bounce_charge: Source,
}

impl SoundRepo {
    pub fn new(ctx: &mut ggez::Context) -> ggez::GameResult<SoundRepo> {
        let bounce_data = SoundData::from_bytes(BOUNCE);
        let bounce_source = Source::from_data(ctx, bounce_data)?;
        let bounce_charge_data = SoundData::from_bytes(BOUNCE_CHARGE);
        let bounce_charge_source = Source::from_data(ctx, bounce_charge_data)?;
        Ok(SoundRepo {
            bounce: bounce_source,
            bounce_charge: bounce_charge_source,
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
