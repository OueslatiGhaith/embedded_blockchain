use alloc::{borrow::ToOwned, string::String};
use cortex_m_semihosting::hprintln;

use crate::{app::App, block};

pub fn print_chain(app: &App) {
    let json: String =
        serde_json::to_string_pretty(&app.blocks).expect("[ERROR] failed to stringify blocks");
    hprintln!("{}", json);
}

pub fn create_block(app: &mut App) {
    let latest_block = app.blocks.last().expect("[ERROR] last block not found");
    let block = block::Block::new(
        latest_block.id + 1,
        latest_block.hash.clone(),
        "block".to_owned(),
    );
    let json = serde_json::to_string(&block).expect("[ERROR] can't stringify request");
    app.blocks.push(block);
    // app.try_add_block(block);
    // hprintln!("[INFO] broadcasting new block...")
}
