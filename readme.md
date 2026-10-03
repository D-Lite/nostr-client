# nostr-client

A small Nostr client written in Rust. It creates a Nostr event, computes its id, signs it with a Schnorr signature (BIP-340), and prints the result.

Based on the series at https://nickmonad.blog/2023/building-nostr-client-index-0/

## Status

Working:
- Event and tag types with Nostr-compliant JSON serialization
- Event id (SHA-256 of the serialized event)
- Loading an `nsec` private key and signing events

Not yet built:
- Connecting to relays
- Reading or subscribing to events

## Requirements

- Rust (stable, install from https://rustup.rs)
- A Nostr private key in `nsec1...` format. You can generate one here: https://nostrtool.com/

## Setup

1. Clone the repo:

```
git clone https://github.com/d-lite/nostr-client.git
cd nostr-client
```

2. Create a `.env` file in the project root:

```
ROOSTR_PRIVATE_KEY=nsec1yourkeyhere
```

3. Run it:

```
cargo run
```

You can also pass the key directly instead of using `.env`:

```
cargo run -- nsec1yourkeyhere
```

## Project layout

```
src/
  lib.rs          public exports
  main.rs         example: build, sign, and print an event
  event/
    mod.rs        Event struct
    data.rs       EventData and id calculation
    kind.rs       event kinds
    tag.rs        tags
    sign.rs       key loading and signing
```

## Tests

```
cargo test
```

The event id test checks the id calculation against a real published Nostr event.
