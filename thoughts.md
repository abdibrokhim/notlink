[logs]

[part 1]

# [startup idea]
[title]: Blazing Fast URL Shortener.
[techstack]: Made using some of the fastest frameworks and tools, including Rust, Actix, Diesel, Shuttle, PostgreSQL, Inferno, and Typescript.
[short_description]: I am building url shortener using Rust, Actix-web, Diesel, Shuttle.dev and shared Database from Shuttle.dev PostgreSQL.
[backend]:
Three main features of this project are:
1. I am using Diesel for database operations.
2. I am using Rust and Actix-web for backend.
3. I am using Shuttle.dev for shared database and deployment.

[part 2]
[premium features]

let's work on premium features.
we need a new table. `connected_wallets` table.
we have following columns: `id`, `wallet_address`, `tries_left`, and `created_at`.


[draft]

user should to pay extra $0.99 usd for each url shortening if they wanna use advanced features.

1. encrypt stored URLs to enhance security and protect user data.
2. custom url aliases.
3. expiration dates.

they can select all three at ones or any of them just paying $0.99 usd.

okey , about:

QR Code Generation. i believe we should take another $0.99 usd for this feature.

API Access - for developers. will think about it later.

maybe i will introduce just monthly subscription for advanced features.

expirable urls.

by defautl urls will never expire. 

but they pay for it. it will expire after 24 hours. defautl for all urls.

it helps; safety, security, and privacy.

--

we need to integrated JWT for authentication.