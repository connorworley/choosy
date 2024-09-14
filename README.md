# choosy
Choosy program that launches other programs depending on its arguments.
I wrote it because I use multiple Chrome browser profiles and I want certain links to always open in certain profiles.

## Installing
```sh
cargo install choosy
```
Make sure to set it as your default browser.

## Configuration
Choosy is configured in `~/.config/choosy.toml`.

Choosy maps regexes to commands, along with additional arguments to be passed to the command.
The longest match is used to select a program.
A default program is required.

Example: always open youtube links with `google-chrome --profile-email=bar@example.org`
```toml
default = { command = 'google-chrome', args = [] }

[overrides]
'^(https?://)?(www\.)?youtube\.com' = { command = 'google-chrome', args = ['--profile-email=bar@example.org'] }
```
