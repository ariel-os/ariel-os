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

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use miette::Diagnostic;
use minijinja::{context, Environment};
use serde::Serialize;

const BOARD_PAGE_TEMPLATE: &str = r#"# {{ board_info.name }}

## Board Info

- **Tier:** {{ board_info.tier }}
- **Ariel OS Name:** `{{ board_info.technical_name }}`
- **Chip:** {{ board_info.chip }}
- **Chip Ariel OS Name:** `{{ board_info.chip_technical_name }}`

### References

- [Manufacturer link]({{ board_info.url }})

## Support Matrix

<table>
  <thead>
    <tr>
      <th>Functionality</th>
      <th>Support Status</th>
    </tr>
  </thead>
  <tbody>
    {%- for functionality in board_info.functionalities %}
    <tr>
      <td>{{ functionality.title }}</td>
      <td class="support-cell" title="{{ functionality.description }}">{{ functionality.icon }}</td>
    </tr>
    {%- endfor %}
  </tbody>
</table>

<style>
.support-cell {
  text-align: center;
}
</style>

Legend:

<dl>
  {%- for support_key in support_keys %}
  <div>
    <dt>{{ support_key.icon }}</dt><dd>{{ support_key.description }}</dd>
  </div>
  {%- endfor %}
</dl>
<style>
dt, dd {
  display: inline;
}
</style>
"#;

const TABLE_TEMPLATE: &str = r##"<!-- This table is auto-generated. Do not edit manually. -->
<table class="support-matrix">
  <thead>
    <tr>
      <th colspan="2">Chip</th>
      <th colspan="2">Testing Board</th>
      <th colspan="{{ matrix.functionalities|length }}">Functionality</th>
    </tr>
    <tr>
      <th>Manufacturer Name</th>
      <th>Ariel OS Name</th>
      <th>Manufacturer Name</th>
      <th>Ariel OS Name</th>
      {%- for functionality in matrix.functionalities %}
      <th>{{ functionality.title }}</th>
      {%- endfor %}
    </tr>
  </thead>
  <tbody>
    {%- for board in boards %}
    <tr>
      <td>{{ board.chip }}</td>
      <td><code>{{ board.chip_technical_name }}</code></td>
      <td><a href="{{ board.url }}">{{ board.name }}</a></td>
      <td><code>{{ board.technical_name }}</code></td>
      {%- for functionality in board.functionalities %}
      <td class="support-cell" title="{{ functionality.description }}">{{ functionality.icon }}</td>
      {%- endfor %}
    </tr>
    {%- endfor %}
  </tbody>
</table>
<style>
@media (min-width: 1920px) {
  .support-matrix {
    position: relative;
    left: 50%;
    transform: translate(-50%, 0);
  }
}
.support-cell {
  text-align: center;
}
</style>
"##;

const KEY_TEMPLATE: &str = r##"<p>Key:</p>

<dl>
  {%- for support_key in matrix.support_keys %}
  <div>
    <dt>{{ support_key.icon }}</dt><dd>{{ support_key.description }}</dd>
  </div>
  {%- endfor %}
</dl>
<style>
dt, dd {
  display: inline;
}
</style>
"##;

#[derive(argh::FromArgs)]
/// utility commands to generate parts of the Ariel OS book
struct Args {
    #[argh(subcommand)]
    command: SubCommand,
}

impl Args {
    fn input_path(&self) -> &Path {
        match self.command {
            SubCommand::Matrix(SubCommandMatrix { ref input_path, .. }) => input_path,
            SubCommand::List(SubCommandList { ref input_path, .. }) => input_path,
            SubCommand::Summary(SubCommandSummary { ref input_path, .. }) => input_path,
            SubCommand::Pages(SubCommandPages { ref input_path, .. }) => input_path,
        }
    }
}

#[derive(argh::FromArgs)]
#[argh(subcommand)]
enum SubCommand {
    Matrix(SubCommandMatrix),
    List(SubCommandList),
    Summary(SubCommandSummary),
    Pages(SubCommandPages),
}

impl SubCommand {
    fn run(self, matrix: &schema::Matrix) -> miette::Result<()> {
        match self {
            Self::Matrix(subcmd_matrix) => subcmd_matrix.run(matrix)?,
            Self::List(subcmd_list) => subcmd_list.run(matrix)?,
            Self::Summary(subcmd_summary) => subcmd_summary.run(matrix)?,
            Self::Pages(subcmd_pages) => subcmd_pages.run(matrix)?,
        }

        Ok(())
    }
}

#[derive(argh::FromArgs)]
#[argh(subcommand, name = "matrix")]
/// generate the HTML support matrix
struct SubCommandMatrix {
    /// just check if the support matrix is up to date
    #[argh(switch)]
    check: bool,
    /// board tier (1, 2, or 3)
    #[argh(option)]
    tier: schema::Tier,
    #[argh(positional)]
    /// path of the input YAML file
    input_path: PathBuf,
    #[argh(positional)]
    /// path of the HTML file to generate
    output_path: PathBuf,
}

impl SubCommandMatrix {
    fn run(self, matrix: &schema::Matrix) -> miette::Result<()> {
        let mut board_info = gen_functionalities(&matrix)?;

        // TODO: read the order from the YAML file instead?
        board_info.sort_unstable_by_key(|b| b.name.to_lowercase());

        let boards = board_info
            .into_iter()
            .filter(|board_support| board_support.tier == self.tier.to_string())
            .collect::<Vec<_>>();

        let mut env = Environment::new();
        env.add_template("matrix", TABLE_TEMPLATE).unwrap();
        env.add_template("matrix_key", KEY_TEMPLATE).unwrap();

        let tmpl = env.get_template("matrix").unwrap();
        let matrix_html = tmpl
            .render(context!(matrix => matrix, boards => boards))
            .unwrap();

        let tmpl = env.get_template("matrix_key").unwrap();
        let key_html = tmpl
            .render(context!(matrix => matrix, boards => boards))
            .unwrap();

        let html = format!("{matrix_html}{key_html}\n");

        if self.check {
            let existing_html = fs::read_to_string(&self.output_path).map_err(|source| {
                Error::ReadingExistingFile {
                    path: self.output_path.clone(),
                    source,
                }
            })?;

            if existing_html != html {
                return Err(Error::ExistingHtmlNotUpToDate {
                    path: self.output_path.clone(),
                }
                .into());
            }
        } else {
            fs::write(&self.output_path, html).map_err(|source| Error::WritingOutputFile {
                path: self.output_path.clone(),
                source,
            })?;
        }

        Ok(())
    }
}

#[derive(argh::FromArgs)]
#[argh(subcommand, name = "board-index")]
/// generate the board index page, featuring a list of supported board grouped by tier
struct SubCommandList {
    /// just check if the board index page is up to date
    #[argh(switch)]
    check: bool,
    #[argh(option)]
    /// path of the board index template
    template_path: PathBuf,
    #[argh(positional)]
    /// path of the input YAML file
    input_path: PathBuf,
    #[argh(positional)]
    /// path of the Markdown file to generate (should be in book/src/boards/index.md)
    output_path: PathBuf,
}

impl SubCommandList {
    fn run(self, matrix: &schema::Matrix) -> miette::Result<()> {
        let board_index_template = fs::read_to_string(&self.template_path).map_err(|source| {
            Error::SummaryTemplateFile {
                path: self.template_path.clone(),
                source,
            }
        })?;

        let mut tier1_boards = matrix
            .boards
            .iter()
            .filter(|(_, info)| info.tier == schema::Tier::Tier1.to_string())
            .collect::<Vec<_>>();
        tier1_boards.sort_unstable_by_key(|b| b.1.name.to_lowercase());

        let mut tier2_boards = matrix
            .boards
            .iter()
            .filter(|(_, info)| info.tier == schema::Tier::Tier2.to_string())
            .collect::<Vec<_>>();
        tier2_boards.sort_unstable_by_key(|b| b.1.name.to_lowercase());

        let mut tier3_boards = matrix
            .boards
            .iter()
            .filter(|(_, info)| info.tier == schema::Tier::Tier3.to_string())
            .collect::<Vec<_>>();
        tier3_boards.sort_unstable_by_key(|b| b.1.name.to_lowercase());

        let mut env = Environment::new();
        env.add_template("board_list", &board_index_template)
            .unwrap();
        let tmpl = env.get_template("board_list").unwrap();
        let board_index_md = tmpl
            .render(context!(
                tier1_boards => tier1_boards,
                tier2_boards => tier2_boards,
                tier3_boards => tier3_boards
            ))
            .unwrap();

        if self.check {
            let existing_board_index_md =
                fs::read_to_string(&self.output_path).map_err(|source| {
                    Error::ReadingExistingFile {
                        path: self.output_path.clone(),
                        source,
                    }
                })?;

            if existing_board_index_md != board_index_md {
                return Err(Error::ExistingHtmlNotUpToDate {
                    path: self.output_path.clone(),
                }
                .into());
            }
        } else {
            fs::write(&self.output_path, board_index_md).map_err(|source| {
                Error::WritingOutputFile {
                    path: self.output_path.clone(),
                    source,
                }
            })?;
        }

        Ok(())
    }
}

#[derive(argh::FromArgs)]
#[argh(subcommand, name = "summary")]
/// generate the book summary with the list of supported boards
struct SubCommandSummary {
    /// just check if the summary is up to date
    #[argh(switch)]
    check: bool,
    #[argh(option)]
    /// path of the template summary
    template_path: PathBuf,
    #[argh(positional)]
    /// path of the input YAML file
    input_path: PathBuf,
    #[argh(positional)]
    /// path of the summary file to generate (should be named SUMMARY.md)
    output_path: PathBuf,
}

impl SubCommandSummary {
    fn run(self, matrix: &schema::Matrix) -> miette::Result<()> {
        let summary_template = fs::read_to_string(&self.template_path).map_err(|source| {
            Error::SummaryTemplateFile {
                path: self.template_path.clone(),
                source,
            }
        })?;

        let mut board_info = gen_functionalities(&matrix)?;

        // TODO: read the order from the YAML file instead?
        board_info.sort_unstable_by_key(|b| b.name.to_lowercase());

        let mut env = Environment::new();
        env.add_template("summary", &summary_template).unwrap();
        let tmpl = env.get_template("summary").unwrap();
        let summary_md = tmpl
            .render(context!(
                boards => board_info,
            ))
            .unwrap();

        if self.check {
            let existing_summary_md = fs::read_to_string(&self.output_path).map_err(|source| {
                Error::ReadingExistingFile {
                    path: self.output_path.clone(),
                    source,
                }
            })?;

            if existing_summary_md != summary_md {
                return Err(Error::ExistingHtmlNotUpToDate {
                    path: self.output_path.clone(),
                }
                .into());
            }
        } else {
            fs::write(&self.output_path, summary_md).map_err(|source| {
                Error::WritingOutputFile {
                    path: self.output_path.clone(),
                    source,
                }
            })?;
        }

        Ok(())
    }
}

#[derive(argh::FromArgs)]
#[argh(subcommand, name = "board-pages")]
/// generate a book page for each supported board
struct SubCommandPages {
    /// just check if the board pages are up to date
    #[argh(switch)]
    check: bool,
    #[argh(positional)]
    /// path of the input YAML file
    input_path: PathBuf,
    #[argh(positional)]
    /// path to the directory to write the generated board pages
    output_path: PathBuf,
}

impl SubCommandPages {
    fn run(self, matrix: &schema::Matrix) -> miette::Result<()> {
        let board_info = gen_functionalities(&matrix)?;

        for board in &board_info {
            let mut env = Environment::new();
            env.add_template("board_page", BOARD_PAGE_TEMPLATE).unwrap();
            let tmpl = env.get_template("board_page").unwrap();
            let board_page = tmpl
                .render(context!(
                    board_info => board,
                    support_keys => matrix.support_keys,
                ))
                .unwrap();

            let mut board_page_path = self.output_path.join(&board.technical_name);
            board_page_path.set_extension("md");

            if self.check {
                let existing_board_page =
                    fs::read_to_string(&board_page_path).map_err(|source| {
                        Error::ReadingExistingFile {
                            path: board_page_path.clone(),
                            source,
                        }
                    })?;

                if existing_board_page != board_page {
                    return Err(Error::ExistingHtmlNotUpToDate {
                        path: board_page_path.clone(),
                    }
                    .into());
                }
            } else {
                fs::write(&board_page_path, board_page).map_err(|source| {
                    Error::WritingOutputFile {
                        path: board_page_path.clone(),
                        source,
                    }
                })?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Serialize)]
struct BoardSupport {
    chip: String,
    chip_technical_name: String,
    url: String,
    technical_name: String,
    name: String,
    tier: String,
    functionalities: Vec<FunctionalitySupport>,
}

#[derive(Debug, Serialize)]
struct FunctionalitySupport {
    title: String,
    icon: String,
    description: String,
    // TODO: add comments
    // TODO: add the PR link
}

fn main() -> miette::Result<()> {
    let args: Args = argh::from_env();

    let input_file = fs::read_to_string(&args.input_path()).map_err(|source| Error::InputFile {
        path: args.input_path().into(),
        source,
    })?;

    let matrix = serde_yaml::from_str(&input_file).map_err(|source| {
        let err_span = miette::SourceSpan::from(source.location().unwrap().index());
        Error::Parsing {
            path: args.input_path().into(),
            src: input_file,
            err_span,
            source,
        }
    })?;

    validate_input(&matrix)?;

    args.command.run(&matrix)
}

fn validate_input(matrix: &schema::Matrix) -> Result<(), Error> {
    let board_errors = matrix.boards.values().flat_map(|b| {
        b.support
            .keys()
            .filter(|f| {
                matrix
                    .functionalities
                    .iter()
                    .all(|functionality| functionality.name != **f)
            })
            .map(|f| Error::InvalidFunctionalityNameBoard {
                found: f.to_string(),
                board: b.name.to_owned(),
            })
    });

    let chip_errors = matrix.chips.values().flat_map(|c| {
        c.support
            .keys()
            .filter(|f| {
                matrix
                    .functionalities
                    .iter()
                    .all(|functionality| functionality.name != **f)
            })
            .map(|f| Error::InvalidFunctionalityNameChip {
                found: f.to_string(),
                chip: c.name.to_owned(),
            })
    });
    let errors = board_errors.chain(chip_errors).collect::<Vec<_>>();

    if errors.is_empty() {
        Ok(())
    } else {
        Err(Error::ValidationIssues { errors })
    }
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

#[derive(Debug, thiserror::Error, Diagnostic)]
enum Error {
    #[error("could not find file `{path}`")]
    InputFile { path: PathBuf, source: io::Error },
    #[error("could not find summary template file `{path}`")]
    SummaryTemplateFile { path: PathBuf, source: io::Error },
    #[error("could not parse YAML file `{path}`")]
    Parsing {
        path: PathBuf,
        #[source_code]
        src: String,
        #[label = "Syntax error"]
        err_span: miette::SourceSpan,
        source: serde_yaml::Error,
    },
    #[error("validation issues")]
    ValidationIssues {
        #[related]
        errors: Vec<Error>,
    },
    #[error("invalid chip name `{found}` for board `{board}`")]
    InvalidChipName { found: String, board: String },
    #[error("invalid functionality name `{found}` for board `{board}`")]
    InvalidFunctionalityNameBoard { found: String, board: String },
    #[error("invalid functionality name `{found}` for chip `{chip}`")]
    InvalidFunctionalityNameChip { found: String, chip: String },
    #[error("invalid support key name `{found}` for functionality `{functionality}` for board `{board}`")]
    InvalidSupportKeyNameBoard {
        found: String,
        functionality: String,
        board: String,
    },
    #[error(
        "invalid support key name `{found}` for functionality `{functionality}` for chip `{chip}`"
    )]
    InvalidSupportKeyNameChip {
        found: String,
        functionality: String,
        chip: String,
    },
    #[error("missing support info on board `{board}` or chip `{chip}` regarding functionality `{functionality}`")]
    MissingSupportInfo {
        board: String,
        chip: String,
        functionality: String,
    },
    #[error("could not write the output HTML file `{path}`")]
    WritingOutputFile { path: PathBuf, source: io::Error },
    #[error("could not read existing output HTML file `{path}`")]
    ReadingExistingFile { path: PathBuf, source: io::Error },
    #[error("existing HTML file `{path}` is not up to date")]
    ExistingHtmlNotUpToDate { path: PathBuf },
}
