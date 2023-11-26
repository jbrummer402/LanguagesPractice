from django.contrib import admin
from .models import ToDo


class ToDoAdmin(admin.ModelAdmin):
    list_display = ('title', 'description', 'completed')


# Register your models here.
admin.site.register(ToDo, ToDoAdmin)
