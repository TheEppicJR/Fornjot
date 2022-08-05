//! # Fornjot Application
//!
//! This library is part of the [Fornjot] ecosystem. Fornjot is an open-source,
//! code-first CAD application; and collection of libraries that make up the CAD
//! application, but can be used independently.
//!
//! Together with the [`fj`] library, this application forms the part of Fornjot
//! that is relevant to end users. Please refer to the [Fornjot repository] for
//! usage examples.
//!
//! [Fornjot]: https://www.fornjot.app/
//! [`fj`]: https://crates.io/crates/fj
//! [Fornjot repository]: https://github.com/hannobraun/Fornjot

mod application;
mod args;
mod config;
// mod ecs_manager;
mod code_editor;
mod editor_window;
mod main_ui;
mod project_manager;
mod syntax_highlighting;
mod window;

// use anyhow::{anyhow, Context as _};
// use fj_export::export;
// use fj_host::{Model, Parameters};
use fj_operations::shape_processor::ShapeProcessor;
use tracing_subscriber::fmt::format;
use tracing_subscriber::EnvFilter;

use crate::{
    application::run_app, args::Args, config::Config, project_manager::Project,
};

fn main() -> anyhow::Result<()> {
    // Respect `RUST_LOG`. If that's not defined or erroneous, log warnings and
    // above.
    //
    // It would be better to fail, if `RUST_LOG` is erroneous, but I don't know
    // how to distinguish between that and the "not defined" case.
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("WARN")),
        )
        .event_format(format().pretty())
        .init();

    let args = Args::parse();
    let config = Config::load()?;
    let shape_processor = ShapeProcessor {
        tolerance: args.tolerance,
    };
    let project = Project::load_file(args, config);

    // if let Some(path) = args.export {
    //     let shape = model.load_once(&parameters)?;
    //     let shape = shape_processor.process(&shape)?;

    //     export(&shape.mesh, &path)?;

    //     return Ok(());
    // }

    // let watcher = model.load_and_watch(parameters)?;
    // run(watcher, shape_processor)?;
    run_app();

    Ok(())
}
