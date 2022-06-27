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
- RUN: ```cargo run```


#### Adding new schema using diesel
+ Run command `diesel migration generate create_posts`
+ This create two files up.sql and down.sql in `/migrations/create_posts/`
+ Add new changes to `up.sql` .
+ `down.sql` contains reverting changes, in case we want to revert a migration.
+ Run `diesel migration run` to apply `up.sql` the migration.
+ Run `diesel migration redo` to apply `down.sql` the migration.
+ 
