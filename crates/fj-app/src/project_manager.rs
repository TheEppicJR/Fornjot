use crate::{args::Args, config::Config};
use anyhow::{anyhow, Context as _};
use fj_host::Model;
use std::path::PathBuf;

pub struct OpenProjects {
    files: Vec<Project>,
}

pub struct Project {
    file_path: PathBuf,
    model: Model,
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

        let loaded_model = Model::from_path(path.clone(), config.target_dir)
            .with_context(|| {
                format!("Failed to load model: {}", path.display())
            })?;
        Ok(Project {
            file_path: path.clone(),
            model: loaded_model,
        })
    }
}
