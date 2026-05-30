# poly-agntcy-dir-flask

Flask adapter for the AGNTCY Directory Service.

## Note: depends on upstream agntcy-dir which is pre-1.0; track upstream releases.

## Install

```sh
uv add poly-agntcy-dir-flask
```

## Usage

```python
from flask import Flask
from poly_agntcy_dir_flask import create_directory_blueprint

app = Flask(__name__)
app.register_blueprint(create_directory_blueprint())
```
