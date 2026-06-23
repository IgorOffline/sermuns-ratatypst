use ratatypst_core::TypstBackend;
use wasm_minimal_protocol::{initiate_protocol, wasm_func};

mod app;

pub use crate::app::App;

initiate_protocol!();

#[wasm_func]
pub fn run(width: &[u8], height: &[u8]) -> Vec<u8> {
    let width = u16::from_le_bytes(width.try_into().expect("width is not u16"));
    let height = u16::from_le_bytes(height.try_into().expect("height is not u16"));

    let mut terminal = ratatui::Terminal::new(TypstBackend::new(width, height)).unwrap();

    let app = App;

    terminal
        .draw(|frame| frame.render_widget(&app, frame.area()))
        .unwrap();

    terminal.backend().to_vec()
}
