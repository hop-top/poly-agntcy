from django.http import HttpRequest, JsonResponse
from django.views.decorators.http import require_POST


@require_POST
def agntcy_handler(request: HttpRequest) -> JsonResponse:
    return JsonResponse({"ok": True})
