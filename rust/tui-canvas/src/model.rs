use crate::canvas::Canvas;

use super::{Id, Msg};

use std::time::Duration;
use tuirealm::props::Shape;
use tuirealm::tui::layout::{Constraint, Direction, Layout};
use tuirealm::{
    terminal::TerminalBridge, Application, EventListenerCfg, NoUserEvent, PollStrategy, Update,
};

pub struct Model {
    application: Application<Id, Msg, NoUserEvent>,
    quit: bool,
    redraw: bool,
    terminal: TerminalBridge,
}

impl Default for Model {
    fn default() -> Self {
        let application = Application::init(
            EventListenerCfg::default().default_input_listener(Duration::from_millis(50)),
        );
        Self {
            application,
            quit: false,
            redraw: true,
            terminal: TerminalBridge::new().expect("failed to init terminal"),
        }
    }
}

impl Update<Msg> for Model {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        match msg.unwrap_or(Msg::None) {
            Msg::Quit => {
                self.quit = true;
            }
            Msg::None => {}
        }
        None
    }
}

impl Model {
    pub fn mount_canvas(&mut self, shape: &[Shape]) -> anyhow::Result<()> {
        let terminal_size = self.terminal.raw().size()?;
        self.application.remount(
            Id::Canvas,
            Box::new(Canvas::new(
                shape,
                terminal_size.width as f64 - 2.0,
                terminal_size.height as f64 - 2.0,
            )),
            vec![],
        )?;
        self.application.active(&Id::Canvas)?;

        Ok(())
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        self.init_terminal();
        while !self.quit {
            // poll and update
            match self.application.tick(PollStrategy::UpTo(3)) {
                Ok(messages) if messages.is_empty() => {}
                Ok(messages) => {
                    self.redraw = true;
                    for msg in messages.into_iter() {
                        if let Some(Msg::Quit) = self.update(Some(msg)) {
                            self.quit = true;
                            break;
                        }
                    }
                }
                Err(err) => {
                    anyhow::bail!(err)
                }
            }
            // View
            if self.redraw {
                self.view()?;
            }
        }
        self.finalize_terminal();

        Ok(())
    }

    pub fn origin(&self) -> anyhow::Result<(f64, f64)> {
        let rect = self.terminal.raw().size()?;
        Ok((rect.x as f64, rect.height as f64 - 2.0))
    }

    fn view(&mut self) -> anyhow::Result<()> {
        self.redraw = false;
        self.terminal.raw_mut().draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());
            self.application.view(&Id::Canvas, f, chunks[0]);
        })?;
        Ok(())
    }

    fn init_terminal(&mut self) {
        let _ = self.terminal.enable_raw_mode();
        let _ = self.terminal.enter_alternate_screen();
        let _ = self.terminal.clear_screen();
    }

    fn finalize_terminal(&mut self) {
        let _ = self.terminal.disable_raw_mode();
        let _ = self.terminal.leave_alternate_screen();
        let _ = self.terminal.clear_screen();
    }
}
