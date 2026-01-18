

DELETE FROM project_members;

-- Удаление проектов
DELETE FROM projects;

-- Удаление сессий
DELETE FROM sessions;

-- Удаление пользователей
DELETE FROM users;

-- Сброс AUTO_INCREMENT
ALTER TABLE project_members AUTO_INCREMENT = 1;
ALTER TABLE projects AUTO_INCREMENT = 1;
ALTER TABLE sessions AUTO_INCREMENT = 1;
ALTER TABLE users AUTO_INCREMENT = 1;
