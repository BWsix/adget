# adget

Unlocks premium link or downloads magnet with All Debrid premium account.

### examples

```bash
# download from premium host with wget, and set the output document to `renamed.zip`
adget "https://alligator.com/somefile.zip" -- -O renamed.zip

# get an all debrid download link of a magnet, and append to `links.txt`
adget -n "magnet:?xt=..." >> links.txt
```

### requirements

- rust <https://www.rust-lang.org/tools/install>
- wget
- All Debrid premium account

### quick start

1. clone and cd into this repo
2. run: `cargo build --release`
3. cp `./target/release/adget` to `/usr/local/bin`
   1. or run the script `./scripts/deploy.sh`
   2. to remove adget, we have `./scripts/undeploy.sh`
4. run `adget <magnet>` or `adget <premium link>`
   1. (you only have to do the following in the first run or when the apikey
      becomes invalid)
   2. generate an apikey here: <https://alldebrid.com/apikeys/>
   3. paste your apikey into the terminal
5. profit

### usage

```txt
> adget --help
All Debrid Downloader

Usage: adget [OPTIONS] <link> [-- <wget-args>...]

Arguments:
  <link>          Magnet or Premium Link To Download
  [wget-args]...  Args for wget

Options:
  -n, --nodl  No Download: only prints out the all debrid link
  -h, --help  Print help
``````
