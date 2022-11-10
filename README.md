![Rust](https://github.com/guewen/gifts/workflows/Rust/badge.svg?branch=master)

# Gifts

Gifts is a tool to organize "Secret Santa" gifts exchange events.

According to a configuration file, it randomly generates "giver/receiver" pairs,
and send e-mails to the giver with the name of the person to whom they offer
something.

Exclusions can be configured, for instance for people already giving gifts
themselves (families).

## Build

```
cargo build
```

## Test

```
cargo test
```

## Configuration

The configuration is a yaml file. A scaffold can be generated with:

```
cargo run -- --scaffold config.yml
```

Which results in something like:

```
people:
  - email: alice@example.com
    name: Alice
    exclude:
      - bob@example.com
  - email: bob@example.com
    name: Bob
    exclude:
      - alice@example.com
  - email: jules@example.com
    name: Jules
    exclude:
      - janet@example.com
  - email: janet@example.com
    name: Janet
    exclude:
      - jules@example.com
config:
  smtp:
    address: stmp.gmail.com
    port: 587
    user: email-user@example.com
    password: password
  email:
    from: email-user@example.com
    subject: Gift for our Lackadaisical party
    body: "Hey {giver},\n\nThe magical thingy decided that you'll offer a gift to... {receiver}."

```

The SMTP configuration is used to send the emails.
In the email configuration, the body has optional 2 placeholders (at least the
receiver is mandatory if you want the giver to know who receives their gift):
``{giver}`` and ``{receiver}``

## Run

Before sending actual emails, you can test with:

```
cargo run -- --config config.yml --debug
```

Which will result in an output like:

```
Computing pairs...
Pairs generated
alice@example.com → janet@example.com
janet@example.com → bob@example.com
jules@example.com → alice@example.com
bob@example.com → jules@example.com
Done!
```

When you are ready, run the final command, the emails will be sent.

```
cargo run -- --config config.yml
```

## Roadmap

* Replace exclusions by a groups, people among a group (e.g. family) do never offer gifts internally,
  however, people who offer their gift to someone in a group see who are the givers for other people
  of the group: it allows the givers to coordinate beforehand to get a common gift
* Better templating abilities: the change above may need better options for
  formatting the e-mail body (e.g. loop over the group members)

## License

Gifts is distributed under the terms of both the MIT license
and the Apache License (Version 2.0)

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.
