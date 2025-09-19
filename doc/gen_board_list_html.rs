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

const LIST_TEMPLATE: &str = r#"
<ul>
{% for name, board in boards %}
  <li><a href=./{{ name }}.html>{{ board.name }}</a></li>
{% endfor %}
</ul>
"#;

#[derive(argh::FromArgs)]
/// generate the list of supported boards, grouped by support tier
struct Args {
    /// board support tier (1, 2, or 3)
    #[argh(option)]
    tier: schema::SupportTier,
    #[argh(positional)]
    /// path of the input YAML file
    input_file: PathBuf,
    #[argh(positional)]
    /// path of the HTML file to generate
    output_file: PathBuf,
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

    let html = render_html(&matrix, &args.tier)?;

    fs::write(&args.output_file, html).map_err(|source| Error::WritingOutputFile {
        path: args.output_file,
        source,
    })?;

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

fn render_html(
    matrix: &schema::Matrix,
    support_tier: &schema::SupportTier,
) -> Result<String, Error> {
    use minijinja::{context, Environment};

    let boards = matrix
        .boards
        .iter()
        .filter(|(_, info)| info.tier == support_tier.to_string())
        .collect::<Vec<_>>();

    let mut env = Environment::new();
    env.add_template("board_list", LIST_TEMPLATE).unwrap();
    let tmpl = env.get_template("board_list").unwrap();
    let board_list_html = tmpl
        .render(context!(
            boards => boards
        ))
        .unwrap();

    Ok(board_list_html)
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
}
