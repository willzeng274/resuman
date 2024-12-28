# Command-Line Help for `resuman`

This document contains the help content for the `resuman` command-line program.

**Command Overview:**

* [`resuman`↴](#resuman)
* [`resuman create`↴](#resuman-create)
* [`resuman list`↴](#resuman-list)
* [`resuman list group`↴](#resuman-list-group)
* [`resuman list template`↴](#resuman-list-template)
* [`resuman list flatten`↴](#resuman-list-flatten)
* [`resuman init`↴](#resuman-init)

## `resuman`

A command-line tool to manage your resume.

**Usage:** `resuman [OPTIONS] [COMMAND]`

###### **Subcommands:**

* `create` — A subcommand for creating something
* `list` — Command related to resume groups
* `init` — Initialize resuman

###### **Options:**

* `-c`, `--config <FILE>` — Sets a custom config file path



## `resuman create`

A subcommand for creating something

**Usage:** `resuman create [OPTIONS] --company <COMPANY>`

###### **Options:**

* `-c`, `--company <COMPANY>` — Company name
* `-g`, `--group <GROUP>` — Group name
* `-t`, `--template <TEMPLATE>` — Template to use
* `-f`, `--file <FILE>` — A tex file path to use as a template
* `-p`, `--position <POSITION>` — Position/role name
* `-a`, `--letter` — Applied with cover letter
* `--applied-time <APPLIED_TIME>` — Date applied
* `-d`, `--length <LENGTH>` — Length of job (Weeks)
* `-l`, `--location <LOCATION>` — Location of job
* `-s`, `--status <STATUS>` — Status of application
* `-u`, `--urls <URLS>` — URLs to job posting, company, etc
* `-n`, `--notes <NOTES>` — Other metadata



## `resuman list`

Command related to resume groups

**Usage:** `resuman list [COMMAND]`

###### **Subcommands:**

* `group` — a subcommand for listing all groups
* `template` — List all templates
* `flatten` — List all resumes



## `resuman list group`

a subcommand for listing all groups

**Usage:** `resuman list group [OPTIONS]`

###### **Options:**

* `-v`, `--verbose` — Verbose output
* `-f`, `--fs` — Use fs instead of SQL db



## `resuman list template`

List all templates

**Usage:** `resuman list template [OPTIONS]`

###### **Options:**

* `-v`, `--verbose` — Verbose output
* `-f`, `--fs` — Use fs instead of SQL db



## `resuman list flatten`

List all resumes

**Usage:** `resuman list flatten [OPTIONS]`

###### **Options:**

* `-v`, `--verbose` — Verbose output
* `-f`, `--fs` — Use fs instead of SQL db
* `-i`, `--ignore <IGNORE>` — Ignore flag for folders/directories, exact match



## `resuman init`

Initialize resuman

**Usage:** `resuman init`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

