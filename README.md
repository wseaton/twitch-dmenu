
# twitch-dmenu

Basic CLI utility for grabbing followed streams and dumping them to stdout, for use in piping to `streamlink` or other utilities. Written in Rust, using [twitch_api2](https://github.com/Emilgardis/twitch_api2).

Example:

```sh
twitch-dmenu | dmenu -l 100 | awk '{print $1}' | xargs -I{} streamlink twitch.tv/{} best --player mpv
```

## Configuration

Currently either these three env vars need to be set:

* `TWITCH_ACCESS_TOKEN`
* `TWITCH_REFRESH_TOKEN`
* `TWITCH_CLIENT_ID` (optional)

AND/OR you can use a configuration file:

`~/.config/twitch_dmenu/conf`

```ini
[twitch-dmenu]
access_token = {ACCESS_TOKEN}
refresh_token = {REFRESH_TOKEN}
client_id = {CLIENT_ID}
```

Tokens can be generated using the [twitch token generator tool](https://twitchtokengenerator.com/)

## TODO

* [ ] Properly handle pagination for paginated endpoints (I don't currently follow that many users).
* [x] Add credential configuration from file
* [ ] Add CLI based auth flow?

## Installation

First clone the repo, then:

```sh
cargo install --path .
```

Or alternatively you can just `cargo install` from source directly:

```sh
cargo install --git https://github.com/wseaton/twitch-dmenu.git  
```