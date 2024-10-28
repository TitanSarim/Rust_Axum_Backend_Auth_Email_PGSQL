# Backend development with RUST

## NEED OF AN SQLX CLI
```
cargo install sqlx-cli --no-default-features --features native-tls,postgres
```

## CREATE A DATABASE
```
sqlx database create
```

## GENERATE MIGRATIONS
```
sqlx migrate add -r user
```
## RUN MIGRATIONS
```
sqlx migrate run
```


