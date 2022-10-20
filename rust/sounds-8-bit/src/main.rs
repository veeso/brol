use rodio::source::{Amplify, SineWave, Source, TakeDuration};
use rodio::{OutputStream, Sink};
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

type Tone = Amplify<TakeDuration<SineWave>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
enum Note {
    Do,
    Re,
    Mi,
    Fa,
    Sol,
    La,
    Si,
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B,
}

impl Note {
    pub fn freq(self, octave: u8) -> f32 {
        let a440 = Note::A.note_nr(4);
        2f32.powf(((self.note_nr(octave) as i16 - a440 as i16) as f32) / 12.0) * 440.0
    }

    fn note_nr(&self, octave: u8) -> u8 {
        self.pitch_class() + 12 * octave
    }

    fn pitch_class(&self) -> u8 {
        match self {
            Self::Do | Self::C => 0,
            Self::Cs => 1,
            Self::Re | Self::D => 2,
            Self::Ds => 3,
            Self::Mi | Self::E => 4,
            Self::Fa | Self::F => 5,
            Self::Fs => 6,
            Self::Sol | Self::G => 7,
            Self::Gs => 8,
            Self::La | Self::A => 9,
            Self::As => 10,
            Self::Si | Self::B => 11,
        }
    }
}

/// Raw Audio track
#[derive(Default, Clone)]
pub struct Track {
    pub tones: Vec<Tone>,
}

impl Track {
    /// Push tone to track
    pub fn tone(mut self, freq: f32, millis: u64, amplify: f32) -> Self {
        self.tones.push(
            SineWave::new(freq)
                .take_duration(Duration::from_millis(millis))
                .amplify(amplify),
        );
        self
    }
}

/// Donmaze sound type
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Sound {
    ArmorEquipped,
    DrinkPotion,
    EnemyApproaching,
    EnemyAttack,
    EnemyScream,
    Error,
    Input,
    ItemCollected,
    LeaveMaze,
    PlayerAttack,
    PlayerDead,
    Rush,
    Sonar,
    Sleep,
    Steps,
    WakeUp,
}

impl Sound {
    pub fn track(self) -> Track {
        match self {
            Sound::ArmorEquipped => Self::armor_equipped(),
            Sound::DrinkPotion => Self::drink_potion(),
            Sound::EnemyApproaching => Self::enemy_approaching(),
            Sound::EnemyAttack => todo!(),
            Sound::EnemyScream => todo!(),
            Sound::Error => Self::error(),
            Sound::Input => Self::input(),
            Sound::ItemCollected => Self::item_collected(),
            Sound::LeaveMaze => todo!(),
            Sound::PlayerAttack => todo!(),
            Sound::PlayerDead => todo!(),
            Sound::Rush => Self::rush(),
            Sound::Sonar => Self::sonar(),
            Sound::Sleep => Self::sleep(),
            Sound::Steps => Self::steps(),
            Sound::WakeUp => todo!(),
        }
    }

    fn armor_equipped() -> Track {
        Track::default()
            .tone(2000.0, 25, 0.2)
            .tone(0.0, 20, 1.0)
            .tone(3000.0, 50, 0.2)
    }

    fn drink_potion() -> Track {
        Track::default()
            .tone(80.0, 200, 1.0)
            .tone(100.0, 125, 1.0)
            .tone(0.0, 400, 1.0)
            .tone(80.0, 200, 1.0)
            .tone(100.0, 125, 1.0)
            .tone(0.0, 400, 1.0)
            .tone(80.0, 200, 1.0)
            .tone(100.0, 125, 1.0)
            .tone(0.0, 400, 1.0)
            .tone(80.0, 200, 1.0)
            .tone(100.0, 125, 1.0)
            .tone(0.0, 400, 1.0)
            .tone(80.0, 200, 1.0)
            .tone(100.0, 125, 1.0)
    }

    fn enemy_approaching() -> Track {
        Track::default()
            .tone(10.0, 200, 4.0)
            .tone(0.0, 600, 0.2)
            .tone(10.0, 200, 4.0)
            .tone(0.0, 600, 0.2)
            .tone(10.0, 200, 4.0)
            .tone(0.0, 600, 0.2)
            .tone(10.0, 200, 4.0)
            .tone(0.0, 600, 0.2)
            .tone(10.0, 200, 4.0)
            .tone(0.0, 600, 0.2)
            .tone(Note::Mi.freq(2), 400, 2.0)
            .tone(Note::Mi.freq(2), 400, 2.0)
            .tone(Note::Mi.freq(2), 400, 2.0)
            .tone(Note::Do.freq(2), 1000, 2.0)
    }

    fn error() -> Track {
        Track::default()
            .tone(150.0, 25, 1.0)
            .tone(0.0, 20, 1.0)
            .tone(200.0, 50, 1.0)
    }

    fn input() -> Track {
        Track::default().tone(4000.0, 15, 0.2)
    }

    fn item_collected() -> Track {
        Track::default()
            .tone(Note::La.freq(5), 200, 0.2)
            .tone(Note::As.freq(5), 200, 0.2)
            .tone(Note::Si.freq(5), 200, 0.2)
            .tone(Note::Do.freq(6), 500, 0.2)
    }

    fn rush() -> Track {
        Track::default()
            .tone(10.0, 200, 4.0)
            .tone(0.0, 100, 0.2)
            .tone(10.0, 200, 4.0)
            .tone(0.0, 100, 0.2)
            .tone(10.0, 200, 4.0)
            .tone(0.0, 100, 0.2)
            .tone(10.0, 200, 4.0)
            .tone(0.0, 100, 0.2)
            .tone(10.0, 200, 4.0)
            .tone(0.0, 100, 0.2)
    }

    fn sleep() -> Track {
        Track::default()
            .tone(Note::Mi.freq(1), 400, 1.0)
            .tone(Note::Mi.freq(1), 400, 1.0)
            .tone(Note::Sol.freq(1), 600, 1.0)
            .tone(0.0, 600, 1.0)
            .tone(Note::Mi.freq(1), 400, 1.0)
            .tone(Note::Mi.freq(1), 400, 1.0)
            .tone(Note::Sol.freq(1), 600, 1.0)
    }

    fn sonar() -> Track {
        Track::default()
            .tone(Note::Mi.freq(7), 200, 0.01)
            .tone(0.0, 300, 1.0)
            .tone(Note::Mi.freq(7), 200, 0.01)
            .tone(0.0, 300, 1.0)
            .tone(Note::Mi.freq(7), 200, 0.01)
            .tone(0.0, 300, 1.0)
            .tone(Note::Mi.freq(7), 1000, 0.01)
    }

    fn steps() -> Track {
        Track::default()
            .tone(10.0, 200, 4.0)
            .tone(0.0, 300, 0.2)
            .tone(10.0, 200, 4.0)
            .tone(0.0, 300, 0.2)
            .tone(10.0, 200, 4.0)
            .tone(0.0, 300, 0.2)
            .tone(10.0, 200, 4.0)
            .tone(0.0, 300, 0.2)
            .tone(10.0, 200, 4.0)
            .tone(0.0, 300, 0.2)
    }
}

impl FromStr for Sound {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "armorequipped" => Ok(Self::ArmorEquipped),
            "drinkpotion" => Ok(Self::DrinkPotion),
            "enemyapproaching" => Ok(Self::EnemyApproaching),
            "enemyattack" => Ok(Self::EnemyAttack),
            "enemyscream" => Ok(Self::EnemyScream),
            "error" => Ok(Self::Error),
            "input" => Ok(Self::Input),
            "itemcollected" => Ok(Self::ItemCollected),
            "leavemaze" => Ok(Self::LeaveMaze),
            "playerattack" => Ok(Self::PlayerAttack),
            "playerdead" => Ok(Self::PlayerDead),
            "rush" => Ok(Self::Rush),
            "sonar" => Ok(Self::Sonar),
            "sleep" => Ok(Self::Sleep),
            "steps" => Ok(Self::Steps),
            "wakeup" => Ok(Self::WakeUp),
            _ => Err("unknown sound name"),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let sound = match std::env::args().collect::<Vec<String>>().get(1) {
        Some(s) => s.to_string(),
        None => anyhow::bail!("usage: <sound>"),
    };
    let sound = Sound::from_str(&sound).map_err(|e| anyhow::anyhow!(e))?;
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    for tone in sound.track().tones {
        sink.append(tone);
    }
    sink.sleep_until_end();
    sleep(Duration::from_millis(100));

    Ok(())
}
