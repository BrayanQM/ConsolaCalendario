mod app;
mod event;
mod ui;
mod ical;
mod config;
mod state;
mod util;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    rt.block_on(async move {
        let mut terminal = ratatui::init();
        let res = app::run(&mut terminal).await;
        ratatui::restore();
        res
    })
}
mod subjects;
