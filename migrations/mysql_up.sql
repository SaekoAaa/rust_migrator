

CREATE TABLE IF NOT EXISTS users (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    email VARCHAR(254) UNIQUE NOT NULL,
    password_hash VARCHAR(254) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    role VARCHAR(254) NOT NULL CHECK (role IN ('user', 'admin')) default 'user'
);

CREATE TABLE IF NOT EXISTS sessions (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    user_id BIGINT UNSIGNED NOT NULL,
    refresh_token_hash VARCHAR(255) NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    revoked BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    device_info VARCHAR(254),
    ip_address VARCHAR(45),

    CONSTRAINT fk_sessions_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    INDEX refresh_token_hash_idx (refresh_token_hash)
);


CREATE TABLE IF NOT EXISTS projects (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    owner_id BIGINT UNSIGNED NOT NULL,
    name VARCHAR(254) NOT NULL,
    valid_name VARCHAR(254) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    description VARCHAR(1024) NOT NULL,
    CONSTRAINT fk_projects_owner FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE RESTRICT,
    -- Unique name per owner
    UNIQUE KEY ux_projects_valid_name (owner_id, valid_name)
);


CREATE TABLE IF NOT EXISTS project_members (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    user_id BIGINT UNSIGNED NOT NULL,
    project_id BIGINT UNSIGNED NOT NULL,
    joined_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_project_members_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_project_members_project FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    -- Unique member per project
    UNIQUE KEY ux_project_members_project_user (project_id, user_id)
);

CREATE TABLE IF NOT EXISTS character_types (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    type VARCHAR(254) NOT NULL
);
CREATE TABLE IF NOT EXISTS event_types (
id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
type VARCHAR(254) NOT NULL
);
CREATE TABLE IF NOT EXISTS characters (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    project_id BIGINT UNSIGNED NOT NULL,
    hero_name VARCHAR(254) NOT NULL,
    age INT,
    gender VARCHAR(254),
    description TEXT,
    entity_type_id BIGINT UNSIGNED NOT NULL,
    source_media VARCHAR(254), -- Game, movie, book, etc.
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    CONSTRAINT fk_character_type FOREIGN KEY (entity_type_id) REFERENCES character_types(id) ON DELETE CASCADE,
    CONSTRAINT fk_character_project FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    -- Unique hero name per project
    UNIQUE KEY ux_characters_project_hero (project_id, hero_name)
);

CREATE TABLE IF NOT EXISTS events (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    project_id BIGINT UNSIGNED NOT NULL,
    event_name VARCHAR(254) NOT NULL,
    description TEXT,
    event_type_id BIGINT UNSIGNED NOT NULL,
    location_details VARCHAR(1024), -- Specific details about the location/place
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    CONSTRAINT fk_event_project FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    CONSTRAINT fk_event_type FOREIGN KEY (event_type_id) REFERENCES event_types(id) ON DELETE CASCADE,
    -- Unique event name per project
    UNIQUE KEY ux_events_project_name (project_id, event_name)
);
CREATE TABLE IF NOT EXISTS relation_types (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(254) NOT NULL
);
CREATE TABLE IF NOT EXISTS relations (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    relation_type_id BIGINT UNSIGNED NOT NULL,
    character_id BIGINT UNSIGNED NOT NULL,
    event_id BIGINT UNSIGNED NOT NULL,
    CONSTRAINT fk_relation_type FOREIGN KEY (relation_type_id) REFERENCES relation_types(id) ON DELETE CASCADE,
    CONSTRAINT fk_relation_character FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE,
    CONSTRAINT fk_relation_event FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE CASCADE,
    UNIQUE KEY ux_relations_type_character_event (relation_type_id, character_id, event_id)
);
