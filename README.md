# nclbt

## nice' command-line budget tool

a simple budget tool, for tracking expenses

supports fixed-income, multiple accounts, interactive and non-interactive cli modes, and json output

### usage:

```
> ./nclbt -h
nice command line budget tool

Usage: nclbt [OPTIONS]
       nclbt -i
       nclbt -Pc
       nclbt -pe <EXPENSE> -a <AMOUNT>

Options:
  -i, --interactive                  enable the interactive ui
  -v, --mem-only                     run without a save file
  -q, --quiet                        no output
  -j, --json                         output as JSON
  -A, --account <ACCOUNT>            account to load
  -D, --default-name <DEFAULT_NAME>  set default account username
  -C, --paycheck <PAYCHECK>          set paycheck amount
  -P, --paid                         get paid
  -c, --clear                        reset all paid expenses to zero
  -p, --pay                          pay one or more expenses
  -s, --set                          set an expense amount
  -e, --expense <EXPENSE>            select an existing expense
  -n, --new-expense <NEW_EXPENSE>    create and select a new expense
  -a, --amount <AMOUNT>              target dollar amount
  -h, --help                         Print help (see more with '--help')
  -V, --version                      Print version
```

### building:

- install rust from `https://rustup.rs/`
- run `cargo build`
