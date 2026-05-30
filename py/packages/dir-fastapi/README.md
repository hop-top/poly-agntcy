# poly-agntcy-dir-fastapi

FastAPI adapter for the AGNTCY Directory Service.

## Note: depends on upstream agntcy-dir which is pre-1.0; track upstream releases.

## Install

```sh
uv add poly-agntcy-dir-fastapi
```

## Usage

```python
from fastapi import FastAPI
from poly_agntcy_dir_fastapi import create_directory_router

app = FastAPI()
app.include_router(create_directory_router())
```

CLI:

```sh
agntcy --help
```
