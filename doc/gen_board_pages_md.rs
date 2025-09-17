#!/usr/bin/env -S cargo +nightly -Zscript
---cargo
[package]
edition = "2024"

[dependencies]
argh = { version = "0.1.12" }
miette = { version = "7.2.0", features = ["fancy"] }
minijinja = { version = "2.0.3" }
serde = { version = "1.0.0", features = ["derive"] }
serde_yaml = { version = "0.9.34" } # TODO: use a maintained crate instead
thiserror = { version = "1.0.61" }
---

mod schema;

use std::{fs, io, path::PathBuf};

use miette::Diagnostic;
use serde::Serialize;

const BOARD_PAGE_TEMPLATE: &str = r#"
# {{ board_info.name }}

## Support Matrix

|Feature|Support Status|
|---|---|
|Support Tier|{{ board_info.tier }}|
|Chip|{{ board_info.chip }}|
|Ariel OS Name|`{{ board_info.technical_name }}`|
|Flashing|TODO|
{%- for f in board_info.functionalities %}
|{{ f.title }}|{{ f.icon}}|
{%- endfor %}

Legend:

{%- for s in support_keys %}
  {{ s.icon }} : {{ s.description }}
{%- endfor %}

## References

- [Manufacturer link]({{ board_info.url }})
"#;

#[derive(argh::FromArgs)]
/// generate a book page for each board we support
struct Args {
    #[argh(positional)]
    /// path of the input YAML file
    input_file: PathBuf,
    #[argh(positional)]
    /// path of the directory in which generated pages will be written to
    output_dir: PathBuf,
}

fn main() -> miette::Result<()> {
    let args: Args = argh::from_env();

    let input_file = fs::read_to_string(&args.input_file).map_err(|source| Error::InputFile {
        path: args.input_file.clone(),
        source,
    })?;

    let matrix = serde_yaml::from_str(&input_file).map_err(|source| {
        let err_span = miette::SourceSpan::from(source.location().unwrap().index());
        Error::Parsing {
            path: args.input_file,
            src: input_file,
            err_span,
            source,
        }
    })?;

    validate_input(&matrix)?;

    let board_info = gen_functionalities(&matrix)?;

    for board in &board_info {
        let board_page = render_board_page(board, &matrix.support_keys);

        let mut output_path = args.output_dir.join(&board.technical_name);
        output_path.set_extension("md");
        fs::write(&output_path, board_page).map_err(|source| Error::WritingOutputFile {
            path: output_path,
            source,
        })?;
    }

    Ok(())
}

fn validate_input(matrix: &schema::Matrix) -> Result<(), Error> {
    for (_, board_info) in &matrix.boards {
        let invalid_functionality_name = board_info.support.keys().find(|f| {
            matrix
                .functionalities
                .iter()
                .all(|functionality| functionality.name != **f)
        });

        if let Some(f) = invalid_functionality_name {
            return Err(Error::InvalidFunctionalityNameBoard {
                found: f.to_owned(),
                board: board_info.name.to_owned(),
            });
        }
    }

    for (_, chip_info) in &matrix.chips {
        let invalid_functionality_name = chip_info.support.keys().find(|f| {
            matrix
                .functionalities
                .iter()
                .all(|functionality| functionality.name != **f)
        });

        if let Some(f) = invalid_functionality_name {
            return Err(Error::InvalidFunctionalityNameChip {
                found: f.to_owned(),
                chip: chip_info.name.to_owned(),
            });
        }
    }

    Ok(())
}

#[derive(Serialize)]
struct BoardSupport {
    chip: String,
    chip_technical_name: String,
    url: String,
    technical_name: String,
    name: String,
    tier: String,
    functionalities: Vec<FunctionalitySupport>,
}

#[derive(Serialize)]
struct FunctionalitySupport {
    title: String,
    icon: String,
    description: String,
    // TODO: add comments
    // TODO: add the PR link
}

fn gen_functionalities(matrix: &schema::Matrix) -> Result<Vec<BoardSupport>, Error> {
    let boards = matrix
        .boards
        .iter()
        .map(|(board_technical_name, board_info)| {
            let board_name = &board_info.name;
            let chip = &board_info.chip;

            // Implement chip info inheritance
            let chip_info = matrix.chips.get(chip).ok_or(vec![Error::InvalidChipName {
                found: chip.to_owned(),
                board: board_name.to_owned(),
            }])?;

            let functionalities = matrix.functionalities.iter().map(|functionality_info| {
                let name = &functionality_info.name;

                let support_key = if let Some(support_info) = board_info.support.get(name) {
                    let status = support_info.status();
                    matrix
                        .support_keys
                        .iter()
                        .find(|s| s.name == status)
                        .ok_or(Error::InvalidSupportKeyNameBoard {
                            found: status.to_owned(),
                            functionality: name.to_owned(),
                            board: board_name.to_owned(),
                        })?
                } else {
                    let support_info =
                        chip_info
                            .support
                            .get(name)
                            .ok_or(Error::MissingSupportInfo {
                                board: board_name.to_owned(),
                                chip: board_info.chip.to_owned(),
                                functionality: functionality_info.title.to_owned(),
                            })?;
                    let status = support_info.status();
                    matrix
                        .support_keys
                        .iter()
                        .find(|s| s.name == status)
                        .ok_or(Error::InvalidSupportKeyNameChip {
                            found: status.to_owned(),
                            functionality: name.to_owned(),
                            chip: chip_info.name.to_owned(),
                        })?
                };

                Ok(FunctionalitySupport {
                    title: functionality_info.title.to_owned(),
                    icon: support_key.icon.to_owned(),
                    description: support_key.description.to_owned(),
                })
            });
            let errors = functionalities
                .clone()
                .filter_map(|f| f.err())
                .collect::<Vec<_>>();
            if errors.is_empty() {
                Ok(BoardSupport {
                    chip: chip_info.name.to_owned(),
                    chip_technical_name: board_info.chip.to_owned(),
                    url: board_info.url.to_owned(),
                    technical_name: board_technical_name.to_owned(),
                    name: board_name.to_owned(),
                    tier: board_info.tier.to_owned(),
                    functionalities: functionalities.map(|f| f.unwrap()).collect(),
                })
            } else {
                Err(errors)
            }
        });

    let errors = boards
        .clone()
        .filter_map(|f| f.err())
        .flatten()
        .collect::<Vec<_>>();
    if errors.is_empty() {
        Ok(boards.map(|f| f.unwrap()).collect())
    } else {
        Err(Error::ValidationIssues { errors })
    }
}

fn render_board_page(board_info: &BoardSupport, support_keys: &[schema::SupportKeyInfo]) -> String {
    use minijinja::{context, Environment};

    let mut env = Environment::new();
    env.add_template("board_page", BOARD_PAGE_TEMPLATE).unwrap();
    let tmpl = env.get_template("board_page").unwrap();
    tmpl.render(context!(
        board_info => board_info,
        support_keys => support_keys,
    ))
    .unwrap()
}

#[derive(Debug, thiserror::Error, Diagnostic)]
enum Error {
    #[error("could not find file `{path}`")]
    InputFile { path: PathBuf, source: io::Error },
    #[error("could not parse YAML file `{path}`")]
    Parsing {
        path: PathBuf,
        #[source_code]
        src: String,
        #[label = "Syntax error"]
        err_span: miette::SourceSpan,
        source: serde_yaml::Error,
    },
    #[error("invalid functionality name `{found}` for board `{board}`")]
    InvalidFunctionalityNameBoard { found: String, board: String },
    #[error("invalid functionality name `{found}` for chip `{chip}`")]
    InvalidFunctionalityNameChip { found: String, chip: String },
    #[error("could not write the output HTML file `{path}`")]
    WritingOutputFile { path: PathBuf, source: io::Error },
    #[error("validation issues")]
    ValidationIssues {
        #[related]
        errors: Vec<Error>,
    },
    #[error("invalid chip name `{found}` for board `{board}`")]
    InvalidChipName { found: String, board: String },
    #[error("invalid support key name `{found}` for functionality `{functionality}` for board `{board}`")]
    InvalidSupportKeyNameBoard {
        found: String,
        functionality: String,
        board: String,
    },
    #[error("missing support info on board `{board}` or chip `{chip}` regarding functionality `{functionality}`")]
    MissingSupportInfo {
        board: String,
        chip: String,
        functionality: String,
    },
    #[error(
        "invalid support key name `{found}` for functionality `{functionality}` for chip `{chip}`"
    )]
    InvalidSupportKeyNameChip {
        found: String,
        functionality: String,
        chip: String,
    },
}
