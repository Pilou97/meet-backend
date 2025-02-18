# Meet 

# Requirement:
 - shuttle
 - cargo
 - rust

# How to start

create a file named `Secrets.dev.toml` and put:

```bash
SWAGGER_URI = "localhost:8000"
```

Then you need to enable sqlx macros:
 - by adding a `DATABASE_URL` in your environment
 - by running `cargo sqlx migrate run`

You can now compile the project by running `cargo build`

And you can run it using:

`shuttle run --secrets Secrets.dev.toml`

> shuttle run --secrets Secrets.dev.toml --port 8081
