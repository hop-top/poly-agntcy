from django.test import Client


def test_agntcy_handler_post() -> None:
    client = Client()
    res = client.post("/agntcy/")
    assert res.status_code == 200
    assert res.json()["ok"] is True


def test_agntcy_handler_rejects_get() -> None:
    client = Client()
    res = client.get("/agntcy/")
    assert res.status_code == 405
