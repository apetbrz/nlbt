mod budget;

use budget::Budget;
use std::io;
use console::Term;
use dialoguer::Input;

const APP_TITLE: &str = "nos-clbt";
const COMMAND_ARGS_LIMIT: usize = 10;
const COMMAND_PROMPT: &str = ">>";
const COMMANDS_LIST: &str ="========================={ nos-clbt }=========================\n\
                            ======{ everything in [square brackets] is an argument }======\n\
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
                            ==============================================================\n\
                            \tMANY FEATURES ARE NOT IMPLEMENTED!!!!\n\
                            \tINCLUDING SAVING/LOADING!!!!!!!!!!!!!\n\
                            ==============================================================\n";

fn main() -> Result<(), io::Error> {
    let term = Term::stdout();
    term.set_title(APP_TITLE);
    term.clear_screen()?;
    fn output(t: &Term, s: &str){
        t.write_line(s).expect("console-should-write");
    }

    //output(&term, "initializing...");

    let mut bud = Budget::new();

    output(&term, &bud.to_string());
    output(&term, ">>  tip: enter 'help' to get started!\n");

    let mut user_input: String;
    loop{
        user_input = Input::<String>::new()
            .with_prompt(COMMAND_PROMPT)
            .interact_text()
            .unwrap();

        if user_input.trim() == "exit" { break; }

        
        let result = parse_command(&term, &user_input, &mut bud);
        
        term.clear_screen()?;

        output(&term, &bud.to_string());

        match result{
            Ok(s) => output(&term, &s),
            Err(s) => {
                output(&term, "error!");
                output(&term, &s);
            }
        }
    }

    Ok(())
}

fn parse_command(term: &Term, input: &str, bud: &mut Budget) -> Result<String, String>{
    let mut command = input.split_whitespace();
    let command: [&str; COMMAND_ARGS_LIMIT] = [(); COMMAND_ARGS_LIMIT].map(|_| command.next().unwrap_or(""));

    let out: Result<String, String> = {
        match command[0]{
            "help" => {
                Ok(COMMANDS_LIST.to_string())
            },
            "income" => {
                match command[1]{
                    "set" => {
                        let amount = budget::parse_dollar_string(command[2])?;
                        bud.set_income(amount);
                        Ok("Input set!".to_string())
                    },
                    "raise" => {
                        let amount = budget::parse_dollar_string(command[2])?;
                        bud.add_income(amount);
                        Ok("Input set!".to_string())
                    },
                    _ => {
                        Err(String::from("invalid-command"))
                    }
                }
            },
            "paid" => {
                match command[1]{
                    "" => {
                        let output = bud.get_paid()?;
                        Ok(format!("You got paid!\n{}",output))
                    },
                    _ => {
                        let amount = budget::parse_dollar_string(command[2])?;
                        bud.get_paid_value(amount);
                        Ok(format!("You got paid {}!", amount))
                    }
                }
            },
            "new" => {
                let name = command[1];
                let amount = budget::parse_dollar_string(command[2])?;
                bud.add_expense(name, amount);
                Ok(String::from("Expense added!"))
            },
            "pay" => {
                let name = command[1];
                match command[2]{
                    "" => bud.make_static_payment(name),
                    _ =>{
                        let amount = budget::parse_dollar_string(command[2])?;
                        bud.make_dynamic_payment(name, amount)
                    }
                }
            },
            "save" => {
                match command[1]{
                    "" => Err(String::from("invalid-command")),
                    "all" => bud.save_all(),
                    _ => {
                        let amount = budget::parse_dollar_string(command[1])?;
                        bud.save(amount)
                    }
                }
            },
            "clear" => {
                match term.clear_screen(){
                    Ok(()) => Ok(String::new()),
                    Err(e) => Err(e.to_string())
                }
            },
            _ => return Err(String::from("invalid-command"))
        }
    };

    out
}

