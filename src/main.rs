mod budget;
use budget::Budget;
mod fileio;
mod ui;

use anyhow::Result;
use clap::*;

fn main() -> Result<()> {
    //get arguments
    let args = parse_args();

    //flags
    let mem_only: bool = args.get_flag("mem_only");
    let interactive_mode: bool = args.get_flag("interactive");
    let verbosity: u8 =
        1 + args.get_count("verbose") - (interactive_mode || args.get_flag("quiet")) as u8;
    let json_output: bool = args.get_flag("json");

    //account info
    let account: Option<&String> = args.get_one("account");
    let new_default_name: Option<&String> = args.get_one("default_name");

    //move to working directory
    fileio::relocate_to_application_dir()?;

    //load or generate a budget
    let mut bud = match mem_only {
        true => {
            let display_name = account.or(new_default_name).map_or("user", |v| v);
            Budget::new(display_name)
        }
        false => fileio::load_budget_account(account.map(String::as_str)).unwrap(),
    };

    //process user commands
    match interactive_mode {
        true => ui::run_interactive(&args, &mut bud),
        false => todo!("non-interactive"),
    }?;

    //output after processing
    if verbosity > 0 {
        match json_output {
            true => todo!("json output"),
            false => {
                println!("{bud}");
            }
        }
    }

    //save changes
    fileio::save_budget_to_account_file(bud)?;

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
                    If ommited, uses a default user account. \
                    Should the account not exist, the user will \
                    be prompted to create a new one. ",
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
            Arg::new("dry_run")
                .short('N')
                .long("dry-run")
                .action(ArgAction::SetTrue)
                .help("Save no changes")
                .long_help("Perform and output changes but do not save to file."),
        )
        .arg(
            Arg::new("interactive")
                .short('i')
                .long("interactive")
                .action(ArgAction::SetTrue)
                .help("Enable the interactive interface")
                .long_help(
                    "Interactive mode. Implies -q, as it will not output upon exit. \
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
