# YarDB : Yet Another Rust Database

This is a learning project. There is no roadmap as of now. I am just gonna start with cstack's tutorial - [Let's Build a Simple Database](https://cstack.github.io/db_tutorial/) and see where to go from there.

```
 __  __     ______     ______     _____     ______
/\ \_\ \   /\  __ \   /\  == \   /\  __-.  /\  == \
\ \____ \  \ \  __ \  \ \  __<   \ \ \/\ \ \ \  __<
 \/\_____\  \ \_\ \_\  \ \_\ \_\  \ \____-  \ \_____\
  \/_____/   \/_/\/_/   \/_/ /_/   \/____/   \/_____/

YarDB Version 0.0.1

yardb> .help
Available commands:
.exit
.help
select
insert
stats
yardb> insert 1 foo foo@bar.com
yardb> select
1 foo foo@bar.com
yardb> stats
Table has 1 rows
Table has 1 pages
yardb> .exit
```

## Building the project

Build from source and run

```sh
> cargo build
> ./target/debug/yardb
```


## TODO

1. Add tests
2. Better error handling
3. Better logging setup

## Interesting things to try out

TBA, a list of interesting topics to deep dive and implement.
