mod app;
mod event;
mod ui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let mut terminal = ratatui::init();
    let res = app::run(&mut terminal).await;
    ratatui::restore();
    res
}
