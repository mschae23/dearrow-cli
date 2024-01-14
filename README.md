# DeArrow CLI
A CLI program to view and vote for DeArrow submissions.

## Usage
### View titles
```
dearrow-cli view title -v <VIDEO_ID>
```

### View thumbnails
```
dearrow-cli view thumbnail -v <VIDEO_ID>
```

### Vote
Voting requires your private ID to be set using the `SPONSORBLOCK_PRIVATE_USERID` environment variable.

```
dearrow-cli vote -v <VIDEO_ID> title "Some title"
dearrow-cli vote -v <VIDEO_ID> --downvote title "A bad title"

dearrow-cli vote -v <VIDEO_ID> thumbnail at 10.123
dearrow-cli vote -v <VIDEO_ID> thumbnail original
dearrow-cli vote -v <VIDEO_ID> --downvote thumbnail at 10.123
dearrow-cli vote -v <VIDEO_ID> --downvote thumbnail original
```

If you are a VIP user and want to disable auto-lock, pass the `--no-auto-lock` option before `title` or `thumbnail`.

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
