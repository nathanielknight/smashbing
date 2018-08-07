use ggez;
use ggez::audio::{SoundData, Source};
use libsmashbing::SoundId;

const BOUNCE: &'static [u8] = include_bytes!("../../sounds/bounce.wav");

pub struct SoundRepo {
    bounce: Source,
}

impl SoundRepo {
    pub fn new(ctx: &mut ggez::Context) -> ggez::GameResult<SoundRepo> {
        let bounce_data = SoundData::from_bytes(BOUNCE);
        let bounce_source = Source::from_data(ctx, bounce_data)?;
        Ok(SoundRepo {
            bounce: bounce_source,
        })
    }

    pub fn play(&self, id: &SoundId) -> ggez::GameResult<()> {
        match id {
            SoundId::Bounce => {
                if !self.bounce.playing() {
                    self.bounce.play()?;
                }
            }
        };
        Ok(())
    }
}
