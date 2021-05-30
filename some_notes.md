#### Install `postgresql` package before installing diesel_cli or building the package.
+ `sudo pacman -S postgresql`
#### Install Rust in arch
```bash
sudo pacman -S rustp
rustp install stable
```
#### Install diesel_cli for postgres only
+ `cargo install diesel_cli --no-default-features --features postgres`

#### Add Database information to .env file
```bash
echo DATABASE_URL=postgres://raushan:password@localhost:5432/gscone > .env
diesel setup
```

#### Build project & run

