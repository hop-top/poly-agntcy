import os

SECRET_KEY = os.environ.get("DJANGO_SECRET_KEY", "demo-secret-not-for-prod")
DEBUG = True
ALLOWED_HOSTS = ["*"]

INSTALLED_APPS = [
    "django.contrib.contenttypes",
    "django.contrib.auth",
    "poly_agntcy_dir_django",
]

ROOT_URLCONF = "agntcy_demo.urls"
DATABASES = {
    "default": {"ENGINE": "django.db.backends.sqlite3", "NAME": ":memory:"},
}
USE_TZ = True
DEFAULT_AUTO_FIELD = "django.db.models.BigAutoField"
