
INSERT INTO users (email, password_hash, role) VALUES
-- password: "password"
('admin@example.com', '$argon2id$v=19$m=19456,t=2,p=1$MTIzNDVzYWx0$Nij6JVFBvTmxtgSvauhZz/KzgL9ROJXP1GhgNEr2upg', 'admin'),
-- password: "password1"
('alice@example.com', '$argon2id$v=19$m=19456,t=2,p=1$MTIzNDVzYWx0$XEbNFYHSWFxlnG0AvaPw/TmBGtt82lxy6JJEw7e608A', 'user'),
-- password: "password2"
('scarlet@example.com', '$argon2id$v=19$m=19456,t=2,p=1$MTIzNDVzYWx0$K0lkuSSJ3o1pdNIfZOIdM4WgCvw0QOlzU49aWXHkr58', 'user'),
-- password: "password2" (повторное использование того же хеша)
('charlie@example.com', '$argon2id$v=19$m=19456,t=2,p=1$MTIzNDVzYWx0$K0lkuSSJ3o1pdNIfZOIdM4WgCvw0QOlzU49aWXHkr58', 'user'),
-- password: "password1" (повторное использование того же хеша)
('diana@example.com', '$argon2id$v=19$m=19456,t=2,p=1$MTIzNDVzYWx0$XEbNFYHSWFxlnG0AvaPw/TmBGtt82lxy6JJEw7e608A', 'user');
-- tx;
INSERT INTO sessions (user_id, refresh_token_hash, expires_at, device_info, ip_address) VALUES
(1, 'hash_admin_token_abc123', '2025-12-09 23:02:00', 'Mozilla/5.0 (Windows NT 10.0)', '192.168.1.100'),
(2, 'hash_alice_token_def456', '2025-12-09 23:02:00', 'Mozilla/5.0 (Macintosh)', '192.168.1.101'),
(3, 'hash_bob_token_ghi789', '2025-12-09 23:02:00', 'Mozilla/5.0 (X11, Linux)', '192.168.1.102'),
(4, 'hash_charlie_token_jkl012', '2025-12-09 23:02:00', 'Mozilla/5.0 (iPhone)', '192.168.1.103'),
(5, 'hash_diana_token_mno345', '2025-12-09 23:02:00', 'Mozilla/5.0 (Android)', '192.168.1.104');

INSERT INTO projects (owner_id, name, valid_name, description) VALUES
(1, 'Admin Dashboard', 'admin-dashboard', 'Панель администратора для управления системой'),
(2, 'E-commerce Platform', 'ecommerce-platform', 'Платформа для онлайн магазина с корзиной и платежами'),
(3, 'Task Manager', 'task-manager', 'Приложение для управления задачами и проектами команды'),
(4, 'Blog Engine', 'blog-engine', 'Движок для создания и публикации статей'),
(5, 'Weather API', 'weather-api', 'REST API для получения данных о погоде');

-- tx;
INSERT INTO project_members (user_id, project_id) VALUES
(1, 1),  -- Admin участвует в Admin Dashboard
(3, 1),  -- Bob участвует в Admin Dashboard
(2, 2),
(3, 3),
(4, 4),
(5, 5);
