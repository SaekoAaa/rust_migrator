# Мигратор базы данных
## Использует mysql crate и чтение из файла для удобной миграции туда и обратно
## Переменная окружения IS_REVERT для обратной миграции базы данных
## task db - для сборки проекта.
Записывает сборку в DOCKER_VERSION.yaml файл
## compose.yaml исключительно для тестирования работоспособности
## Нужна дополнительная работа над конфигом

## Типы миграций (MIGRATION_TYPE env):
1 - ApplyMigration
2 - RevertMigration
3 - ApplyWithData
4 - ApplyAndClearData
