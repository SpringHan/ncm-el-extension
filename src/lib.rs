// Rust extension for netease-cloud-music.el

// Copyright (c) 2022 SpringHan

mod api;

use emacs::{defun, Env, Result, Value};

emacs::plugin_is_GPL_compatible!();

#[emacs::module]
fn init(_: &Env) -> Result<()> {
    api::init_api();
    Ok(())
}
