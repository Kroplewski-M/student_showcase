-- Add down migration script here

-- 1. Drop indexes first (if they exist)
DROP INDEX IF EXISTS software_tools_name_ci_idx;
DROP INDEX IF EXISTS link_types_name_ci_idx;
DROP INDEX IF EXISTS projects_one_featured_per_user_idx;
DROP INDEX IF EXISTS project_tools_tool_id;
DROP INDEX IF EXISTS project_files_file_id;
DROP INDEX IF EXISTS users_course_id;
DROP INDEX IF EXISTS users_image_id;

-- 2. Drop tables in dependency order
DROP TABLE IF EXISTS project_links;
DROP TABLE IF EXISTS project_files;
DROP TABLE IF EXISTS project_tools;
DROP TABLE IF EXISTS projects;
DROP TABLE IF EXISTS user_certificates;
DROP TABLE IF EXISTS user_tools;
DROP TABLE IF EXISTS user_links;
DROP TABLE IF EXISTS files;
DROP TABLE IF EXISTS link_types;
DROP TABLE IF EXISTS software_tools;
DROP TABLE IF EXISTS courses;
DROP TABLE IF EXISTS user_password_resets;
DROP TABLE IF EXISTS user_verifications;

-- 3. Drop columns added to users last
ALTER TABLE users
    DROP COLUMN IF EXISTS course_id,
    DROP COLUMN IF EXISTS image_id;
