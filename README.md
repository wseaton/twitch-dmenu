
# twitch-dmenu

Basic CLI utility for grabbing followed streams and dumping them to stdout, for use in piping to `streamlink` or other utilities. Written in Rust, using [twitch_api2](https://github.com/Emilgardis/twitch_api2).

Example:

```sh
twitch-dmenu | dmenu -l 30 | awk '{print $1}' | xargs -I{} streamlink twitch.tv/{} best --player vlc
```

## Configuration

Currently these three env vars need to be set:

* `TWITCH_ACCESS_TOKEN`
* `TWITCH_REFRESH_TOKEN`
* `TWITCH_CLIENT_ID` (optional)

Tokens can be generated using the [twitch token generator tool](https://twitchtokengenerator.com/)

## TODO

- [ ] Properly handle pagination for paginated endpoints (I don't currently follow that many users).
- [ ] Add credential configuration from file
- [ ] Add CLI based auth flow?

## Installation

First clone the repo, then:

```
cargo install --path .
```