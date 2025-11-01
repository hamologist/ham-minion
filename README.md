# Ham Minion
**Ham Minion** is a Discord bot written in Rust.
It provides access to the following slash commands:
- emojify - spices up a blob of text with a random assortment of emojis
  - Using the [emojify](https://github.com/hamologist/emojify) repo
- roll - simulates dice rolls with varying sides and counts
  - Using the [dice-roll](https://github.com/hamologist/dice-roll) repo
- ping - replies with pong

## Installation

### Local Installation
You can install the package directly to your machine using cargo:
```bash
$ cargo install --git https://github.com/hamologist/ham-minion.git --branch main
```

Likewise, you can uninstall the application using:
```bash
$ cargo uninstall ham-minion
```

### Docker
You can also install and run the package using Docker like so:

First, build the image for the package:
```bash
$ docker build -t ham-minion https://github.com/hamologist/ham-minion.git#main
```

Next, run a container using the image you built:
```bash
$ docker run --rm ham-minion -e DISCORD_TOKEN='{DISCORD_TOKEN_VALUE}'
```
This will run the `ham-minion` command (further detailed below).

## Usage
Once ham-minion has been installed, you can run it using the `ham-minion` command.

The command does require running with a `DISCORD_TOKEN` environment variable.
In order to get a token, users will need to create a bot using the [Discord Developer](https://discord.com/developers) pages.
Likewise, the bot you create will need to be added to a Discord Server.
Consider reading over the [Discord Developer Docs](https://discord.com/developers/docs/intro) for more information.

Here is an example of what using the tool looks like after you have a properly configured bot and token:
```bash
$ DISCORD_TOKEN={DISCORD_TOKEN_VALUE} ham-minion
```

You should then see slash commands associated with this bot appear on the Discord servers you added your bot too.
**Note:** You may need to first refresh your Discord window before seeing the new slash commands on first try.
