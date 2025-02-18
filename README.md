<!-- markdownlint-disable -->
<p align="center">
  <a href="" rel="noopener">
    <img width=200px height=200px src="https://raw.githubusercontent.com/abdibrokhim/notlink/refs/heads/main/assets/notlink-logo.png">
  </a>
</p>

<p align="center">
  <a href="https://github.com/abdibrokhim/notlink/search?l=rust">
    <img alt="language" src="https://img.shields.io/badge/language-Rust-black.svg">
  </a>
  <a href="https://yaps.gg">
    <img alt="feedback" src="https://img.shields.io/badge/feedback-notl.ink-black">
  </a>
    <a href="https://notl.ink">
    <img alt="github stars" src="https://img.shields.io/github/stars/abdibrokhim/notlink?style=social"/>
  </a>
</p>
<p align="center">
  <a href="https://github.com/abdibrokhim/notlink/issues">
    <img alt="crates" src="https://img.shields.io/github/issues-raw/abdibrokhim/notlink.svg">
  </a>
  <a href="https://discord.gg/nUdcd9p8Ae">
    <img alt="discord" src="https://img.shields.io/discord/1325139842388070520?logo=discord"/>
  </a>
  <a href="https://yaps.gg">
    <img alt="x(twitter) follow" src="https://img.shields.io/twitter/follow/abdibrokhim">
  </a>
</p>

<h1 align="center">The Fastest URL Shortener Ever</h1>
<div align="center">
it takes less than 1s to shorten a URL.<br>
try <a href="https://notl.ink">notl.ink</a>
</div>

<h3 align="center">Super Simple. &nbsp; Blazingly Fast. &nbsp; Open Source.</h3>

<p align="center">
    <a href="https://yaps.gg">Report Bug</a>
    ·
    <a href="https://yaps.gg">Request a Feature</a>
    ·
  <a href="https://discord.gg/nUdcd9p8Ae">Join us on Discord</a>
    ·
    <a href="https://x.com/abdibrokhim">Follow us on X</a>
  </p>

<!-- <div align="center"><img src="" width="100%" ></div> -->

*<div align="center">⭐ If you find notlink interesting, consider starring this repo to help spread the word.</div>*

<h1 align="center">notlink is officially live on Product Hunt!</h1>

[![live demo](/assets/ph-live.png)](https://cards.producthunt.com/cards/products/992403)

## About The Project
This is notlink - a blazingly fast URL shortener ever built with Rust programming language. It is designed to be simple, secure, and fast. notlink is open-source and free to use. It is also fully customizable and can be self-hosted on your own server. [Join us!!](https://discord.gg/nUdcd9p8Ae).

[![live demo](/assets/notlink-yt.png)](https://youtu.be/ahZV6aAdnVI?si=N0O3RFYXE4Zz1Uhc)


## Built With
notlink is built with the following awesome open-source technologies:
- [Rust](https://www.rust-lang.org/)
- [Actix](https://actix.rs/)
- [Diesel](http://diesel.rs/)
- [Shuttle](https://shuttle.dev/)
- [Neon](https://neon.tech/)
- [NextJS](https://nextjs.org/)
- [Typescript](https://www.typescriptlang.org/)

notlink's UI could be found in [this repo](https://github.com/abdibrokhim/notlink-ui)

## Features

1. **Shorten URLs:** 
Convert long URLs into short, easy-to-share links.

2. **Data Encryption:**
Encrypt stored URLs to enhance security and protect user data.

3. **Expiration Dates:**
Enable URLs to expire after a certain date or number of uses.

4. **API Access:**
Provide a minimal RESTful API for programmatic access to the URL shortener.

## Roadmap

1. **Custom URL Aliases:** 
Allow users to create custom shortened URLs.

2. **QR Code Generation:**
Automatically generate QR codes for each shortened URL.

## Quick Start

Clone the repository:

```shell
git clone https://github.com/abdibrokhim/notlink.git
```

Install the dependencies with:

```shell
cargo install
```

Copy the `.env.example` file to `.env`:

```shell
cp .env.example .env
```

Replace with your stuff:

```shell
DATABASE_URL=postgresql://...:...@.../...?sslmode=require
CRYPTO_KEY=...
HOST=...
```

Generate a secure random key (you can use openssl command line):

```shell
openssl rand -hex 32
```

Run the development server with:

```shell
cargo shuttle run
```

Send request here: [http://localhost:8000](http://localhost:8000). 
You may try to run the tests with. But there's no tests yet, lol.
Anyway refer to [APITESTS.md](./APITESTS.md) for examples of how to test the API endpoints.

## Tutorial 🥳

Tutorial is available on [TUTORIAL.md](https://github.com/abdibrokhim/notlink/blob/main/TUTORIAL.md). Check it out!!

For the video tutorial, you can check it out on [YouTube](https://youtu.be/Mhz_eVH3XnQ?si=fX6GqIq79QGq6C69)

[![live tutorial](/assets/notlink-ttrl.png)](https://youtu.be/Mhz_eVH3XnQ?si=ZhmY5BtrsblCrjaS)

## Donate & Support

- [Support on Open Collective](https://opencollective.com/opencommunity)
- [Support on Patreon](https://www.patreon.com/abdibrokhim)

### Crypto

- Bitcoin: bc1qpylxaqwapk0tgdmpnnljj545z4lk2z9q5us6p6
- Ethereum: 0xb4a8e71d82e8Bf84a02C7770585F9cD8b267aDB9
- Solana: 4MpPHapcdb5MwRy57juUQ2wUt1EJo8BYqXsYKwSfCvz1

Wanna learn more about me? refer to [my official website](https://yaps.gg).

## Contributing

Contributing is highly encouraged!
Even if you are not planning to submit any code, joining our [Discord server](https://discord.gg/nUdcd9p8Ae) and providing feedback helps us a lot!

Check out our [CONTRIBUTING.md](./CONTRIBUTING.md) and find the appropriate repo above to contribute to.

## Open Community

Join the awesome community on [Discord](https://discord.gg/nUdcd9p8Ae)

## Credits

Logo made by Ibrohim Abdivokhidov (@abdibrokhim). Follow him on [LinkedIn](https://www.linkedin.com/in/abdibrokhim/) and/ or [X (twitter)](https://twitter.com/abdibrokhim)

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=abdibrokhim/notlink&type=Date)](https://star-history.com/#abdibrokhim/notlink&Date)

*⭐ If you find notlink interesting, consider starring this repo to help spread the word.*

## Contributors ✨

Thanks goes to these wonderful people:

<a href="https://github.com/abdibrokhim/notlink/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=abdibrokhim/notlink" />
</a>

Made with [contrib.rocks](https://contrib.rocks).