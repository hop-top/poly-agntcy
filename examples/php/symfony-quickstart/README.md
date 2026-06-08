# php / symfony-quickstart

Shows how `hop-top/agntcy-dir-symfony` registers its `AgntcyBundle`
into a Symfony app and wires the `Client` via DI. Includes:

- `config/bundles.php` — bundle registration
- `config/services.yaml` — service wiring
- `src/Command/RegisterInventoryCommand.php` — console command
  using the autowired `Client`

## Run

```sh
composer install
php bin/console agntcy:register --capability inventory
```
