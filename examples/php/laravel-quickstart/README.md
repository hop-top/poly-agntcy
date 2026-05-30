# php / laravel-quickstart

Shows how `poly-agntcy/dir-laravel` wires the DIR `Client` into a
Laravel app via its `AgntcyServiceProvider`. Includes:

- `config/agntcy.php` — the published config stub
- `routes/web.php` — controller resolves the `Client` from the
  container and calls `discover()`
- `app/Console/Commands/RegisterInventoryAgent.php` — Artisan
  command that calls `register()`

## Run

```sh
composer install
php artisan agntcy:register --capability inventory
```
