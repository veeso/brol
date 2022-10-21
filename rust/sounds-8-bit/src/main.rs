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
            Sound::EnemyAttack => Self::enemy_attack(),
            Sound::EnemyScream => Self::enemy_scream(),
            Sound::Error => Self::error(),
            Sound::Input => Self::input(),
            Sound::ItemCollected => Self::item_collected(),
            Sound::LeaveMaze => Self::leave_maze(),
            Sound::PlayerAttack => Self::player_attack(),
            Sound::PlayerDead => Self::player_dead(),
            Sound::Rush => Self::rush(),
            Sound::Sonar => Self::sonar(),
            Sound::Sleep => Self::sleep(),
            Sound::Steps => Self::steps(),
            Sound::WakeUp => Self::wake_up(),
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

    fn enemy_attack() -> Track {
        Track::default()
            .tone(50.0, 300, 15.0)
            .tone(70.0, 100, 15.0)
            .tone(100.0, 250, 15.0)
    }

    fn enemy_scream() -> Track {
        Track::default()
            .tone(100.0, 120, 20.0)
            .tone(120.0, 120, 20.0)
            .tone(140.0, 500, 20.0)
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

    fn leave_maze() -> Track {
        Track::default()
            .tone(Note::Mi.freq(4), 150, 1.0)
            .tone(Note::Mi.freq(4), 150, 1.0)
            .tone(Note::Mi.freq(4), 150, 1.0)
            .tone(Note::La.freq(4), 750, 1.0)
    }

    fn player_attack() -> Track {
        Track::default()
            .tone(120.0, 130, 25.0)
            .tone(110.0, 30, 25.0)
    }

    fn player_dead() -> Track {
        Track::default()
            .tone(Note::Ds.freq(5), 200, 0.3)
            .tone(Note::D.freq(5), 200, 0.3)
            .tone(Note::Ds.freq(5), 200, 0.3)
            .tone(Note::D.freq(5), 200, 0.3)
            .tone(Note::Ds.freq(5), 400, 0.3)
            .tone(Note::D.freq(5), 400, 0.3)
            .tone(Note::Ds.freq(5), 700, 0.3)
            .tone(Note::D.freq(5), 700, 0.3)
            .tone(Note::C.freq(4), 700, 1.0)
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

    fn wake_up() -> Track {
        Track::default()
            .tone(Note::G.freq(5), 700, 0.3)
            .tone(Note::B.freq(5), 700, 0.3)
            .tone(Note::A.freq(5), 700, 0.3)
            .tone(Note::D.freq(5), 1000, 0.3)
            .tone(Note::D.freq(5), 700, 0.3)
            .tone(Note::A.freq(5), 700, 0.3)
            .tone(Note::B.freq(5), 700, 0.3)
            .tone(Note::G.freq(5), 1000, 0.3)
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

/// Donmaze theme type
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Theme {
    Fight,
    GameOver,
    Maze,
    Menu,
    Victory,
    None,
}

impl Theme {
    pub fn track(self) -> Track {
        match self {
            Theme::Fight => Self::fight(),
            Theme::GameOver => Self::game_over(),
            Theme::Maze => Self::maze(),
            Theme::Menu => Self::menu(),
            Theme::Victory => Self::victory(),
            Theme::None => Track::default(),
        }
    }

    fn fight() -> Track {
        Track::default()
            .tone(Note::Cs.freq(2), 300, 4.0)
            .tone(Note::Cs.freq(2), 300, 4.0)
            .tone(Note::D.freq(2), 300, 4.0)
            .tone(Note::Cs.freq(2), 300, 4.0)
            .tone(Note::Cs.freq(2), 300, 4.0)
            .tone(Note::D.freq(2), 300, 4.0)
            .tone(Note::Cs.freq(2), 300, 4.0)
            .tone(Note::Cs.freq(2), 300, 4.0)
            .tone(Note::Ds.freq(2), 300, 4.0)
            .tone(Note::E.freq(2), 600, 4.0)
    }

    fn game_over() -> Track {
        Track::default()
            .tone(Note::E.freq(4), 500, 0.8)
            .tone(Note::D.freq(4), 500, 0.8)
            .tone(Note::Cs.freq(4), 2000, 0.8)
            .tone(Note::As.freq(3), 500, 2.0)
            .tone(Note::Gs.freq(3), 500, 2.0)
            .tone(Note::E.freq(3), 2000, 3.0)
            .tone(Note::F.freq(3), 500, 2.0)
            .tone(Note::Gs.freq(3), 1200, 2.0)
            .tone(Note::C.freq(4), 1000, 0.8)
    }

    fn maze() -> Track {
        Track::default()
            .tone(51.9, 2600, 4.0)
            .tone(116.6, 2600, 3.2)
            .tone(123.2, 2600, 3.0)
            .tone(116.6, 2600, 3.0)
    }

    fn menu() -> Track {
        Track::default()
            .tone(50.0, 300, 1.5)
            .tone(70.0, 300, 2.5)
            .tone(60.0, 300, 3.5)
            .tone(30.0, 300, 4.5)
            .tone(40.0, 300, 5.5)
            .tone(30.0, 300, 4.5)
            .tone(60.0, 300, 3.5)
            .tone(50.0, 300, 1.5)
            .tone(70.0, 300, 2.5)
    }

    fn victory() -> Track {
        Track::default()
            .tone(Note::E.freq(4), 500, 0.4)
            .tone(Note::E.freq(4), 500, 0.4)
            .tone(Note::F.freq(4), 500, 0.4)
            .tone(Note::G.freq(4), 500, 0.4)
            .tone(Note::Gs.freq(4), 500, 0.4)
            .tone(Note::F.freq(4), 500, 0.4)
            .tone(Note::E.freq(4), 500, 0.4)
            .tone(Note::E.freq(4), 500, 0.4)
            .tone(Note::F.freq(4), 500, 0.4)
            .tone(Note::Gs.freq(4), 500, 0.4)
            .tone(Note::Gs.freq(4), 500, 0.4)
            .tone(Note::G.freq(4), 500, 0.4)
            .tone(Note::F.freq(4), 500, 0.4)
    }
}

impl FromStr for Theme {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fight" => Ok(Self::Fight),
            "gameover" => Ok(Self::GameOver),
            "maze" => Ok(Self::Maze),
            "menu" => Ok(Self::Menu),
            "victory" => Ok(Self::Victory),
            _ => Err("unknown theme"),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let sound = match std::env::args().collect::<Vec<String>>().get(1) {
        Some(s) => s.to_string(),
        None => anyhow::bail!("usage: <sound>"),
    };
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    if let Ok(sound) = Sound::from_str(&sound) {
        for tone in sound.track().tones {
            sink.append(tone);
        }
        sink.sleep_until_end();
        sleep(Duration::from_millis(100));
    }
    if let Ok(theme) = Theme::from_str(&sound) {
        for _ in 0..3 {
            for tone in theme.track().tones {
                sink.append(tone);
            }
            sink.sleep_until_end();
        }
    }

    Ok(())
}
