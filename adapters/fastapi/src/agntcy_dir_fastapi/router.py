from typing import Any

from fastapi import APIRouter


def create_directory_router(*, endpoint: str) -> APIRouter:
    if not endpoint:
        raise ValueError("endpoint required")
    router = APIRouter()

    @router.post("/agntcy")
    async def _agntcy_handler() -> dict[str, Any]:
        return {"ok": True, "endpoint": endpoint}

    return router
