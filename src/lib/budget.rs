use crate::error::{Error, Result};
use crate::{parse_command, util::*};
use crate::{BudgetCommand, BudgetCommands};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

// const AUTOMATIC_PAYMENT_PREFIX: char = '*';

#[cfg_attr(feature = "wasm", wasm_bindgen(getter_with_clone))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Budget {
    pub account: String,
    current_balance: i32,
    expected_income: i32,
    expected_expenses: HashMap<String, i32>,
    current_expenses: HashMap<String, i32>,
    savings: i32,
}
#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Budget {
    //new(): factory method, returning a new Budget
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new(account: &str) -> Budget {
        Budget {
            account: account.into(),
            expected_income: 0,
            current_balance: 0,
            expected_expenses: HashMap::new(),
            current_expenses: HashMap::new(),
            savings: 0,
        }
    }
    //stringify: WASM interface for Display
    #[cfg(feature = "wasm")]
    #[wasm_bindgen]
    pub fn stringify(&self) -> String {
        self.to_string()
    }

    #[cfg_attr(feature = "wasm", wasm_bindgen)]
    pub fn json(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen]
    pub fn as_obj(&self) -> Result<JsValue> {
        //TODO: WHY IS IT NOT RECURSIVE???
        Ok(serde_wasm_bindgen::to_value(self)?)
    }

    //for WASM, avoid inner mutation
    #[cfg_attr(feature = "wasm", wasm_bindgen)]
    pub fn execute_string_immut(self, input: &str) -> Result<Budget> {
        let mut out = self.clone();
        let cmd = parse_command(input)?;
        out.execute_cmd(cmd, 0)?;
        Ok(out)
    }

    pub fn execute_string_mut(&mut self, input: &str) -> Result<()> {
        let cmd = parse_command(input)?;
        self.execute_cmd(cmd, 0)
    }
}
impl Budget {
    pub fn execute_cmds(&mut self, cmds: BudgetCommands, force: u8) -> Result<()> {
        for cmd in cmds {
            self.execute_cmd(cmd, force)?
        }
        Ok(())
    }
    pub fn execute_cmd(&mut self, cmd: BudgetCommand, force: u8) -> Result<()> {
        //TODO: IMPLEMENT force VALUE
        use crate::BudgetCommand as BC;
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
                if targets.is_empty() && !invert_selection {
                    self.refresh();
                } else {
                    todo!("clear command expense selection")
                }
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
            BC::Nothing => {}
        }
        Ok(())
    }

    //set_income(): sets expected_income to the new value
    fn set_income(&mut self, cents: i32) {
        self.expected_income = cents;
    }

    //add_income(): adds new value to expected_income
    fn add_income(&mut self, cents: i32) {
        self.set_income(self.expected_income + cents);
    }

    //get_paid(): adds expected_income to current_balance
    fn get_paid(&mut self) {
        self.get_paid_value(self.expected_income)
    }

    //get_paid_value(): adds given value to current_balance
    fn get_paid_value(&mut self, cents: i32) {
        self.current_balance += cents;
    }

    //refresh(): resets current_expenses
    fn refresh(&mut self) {
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
    fn add_expense(&mut self, name: &str, cents: i32) {
        self.expected_expenses
            .insert(name.to_string().to_ascii_lowercase(), cents);
        self.current_expenses
            .insert(name.to_string().to_ascii_lowercase(), 0);
    }

    //make_static_payment(): makes a payment into current_expenses, with the value from expected_expenses
    fn make_static_payment(&mut self, name: &str) -> Result<String> {
        match self.expected_expenses.get(name) {
            Some(n) => self.make_dynamic_payment(name, *n),
            None => Err(Error::ExpenseDoesNotExist(name.into())),
        }
    }

    //make_dynamic_payment(): makes a payment into current_expenses, with the given value
    fn make_dynamic_payment(&mut self, name: &str, cents: i32) -> Result<String> {
        let name = name.to_ascii_lowercase();
        if let Some(n) = self.current_expenses.get_mut(&name) {
            self.current_balance -= cents;
            *n += cents;
        } else {
            return Err(Error::ExpenseDoesNotExist(name));
        };

        Ok(format!(
            "Payment made: {} to {}",
            format_dollars(cents),
            to_title_case(name)
        ))
    }

    //save(): adds the given amount into savings
    fn save(&mut self, cents: i32) -> Result<String> {
        if self.current_balance < cents {
            Err(Error::CannotAfford {
                expense: "savings".into(),
                amount: cents,
                remaining_balance: self.current_balance,
            })
        } else {
            self.current_balance -= cents;
            self.savings += cents;
            Ok(format!("{} saved!", format_dollars(cents)))
        }
    }

    //save_all(): moves current_balance to savings
    fn save_all(&mut self) -> Result<String> {
        self.save(self.current_balance)
    }
}
impl std::fmt::Display for Budget {
    //fmt(): Display String has a header, with account, followed by balance and expected pay, and then all expenses
    //TODO: better?? lol
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "==={{ Welcome, {}! }}===", self.account)?;
        writeln!(f, "Balance: {}", format_dollars(self.current_balance))?;
        writeln!(f, "Income: {}", format_dollars(self.expected_income))?;
        writeln!(f, "Savings: {}", format_dollars(self.savings))?;
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
                format_dollars(*current_amount),
                format_dollars(*key.1)
            );
            if loop_output.is_err() {
                break;
            }
        }

        loop_output
    }
}
