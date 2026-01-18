-- Project data layer tests

INSERT INTO users (id, email, password_hash, role)
VALUES
  (1, 'owner@mail.com', '$argon2id$v=19$m=65536,t=3,p=1$abcd...hash1', 'user'),
  (2, 'member@mail.com', '$argon2id$v=19$m=65536,t=3,p=1$abcd...hash2', 'user'),
  (3, 'other@mail.com', '$argon2id$v=19$m=65536,t=3,p=1$abcd...hash3', 'user');

INSERT INTO projects (id, owner_id, name, valid_name, description, created_at)
VALUES
  (1, 1, 'Test Project', 'test-project', 'initial fixture project', NOW());

INSERT INTO sessions (id, user_id, refresh_token_hash, revoked, expires_at, created_at)
VALUES
  (1, 1, 'DEADBEEFCAFEBABE1234', FALSE, DATE_ADD(NOW(), INTERVAL 2 HOUR), NOW());

CREATE INDEX IF NOT EXISTS idx_refresh_token_hash ON sessions (refresh_token_hash);
