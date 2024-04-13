# adget

Unlocks premium link or downloads magnet with All Debrid premium account.

### examples

```bash
# download from premium host with wget
adgetd "https://alligator.com/somefile.zip" -- -O renamed.zip

or

# get all debrid download link of a magnet
adgetm -n "magnet:?xt=..." >> links.txt
```

### requirements

- rust <https://www.rust-lang.org/tools/install>
- wget
- an All Debrid premium account

### quick start

1. clone and cd into this repo
2. run: `cargo build --release`
3. cp `./target/release/adgetd` and `./target/release/adgetm` to
   `/usr/local/bin`
   1. or run the script `./scripts/deploy.sh`
   2. to remove adget, we have `./scripts/undeploy.sh`
4. run `adgetm <magnet>` or `adgetd <premium link>`
   1. (you only have to do the following in the first run or when the apikey
      becomes invalid)
   2. generate an apikey here: <https://alldebrid.com/apikeys/>
   3. paste your apikey into the terminal
5. profit

### usage

```txt
> adgetm --help
Magnet downloader

Usage: adgetm [OPTIONS] <magnet> [-- <wget-args>...]

Arguments:
  <magnet>        The magnet to download
  [wget-args]...  Args for wget

Options:
  -n, --nodl  Do not download with wget; only prints out the link
  -h, --help  Print help
```

```txt
> adgetd --help
Premium link downloader

Usage: adgetd [OPTIONS] <link> [-- <wget-args>...]

Arguments:
  <link>          The premium link to unlock
  [wget-args]...  Args for wget

Options:
  -n, --nodl  Do not download with wget; only prints out the unlocked link
  -h, --help  Print help
```

### how adgetm works

1. upload magnet to AllDebrid with api `/magnet/upload`
2. get magnet status and locked download link with `/magnet/status`
3. unlock download link with `/link/unlock`
4. spawn and switch to wget with unlocked download link
5. wget takes care of the rest of the downloading process
