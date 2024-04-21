# RS Reminder Bot

This is a Discord bot that sends reminders about events in Runescape.

It uses the Discord REST API to send messages, thanks to the [twilight](https://github.com/twilight-rs/twilight) library.

Uses less than 800KB of RAM and uses exactly zero CPU when idle.

Note that the reminders are tailored specifically to me and not currently customizatable.

## Deployment - NixOS

First add the contents of systemd-service.nix to `/etc/nixos/configuration.nix`
(yes I know I could use modules or flakes eventually). Then run this:

```
./deploy.sh
```

## Disclaimer

This code is not designed to be re-used or repurposed for in any serious context.
