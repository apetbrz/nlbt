# nclbt

## nice' command-line budget tool

a simple budget tool, for tracking expenses

supports fixed-income, multiple accounts, interactive and non-interactive cli modes, and json output

### usage:

```
> nclbt -h
nice command line budget tool

Usage: nclbt [OPTIONS]

Options:
  -A, --account <account>                 Select account to load/modify
  -D, --set-default-name <default_name>   Set default account username
  -p, --pay <expense> <[amount]>          Pay an expense
  -e, --edit <expense> <modification>...  Edit an existing expense
  -n, --new <expense> <amount>            Create a new expense
  -C, --set-paycheck <paycheck>           Set paycheck amount
  -c, --clear [<expense>...]              Clear amount(s) paid to expense(s)
  -f, --force...                          Force payments
  -i, --interactive                       Enable the interactive interface
  -m, --mem-only                          Run without a save file
  -q, --quiet                             Silence output
  -v, --verbose...                        Increase detail of output
  -j, --json                              Output as json
  -h, --help                              Print help (see more with '--help')
  -V, --version                           Print version
```

### building:

- install rust from `https://rustup.rs/`
- run `cargo build`
