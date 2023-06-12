// main.rs

mod ui;
mod fonttools;

use std::io;

fn main() -> Result<(), io::Error>{
    let fname = "/home/marchall/.local/share/fonts/NerdFonts/FiraCodeNerdFont-Regular.ttf";
    // fonttools::get_unicode(fname);

    ui::setup_and_run()?;

    Ok(())
}

