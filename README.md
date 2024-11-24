# DeArrow CLI
A CLI program to view and vote for DeArrow submissions.

## Installation
System packages are available for Fedora and Arch. Using these is recommended over installing the binary manually.

### Fedora
Add the repository first:
```
sudo dnf config-manager --add-repo https://mschae23.de/git/api/packages/mschae23/rpm/fc41.repo
```

On newer releases, you may have to use the following instead:
```
sudo dnf config-manager addrepo --from-repofile=https://mschae23.de/git/api/packages/mschae23/rpm/fc41.repo
```

When DNF prompts you for the repository's signing key, make sure it has the following fingerprint before accepting:
```
60C9 7C71 B476 4DF9 319D  4F02 1315 D06E 9C20 41E8
```

Then install the package:
```
sudo dnf install dearrow-cli
```

### Arch
Import the repository's signing key first:
```
wget -O mschae23-pkg-sign.gpg https://mschae23.de/git/api/packages/mschae23/arch/repository.key
sudo pacman-key --add mschae23-pkg-sign.gpg
sudo pacman-key --lsign-key 'pkg@mschae23.de'
```

Add the repository to `/etc/pacman.conf`:
```
[mschae23]
SigLevel = Required
Server = https://mschae23.de/git/api/packages/mschae23/arch/mschae23/$arch
```

Then install the package:
```
sudo pacman -Sy dearrow-cli
```

### From source
Run `cargo install --path .` in a local clone of this repository. If Cargo complains about the `mschae23` registry being missing,
add the following to `~/.cargo/config.toml`:

```toml
[registries.mschae23]
index = "sparse+https://mschae23.de/git/api/packages/mschae23/cargo/"
```

## Usage
### View submissions
```
dearrow-cli view <VIDEO_ID> title
dearrow-cli view <VIDEO_ID> thumbnail
dearrow-cli view <VIDEO_ID> main
```

For `title` and `thumbnail`, this uses [DeArrow Browser](https://github.com/mini-bomba/DeArrowBrowser)'s **internal** API
by default. For `main`, the default is the main SponsorBlockServer instance at <https://sponsor.ajay.app/>.

The DeArrow data displayed is licensed under [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/)
from <https://dearrow.ajay.app/>.

Legend for "score" column:
- `o`: Original title
- `m`: Removed by a VIP
- `x`: Shadowhidden
- `d`: Removed by downvotes
- `r`: Replaced by submitter
- `h`: Title / thumbnail should only appear in submission menu (score < 0)
- `u`: Submitted by unverified user
- `l`: Locked by a VIP
- `v`: Submitted by a VIP

### Vote
Voting requires your private ID to be set using the `SPONSORBLOCK_PRIVATE_USERID` environment variable.

```
dearrow-cli vote <VIDEO_ID> title "Some title"
dearrow-cli vote <VIDEO_ID> --downvote title "A bad title"

dearrow-cli vote <VIDEO_ID> thumbnail at 10.123
dearrow-cli vote <VIDEO_ID> thumbnail original
dearrow-cli vote <VIDEO_ID> --downvote thumbnail at 10.123
dearrow-cli vote <VIDEO_ID> --downvote thumbnail original
```

If you are a VIP user and want to disable auto-lock ("act as VIP" in UI), pass the `--no-auto-lock` option before
`title` or `thumbnail`.

### Warnings
```
dearrow-cli user <USER_ID> warnings received
dearrow-cli user <USER_ID> warnings issued
```

Shows tips (internally still called warnings) received or issued by a specific SponsorBlock or DeArrow user. You can
pass `--newest <N>` or `-n <N>` to only show the newest `N` warnings.

This uses [DeArrow Browser](https://github.com/mini-bomba/DeArrowBrowser)'s **internal** API by default.

## License
Copyright (C) 2024  mschae23

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

Note that DeArrow CLI actually has to be distributed under the terms
of the GNU Affero General Public License, version 3 (only), as published
by the Free Software Foundation, due to its dependency on DeArrow Browser.

