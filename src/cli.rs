use crate::commands::*;
use crate::error::{Error, Result};
use nlbl::*;

use clap::*;
use console::Term;
use dialoguer::Input;

const APP_TITLE: &str = "nlbt";
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

pub fn init_settings() -> Result<AppConfig> {
    let args = parse_args();
    parse_settings(args)
}

pub fn parse_args() -> ArgMatches {
    Command::new("nlbt")
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

pub fn parse_settings(args: clap::ArgMatches) -> Result<AppConfig> {
    //flags
    let mem_only: bool = args.get_flag("mem_only");
    let interactive_mode: bool = args.get_flag("interactive");
    let force: u8 = args.get_count("force");
    let dry_run: bool = args.get_flag("dry_run");
    //verbosity logic:
    //default to 1
    //each -v is +1
    //-q or -i is -1
    let verbosity: u8 =
        1 + args.get_count("verbose") - (interactive_mode || args.get_flag("quiet")) as u8;
    let json: bool = args.get_flag("json");

    let app_settings = AppSettings {
        mem_only,
        interactive_mode,
        dry_run,
        force,
        verbosity,
        json,
    };

    //account settings/commands
    let account = args
        .get_one::<String>("account")
        .or(args.get_one("new_account"))
        .cloned();
    let create: bool = args.contains_id("new_account");
    let default_rename: Option<String> = args.get_one("default_name").cloned();

    let account_options = AccountOptions {
        account,
        create,
        default_rename,
    };

    let budget_commands: BudgetCommands = ["paycheck", "paid", "clear", "edit", "new", "pay"]
        .iter()
        .filter(|id| args.contains_id(id))
        .flat_map(|id| {
            args.get_occurrences(id).unwrap().filter_map(|arg_iter| {
                command_from_arg(id, arg_iter)
                    .map_err(|e| {
                        println!("{e}");
                        e
                    })
                    .ok()
            })
        })
        .collect::<BudgetCommands>();

    #[cfg(debug_assertions)]
    {
        println!("[DEV] parsed app settings: {app_settings:?}");
        println!("[DEV] parsed account options: {account_options:?}");
        println!("[DEV] parsed budget commands: {budget_commands:?}");
    }

    Ok(AppConfig {
        app_settings,
        account_options,
        budget_commands,
    })
}

pub fn parse_command(input: &str) -> Result<BudgetCommand> {
    use BudgetCommand as BC;

    let command = input.split_whitespace();
    let command: Vec<&str> = command.collect();

    let cmd = match *command.first().unwrap_or(&"") {
        "income" => match *command.get(1).unwrap_or(&"") {
            "set" => {
                let amount = *command.get(2).ok_or(Error::InvalidCommand("new".into()))?;
                let amount = util::parse_dollar_string(amount)?;
                BC::SetPaycheck { amount }
            }
            "raise" => {
                todo!("raise income command!");
                let amount = *command.get(2).ok_or(Error::InvalidCommand("new".into()))?;
                let amount = util::parse_dollar_string(amount)?;
                BC::SetPaycheck { amount }
            }
            other => Err(Error::InvalidCommand(other.into()))?,
        },
        "paid" => match *command.get(1).unwrap_or(&"") {
            "" => BC::Paid { amount: None },
            val => {
                let amount = util::parse_dollar_string(val)?;
                BC::Paid {
                    amount: Some(amount),
                }
            }
        },
        "new" => {
            let name = String::from(*command.get(1).ok_or(Error::InvalidCommand("new".into()))?);
            let amount = *command.get(2).ok_or(Error::InvalidCommand("new".into()))?;
            let amount = util::parse_dollar_string(amount)?;
            BC::NewExpense { name, amount }
        }
        "pay" => {
            let name = String::from(*command.get(1).ok_or(Error::InvalidCommand("pay".into()))?);
            match *command.get(2).unwrap_or(&"") {
                "" => BC::PayExpense { name, amount: None },
                val => {
                    let amount = util::parse_dollar_string(val)?;
                    BC::PayExpense {
                        name,
                        amount: Some(amount),
                    }
                }
            }
        }
        "save" => {
            todo!("savings!");
            // match command[1] {
            // "" => Err(Error::InvalidCommand("empty save amount".into()))?,
            // "all" => bud.save_all(),
            // _ => {
            //     let amount = util::parse_dollar_string(command[1])?;
            //     bud.save(amount)
            // }
        }
        // "clear" => match term.clear_screen() {
        //     Ok(()) => Ok(String::new()),
        //     Err(e) => Err(Error::IoFailure(e)),
        // },
        cmd => return Err(Error::InvalidCommand(cmd.into()))?,
    };

    Ok(cmd)
}

// for use in CLI
pub fn run_interactive(mut bud: Budget) -> Result<Budget> {
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

        let cmd = parse_command(&user_input)?;
        let err = match execute_cmd(bud.clone(), cmd, 0) {
            Ok(b) => {
                bud = b;
                None
            }
            Err(e) => Some(e),
        };

        term.clear_screen()?;

        output(&term, &bud.to_string());

        if let Some(e) = err {
            output(&term, "error!");
            output(&term, &e.to_string());
        }
    }

    term.set_title("");

    Ok(bud)
}

fn output(t: &Term, s: &str) {
    t.write_line(s).expect("console-should-write");
}
