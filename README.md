![Rust](https://github.com/guewen/gifts/workflows/Rust/badge.svg?branch=master)

# Gifts

Gifts is a tool to organize "Secret Santa" gifts exchange events.

According to a configuration file, it randomly generates "giver/receiver" pairs,
and send e-mails to the giver with the name of the person to whom they offer
something.

People are organized in groups (e.g. families, couples, ...). Gifts are not exchanged
within a group. If everybody can be the Secret Santa of anybody, even within a family or couple,
groups of 1 person can be used.

Hints can be given to Secret Santas: if Alice and Bob are in the same group, Alice and Bob will
not give a gift to one another. If Janet is drawn to be the Secret Santa of Alice, and John to
be Bob's one, Janet will receive a hint that John is the Secret Santa of Bob and John will be informed
that Janet is Alice's one. So they can possibly synchronize and offer a shared gift for both.

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
groups:
  - people:
      - email: alice@example.com
        name: Alice
      - email: bob@example.com
        name: Bob
  - people:
      - email: jules@example.com
        name: Jules
      - email: janet@example.com
        name: Janet
  - people:
      - email: john@example.com
        name: John
  - people:
      - email: foo@example.com
        name: Foo
      - email: bar@example.com
        name: Bar
config:
  smtp:
    address: stmp.gmail.com
    port: 587
    user: email-user@example.com
    password: password
  email:
    from: email-user@example.com
    subject: Gift for our Lackadaisical party
    body:  |-
      Hey {giver},
      
      The magical thingy decided that you'll offer a gift to... {receiver}.
      
      {{- if has_secrets }}
      I can tell you something...
      {{ for secret in secrets }}
      { secret.0.name } is the secret santa of { secret.1.name }... 🤫
      {{- endfor }}
      {{- endif }}
```

The SMTP configuration is used to send the emails.

In the email configuration, the body has optional 2 placeholders (at least the
receiver is mandatory if you want the giver to know who receives their gift):
``{giver}`` and ``{receiver}``.

Hints can be used in a section as following:


```
      {{- if has_secrets }}
      I can tell you something...
      {{ for secret in secrets }}
      { secret.0.name } is the secret santa of { secret.1.name }... 🤫
      {{- endfor }}
      {{- endif }}
```

If you don't want hints, leave it out of the email body.


## Run

Before sending actual emails, you can test with:

```
cargo run -- --config config.yml --debug
```

When you are ready, run the final command, the emails will be sent.

```
cargo run -- --config config.yml
```

## License

Gifts is distributed under the terms of both the MIT license
and the Apache License (Version 2.0)

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.
