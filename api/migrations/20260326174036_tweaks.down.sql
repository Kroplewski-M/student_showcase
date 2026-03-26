-- Add down migration script here

DELETE FROM software_tools WHERE name IN (
'Prototyping',
'Simulation'
)
