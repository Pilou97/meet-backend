# Meet 

# Requirement:
 - shuttle
 - cargo
 - rust

# How to start

create a file named `Secrets.dev.toml` and put:

`
SWAGGER_URI = "localhost:8000"
DATABASE_URI = "postgresql://postgres:azerty@localhost/postgres"
`

Then start the project by running:

`shuttle run --secrets Secrets.dev.toml`

> shuttle run --secrets Secrets.dev.toml --port 8081
> Don't forget to change the port in your Secrets.toml
