-- Add down migration script here
DELETE FROM link_types
WHERE name IN (
    'GitHub',
    'YouTube',
    'LinkedIn',
    'GitLab',
    'Bitbucket',
    'Stack Overflow',
    'Figma',
    'Live Preview',
    'Other'
);
