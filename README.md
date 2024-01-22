# Nostr Metadata Enhancer 

日本語ドキュメントこちら(jp only) -> [README_ja](./README_ja.md)

What is this?

This application retrieves the Kind: 0 of your specified account on Nostr and Publish to the specified relay.  

Why is this necessary?

Nostr's relays do not have a clear retention period, and there are no guarantees regarding loss of data.  
Posts are always up-to-date, but you risk the loss of previously created profile information kind: 0, which is never updated.

Look around you.

Is the bot's profile information being maintained on a server that is running with no maintenance? You wouldn't forget it like jam in the fridge, would you?

## Usage

This is built in Rust. First, please prepare to run Rust on your PC or server.

When you are ready, clone this repository.

```` git clone
git clone 
````

By the way, the game Pulworld is fun, isn't it? Let's find a gap and don't touch it, shall we?

Next, copy config.example.json.

````
cp config.example.json config.json
```

Then edit the JSON content. Write nsec1... for the account you want to maintain. Of course, add any relays you want to spread.

Build the program; we don't distribute it via Cargo.

````
cargo build
build

Run the program.

````
cargo run
```

## Ongoing operations

Running the program every day is hard, harder than contributing a day. You will forget. If you want to run it on a server, crontab is the way to go.

You can run it on your PC, but if you are running a bot, you have a server there, right? Let's do it on the server.
