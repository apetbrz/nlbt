mod budget;
use budget::Budget;
use error::Result;
mod commands;
mod error;
mod fileio;
mod ui;

use clap::*;
use commands::parse_input;

fn main() -> Result<()> {
    //get arguments
    let args = parse_args();

    //arguments -> options and commands
    let (app_settings, account_options, budget_commands) = parse_input(args)?;

    //move to working directory
    fileio::relocate_to_application_dir()?;

    //load or generate a budget
    let mut bud = match app_settings.mem_only {
        true => Budget::new(&account_options.account),
        false => match account_options.create {
            true => fileio::create_new_budget_account(&account_options.account)?,
            false => fileio::load_budget_account(&account_options.account)?,
        },
    };

    if let Some(new_name) = account_options.default_rename {
        fileio::change_default_account_display_name(&new_name)?
    };

    //process user commands
    match app_settings.interactive_mode {
        true => ui::run_interactive(&mut bud),
        false => bud.execute(budget_commands, app_settings.force),
    }?;

    //output after processing
    if app_settings.verbosity > 0 {
        match app_settings.json {
            true => todo!("json output"),
            false => {
                println!("{bud}");
            }
        }
    }

    //save changes
    if !(app_settings.mem_only || app_settings.dry_run) {
        fileio::save_budget_to_account_file(bud)?;
    }

    Ok(())
}

fn parse_args() -> ArgMatches {
    Command::new("nclbt")
        .version(crate_version!())
        .author(crate_authors!())
        .about("nice command line budget tool")
        .arg(
            Arg::new("account")
                .short('A')
                .long("account")
                .num_args(1)
                .help("Select account to load/modify")
                .long_help(
                    "Load an account file. \
                    If ommited, uses a default user account.",
                ),
        )
        .arg(
            Arg::new("pay")
                .short('p')
                .long("pay")
                .action(ArgAction::Append)
                .num_args(1..=2)
                .value_names(["expense", "[amount]"])
                .help("Pay an expense")
                .long_help(
                    "Requires an expense \
                    and optionally an amount. \
                    Without an amount, pay a given expense in full.",
                ),
        )
        .arg(
            Arg::new("edit")
                .short('e')
                .long("edit")
                .action(ArgAction::Append)
                .num_args(2..=3)
                .value_names(["expense", "modification"])
                .help("Edit an existing expense")
                .long_help(
                    "Requires at least two values: \
                    an existing expense and one or both of: \
                    a string to rename it to \
                    and a dollar amount to change it to.",
                ),
        )
        .arg(
            Arg::new("new")
                .short('n')
                .long("new")
                .action(ArgAction::Append)
                .num_args(2)
                .value_names(["expense", "amount"])
                .help("Create a new expense")
                .long_help(
                    "Creates a new expense with \
                    the provided name and amount.",
                ),
        )
        .arg(
            Arg::new("paid")
                .short('P')
                .long("paid")
                .action(ArgAction::Append)
                .num_args(0..=1)
                .default_missing_value(None)
                .help("Get paid")
                .long_help(
                    "Get paid, either a provided amount or the fixed \
                    income set by -C.",
                ),
        )
        .arg(
            Arg::new("paycheck")
                .short('C')
                .long("set-paycheck")
                .num_args(1)
                .help("Set paycheck amount")
                .long_help(
                    "Set the fixed income paycheck for the current account. \
                    Used whenever -P is present without a value.",
                ),
        )
        .arg(
            Arg::new("clear")
                .short('c')
                .long("clear")
                .num_args(0..)
                .value_names(["expense"])
                .help("Clear amount(s) paid to expense(s)")
                .long_help(
                    "Resets amounts paid to expenses to zero. \
                    If given without any names, resets all, otherwise only clears \
                    provided expense names. Supports \"!<name>\" for inverted filtering.",
                ),
        )
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .action(ArgAction::Count)
                .help("Force payments")
                .long_help(
                    "If present, skips the confirmation dialogue. Use twice to skip the \
                    \"Cannot afford\" confirmation message and use remamining balance \
                    for payments. Use thrice to enable overdrafting.",
                ),
        )
        .arg(
            Arg::new("default_name")
                .short('D')
                .long("set-default-name")
                .num_args(1)
                .help("Set default account username")
                .long_help(
                    "Modifies the name presented when the \
                    default user account is used.",
                ),
        )
        .arg(
            Arg::new("new_account")
                .short('N')
                .long("new-account")
                .conflicts_with("account")
                .num_args(1)
                .help("Create a new account")
                .long_help("Creates and selects a new account with the given name."),
        )
        .arg(
            Arg::new("dry_run")
                .short('X')
                .long("dry-run")
                .action(ArgAction::SetTrue)
                .help("Save no changes")
                .long_help(
                    "Perform and output changes but do not save to file. \
                    Redundant with -m.",
                ),
        )
        .arg(
            Arg::new("interactive")
                .short('i')
                .long("interactive")
                .action(ArgAction::SetTrue)
                .help("Enable the interactive interface")
                .long_help(
                    "Interactive mode. Overrides all budget command arguments. \
                    Implies -q, as it will not output upon exit. \
                    Include -v once to reverse this.",
                ),
        )
        .arg(
            Arg::new("mem_only")
                .short('m')
                .long("mem-only")
                .action(ArgAction::SetTrue)
                .help("Run without a save file")
                .long_help(
                    "Effectively creates a new, blank account, \
                    which can be interacted with normally, but \
                    which does not get saved to disk.",
                ),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .action(ArgAction::SetTrue)
                .help("Silence output")
                .long_help("Silence output. Effectively counts as a negative verbose flag."),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::Count)
                .help("Increase detail of output")
                .long_help(
                    "Adds additional information upon output, such as \
                    modifications made.",
                ),
        )
        .arg(
            Arg::new("json")
                .short('j')
                .long("json")
                .action(ArgAction::SetTrue)
                .help("Output as json")
                .long_help("Replaces output with a JSON object. Compatible with -v for more data."),
        )
        .get_matches()
}
