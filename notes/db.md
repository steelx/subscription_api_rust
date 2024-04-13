## Create/Launch PostgresSQL Database

```bash
# check if script is executable
chmod +x scripts/init_db.sh
```

### Run Docker
Read Docker installation [here](https://www.digitalocean.com/community/tutorials/how-to-install-and-use-docker-on-ubuntu-20-04)
```bash
# check status
sudo systemctl status docker

# Run Docker
systemctl --user start docker-desktop
```

### Database migration
We can use SQLX cli
```bash
# install sqlx CLI
cargo install --version="~0.7" sqlx-cli --no-default-features --features rustls,postgres

# add on CI, if needed
SKIP_DOCKER=true ./scripts/init_db.sh
```
> Postgres Docker instance already comes with a default database named `newsletter` using `scripts/init_db.sh`,
hence no need to run : `sqlx database create`


