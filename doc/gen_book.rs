#!/usr/bin/env -S cargo +nightly -Zscript
---cargo
[package]
edition = "2024"

[dependencies]
argh = { version = "0.1.12" }
miette = { version = "7.2.0", features = ["fancy"] }
minijinja = { version = "2.0.3" }
serde = { version = "1.0.0", features = ["derive"] }
serde_json = { version = "1.0" }
serde_yaml = { version = "0.9.34" } # TODO: use a maintained crate instead
slug = { version = "0.1" }
thiserror = { version = "1.0.61" }
---

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
    /// board tier (1, 2, or 3)
    #[argh(option)]
    tier: schema::Tier,
    #[argh(option)]
    /// path of the support matrix html template
    template_path: PathBuf,
    #[argh(option)]
    /// path of the laze info JSON file
    laze_info_path: PathBuf,
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

        let laze_info = parse_laze_info_json(&self.laze_info_path)?;
        let mut board_info = gen_board_functionalities(&matrix, &laze_info)?;

        // TODO: read the order from the YAML file instead?
        board_info.sort_unstable_by_key(|b| b.name.to_lowercase());

        let boards = board_info
            .into_iter()
            .filter(|board_support| board_support.tier == self.tier.to_string())
            .collect::<Vec<_>>();

        let mut env = Environment::new();
        env.add_template("matrix", &matrix_template).unwrap();
        let tmpl = env.get_template("matrix").unwrap();
        let matrix_html = tmpl
            .render(context!(matrix => matrix, boards => boards))
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
    #[argh(option)]
    /// path of the laze info JSON file
    laze_info_path: PathBuf,
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

        let laze_info = parse_laze_info_json(&self.laze_info_path)?;
        let mut board_info = gen_board_functionalities(&matrix, &laze_info)?;
        board_info.sort_unstable_by_key(|b| b.name.to_lowercase());

        let mut env = Environment::new();
        env.add_template("summary", &summary_template).unwrap();
        let tmpl = env.get_template("summary").unwrap();
        let summary_md = tmpl
            .render(context!(
                boards => board_info,
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
    #[argh(option)]
    /// path of the laze info JSON file
    laze_info_path: PathBuf,
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

        let laze_info = parse_laze_info_json(&self.laze_info_path)?;
        let board_info = gen_board_functionalities(&matrix, &laze_info)?;

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
    template_path: PathBuf,
    #[argh(option)]
    /// path of the laze info JSON file
    laze_info_path: PathBuf,
    #[argh(positional)]
    /// path of the input YAML file
    input_path: PathBuf,
    #[argh(positional)]
    /// path to the directory to write the generated chip pages
    output_path: PathBuf,
}

impl SubCommandChipPages {
    fn run(self, matrix: &schema::Matrix) -> miette::Result<()> {
        let chip_page_template = fs::read_to_string(&self.template_path).map_err(|source| {
            Error::SummaryTemplateFile {
                path: self.template_path.clone(),
                source,
            }
        })?;

        let chip_info = gen_chip_functionalities(&matrix)?;

        for chip in &chip_info {
            let mut env = Environment::new();
            add_custom_filters(&mut env);
            env.add_template("chip_page", &chip_page_template).unwrap();
            let tmpl = env.get_template("chip_page").unwrap();
            let chip_page = tmpl
                .render(context!(
                    chip_info => chip,
                    support_keys => matrix.support_keys,
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

#[derive(Debug, Serialize)]
struct BoardSupport {
    chip: String,
    chip_technical_name: String,
    url: String,
    board_doc: String,
    chip_doc: String,
    technical_name: String,
    name: String,
    tier: String,
    functionalities: Vec<FunctionalitySupport>,
}

#[derive(Debug, Serialize)]
struct ChipSupport {
    technical_name: String,
    name: String,
    functionalities: Vec<FunctionalitySupport>,
}

#[derive(Debug, Serialize)]
struct FunctionalitySupport {
    title: String,
    icon: String,
    description: String,
    comments: Option<Vec<String>>,
    // TODO: add the PR link
}

fn parse_laze_info_json(laze_info_path: &PathBuf) -> miette::Result<LazeInfo> {
    let contents = std::fs::read(&laze_info_path).map_err(|source| Error::LazeInfoFile {
        path: laze_info_path.clone(),
        source,
    })?;

    let laze_info = serde_json::from_slice::<LazeInfo>(&contents).map_err(|source| {
        let src = String::from_utf8(contents).expect("failed to build String from JSON bytes");
        let err_span = miette::SourceOffset::from_location(&src, source.line(), source.column());
        Error::ParsingJson {
            path: laze_info_path.clone(),
            src,
            err_span,
            source,
        }
    })?;

    Ok(laze_info)
}

fn main() -> miette::Result<()> {
    let args: Args = argh::from_env();

    let input_file = fs::read_to_string(&args.input_path()).map_err(|source| Error::InputFile {
        path: args.input_path().into(),
        source,
    })?;

    let matrix = serde_yaml::from_str(&input_file).map_err(|source| {
        let err_span = miette::SourceSpan::from(source.location().unwrap().index());
        Error::ParsingYaml {
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

#[derive(serde::Deserialize)]
struct LazeModule {
    deps: Option<Vec<String>>,
}

#[derive(serde::Deserialize)]
struct LazeApp {
    modules: HashMap<String, LazeModule>,
}

#[derive(serde::Deserialize)]
struct LazeInfo {
    builds: HashMap<String, HashMap<String, LazeApp>>,
}

fn gen_board_functionalities(
    matrix: &schema::Matrix,
    laze_info: &LazeInfo,
) -> Result<Vec<BoardSupport>, Error> {
    let boards = matrix
        .boards
        .iter()
        .map(|(board_technical_name, board_info)| {
            // Implement chip info inheritance
            let chip_info =
                matrix
                    .chips
                    .get(&board_info.chip)
                    .ok_or(vec![Error::InvalidChipName {
                        found: board_info.chip.to_owned(),
                        board: board_info.name.to_owned(),
                    }])?;

            let functionalities = matrix.functionalities.iter().map(|functionality_info| {
                let support_key = if laze_info.builds[board_technical_name].values().any(|app| {
                    app.modules
                        .contains_key(&functionality_info.support_criteria)
                }) {
                    matrix
                        .support_keys
                        .iter()
                        .find(|s| s.name == "supported")
                        // TODO: return error
                        .unwrap()
                } else {
                    matrix
                        .support_keys
                        .iter()
                        .find(|s| s.name == "not_currently_supported")
                        // TODO: return error
                        .unwrap()
                };

                let comments = board_info
                    .support
                    .get(&functionality_info.name)
                    .or_else(|| chip_info.support.get(&functionality_info.name))
                    .and_then(|s| match s {
                        schema::SupportInfo::StatusOnly(_) => None,
                        schema::SupportInfo::Details { comments, .. } => comments.clone(),
                    });

                Ok(FunctionalitySupport {
                    title: functionality_info.title.to_owned(),
                    icon: support_key.icon.to_owned(),
                    description: support_key.description.to_owned(),
                    comments,
                })
            });

            let board_doc_page = ["boards/", board_technical_name, ".html"].concat();
            let chip_doc_page = ["chips/", &board_info.chip, ".html"].concat();
            let errors = functionalities
                .clone()
                .filter_map(|f| f.err())
                .collect::<Vec<_>>();
            if errors.is_empty() {
                Ok(BoardSupport {
                    chip: chip_info.name.to_owned(),
                    chip_technical_name: board_info.chip.to_owned(),
                    url: board_info.url.to_owned(),
                    board_doc: board_doc_page,
                    chip_doc: chip_doc_page,
                    technical_name: board_technical_name.to_owned(),
                    name: board_info.name.to_owned(),
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

fn gen_chip_functionalities(matrix: &schema::Matrix) -> Result<Vec<ChipSupport>, Error> {
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
                        &matrix.boards,
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
            Ok(ChipSupport {
                technical_name: chip_technical_name.to_owned(),
                name: chip_info.name.to_owned(),
                functionalities: functionalities.map(|f| f.unwrap()).collect(),
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
    boards: &HashMap<String, schema::BoardInfo>,
) -> Option<schema::SupportInfo> {
    if boards
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
    #[error("could not find laze info file `{path}`")]
    LazeInfoFile { path: PathBuf, source: io::Error },
    #[error("could not parse YAML file `{path}`")]
    ParsingYaml {
        path: PathBuf,
        #[source_code]
        src: String,
        #[label = "Syntax error"]
        err_span: miette::SourceSpan,
        source: serde_yaml::Error,
    },
    #[error("could not parse JSON file `{path}`")]
    ParsingJson {
        path: PathBuf,
        #[source_code]
        src: String,
        #[label = "{source}"]
        err_span: miette::SourceOffset,
        source: serde_json::Error,
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
    #[error("existing HTML file `{path}` is not up to date. To update the book, run `laze build update-book`")]
    ExistingHtmlNotUpToDate { path: PathBuf },
    #[error("existing Markdown file `{path}` is not up to date. To update the book, run `laze build update-book`")]
    ExistingMarkdownNotUpToDate { path: PathBuf },
}
