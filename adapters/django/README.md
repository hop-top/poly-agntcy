# poly-agntcy-dir-django

Django adapter for the AGNTCY Directory Service.

## Note: depends on upstream agntcy-dir which is pre-1.0; track upstream releases.

## Install

```sh
uv add poly-agntcy-dir-django
```

## Usage

Add to `INSTALLED_APPS`:

```python
INSTALLED_APPS = [
    # ...
    "poly_agntcy_dir_django",
]
```

Include the URLs:

```python
from django.urls import include, path

urlpatterns = [
    path("agntcy/", include("poly_agntcy_dir_django.urls")),
]
```
