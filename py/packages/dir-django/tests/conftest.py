import django
from django.conf import settings


def pytest_configure() -> None:
    if settings.configured:
        return
    settings.configure(
        DEBUG=True,
        DATABASES={},
        INSTALLED_APPS=["poly_agntcy_dir_django"],
        ROOT_URLCONF="poly_agntcy_dir_django.urls",
        SECRET_KEY="test",
        ALLOWED_HOSTS=["*"],
    )
    django.setup()
