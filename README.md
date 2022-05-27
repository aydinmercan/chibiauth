# ChibiAuth

Naive OpenID Provider.

## Documentation

Relevant info for development is inside the `doc` folder.

For administration and usage, a reference handbook will be provided in the future.

## Usage

```sh
chibiauth run --database=/path/to/database/of/chibiauth.db --port 9909 --base-url 'https://example.com'
```

* ChibiAuth will bind to localhost, bring your own TLS terminator.
* The database is just a SQLite file, feel free to use something like litestream.

## License

This repository is licensed under the EUPL-1.2. An English copy of the license text is available under `LICENSE`.
