use anyhow::Context;
use clap::{Parser, Subcommand};
use messiah_resources::Repository;
use tracing::error;
use walkdir::WalkDir;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(
        help = "Input .repository file, we derive all the required files based on that and it's location"
    )]
    repository_file: String,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Arrange files in using the given repository file
    Arrange {
        #[clap(
            help = "Target Directory to arrange files into, defaults to .repository file directory"
        )]
        target_dir: Option<String>,
    },
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let repository_path = std::fs::canonicalize(std::path::Path::new(&args.repository_file))?;

    match args.command {
        Command::Arrange { ref target_dir } => {
            let repository = Repository::from_file(&repository_path)?;

            let repository_path = repository_path.parent().unwrap();

            let target_dir = if let Some(target_dir) = &target_dir {
                std::path::Path::new(target_dir)
            } else {
                repository_path
            };

            for file in repository.files() {
                let file_name = file.hash_file_name();
                let file_name = if !repository_path.join(&file_name).exists() {
                    let mut j = 0;
                    loop {
                        let temp_file_name = format!("{}.{}", file_name, j);
                        if repository_path.join(&temp_file_name).exists() {
                            break temp_file_name;
                        }
                        j += 1;
                        if j > 6 {
                            break file_name;
                        }
                    }
                } else {
                    file_name
                };

                let source = repository_path.join(&file_name);
                let target = target_dir.join(file.file_path());

                std::fs::create_dir_all(&target.parent().unwrap())?;

                if let Err(e) = std::fs::rename(&source, &target)
                    .with_context(|| format!("{} -> {}", source.display(), target.display()))
                {
                    error!("{:?}", e);
                }
            }

            // Cleanup empty directories
            for entry in WalkDir::new(repository_path) {
                let entry = entry?;
                if !entry.path().is_dir() {
                    continue;
                }
                let is_empty = entry
                    .path()
                    .read_dir()
                    .context(entry.path().display().to_string())?
                    .next()
                    .is_none();
                if is_empty {
                    std::fs::remove_dir_all(entry.path())
                        .context(entry.path().display().to_string())?;
                }
            }
        }
    }

    Ok(())
}
