use std::f32::consts::SQRT_2;

use ratatui::{
    DefaultTerminal, crossterm,
    prelude::*,
    text::ToLine,
    widgets::{Block, Borders, Paragraph, RatatuiMascot},
};
use tui_widgets::big_text::{BigText, PixelSize};

pub struct App;

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        loop {
            terminal.draw(|frame| frame.render_widget(&*self, frame.area()))?;
            if crossterm::event::read()?.is_key_press() {
                break Ok(());
            }
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        const A4_RATIO: f32 = 1. / SQRT_2;
        let portrait_area = fit_aspect(area, A4_RATIO);

        let block = Block::bordered();
        (&block).render(portrait_area, buf);

        let inner_area = block.inner(area);

        let [heading_area, body_area] = inner_area.layout(&Layout::vertical([
            Constraint::Length(10),
            Constraint::Fill(1),
        ]));

        BigText::builder()
            .lines(["Samuel Akesson".into()])
            .pixel_size(PixelSize::Octant)
            .centered()
            .build()
            .render(heading_area, buf);

        let lines = [["Software Engineer: ".bold(), "i'm cookig rn ".into()]]
            .map(Line::from_iter)
            .to_vec();
        Paragraph::new(lines).centered().render(body_area, buf);

        RatatuiMascot::new().render(
            body_area.centered_horizontally(Constraint::Percentage(50)),
            buf,
        );
    }
}

fn fit_aspect(area: Rect, aspect: f32) -> Rect {
    let w = area.width as f32;
    let h = area.height as f32;

    let (width, height) = if h / w > aspect {
        let width = area.width;
        let height = (w * aspect) as u16;
        (width, height)
    } else {
        let height = area.height;
        let width = (h / aspect) as u16;
        (width, height)
    };

    Rect::new(
        area.x + (area.width - width) / 2,
        area.y + (area.height - height) / 2,
        width,
        height,
    )
}
