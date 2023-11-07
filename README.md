<img src=".github/assets/regy_banner.png">

# Regy-Bot

> Regex moderation bot for Discord.

## Important info

* MSRV Policy 1.67.1 (Last checked 10/27/23)
* Built to run on Windows (Linux platforms are tested infrequently but have a known support for the bot as shown in the [Features](#Features) section)

## Features

| Feature         |  Windows  |  Linux  |
|-----------------|-----------|---------|
| Message Scanning                   | ✅ | ✅ |
| Username Scanning                  | ✅ | ✅ |
| Infraction Tracking                | ✅ | ✅ |
| Message logging                    | ✅ | ✅ |
| Server Cloning                     | ✅ | ✅ |
| Auto Raid Detection ([IPM](#IPM))  | ✅ | ✅ |
| Auto-mod Integration               | ✅ | ✅ |
| Regex Template Index ([RTI](#RTI)) | ✅ | ✅ |
| Dead Zone Channels                 | ✅ | ✅ |
| Seamless Poll Detection            | ✅ | ✅ |
| Message Nuking                     | ✅ | ✅ |
| Compiled Regx Cache ([CRC](#CRC))  | ✅ | ✅ |
| Self Update System                 | ✅ | ❌ |

### IPM

IPM is a system that allows the bot to detect raids and automatically ban the raiders. It works by scanning messages, whenever it moderates a user it will add a tally for the server which resets every minute. If the tally reaches a certain threshold it will sent a ping (alert) to the staff inside the logging channel notifying them of a possible raid so that they can take appropriate action based on the situation.

### RTI

RTI is a index of community made regex templates that can be used to moderate your server. Server moderators can use `/search_rti <phrase>` to search the frequently updated list for phrases that match the tags of that search. If a template is found that matches the search it can be added to the server by reacting to it. RTI phrases have versions and can automatically update when a new version is released by the moderators running `/update_rti` this command will update all RTI phrases in the server to the latest version available inside the RTI index.

### CRC

CRC is a cache generated during runtime that contains compiled regex phrases, this is a optimization regy uses to speed up its message scanning.

## Quick-Start

### Before building

* Make sure you have the latest version of rust installed preferably if not make sure it follows the [MSRV Policy](#important-info)
* Make sure you have git installed and added to your path

### When building

1. Clone the repository and open up the main branch or develop branch, note that the develop branch may have in progress features and will be more
prone to bugs.
2. Build using cargo like so: `cargo build --release`
3. Run the executable in `target/release/regy_bot.exe`
4. The bot will generate a template config file in the same directory as the executable and then exit
5. Configure the bot using the config file under the global options and add your bot token
6. Re-run the executable and the bot should start up and be ready to use

### When using

* The bot has to be set up in each server before it can be used to its full potential with `/config_setup <channel>` the channel will be the designated channel for the bot to log messages and infractions in