# nlbt

## nice little budget tool

A simple budget tool, for tracking expenses.

Supports fixed-income, multiple accounts, interactive and non-interactive CLI modes, and JSON output.

Currently in-development.

### usage:

```
> ./nlbt -h
nice little budget tool

Usage: nlbt [OPTIONS]

Options:
  -A, --account <account>                 Select account to load/modify
  -p, --pay <expense> <[amount]>          Pay an expense
  -e, --edit <expense> <modification>...  Edit an existing expense
  -n, --new <expense> <amount>            Create a new expense
  -P, --paid [<amount>]                   Get paid
  -C, --set-paycheck <amount>             Set paycheck amount
  -c, --clear [<expense>...]              Clear amount(s) paid to expense(s)
  -f, --force...                          Force payments [unimpl.]
  -D, --set-default-name <default_name>   Set default account username
  -N, --new-account <new_account>         Create a new account
  -X, --dry-run                           Save no changes
  -i, --interactive                       Enable the interactive interface
  -m, --mem-only                          Run without a save file
  -q, --quiet                             Silence output
  -v, --verbose...                        Increase detail of output
  -j, --json                              Output as json [unimpl.]
  -h, --help                              Print help (see more with '--help')
  -V, --version                           Print version
```

### building:

- Install rust from `https://rustup.rs/`
- Run `cargo build`
