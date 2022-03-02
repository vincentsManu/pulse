# Manupulse

## Pre-requisite

You'll need to install:

- [Rust](https://www.rust-lang.org/tools/install)
- a postgres instance
- docker
- azure cloud cli

## Configuration

you need one config per environment. Default environment is local.
You can create dev, local and prodution yaml files and override whichever values from base in these.

## Env

You need a .env in order to let sqlx connect to the local database, to check queries at compile time.
This should be just a connection string.

## Migration

Local is a local postgres (k3d with open ports 5432), used by local app.
Dev is a dev postgres (k3d), used by app running inside the cluster.
Prod is a prod postgres (AKS), used by app running inside the cluster.

for local and dev

```bash
    DB_PASSWORD=<PASS> DB_SUFFIX="_local" make db-init 
    DB_PASSWORD=<PASS> DB_SUFFIX="_dev" make db-init 
    make migrate
```

for prod, after you port-fwded the postgres

```bash
    kubectl port-forward svc/mypostgres 5432:5432 -n postgres
```

```bash
    DB_PASSWORD=<PASS> DB_SUFFIX="" make db-init 
```

## How to test

Using `cargo`:

```bash
make test
```

## How to build

### On linux

No special thing to do on linux.

Using `cargo` and docker:

```bash
make build
```

### On Mac

Building via docker is extremely slow due to IO limitations.

You can find a solution [here](https://chr4.org/posts/2017-03-15-cross-compile-and-link-a-static-binary-on-macos-for-linux-with-cargo-and-rust/).

You will have to install a musl build toolchain:

```bash
brew install filosottile/musl-cross/musl-cross
ln -s /usr/local/bin/x86_64-linux-musl-gcc /usr/local/bin/musl-gcc
```

and add a .cargo/config

```config
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
```

Using `cargo` and docker:

```bash
make build-mac
```

## How to benchmark

You ll need [k6](https://k6.io/).
Then you can run, depending on which frontend you run it (user pass are the basic auth).

```bash
USER_MANUPULSE=user PASSWORD_MANUPULSE=pass ENV_PULSE=dev ./benchmarks/run.sh
```

You can change the URL in order to have real numbers (a non compile rust is not optimised).

## Deploying

See ./deploy/README.md

## TODO

- [ ] tracing: continue traces on existing spans coming from req headers
- [ ] tracing: configuration for sampling
- [ ] create a channel to receive supabase/realtime updates
- [ ] missing errors in logs or traces
- [ ] split modules in packages, between db related things and http related things
- [ ] add criterion and tarpaulin in the config
