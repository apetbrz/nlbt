mod budget;
use budget::Budget;
mod ui;
mod fileio;

use clap::Parser;

/// nice command line budget tool
#[derive(Parser, Debug)]
#[command(version, about, author, long_about = None,
override_usage("nclbt [OPTIONS]
       nclbt -i
       nclbt -Pc
       nclbt -pe <EXPENSE> -a <AMOUNT>")
)]
struct Args {
    ///enable the interactive ui
    ///
    ///implies -q, as it will not output on exit
    ///include -q to reverse this
    #[arg(short, long)]
    interactive: bool,
    
    ///run without a save file
    #[arg(short='v', long)]
    mem_only: bool,

    ///no output
    #[arg(short, long)]
    quiet: bool,

    ///output as JSON
    #[arg(short, long)]
    json: bool,

    ///account to load
    ///
    ///when ommited, uses default account, with name set by -D
    #[arg(short='A', long)]
    account: Option<String>,

    ///set default account username
    ///
    ///used for the default account when -A is ommited
    #[arg(short = 'D', long)]
    default_name: Option<String>,

    ///set paycheck amount
    ///
    ///the default amount added to the account
    ///when -P is used
    #[arg(short='C', long)]
    paycheck: Option<String>,

    ///get paid
    ///
    ///optionally takes an amount
    ///without an amount, uses paycheck value from -C
    ///recommended to use with -c
    #[arg(short='P', long)]
    paid: bool,

    ///reset all paid expenses to zero
    #[arg(short, long)]
    clear: bool,

    ///pay one or more expenses
    ///
    ///requires an expense (-e) and optionally an amount (-a)
    ///without an amount, expense is paid in full
    #[arg(short,long)]
    pay: bool,

    ///set an expense amount
    ///
    ///requires an expense (-e) and an amount (-a)
    #[arg(short,long)]
    set: bool,

    ///select an existing expense
    ///
    ///selects a target for one of -p or -s
    #[arg(short, long)]
    expense: Vec<String>,

    ///create and select a new expense
    #[arg(short, long)]
    new_expense: Vec<String>,

    ///target dollar amount
    ///
    ///provides a quantity of money for -p and -s commands
    #[arg(short,long)]
    amount: Vec<f32>,
}

fn main() -> Result<(),std::io::Error> {

    let args = Args::parse();

    let mut bud = match args.mem_only {
        true => Budget::new(args.account.as_ref().or(args.default_name.as_ref())),
        false => todo!("saving/loading")
    };

    match args.interactive {
        true => ui::run_interactive(&args, &mut bud),
        false => todo!("non-interactive")
    }?;

    if !(args.interactive ^ args.quiet) {
        println!("{bud}");
    }
    Ok(())
}

