use crate::ecs::tabs::EditorTab;
use crate::{args::Args, config::Config};
use anyhow::{anyhow, Context as _, Ok};
use bevy::prelude::*;
use fj_host::Model;
use fj_operations::shape_processor::ShapeProcessor;
use std::path::PathBuf;

pub enum file_source {
    File(PathBuf),
    New,
}

#[derive(Component)]
pub struct Project {
    file_origin: PathBuf,
    model: Model,
    name: String,
}

impl Project {
    pub fn new() -> Project {
        todo!()
    }
    pub fn load_file(args: Args, config: Config) -> anyhow::Result<Project> {
        let mut path = config.default_path.unwrap_or_else(|| PathBuf::from(""));
        let loaded_model =
            args.model.or(config.default_model).ok_or_else(|| {
                anyhow!(
                    "No model specified, and no default model configured.\n\
                Specify a model by passing `--model path/to/model`."
                )
            })?;
        path.push(loaded_model);

        let name = "todo";

        let loaded_model =
            Model::from_path(path.clone()).with_context(|| {
                format!("Failed to load model: {}", path.display())
            })?;
        Ok(Project {
            file_path: path.clone(),
            model: loaded_model,
            name: name.to_string(),
        })
    }
}

impl Default for Project {
    fn default() -> Self {
        todo!()
    }
}

// let model = Model::from_path(path.clone())
//     .with_context(|| format!("Failed to load model: {}", path.display()))?;
// let parameters = args.parameters.unwrap_or_else(Parameters::empty);

// let shape_processor = ShapeProcessor {
//     tolerance: args.tolerance,
// };
// let project = Project::load_file(args, config);

// if let Some(path) = args.export {
//     let shape = model.load_once(&parameters, &mut status)?;
//     let shape = shape_processor.process(&shape)?;
// }
//     export(&shape.mesh, &path)?;

//     return Ok(());
// }

// let watcher = model.load_and_watch(parameters)?;
// run(watcher, shape_processor)?;
