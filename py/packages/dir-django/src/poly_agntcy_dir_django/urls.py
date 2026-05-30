from django.urls import path

from .views import agntcy_handler

urlpatterns = [path("agntcy/", agntcy_handler, name="agntcy")]
