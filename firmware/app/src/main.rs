mod init;
mod config;
mod hardware;
mod app;

use anyhow::Result;

fn main() -> Result<()> {
    init::system()?;
    let hw = hardware::init()?;
    let mut app = app::App::new(hw);
    app.run();
}
