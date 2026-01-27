-- Add up migration script here
CREATE TABLE user_verifications
(
    token UUID PRIMARY KEY,
    user_id VARCHAR(7) REFERENCES users(id) NOT NULL,
    expired_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE user_password_resets
(
    token UUID PRIMARY KEY,
    user_id VARCHAR(7) REFERENCES users(id) NOT NULL,
    expired_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE courses
(
    id UUID PRIMARY KEY,
    name VARCHAR(250) NOT NULL,
    moa VARCHAR(10) NULL,
    mcr_code VARCHAR(20) NULL,
    crs VARCHAR(15) NULL,
    route VARCHAR(10) NULL,
    award VARCHAR(15) NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE software_tools
(
    id UUID PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE UNIQUE INDEX software_tools_name_ci_idx
ON software_tools (lower(name));

CREATE TABLE link_types
(
    id UUID PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE UNIQUE INDEX link_types_name_ci_idx
ON link_types (lower(name));

CREATE TABLE files
(
    id UUID PRIMARY KEY,
    old_file_name TEXT NOT NULL,
    new_file_name TEXT NOT NULL,
    file_type VARCHAR(50) NOT NULL,
    size_bytes BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (size_bytes > 0)
);

CREATE TABLE user_links
(
    id UUID PRIMARY KEY,
    user_id VARCHAR(7) REFERENCES users(id) NOT NULL,
    link_type_id UUID REFERENCES link_types(id) NOT NULL,
    url TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (user_id, url)
);

CREATE TABLE user_tools
(
    user_id VARCHAR(7) REFERENCES users(id) NOT NULL,
    software_tool_id UUID REFERENCES software_tools(id) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (user_id, software_tool_id)
);

CREATE TABLE user_certificates
(
    user_id VARCHAR(7) REFERENCES users(id) NOT NULL,
    certificate TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY(user_id, certificate)
);

CREATE TABLE projects
(
    id UUID PRIMARY KEY,
    user_id VARCHAR(7) REFERENCES users(id) NOT NULL,
    name VARCHAR(250) NOT NULL,
    description TEXT NOT NULL,
    embedding vector(384),
    live_link TEXT NULL,
    featured BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX ON projects(user_id);
CREATE UNIQUE INDEX projects_one_featured_per_user_idx
ON projects (user_id)
WHERE featured = TRUE;

CREATE TABLE project_tools
(
    project_id UUID REFERENCES projects(id) NOT NULL,
    tool_id UUID REFERENCES software_tools(id) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (project_id, tool_id)
);

CREATE TABLE project_files
(
    project_id UUID REFERENCES projects(id) NOT NULL,
    file_id UUID REFERENCES files(id) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY(project_id, file_id)
);

CREATE TABLE project_links
(
    id UUID PRIMARY KEY,
    project_id UUID REFERENCES projects(id) NOT NULL,
    link_type_id UUID REFERENCES link_types(id) NOT NULL,
    url TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (project_id, url)
);

ALTER TABLE users
ADD course_id UUID REFERENCES courses(id) NULL,
ADD image_id UUID REFERENCES files(id) NULL;

CREATE INDEX ON users(course_id);
CREATE INDEX ON users(image_id);
