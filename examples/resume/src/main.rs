use resume::App;

fn main() {
    let mut app = App;
    ratatui::run(|t| app.run(t)).unwrap();
}
