INSERT INTO users (email, password_hash, role)
VALUES
('alice@example.com', '$argon2id$v=19$m=65536,t=3,p=1$abcd...hash1', 'user'),
('bob@example.com', '$argon2id$v=19$m=65536,t=3,p=1$abcd...hash2', 'admin'),
('charlie@example.com', '$argon2id$v=19$m=65536,t=3,p=1$abcd...hash3', 'user');
