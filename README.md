<div align="center">

# User Searcher

**Check if a username is taken across 30+ platforms — all from your terminal.**

[![Rust](https://img.shields.io/badge/built_with-Rust-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Made by Skyrix](https://img.shields.io/badge/made%20by-Skyrix-9ff5a9)](https://github.com/Skyrix-7/)

![image alt](https://github.com/Skyrix-7/userfinder/blob/ad0e1dc639fa4b1525a5cc9d0201ad260c06f487/preview.png)
</div>

---

## What is it?

**User Searcher** is a fast, async Rust CLI tool that checks whether a specific username is available or already taken across more than 40 social media and developer platforms. Type in a username, hit Enter, and watch it sweep the web — reporting exactly where that name is **found** or **not found**.

```
? [ ] Input Username: john
* [ ] Checking username john on:

[+] GitHub:      Found!      https://github.com/john
[-] Facebook:    Not Found!
[+] Reddit:      Found!      https://reddit.com/user/john
...
```

## Features

- **40+ platforms** checked in one shot — social media, dev tools, and indie sites
- **Fully asynchronous** (Tokio + Reqwest) for fast concurrent lookups
- **Color-coded output** — green `[+]` for found, red `[-]` for not found
- **Terminal-friendly** UI with an ASCII-art banner and live link
- **Zero configuration** — compile, run, type a username, done

## Supported Platforms

| Category | Platforms |
| --- | --- |
| **Social** | Instagram, Facebook, Twitter / X, Snapchat, TikTok, Pinterest, Reddit, YouTube, LinkedIn, Discord, Telegram, Mastodon, Bluesky, Tumblr |
| **Coding / Dev** | GitHub, GitLab, Bitbucket, Stack Overflow, Dev.to, Codeberg, SourceHut, crates.io, npm, PyPI |
| **Creative / Media** | Medium, Spotify, SoundCloud, Bandcamp, Vimeo, Dribbble, Behance, Twitch, itch.io |
| **Community / Other** | Quora, Patreon, Kickstarter, Steam, Docker Hub and more |

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024, stable toolchain)

### Run

```bash
# From the project directory
cargo run
```

> Tip: use a release build for the fastest scanning:
> ```bash
> cargo run --release
> ```

Then simply enter the username you want to check and press **Enter**.

## Build

```bash
cargo build --release
./target/release/web_extractor
```

## How It Works

1. You input a username.
2. The tool builds a profile URL for each platform (e.g. `https://github.com/<user>`).
3. It sends an HTTP request to every URL asynchronously.
4. A URL is reported as **Found** when the page responds successfully **and** doesn't contain known "user doesn't exist" markers.
5. Everything else is reported as **Not Found**.

## Disclaimer

- Results depend on network conditions and platform anti-bot behavior; some sites may block automated requests.
- This tool only checks whether a URL returns an "exists" style page — it does **not** scrape, collect, or store personal data.
- Use responsibly and respect each platform's terms of service.

## Contributing

Want more platforms? Pull requests are welcome. Just add an entry to the `urls` list in [`src/main.rs`](src/main.rs).

## Author

Developed by [Skyrix-7](https://github.com/Skyrix-7/) — v1.0

## License

MIT License — this project is released under the MIT License.
