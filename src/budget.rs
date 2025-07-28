use crate::commands::BudgetCommands;
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// const AUTOMATIC_PAYMENT_PREFIX: char = '*';

#[derive(Serialize, Deserialize, Debug)]
pub struct Budget {
    pub account: String,
    current_balance: i32,
    expected_income: i32,
    expected_expenses: HashMap<String, i32>,
    current_expenses: HashMap<String, i32>,
    savings: i32,
}
impl Budget {
    //new(): factory method, returning a new Budget
    //TODO: SAVING/LOADING, accountS
    pub fn new(account: &str) -> Budget {
        Budget {
            account: String::from(account),
            expected_income: 0,
            current_balance: 0,
            expected_expenses: HashMap::new(),
            current_expenses: HashMap::new(),
            savings: 0,
        }
    }

    #[allow(unused_variables)]
    pub fn execute(&mut self, cmds: BudgetCommands, force: u8) -> Result<()> {
        use crate::commands::BudgetCommand as BC;
        for cmd in cmds {
            #[cfg(debug_assertions)]
            println!("[DEV] executing command: {cmd:?}");
            match cmd {
                BC::SetPaycheck { amount } => {
                    self.set_income(amount);
                }
                BC::Paid { amount } => {
                    if let Some(c) = amount {
                        self.get_paid_value(c);
                    } else {
                        self.get_paid();
                    }
                }
                BC::ClearExpense {
                    targets,
                    invert_selection,
                } => {
                    todo!("clear expenses")
                }
                BC::EditExpense {
                    target,
                    new_name,
                    new_amount,
                } => {
                    todo!("edit existing expense")
                }
                BC::NewExpense { name, amount } => {
                    self.add_expense(&name, amount);
                }
                BC::PayExpense { name, amount } => {
                    if let Some(c) = amount {
                        self.make_dynamic_payment(&name, c)?;
                    } else {
                        self.make_static_payment(&name)?;
                    }
                }
            }
        }
        Ok(())
    }

    //set_income(): sets expected_income to the new value
    pub fn set_income(&mut self, cents: i32) {
        self.expected_income = cents;
    }

    //add_income(): adds new value to expected_income
    pub fn add_income(&mut self, cents: i32) {
        self.set_income(self.expected_income + cents);
    }

    //get_paid(): adds expected_income to current_balance
    pub fn get_paid(&mut self) {
        self.get_paid_value(self.expected_income)
    }

    //get_paid_value(): adds given value to current_balance
    pub fn get_paid_value(&mut self, cents: i32) {
        self.current_balance += cents;
    }

    //refresh(): resets current_expenses
    pub fn refresh(&mut self) {
        for key in self.current_expenses.iter_mut() {
            *key.1 = 0;
        }
    }

    //TODO: automatic payments? default off, opt-in with a saved toggle? something like that
    //make_automatic_payments(): adds up total of automatic payments, returns money left over (if positive -> Ok, if negative -> Err)
    /*pub fn make_automatic_payments(&mut self, cents: i32) -> Result<i32> {
        let mut autos: Vec<String> = Vec::new();
        let mut payment = 0;
        for key in self.expected_expenses.iter() {
            if key.0.chars().next().unwrap() == AUTOMATIC_PAYMENT_PREFIX {
                autos.push(key.0.clone());
                payment += key.1;
            }
        }
        if payment == 0 {
            return Ok(-1);
        }
        payment = cents - payment;
        if payment < 0 {
            Err(payment)
        } else {
            for name in autos {
                let _ = self.make_static_payment(&name);
            }
            Ok(payment)
        }
    }
    */

    //add_expense(): creates a new expense in both HashMaps, with the new value as the expected value in expected_expenses
    pub fn add_expense(&mut self, name: &str, cents: i32) {
        self.expected_expenses
            .insert(name.to_string().to_ascii_lowercase(), cents);
        self.current_expenses
            .insert(name.to_string().to_ascii_lowercase(), 0);
    }

    //make_static_payment(): makes a payment into current_expenses, with the value from expected_expenses
    pub fn make_static_payment(&mut self, name: &str) -> Result<String> {
        match self.expected_expenses.get(name) {
            Some(n) => self.make_dynamic_payment(name, *n),
            None => Err(Error::BudgetErrorExpenseDoesNotExist(name.into())),
        }
    }

    //make_dynamic_payment(): makes a payment into current_expenses, with the given value
    pub fn make_dynamic_payment(&mut self, name: &str, cents: i32) -> Result<String> {
        let name = name.to_ascii_lowercase();
        if let Some(n) = self.current_expenses.get_mut(&name) {
            self.current_balance -= cents;
            *n += cents;
        } else {
            return Err(Error::BudgetErrorExpenseDoesNotExist(name));
        };

        Ok(format!(
            "Payment made: {} to {}",
            format_dollars(&cents),
            to_title_case(name)
        ))
    }

    //save(): adds the given amount into savings
    pub fn save(&mut self, cents: i32) -> Result<String> {
        if self.current_balance < cents {
            Err(Error::BudgetErrorCannotAfford {
                expense: "savings".into(),
                amount: cents,
                remaining_balance: self.current_balance,
            })
        } else {
            self.current_balance -= cents;
            self.savings += cents;
            Ok(format!("{} saved!", format_dollars(&cents)))
        }
    }

    //save_all(): moves current_balance to savings
    pub fn save_all(&mut self) -> Result<String> {
        self.save(self.current_balance)
    }
}

impl std::fmt::Display for Budget {
    //fmt(): Display String has a header, with account, followed by balance and expected pay, and then all expenses
    //TODO: better?? lol
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "==={{ Welcome, {}! }}===", self.account)?;
        writeln!(f, "Balance: {}", format_dollars(&self.current_balance))?;
        writeln!(f, "Income: {}", format_dollars(&self.expected_income))?;
        writeln!(f, "Savings: {}", format_dollars(&self.savings))?;
        writeln!(f, "\nExpenses:")?;

        let mut loop_output: std::fmt::Result = Ok(());

        for key in self.expected_expenses.iter() {
            let current_amount = self
                .current_expenses
                .get(key.0)
                .unwrap_or_else(|| panic!("{}-missing-from-current-expenses", key.0));
            let category_name = to_title_case(key.0.clone());

            loop_output = writeln!(
                f,
                "{}: {}/{}",
                category_name,
                format_dollars(current_amount),
                format_dollars(key.1)
            );
            if loop_output.is_err() {
                break;
            }
        }

        loop_output
    }
}

//format_dollars(): takes an amount of cents and formats it to ${X}+.XX
pub fn format_dollars(cents: &i32) -> String {
    let cents = { cents.to_string() };
    let dollars = match cents.len() {
        3.. => cents.split_at(cents.len() - 2),
        2 => ("0", cents.as_str()),
        1 => ("0", &format!("0{cents}")[..]),
        _ => ("0", "00"),
    };
    let mut output = String::from("$");
    output.push_str(dollars.0);
    output.push('.');
    output.push_str(dollars.1);
    output.to_string()
}

//TODO: DOLLAR TRAIT FOR STRINGS AND f32???

//dollars_to_cents(): takes a decimal amount of dollars and returns it in integer cents
pub fn dollars_to_cents(dollars: f32) -> i32 {
    (dollars * 100.0) as i32
}

//parse_dollar_string(): takes a string literal and returns an integer cent amount if valid, or error message if not
pub fn parse_dollar_string(s: &str) -> Result<i32> {
    if s.is_empty() {
        return Err(Error::InvalidDollarValue(s.into()));
    }
    let mut s = s;
    if s.starts_with('$') {
        s = &s[1..];
    }
    match s.parse::<i32>() {
        Ok(n) => Ok(n * 100),
        Err(_) => match s.parse::<f32>() {
            Ok(m) => Ok(dollars_to_cents(m)),
            Err(_) => Err(Error::InvalidDollarValue(s.into())),
        },
    }
}

//to_title_case(): takes a String and returns a new String with the first letter uppercase, and the rest lowercase
pub fn to_title_case(s: String) -> String {
    let mut out = s;
    if let Some(r) = out.get_mut(0..1) {
        if r == "*" {
            if let Some(s) = out.get_mut(1..2) {
                s.make_ascii_uppercase();
            }
        } else {
            r.make_ascii_uppercase();
        }
    }
    out.clone()
}
