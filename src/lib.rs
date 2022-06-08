// Rust extension for netease-cloud-music.el

// Copyright (c) 2022 SpringHan

mod api;

use emacs::{Env, Result};
// use std::thread;

emacs::plugin_is_GPL_compatible!();

#[emacs::module(mod_in_name = false)]
fn init(_: &Env) -> Result<()> {
    api::init_api();
    Ok(())
}
