# py / django-quickstart

Minimal Django skeleton mounting `poly-agntcy-dir-django`. The app
registers `poly_agntcy_dir_django` in `INSTALLED_APPS` and includes
its URLconf at the project root.

## Run

```sh
uv sync
uv run python manage.py runserver
curl -X POST http://localhost:8000/agntcy/
```
