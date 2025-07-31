mod commands;
mod error;
mod fileio;

use clap::*;
use commands::*;
use console::Term;
use dialoguer::Input;
use error::Result;
use nlbl::{parse_command, Budget};

const APP_TITLE: &str = "nclbt";
const COMMAND_PROMPT: &str = ">>";
const COMMANDS_LIST: &str = "=============={ nos' command-line budget tool }===============\n\
                            ========{ everything in [square brackets] is a value }========\n\
                            \thelp: shows this menu, lol!\n\
                            \tincome set [amount]: sets your expected income\n\
                            \tincome raise [amount]: adds to your income\n\
                            \tpaid: receive your income\n\
                            \tpaid [amount]: receive some amount\n\
                            \tnew [name] [amount]: create a new expenditure\n\
                            \t\t(overrides existing copies)\n\
                            \t\t(prefix with \"*\" to make it automatic)\n\
                            \tpay [name]: pay a static expenditure\n\
                            \tpay [name] [amount]: pay some amount to an expenditure\n\
                            \tsave [amount]: add an amount into savings\n\
                            \tsave all: add the remaining balance into savings\n\
                            \tclear: clear the terminal\n\
                            \texit: close the app\n\
                            ==============================================================\n";

fn main() -> Result<()> {
    //arguments -> options and commands
    let (app_settings, account_options, budget_commands) = {
        //get arguments
        let args = parse_args();

        //parse into structs
        parse_input(args)?
    };

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
        true => run_interactive(&mut bud)?,
        false => bud.execute_cmds(budget_commands, app_settings.force)?,
    };

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

#[cfg(feature = "cli")]
fn parse_args() -> ArgMatches {
    Command::new("nclbt")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
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
                .value_name("amount")
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
                .value_name("amount")
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

// for use in CLI
pub fn run_interactive(bud: &mut Budget) -> Result<()> {
    let term = Term::stdout();
    term.set_title(APP_TITLE);
    term.clear_screen()?;

    output(&term, &bud.to_string());
    output(&term, ">>  tip: enter 'help' to get started!\n");

    let mut user_input: String;
    loop {
        user_input = Input::<String>::new()
            .with_prompt(COMMAND_PROMPT)
            .interact_text()
            .unwrap();

        if user_input.trim() == "exit" || user_input.trim() == "q" {
            break;
        }

        let result = {
            || {
                let cmd = parse_command(&user_input)?;
                bud.execute_cmd(cmd, 0)
            }
        }();

        term.clear_screen()?;

        output(&term, &bud.to_string());

        if let Err(e) = result {
            output(&term, "error!");
            output(&term, &e.to_string());
        }
    }

    term.set_title("");

    Ok(())
}

fn output(t: &Term, s: &str) {
    t.write_line(s).expect("console-should-write");
}
