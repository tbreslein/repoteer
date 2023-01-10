# repoteer

CLI tool for keeping multiple git repositories across your machine in sync with their remotes.

I wrote this because I have a ton of repositories on my system and I work on them from multiple machines, so I needed an easy command to go through all of those local repos and synchronise them.
This tool does exactly what I need it to do, and apart from squashing the one bug I found, I probably won't do anything to it anytime soon, so don't mind it if the last commit is months old.

## Installation

I have not really packaged this for any system yet, and I so far have only tested the program on Linux.
Since I use NixOS though, I have written a `flake.nix` and I am installing this tool in my NixOS config through this repo's flake.
Thus your only options currently are cloning this repo and running `cargo install --path /path/to/the/repo`, or, if you are running a system with the `nix` package manager and have flake support, you can install the repo.

## Usage

Run `repoteer --help` to get an overview of the commands.
In order for `repoteer` to do anything though, you need to write a repository manifest, which tells `repoteer` which repositories to operate on.
This file is a TOML file that repoteer will look at `$HOME/.config/repoteer/manifest.toml`, at least on linux systems.
I have not implemented support for other OS' in this regard.
You can alternatively provide a path to a manifest file using the `-m` flag.

### `manifest.toml`

The manifest has a pretty simple structure.
Here is an example of a file like this:

```toml
[[repos]]
url = "https://www.github.com/testuser/testrepo.git"
path = "/home/foo/testrepo"

[[repos]]
url = "git@bitbucket.com:bbuser/somerepo.git"
path = "/home/bar/somerepo"

[[repos]]
url = "git@gitlab.com:gitlabuser/gitlabrepo.git"
path = "/root/gitlabrepo"
```

Basically, each repository you want to operate on, needs a `[[repos]]` entry, and each one of those needs:

- `url`: This can either be an http/https address, or an ssh slug.
  Currently, repoteer only supports git repositories.
- `path`: The absolute path on your filesystem, where the repository's clone should reside.

### Commands

`repoteer` supports several commands that tell it what kind of operation to run on your manifest.
These commands are:

- `clone`: Clones the repositories that have not been cloned yet
- `pull`: Pull changes in all repositories and their branches
- `push`: Push local changes for all branches
- `sync`: chain `clone`, `pull`, and `push` commands

If you do not provide a command to `repoteer`, it will default to `sync`.

## Known bugs

- non-clone git commands do not print errors

## TODO

- [ ] figure out packaging?

## Contributing

Please use the github issues for suggestions, bugs, etc., and if you would like to contribute (especially when it comes to packaging or OS support, because I probably will not implement that myself) just open pull requests.

Just keep things civil, make your intent clear on the issues and PRs, etc.

## Licensing

> _Copyright © `2022-2023`, `Tommy Breslein`_
>
> _All rights reserved._
>
> Redistribution and use in source and binary forms, with or without
> modification, are permitted provided that the following conditions are met:
>
> 1.  Redistributions of source code must retain the above copyright
>     notice, this list of conditions and the following disclaimer.
> 2.  Redistributions in binary form must reproduce the above copyright
>     notice, this list of conditions and the following disclaimer in the
>     documentation and/or other materials provided with the distribution.
> 3.  Neither the name of the copyright holder nor the
>     names of its contributors may be used to endorse or promote products
>     derived from this software without specific prior written permission.
>
> THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS “AS IS” AND
> ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
> WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
> DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER BE LIABLE FOR ANY
> DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
> (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
> LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
> ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
> (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
> SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
