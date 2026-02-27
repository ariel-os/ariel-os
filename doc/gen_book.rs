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
slug = { version = "0.1" }
thiserror = { version = "1.0.61" }
---

// TODO: remove when upstream fixes issue
#![allow(unused)]

mod schema;

use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

use miette::Diagnostic;
use minijinja::{context, Environment};
use serde::Serialize;
use slug::slugify;

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
            SubCommand::ChipIndex(SubCommandChipIndex { ref input_path, .. }) => input_path,
            SubCommand::Summary(SubCommandSummary { ref input_path, .. }) => input_path,
            SubCommand::BoardPages(SubCommandBoardPages { ref input_path, .. }) => input_path,
            SubCommand::ChipPages(SubCommandChipPages { ref input_path, .. }) => input_path,
        }
    }
}

#[derive(argh::FromArgs)]
#[argh(subcommand)]
enum SubCommand {
    Matrix(SubCommandMatrix),
    List(SubCommandList),
    ChipIndex(SubCommandChipIndex),
    Summary(SubCommandSummary),
    BoardPages(SubCommandBoardPages),
    ChipPages(SubCommandChipPages),
}

impl SubCommand {
    fn run(self, matrix: &schema::Matrix) -> miette::Result<()> {
        match self {
            Self::Matrix(subcmd_matrix) => subcmd_matrix.run(matrix)?,
            Self::List(subcmd_list) => subcmd_list.run(matrix)?,
            Self::ChipIndex(subcmd_chip_index) => subcmd_chip_index.run(matrix)?,
            Self::Summary(subcmd_summary) => subcmd_summary.run(matrix)?,
            Self::BoardPages(subcmd_board_pages) => subcmd_board_pages.run(matrix)?,
            Self::ChipPages(subcmd_chip_pages) => subcmd_chip_pages.run(matrix)?,
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
    #[argh(option)]
    /// path of the support matrix html template
    template_path: PathBuf,
    #[argh(positional)]
    /// path of the input YAML file
    input_path: PathBuf,
    #[argh(positional)]
    /// path of the HTML file to generate
    output_path: PathBuf,
}

impl SubCommandMatrix {
    fn run(self, matrix: &schema::Matrix) -> miette::Result<()> {
        let matrix_template = fs::read_to_string(&self.template_path).map_err(|source| {
            Error::SummaryTemplateFile {
                path: self.template_path.clone(),
                source,
            }
        })?;

        let mut board_info = gen_board_functionalities(&matrix)?;

        for board in board_info.iter_mut() {
            board
                .builders
                .sort_unstable_by_key(|builder| builder.tier.clone());
        }
        board_info.sort_unstable_by_key(|board| {
            (
                board
                    .builders
                    .iter()
                    .min_by_key(|builder| builder.tier.clone())
                    .unwrap()
                    .tier
                    .clone(),
                board.name.to_lowercase(),
            )
        });

        let mut env = Environment::new();
        env.add_template("matrix", &matrix_template).unwrap();
        let tmpl = env.get_template("matrix").unwrap();
        let matrix_html = tmpl
            .render(context!(matrix => matrix, boards => board_info, url_prefix => ""))
            .unwrap();

        if self.check {
            let existing_matrix_html = fs::read_to_string(&self.output_path).map_err(|source| {
                Error::ReadingExistingFile {
                    path: self.output_path.clone(),
                    source,
                }
            })?;

            if existing_matrix_html != matrix_html {
                return Err(Error::ExistingHtmlNotUpToDate {
                    path: self.output_path.clone(),
                }
                .into());
            }
        } else {
            fs::write(&self.output_path, matrix_html).map_err(|source| {
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

        let mut env = Environment::new();
        add_custom_filters(&mut env);
        env.add_template("board_list", &board_index_template)
            .unwrap();
        let tmpl = env.get_template("board_list").unwrap();
        let board_index_md = tmpl
            .render(context!(
                boards => &matrix.boards,
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
                return Err(Error::ExistingMarkdownNotUpToDate {
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
#[argh(subcommand, name = "chip-index")]
/// generate the chip index page, featuring a list of supported chips
struct SubCommandChipIndex {
    /// just check if the chip index page is up to date
    #[argh(switch)]
    check: bool,
    #[argh(option)]
    /// path of the chip index template
    template_path: PathBuf,
    #[argh(positional)]
    /// path of the input YAML file
    input_path: PathBuf,
    #[argh(positional)]
    /// path of the Markdown file to generate (should be in book/src/chips/index.md)
    output_path: PathBuf,
}

#[derive(Serialize)]
struct ChipIndex {
    name: String,
    technical_name: String,
    manufacturer: String,
}

impl SubCommandChipIndex {
    fn run(self, matrix: &schema::Matrix) -> miette::Result<()> {
        let chip_index_template = fs::read_to_string(&self.template_path).map_err(|source| {
            Error::SummaryTemplateFile {
                path: self.template_path.clone(),
                source,
            }
        })?;

        let chips: Vec<_> = matrix
            .chips
            .iter()
            .map(|(k, v)| ChipIndex {
                name: v.name.to_owned(),
                technical_name: k.to_owned(),
                manufacturer: v.manufacturer.to_owned(),
            })
            .collect();

        let mut env = Environment::new();
        env.add_template("chip_list", &chip_index_template).unwrap();
        let tmpl = env.get_template("chip_list").unwrap();
        let chip_index_md = tmpl
            .render(context!(
                chips => chips,
            ))
            .unwrap();

        if self.check {
            let existing_chip_index_md =
                fs::read_to_string(&self.output_path).map_err(|source| {
                    Error::ReadingExistingFile {
                        path: self.output_path.clone(),
                        source,
                    }
                })?;

            if existing_chip_index_md != chip_index_md {
                return Err(Error::ExistingMarkdownNotUpToDate {
                    path: self.output_path.clone(),
                }
                .into());
            }
        } else {
            fs::write(&self.output_path, chip_index_md).map_err(|source| {
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
/// generate the book summary with the list of supported boards and chips
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

        let mut env = Environment::new();
        add_custom_filters(&mut env);
        env.add_template("summary", &summary_template).unwrap();
        let tmpl = env.get_template("summary").unwrap();
        let summary_md = tmpl
            .render(context!(
                boards => matrix.boards,
                chips => matrix.chips,
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
                return Err(Error::ExistingMarkdownNotUpToDate {
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
struct SubCommandBoardPages {
    /// just check if the board pages are up to date
    #[argh(switch)]
    check: bool,
    #[argh(option)]
    /// path of the template summary
    template_path: PathBuf,
    #[argh(positional)]
    /// path of the input YAML file
    input_path: PathBuf,
    #[argh(positional)]
    /// path to the directory to write the generated board pages
    output_path: PathBuf,
}

impl SubCommandBoardPages {
    fn run(self, matrix: &schema::Matrix) -> miette::Result<()> {
        let board_page_template = fs::read_to_string(&self.template_path).map_err(|source| {
            Error::SummaryTemplateFile {
                path: self.template_path.clone(),
                source,
            }
        })?;

        let board_info = gen_board_functionalities(&matrix)?;

        for board in &board_info {
            let mut env = Environment::new();
            add_custom_filters(&mut env);
            env.add_template("board_page", &board_page_template)
                .unwrap();
            let tmpl = env.get_template("board_page").unwrap();
            let board_page = tmpl
                .render(context!(
                    board_info => board,
                    support_keys => matrix.support_keys,
                ))
                .unwrap();

            let mut board_page_path = self.output_path.join(&slugify(&board.name));
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
                    return Err(Error::ExistingMarkdownNotUpToDate {
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

#[derive(argh::FromArgs)]
#[argh(subcommand, name = "chip-pages")]
/// generate a book page for each supported chip
struct SubCommandChipPages {
    /// just check if the chip pages are up to date
    #[argh(switch)]
    check: bool,
    #[argh(option)]
    /// path of the template summary
    chip_template_path: PathBuf,
    #[argh(option)]
    /// path of the support matrix html template
    matrix_template_path: PathBuf,
    #[argh(positional)]
    /// path of the input YAML file
    input_path: PathBuf,
    #[argh(positional)]
    /// path to the directory to write the generated chip pages
    output_path: PathBuf,
}

impl SubCommandChipPages {
    fn run(self, matrix: &schema::Matrix) -> miette::Result<()> {
        let matrix_template = fs::read_to_string(&self.matrix_template_path).map_err(|source| {
            Error::SummaryTemplateFile {
                path: self.matrix_template_path.clone(),
                source,
            }
        })?;
        let chip_page_template =
            fs::read_to_string(&self.chip_template_path).map_err(|source| {
                Error::SummaryTemplateFile {
                    path: self.chip_template_path.clone(),
                    source,
                }
            })?;

        let chip_info = gen_chip_functionalities(&matrix)?;
        let mut board_info = gen_board_functionalities(&matrix)?;

        // sort by tier
        for board in board_info.iter_mut() {
            board
                .builders
                .sort_unstable_by_key(|builder| builder.tier.clone());
        }
        board_info.sort_unstable_by_key(|board| {
            (
                board
                    .builders
                    .iter()
                    .min_by_key(|builder| builder.tier.clone())
                    .unwrap()
                    .tier
                    .clone(),
                board.name.to_lowercase(),
            )
        });

        for chip in &chip_info {
            let board_matrix_html = {
                let current_boards: Vec<BoardInfo> = board_info
                    .iter()
                    .filter_map(|b| {
                        let builders: Vec<LazeBuilder> = b
                            .builders
                            .iter()
                            .filter(|builder| builder.chip_technical_name == chip.technical_name)
                            .cloned()
                            .collect();
                        if builders.len() > 0 {
                            Some(BoardInfo {
                                builders,
                                ..b.clone()
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                if current_boards.len() == 0 {
                    "".to_string()
                } else {
                    let mut env = Environment::new();
                    env.add_template("matrix", &matrix_template).unwrap();
                    let tmpl = env.get_template("matrix").unwrap();
                    tmpl.render(
                        context!(matrix => matrix, boards => current_boards, url_prefix => "../" ),
                    )
                    .unwrap()
                }
            };

            let mut env = Environment::new();
            add_custom_filters(&mut env);
            env.add_template("chip_page", &chip_page_template).unwrap();
            let tmpl = env.get_template("chip_page").unwrap();
            let chip_page = tmpl
                .render(context!(
                    chip_info => chip,
                    support_keys => matrix.support_keys,
                    board_matrix_html => board_matrix_html,
                ))
                .unwrap();

            let mut chip_page_path = self.output_path.join(&chip.technical_name);
            chip_page_path.set_extension("md");

            if self.check {
                let existing_chip_page = fs::read_to_string(&chip_page_path).map_err(|source| {
                    Error::ReadingExistingFile {
                        path: chip_page_path.clone(),
                        source,
                    }
                })?;

                if existing_chip_page != chip_page {
                    return Err(Error::ExistingMarkdownNotUpToDate {
                        path: chip_page_path.clone(),
                    }
                    .into());
                }
            } else {
                fs::write(&chip_page_path, chip_page).map_err(|source| {
                    Error::WritingOutputFile {
                        path: chip_page_path.clone(),
                        source,
                    }
                })?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Clone)]
struct BoardInfo {
    url: String,
    board_doc: String,
    name: String,
    builders: Vec<LazeBuilder>,
}

#[derive(Debug, Serialize)]
struct ChipInfo {
    technical_name: String,
    name: String,
    functionalities: Vec<FunctionalitySupport>,
    prose: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
struct LazeBuilder {
    chip: String,
    chip_technical_name: String,
    chip_doc: String,
    tier: String,
    technical_name: String,
    functionalities: Vec<FunctionalitySupport>,
    prose: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
struct FunctionalitySupport {
    title: String,
    icon: String,
    description: String,
    comments: Option<Vec<String>>,
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
    let board_errors = matrix
        .builders
        .iter()
        .flat_map(|(builder_technical_name, builder_info)| {
            builder_info
                .support
                .keys()
                .filter(|f| {
                    matrix
                        .functionalities
                        .iter()
                        .all(|functionality| functionality.name != **f)
                })
                .map(|f| Error::InvalidFunctionalityNameBoard {
                    found: f.to_string(),
                    board: builder_technical_name.to_owned(),
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

fn gen_board_functionalities(matrix: &schema::Matrix) -> Result<Vec<BoardInfo>, Error> {
    let boards =
        matrix.boards.iter().map(|board_info| {
            let builders =
                board_info.builders.iter().map(|builder_technical_name| {
                    let builder_info = matrix.builders.get(builder_technical_name).ok_or(vec![
                        Error::InvalidBuilderName {
                            found: builder_technical_name.to_owned(),
                            board: board_info.name.to_owned(),
                        },
                    ])?;

                    // Implement chip info inheritance
                    let chip_info = matrix.chips.get(&builder_info.chip).ok_or(vec![
                        Error::InvalidChipName {
                            found: builder_info.chip.to_owned(),
                            board: board_info.name.to_owned(),
                        },
                    ])?;

                    let functionalities = matrix.functionalities.iter().map(|functionality_info| {
                        let name = &functionality_info.name;

                        let (support_info, support_key) =
                            if let Some(support_info) = builder_info.support.get(name) {
                                let status = support_info.status();
                                (
                                    support_info,
                                    matrix
                                        .support_keys
                                        .iter()
                                        .find(|s| s.name == status)
                                        .ok_or(Error::InvalidSupportKeyNameBoard {
                                            found: status.to_owned(),
                                            functionality: name.to_owned(),
                                            board: board_info.name.to_owned(),
                                        })?,
                                )
                            } else {
                                let support_info = chip_info.support.get(name).ok_or(
                                    Error::MissingSupportInfo {
                                        board: board_info.name.to_owned(),
                                        chip: builder_info.chip.to_owned(),
                                        functionality: functionality_info.title.to_owned(),
                                    },
                                )?;
                                let status = support_info.status();
                                (
                                    support_info,
                                    matrix
                                        .support_keys
                                        .iter()
                                        .find(|s| s.name == status)
                                        .ok_or(Error::InvalidSupportKeyNameChip {
                                            found: status.to_owned(),
                                            functionality: name.to_owned(),
                                            chip: chip_info.name.to_owned(),
                                        })?,
                                )
                            };

                        let comments = match support_info {
                            schema::SupportInfo::StatusOnly(_) => None,
                            schema::SupportInfo::Details { comments, .. } => comments.clone(),
                        };

                        Ok(FunctionalitySupport {
                            title: functionality_info.title.to_owned(),
                            icon: support_key.icon.to_owned(),
                            description: support_key.description.to_owned(),
                            comments,
                        })
                    });
                    let chip_doc_page = ["chips/", &builder_info.chip, ".html"].concat();
                    let errors = functionalities
                        .clone()
                        .filter_map(|f| f.err())
                        .collect::<Vec<_>>();
                    if errors.is_empty() {
                        Ok(LazeBuilder {
                            chip: chip_info.name.to_owned(),
                            chip_technical_name: builder_info.chip.to_owned(),
                            chip_doc: chip_doc_page,
                            tier: builder_info.tier.to_owned(),
                            technical_name: builder_technical_name.to_owned(),
                            functionalities: functionalities.map(|f| f.unwrap()).collect(),
                            prose: builder_info.prose.clone(),
                        })
                    } else {
                        Err(errors)
                    }
                });

            let board_doc_page = ["boards/", &slugify(&board_info.name), ".html"].concat();
            let errors = builders
                .clone()
                .filter_map(|f| f.err())
                .flatten()
                .collect::<Vec<_>>();
            if errors.is_empty() {
                Ok(BoardInfo {
                    url: board_info.url.to_owned(),
                    board_doc: board_doc_page,
                    name: board_info.name.to_owned(),
                    builders: builders.map(|b| b.unwrap()).collect(),
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

fn gen_chip_functionalities(matrix: &schema::Matrix) -> Result<Vec<ChipInfo>, Error> {
    let chips = matrix.chips.iter().map(|(chip_technical_name, chip_info)| {
        let functionalities = matrix.functionalities.iter().map(|functionality_info| {
            let support_info = chip_info
                .support
                .get(&functionality_info.name)
                .cloned()
                .or_else(|| {
                    get_chip_functionality_support(
                        &chip_technical_name,
                        &functionality_info.name,
                        &matrix.builders,
                    )
                })
                .ok_or_else(|| Error::MissingSupportInfo {
                    functionality: functionality_info.name.to_owned(),
                    chip: chip_info.name.to_owned(),
                    board: String::new(),
                })?;

            let support_key = matrix
                .support_keys
                .iter()
                .find(|s| s.name == support_info.status())
                .ok_or_else(|| Error::InvalidSupportKeyNameChip {
                    functionality: functionality_info.name.to_owned(),
                    chip: chip_info.name.to_owned(),
                    found: support_info.status().to_owned(),
                })?;

            let comments = match support_info {
                schema::SupportInfo::StatusOnly(_) => None,
                schema::SupportInfo::Details { comments, .. } => comments,
            };

            Ok(FunctionalitySupport {
                title: functionality_info.title.to_owned(),
                icon: support_key.icon.to_owned(),
                description: support_key.description.to_owned(),
                comments,
            })
        });

        let errors = functionalities
            .clone()
            .filter_map(|f| f.err())
            .collect::<Vec<_>>();

        if errors.is_empty() {
            Ok(ChipInfo {
                technical_name: chip_technical_name.to_owned(),
                name: chip_info.name.to_owned(),
                functionalities: functionalities.map(|f| f.unwrap()).collect(),
                prose: chip_info.prose.clone(),
            })
        } else {
            Err(errors)
        }
    });

    let errors = chips
        .clone()
        .filter_map(|f| f.err())
        .flatten()
        .collect::<Vec<_>>();
    if errors.is_empty() {
        Ok(chips.map(|f| f.unwrap()).collect())
    } else {
        Err(Error::ValidationIssues { errors })
    }
}

fn get_chip_functionality_support(
    chip: &str,
    functionality: &str,
    builders: &HashMap<String, schema::BuilderInfo>,
) -> Option<schema::SupportInfo> {
    if builders
        .iter()
        .filter(|(_, info)| info.chip == chip)
        .any(|(_, info)| {
            info.support
                .get(functionality)
                .map(|s| s.status() == "supported" || s.status() == "supported_with_caveats")
                .unwrap_or(false)
        })
    {
        Some(schema::SupportInfo::StatusOnly("supported".to_owned()))
    } else {
        None
    }
}

fn capitalize_first(value: String) -> String {
    let mut chars = value.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn add_custom_filters(env: &mut Environment) {
    env.add_filter("slugify", slugify::<String>);
    env.add_filter("capitalize_first", capitalize_first);
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
    #[error("invalid laze builder `{found}` for board `{board}`")]
    InvalidBuilderName { found: String, board: String },
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
    #[error("existing HTML file `{path}` is not up to date. To update the book, run `laze build update-book`")]
    ExistingHtmlNotUpToDate { path: PathBuf },
    #[error("existing Markdown file `{path}` is not up to date. To update the book, run `laze build update-book`")]
    ExistingMarkdownNotUpToDate { path: PathBuf },
}
