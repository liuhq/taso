# Taso Design

taso is a simple command line tool for managing todos. It can manage todolist via `.todo`.

## Configuration

taso read config file via `$XDG_CONFIG_HOME/taso/config.ini`, or `.config/taso/config.ini` (`XDG_CONFIG_HOME` is unset), or default config in hardcode.

`default_data_path` - `String` : `.local/share/taso/data`

`interactive_height` - `u8` : `50`

`sort_descending_order` - `bool` : `true`

`separate_list_into_file` - `bool` : `false`

(config WIP)

## Store


`taso` will look for a `.todo` file in the current working directory. If not found, it will search upward through parent directories. If still not found, the default data file at `.local/share/taso/data` will be used, or the path specified in the configuration file.

List names are used as key at the top level of data structure, with todos stored as array value.

```
[default.todo_id_1]
desc = ...
link = ...
assign_at = ...
children = ...
create_at = ...
complete_at = ...

[default.todo_id_2]
desc = ...
link = ...
assign_at = ...
children = ...
create_at = ...
complete_at = ...

[work.todo_id_1]
desc = ...
link = ...
assign_at = ...
children = ...
create_at = ...
complete_at = ...

[work.todo_id_2]
desc = ...
link = ...
assign_at = ...
children = ...
create_at = ...
complete_at = ...
```

## Todo Struct

`Todo` contains fields as follow:

- `desc` - `String`: a description displayed in outputs
- `link` - `String`: a link that can be opened by default action
- `assign_at` - `NaiveDate`: date `YYYY-MM-DD` of todo planned. *use only the user's local time*
- `children` - `Vec<String>`: a set of sub-todo
- (*plan*) `parent` - `Option<Vec<String>>`: a set of super-todo(s)
- `create_at` - `NaiveDate`: date `YYYY-MM-DD` of todo created. *use only the user's local time*
- `complete_at` - `Option<NaiveDate>`: date `YYYY-MM-DD` of `done` updated. if `children` is filled in, this field will controlled by all subtodo. *use only the user's local time*

## List

There is a `default` list that stores todolist without a specified list. Additionally, todos for a specified list can be stored and managed using the `--list` option. Todos and lists are stored by default in a single file, but can configure `separate_list_into_file` to store todos from different lists in separate files, These files are organized by list and named `.todo.LIST_NAME`. *Note: the file for default list is still named `.todo`*.

## CLI

`taso [SUBCOMMAND] [OPTIONS]`

### SUBCOMMAND

`init`: create a new `.todo` data file in current working directory.
- `--separate-list` - `bool`: specify this data file as `separate_list_into_file`
- `--default-list <DEFAULT_LIST_NAME>` - `String`: specify `DEFAULT_LIST_NAME` as default list instead of `default`
- `--lists <LIST_NAME_1> [<LIST_NAME_2> ...]` or `--lists <LIST_NAME_1>[,<LIST_NAME_2>,...]` or `--lists <LIST_NAME_1> [--lists <LIST_NAME_2> ...]` - `Vec<String>`: create additional list(s) for this data file. *Note: default list will be created automatically*

`list [DATE_OPTION]`: list all todos in specified [DATE_OPTION](#DATE_OPTION) Default be sorted in descending order by date. (or can be configured to ascending order via configuration file)
- `--todo` - `bool` (conflict with `--done`): filter incomplete todos
- `--done` - `bool`: filter completed todos
- `--reverse` - `bool`: reverse sort order
- `--list <LIST_NAME>` - `String`: list todos in list `LIST_NAME`, default is `--list default`.

`add`: add a new todo (interactive mode when the following options are missing).
- `--desc <STRING>` - `String`: set `desc` (non-interactively)
- `--link <STRING>` - `String`: set `link` (non-interactively)
- `--assign-date <STRING>` - `String`: set `assign-date` (non-interactively)
- `--list <LIST_NAME>` - `String`: add todo to list `LIST_NAME`, default is `--list default`.

`modify <todo_id>`: modify todo `todo_id` (interactive mode when the following options are missing).
- `--desc <NEW_STRING>` - `String`: modify `desc` (non-interactively)
- `--link <NEW_STRING>` - `String`: modify `link` (non-interactively)
- `--assign-date <NEW_STRING>` - `String`: modify `assign-date` (non-interactively)
- `--list <LIST_NAME>` - `String`: modify todo in list `LIST_NAME`, default is `--list default`.

`move <sub_todo_id>`: change the level of todo `sub_todo_id` with its subtodos.
- `--down <super_todo_id>` - `String` (conflict with `--up`): move into todo `super_todo_id` as a subtodo. `super_todo_id` can't be a subtodo of `sub_todo_id`
- `--up` - `bool`: move to the top level
- `--list <LIST_NAME>` - `String`: move todos in list `LIST_NAME`, default is `--list default`.

`remove <todo_id> [<todo_id_2> <todo_id_3 ...]`: remove an exist todo `todo_id` (or more). `todo_id` must have no subtodos.
- `--recursive` - `bool`: remove an exist todo `todo_id` and its subtodos recursively
- `--list <LIST_NAME>` - `String`: remove todos from list `LIST_NAME`, default is `--list default`.

`done <todo_id> [<todo_id_2> <todo_id_3 ...]`: mark todo `todo_id` (or more) as `done`. `todo_id` must have no subtodos.
- `--recursive` - `bool`: mark todo `todo_id` and its subtodos as `done` recursively
- `--list <LIST_NAME>` - `String`: mark todos in list `LIST_NAME` to done, default is `--list default`.

(*plan*) `track <todo_id>`: track the relationship between todo `todo_id` and other todos.
- `--children` - `bool` (conflict with `--super`): print all subtodos under `todo_id`
- (*plan, require `super_todo`*) `--super` - `bool`: print all super-todo(s) of `todo_id`
- `--list <LIST_NAME>` - `String`: track todos in list `LIST_NAME`, default is `--list default`.

`list <SUBCOMMAND>`: manage lists.
- `list`: list exist lists in date mode.
- `add <NEW_LIST_NAME>`: add new list(s) `NEW_LIST_NAME`.
- `remove <LIST_NAME>`: remove an exist list `LIST_NAME` and move all todos from list `LIST_NAME` to list `default`. list `default` can't be removed.
  - `--recursive` - `bool`: remove an exist list `LIST_NAME` and all todos under it
- `move <NEW_LIST_NAME>`: move todo from list `OLD_LIST_NAME` (if `--list <OLD_LIST_NAME>` is omitted, default is `--list default`) to the top level of list `NEW_LIST_NAME`. list `NEW_LIST_NAME` must be exist.
  - `--list <OLD_LIST_NAME>` - `String`: specify todos that will be moved from list `OLD_LIST_NAME`, default is `--list default`.
  - `--id <todo_id> [<todo_id_2> ...]` or `--id <todo_id>[,<todo_id_2>,...]` or `--id <todo_id> [--id <todo_id_2> ...]` - `Vec<String>` (conflict with `--all`): move todo `todo_id` (or more) and its subtodos
  - `--all` - `bool`: move all todos which under list `OLD_LIST_NAME`

`check [DATE_OPTION]`: check and list all overdue todos that are not yet completed in specified DATE_OPTION.
- `--all` - `bool` (conflict with [`--id`, `--desc`, `--create-date`, `--assign-date`]): print all information (**default**)
- `--id`: print the `todo_id`
- `--desc`: print the `desc`
- `--create-date`: print the `create_date`
- `--assign-date`: print the `assign_date`
- `--list <LIST_NAME>` - `String`: check todos in list `LIST_NAME`, default is `--list default`.

`clean [DATE_OPTION]`: clean up all completed todos in specified DATE_OPTION.
- `--list <LIST_NAME>` - `String`: clean up todos in list `LIST_NAME`, default is `--list default`.

`extract`: extract and archive todos to `.todo[.YEAR].done` or `.todo[.LIST_NAME[.YEAR]].done` (`separate_list_into_file` is `true`). If the file already exists, merge the data.
- `--done` - `bool`: extract completed todos
- `--year <YEAR>` - `bool`: extract todos of the specified year `YEAR`
- `--list <LIST_NAME>` - `String`: extract todos from list `LIST_NAME`, default is `--list default`.

`merge <ARCHIVE_FILE>`: merge data `ARCHIVE_FILE` into `.todo` or `.todo[.LIST_NAME]` (`separate_list_into_file` is `true`).
- `--list <LIST_NAME>` - `String`: merge todos to list `LIST_NAME`, default is `--list default`.

## GENERAL OPTIONS

`--dry-run`: output the result without performing any actual operation.

`--version`: version information.

`--help`: help information.

### DATE_OPTION

**Relative Date (PRIORITY)**

The following options are mutually exclusive.

`-d --day-rel [<OFFSET>]`: manage lists by day. `OFFSET` default value is `0`, which means *today*. `+OFFSET` indicates *the future* and `-OFFSET` indicates *the past*, e.g. `+1` is tomorrow and `-1` is yesterday.

`-w --week-rel [<OFFSET>]`: manage lists by week. Same as `--day-rel`, default value is `--week=0` means this week.

`-m --month-rel [<OFFSET>]`: manage lists by month. Same as `--day-rel`, default value is `--month=0` means this month.

`-y --year-rel [<OFFSET>]`: manage lists by year. Same as `--day-rel`, default value is `--year=0` means this year.

if `DATE_OPTION` is omitted, `--day-rel 0` will be used as the default option.

**Absolute Date**

The following options can be combined with each other, ~~and also with **a single relative date option of a larger granularity** ("A larger granularity" means `year > month > week > day`. For example, `--day 1` can be combined with `--month-rel -1` (last month), but not with `--day-rel -1`)~~ (*not plan*). When absolute fields do not specify a larger granularity, the current year/month/week is assumed by default.

`-D --date <YYYY-MM-DD>`: specify a date using the `YYYY-MM-DD` format. e.g. `2024-12-12`, `2025-06-03`.

(*not plan*)`-W --week <N>`: `N` must be a positive integer indicating the N-th week of the specified month or year. e.g. `--week 2 --month-rel -1` is 2nd week of last month; `--week 16 --year 2025` is 16th week of 2025.

`-M --month <N>`:`N` must be between 1 and 12 indicating the N-th month of the specified year. e.g. `--month 5 --year-rel -1` is May of last year; `--month 12 --year 2025` is December of this year.

`-Y --year <N>`: (default: current year) `N` must be a positive integer indicating the specific year.

more example:

~~`--day 9 --month 6 --year-rel 1`: June 9th of next year.~~

`--year 2025 --month 6`: June, 2025

`--date 2025-06-01`: June 1st, 2025

<!-- vim: set wrap linebreak: -->
