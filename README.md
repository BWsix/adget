# adget

wget-like magnet downloader, powered by your premium all debrid account.

> (folders are not supported .\_.)

### requirements

- rust <https://www.rust-lang.org/tools/install>
- all debrid premium account

### quick start

1. clone and cd into this repo
2. run: `cargo build --release`
3. cp `./target/release/adget` to `/usr/local/bin/`
4. get apikey here: <https://alldebrid.com/apikeys/>
5. run: `adget <apikey> <magnet>`

### how adget works

1. upload magnet to add debrid with `/magnet/upload`
2. get magnet status and locked download link with `/magnet/status`
3. unlock download link with `/link/unlock`
4. spawn and switch to wget with unlocked download link
5. wget takes care of the rest of the downloading process
