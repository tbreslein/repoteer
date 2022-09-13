# Design docs for repoteer

## Workflow

Write a manifest file that describes the location and configuration of repos.
According to the info in that file, repoteer commands can sync those repos.

## CLI commands

repoteer should accept several commands, which are:

- sync (clone (if not present), pull, then push repos)
    - user should be able to configure sync behaviour such that maybe only pull is allowed when there are unmerged changes, or that push can be forced, or that not even pull can be performed when there are changes
- pull (perform git pull on all repos)
- push (perform git push on all repos)
- status (perform git status on all repos)
- clone (clones missing repos)
- help (duh)

## output

All these commands should return a comprehensible and concise output about the state of these repos.

## Manifest file

Probably a yaml, toml, or json file.
Contains info about all the repos that are supposed to be synced.
Configs per repo are:

- filesystem location
- remote address
- included branches (defaults to all)
- excluded branches (defaults to none)

## options / global config

CLI flags that modify behaviour.
May also be written to a config file under XDG_CONFIG_DIR/repoteer/repoteer.yaml
Options are:

- colour output
- verbosity
- concurrency
- sync behaviour
- location of the manifest file (with some default)
- --help / -h > just runs repoteer help

## functionality

The idea is basically that repoteer reads the manifest, and then goes into each repo and performs the issues command(s).
Meanwhile, it prints an output on the state of these processes.
These processes should be trivial to run concurrently, since they do not depend on one another, though that does make writing the output pretty tough, probably, if I want to write output for processes that are still ongoing too.

