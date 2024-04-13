# adget

wget-like magnet downloader, powered by your premium AllDebrid account.

adget is composed of the following two binaries

- `adgetd` for ddl
- `adgetm` for magnetes

> folders are not supported .\_.

### requirements

- rust <https://www.rust-lang.org/tools/install>
- wget
- an All Debrid premium account

### quick start

1. clone and cd into this repo
2. run: `cargo build --release`
3. cp `./target/release/adgetd` and `./target/release/adgetm` to `/usr/local/bin`
4. run: `adgetm <magnet>` or `adgetd <premium ddl>`
5. generate an apikey here: <https://alldebrid.com/apikeys/>
6. paste your apikey into the terminal (you only have to do this once)
7. profit

### how adm works

1. upload magnet to AllDebrid with `/magnet/upload`
2. get magnet status and locked download link with `/magnet/status`
3. unlock download link with `/link/unlock`
4. spawn and switch to wget with unlocked download link
5. wget takes care of the rest of the downloading process
