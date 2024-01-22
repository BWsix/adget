# adget

wget-like magnet downloader, powered by your premium AllDebrid account.

> folders are not supported .\_.

### requirements

- rust <https://www.rust-lang.org/tools/install>
- all debrid premium account

### quick start

1. clone and cd into this repo
2. run: `cargo build --release`
3. cp `./target/release/adget` to `/usr/local/bin/`
4. run: `adget <magnet>`
5. generate an apikey here: <https://alldebrid.com/apikeys/>
6. paste your apikey into the terminal (you only have to do this once)
7. profit

### how adget works

1. upload magnet to add debrid with `/magnet/upload`
2. get magnet status and locked download link with `/magnet/status`
3. unlock download link with `/link/unlock`
4. spawn and switch to wget with unlocked download link
5. wget takes care of the rest of the downloading process
