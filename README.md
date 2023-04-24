# text_on_server_death

A simple Telegram bot that texts me when my sad little server dies. You can use it for your sad little servers, too!

## Installation

1. Open a terminal. I like [this one](https://flathub.org/apps/com.raggesilver.BlackBox)...
2. Clone the project: `git clone https://github.com/onkoe/text_on_server_death`
3. Go into the project folder: `cd text_on_server_death`
4. Ensure Rust is installed: `rustup`
  a. If you get a "command not found" or something, go to the [Rust site](https://rustup.rs/) and grab it! Then try again after restarting your terminal.
5. Install the project: `cargo install --path .`

## Usage

1. Fill out the short configuration file located at `~/.config/text_on_server_death/config.toml`
  a. You should just need an [API key](https://core.telegram.org/bots/api) and your username/channel name! Other entries are optional/already filled out.
2. Type `text_on_server_death` and wait. You should get a text confirming that the bot is on. Check that the green light is on every few days or so... :)

### Arguments

You can supply the bot application with a few different arguments to change how it behaves.

- `-v`, `--verbose`: Enable verbose console output, giving you more information.
- `-d`, `--daemonize`: Daemonize the bot, making it run in the background.
- `-c <location>`, `--config <location>`: Specify a different config to load.
  - Accepts both relative (config.toml) and absolute (/home/barrett/Downloads/weird_config.toml) paths.
- `-nw`, `--no-welcome`: Disables the welcome message when the bot is started.
  - Useful for bad internet connections!
  - Slightly more lonely.
- `-na`, `--no-admin`: Disables administrative commands in the Telegram app, like `config` and `kill`.
- `-ni`, `--no-input`: Disables all user input in the Telegram app.
  - If you really don't trust people, this is a nice option...

## Commands

You can ask the bot to do some thing in the app, with a cooldown. In Telegram:

- `check`: Instantly checks if the website is up.
- `status`: Gets the current status of the bot.
- `ping`: Pong! If the bot is up, you should get a reply.
- `kill`: If you're a channel moderator/API key owner, you can instantly kill the bot with this commmand.
- `config`: Get/set configuration info. Won't leak your bot key. :)
  - `config get (thing)`: Gets current setting value.
  - `config set (thing) (value)`: Sets a setting to have the given value.
  - `config list`: Lists all settings you can configure.
- `info`: Get version/source info about the bot itself.
- `help`: Lists the in-app commands.
