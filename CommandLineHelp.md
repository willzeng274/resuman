# Command-Line Help for `resuman`

This document contains the help content for the `resuman` command-line program.

**Command Overview:**

* [`resuman`↴](#resuman)
* [`resuman create`↴](#resuman-create)
* [`resuman list`↴](#resuman-list)
* [`resuman list group`↴](#resuman-list-group)
* [`resuman list template`↴](#resuman-list-template)
* [`resuman list flatten`↴](#resuman-list-flatten)
* [`resuman list all`↴](#resuman-list-all)
* [`resuman init`↴](#resuman-init)
* [`resuman update`↴](#resuman-update)
* [`resuman delete`↴](#resuman-delete)
* [`resuman find`↴](#resuman-find)
* [`resuman clean`↴](#resuman-clean)

## `resuman`

A command-line tool to manage your resume.

**Usage:** `resuman [OPTIONS] [COMMAND]`

###### **Subcommands:**

* `create` — A subcommand for creating something
* `list` — Command related to resume groups
* `init` — Initialize resuman
* `update` — Update resume metadata
* `delete` — Delete a resume
* `find` — Find a resume and return its id
* `clean` — Clean up resuman

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
* `all` — List all resumes in columns



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



## `resuman list all`

List all resumes in columns

**Usage:** `resuman list all [OPTIONS]`

###### **Options:**

* `-v`, `--verbose` — Verbose output
* `-i`, `--id` — ID of the resume
* `-g`, `--group` — Group of the resume
* `-t`, `--template` — Template of the resume
* `-c`, `--company` — Company of the resume
* `--letter` — Cover letter status of the resume
* `-f`, `--file-path` — File path of the resume
* `--created-at` — Date created of the resume
* `-a`, `--applied-time` — Applied time of the resume
* `--copied-from` — Copied from of the resume
* `-m`, `--metadata-file-path` — Metadata file path of the resume
* `--length` — Length of the job
* `-l`, `--location` — Location of the job
* `-s`, `--status` — Status of the application
* `-u`, `--urls` — URLs of the resume
* `-n`, `--notes` — Notes of the resume



## `resuman init`

Initialize resuman

**Usage:** `resuman init`



## `resuman update`

Update resume metadata

**Usage:** `resuman update [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — ID of the resume to update

###### **Options:**

* `-c`, `--company <COMPANY>` — Update company name (metadata only)
* `-g`, `--group <GROUP>` — Update group name (metadata only)
* `-t`, `--template <TEMPLATE>` — Update template used (metadata only)
* `-f`, `--file <FILE>` — Update "copied from" (metadata only)
* `-p`, `--position <POSITION>` — Update position/role (metadata only)
* `-a`, `--letter <HAS_COVER_LETTER>` — Update cover letter status (metadata only)

  Possible values: `true`, `false`

* `--created-at <CREATED_AT>` — Update created date (metadata only)
* `--applied-time <APPLIED_TIME>` — Update date applied (metadata)
* `-d`, `--length <LENGTH>` — Update length of job (metadata)
* `-l`, `--location <LOCATION>` — Update location of job (metadata)
* `-s`, `--status <STATUS>` — Update status of application (metadata)
* `-u`, `--urls <URLS>` — Update URLs to job posting, company, etc (metadata)
* `-n`, `--notes <NOTES>` — Update other metadata



## `resuman delete`

Delete a resume

**Usage:** `resuman delete [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — ID of the resume to delete

###### **Options:**

* `-f`, `--file` — Remove the actual file



## `resuman find`

Find a resume and return its id

**Usage:** `resuman find [OPTIONS]`

###### **Options:**

* `-c`, `--company <COMPANY>` — Filter by company
* `-g`, `--group <GROUP>` — Filter by group
* `-t`, `--template <TEMPLATE>` — Filter by template
* `-p`, `--position <POSITION>` — Filter by position
* `--letter <HAS_COVER_LETTER>` — Filter by cover letter status

  Possible values: `true`, `false`

* `--created-at <CREATED_AT>` — Filter by created date
* `--applied-time <APPLIED_TIME>` — Filter by date applied
* `--length <LENGTH>` — Filter by length of job
* `-l`, `--location <LOCATION>` — Filter by location
* `-s`, `--status <STATUS>` — Filter by status
* `-f`, `--file-path <FILE_PATH>` — Filter by file path
* `--copied-from <COPIED_FROM>` — Filter by copied from
* `-a`, `--all <ALL>` — Filter by all



## `resuman clean`

Clean up resuman sqlite database by checking for missing paths

**Usage:** `resuman clean`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

