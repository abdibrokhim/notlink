# notlink

<div align="center">

[![GitHub issues](https://img.shields.io/github/issues-raw/abdibrokhim/notlink.svg)](https://github.com/abdibrokhim/notlink/issues)
[![License](https://img.shields.io/github/license/abdibrokhim/notlink.svg)](LICENSE)
![GitHub stars](https://img.shields.io/github/stars/abdibrokhim/notlink?style=social)

</div>

<p align="center">
  <a href="" rel="noopener">
 <img width=200px height=200px src="assets/notlink-logo.png"></a>

 <h3 align="center"><a href="">notlink</a></h3>
  <p align="center">
  this is notlink - a new kind of url shortener
    <br />
    <br />
    <a href="https://discord.gg/nUdcd9p8Ae">Join Community</a>
    ·
    <a href="https://www.linkedin.com/in/abdibrokhim/">Documentation</a>
  </p>
</p>

## About The Project

i'm building blazingly fast url shortener ever.

## Built With

- [Rust](https://www.rust-lang.org/)
- [Actix](https://actix.rs/)
- [Diesel](http://diesel.rs/)
- [Shuttle](https://shuttle.dev/)
- [Neon](https://neon.tech/)
- [NextJS](https://nextjs.org/)
- [Typescript](https://www.typescriptlang.org/)

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

## Run Locally

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
Anyway refer to [APITESTS.md]() for examples of how to test the API endpoints.

## Tutorial 🥳

Tutorial is available on [Tutorial.md](https://github.com/abdibrokhim/notlink/blob/main/TUTORIAL.md). Check it out!!

## Future Plans

i have no idea. but you can still donate and support project. look below.

## Donate & Support

- [Support on Open Collective](https://opencollective.com/opencommunity)
- [Support on Patreon](https://www.patreon.com/abdibrokhim)

### Crypto

- Bitcoin: bc1qpylxaqwapk0tgdmpnnljj545z4lk2z9q5us6p6
- Ethereum: 0xb4a8e71d82e8Bf84a02C7770585F9cD8b267aDB9
- Solana: 4MpPHapcdb5MwRy57juUQ2wUt1EJo8BYqXsYKwSfCvz1

You can also find them here on [my official website](https://imcook.in) (scroll a little down)

## Contributing

You can just Contribute. Kindly check the [CONTRIBUTING.md](https://github.com/abdibrokhim/notlink/blob/main/CONTRIBUTING.md)

## Community

Join the community on [Discord](https://discord.gg/nUdcd9p8Ae)

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=abdibrokhim/notlink&type=Date)](https://star-history.com/#abdibrokhim/notlink&Date)

Put a star ⭐️

## Credits

Logo made by Ibrohim Abdivokhidov (@abdibrokhim). Follow him on [LinkedIn](https://www.linkedin.com/in/abdibrokhim/) and/ or [X (twitter)](https://twitter.com/abdibrokhim)