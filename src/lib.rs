// Rust extension for netease-cloud-music.el

// Copyright (c) 2022 SpringHan

mod api;

use emacs::{defun, Env, IntoLisp, Result, Value};

emacs::plugin_is_GPL_compatible!();

#[emacs::module(mod_in_name = false)]
fn init(_: &Env) -> Result<()> {
    api::init_api();
    Ok(())
}

/// Search song or playlist.
/// SEARCH_CONTENT is the content you want to search,
/// If PLAYLISTP is a number, then search playlist, otherwise it's nil, search song.
/// LIMIT is the limitation of each search.
/// PAGE is the current search page.
#[defun]
    #[tokio::main]
async fn search(
    env: &Env,
    search_content: String,
    playlistp: Option<i64>,
    limit: i64,
    page: i64,
) -> Result<Value<'_>> {
    // match playlistp {
    //     Some(_) => {
    //         let result = api::search_playlist(&search_content, limit, page).await.ok();
    //         match result {
    //             None => Ok(().into_lisp(env)?),
    //             Some(a) => {
    //                 Ok(env.call("list", &a)?)
    //             },
    //         }
    //     },
    //     None => {
    //         let result = api::search_song(&search_content, limit, page).await.ok();
    //         match result {
    //             None => Ok(().into_lisp(env)?),
    //             Some(a) => Ok(env.list(a)?),
    //         }
    //     },
    // }

    // TODO: Come up with a solution for this converting.
    let mut a = vec![];
    a.push(("a".into_lisp(env)?, 1i64.into_lisp(env)?));
    a.push(("b".into_lisp(env)?, 2i64.into_lisp(env)?));
    env.list(&a)
}

#[defun]
fn login(env: &Env) -> Result<Value<'_>> {
    let phone = env
        .call("read-number", ["name".into_lisp(env)?])?
        .into_rust::<u16>()?;
    let password = env
        .call("read-passwd", ["Enter your password: ".into_lisp(env)?])?
        .into_rust::<String>()?;

    env.message(&format!("{:#?} {:?}", phone, password))
}
