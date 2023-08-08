# Intranet-music-player

Simple web-based music player

```sh
$ MUS_DIR=/home/me/some/dir PORT=3000 musrs
```

Run on a local network and access music by visiting `http://host:3000`

I wanted to have my music in one place, and be able to listen from any computer on my local network with no setup required.

Currently it is hard coded to assume your library is laid out in `{Artist}/{Album}/{Song}` format.

Code is not ideal, it blocks in place on startup and there are `.clone()`s everywhere.

## Libraries used
- Rust stdlib
- tokio (async runtime)
- actix (web framework)
- tera (jinja-style templates)
- serde (serialization)
- env_logger (logging)
- static-files (bundling assets into binary)
- actix-web-static-files (static-files integration for actix)

See TODO.md for planned features