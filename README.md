# Advent of code - Rusty w/ CLI

Lots of todos - might make the CLI generate better and handle multi years without changing `main.rs`, but idk, works well enough for now
Adding README to make it easier to dust off after a year of not using it

### Commands
- Run day / challenge
```
cargo run -- -d <day number> -c <challenge number>
```
- Generate day
``` 
cargo run -- -g -d <day number>
# Must add in main.rs 
# ...let handlers...
```

#### TODO

- Make every year / day / challenge runnable without code 
- Automate year creation
- Automate input loading and saving (beyond getting the session token)
- CLI bootstrap itself
