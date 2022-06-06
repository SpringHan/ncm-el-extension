// Rust extension for netease-cloud-music.el

// Copyright (c) 2022 SpringHan

mod api;

use emacs::{defun, Env, Result, Value, IntoLisp};

emacs::plugin_is_GPL_compatible!();

#[emacs::module(mod_in_name = false)]
fn init(_: &Env) -> Result<()> {
    api::init_api();
    Ok(())
}

// #[defun]
// fn search_song() -> Type {
    
// }

#[defun]
fn login(env: &Env) -> Result<Value<'_>> {
    let phone = env.call("read-number", ["name".into_lisp(env)?])?
        .into_rust::<u16>()?;
    let password = env.call("read-passwd", ["Enter your password: ".into_lisp(env)?])?
        .into_rust::<String>()?;

    env.message(&format!("{:#?} {:?}", phone, password))
}
