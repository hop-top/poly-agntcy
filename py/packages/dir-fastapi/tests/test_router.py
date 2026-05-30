import pytest
from fastapi import FastAPI
from fastapi.testclient import TestClient
from poly_agntcy_dir_fastapi import create_directory_router


def test_router_mounts() -> None:
    app = FastAPI()
    app.include_router(create_directory_router(endpoint="https://dir.example"))
    client = TestClient(app)
    res = client.post("/agntcy")
    assert res.status_code == 200
    assert res.json()["ok"] is True


def test_endpoint_required() -> None:
    with pytest.raises(ValueError):
        create_directory_router(endpoint="")
