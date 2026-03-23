-- Add up migration script here
INSERT INTO software_tools (id, name)
VALUES
-- Adobe Creative Cloud
(uuid_generate_v4(), 'Adobe Photoshop'),
(uuid_generate_v4(), 'Adobe Illustrator'),
(uuid_generate_v4(), 'Adobe XD'),
(uuid_generate_v4(), 'Adobe After Effects'),
(uuid_generate_v4(), 'Adobe Premiere Pro'),
(uuid_generate_v4(), 'Adobe InDesign'),
(uuid_generate_v4(), 'Adobe Lightroom'),
(uuid_generate_v4(), 'Adobe Animate'),
(uuid_generate_v4(), 'Adobe Audition'),
(uuid_generate_v4(), 'Adobe Substance Painter'),
(uuid_generate_v4(), 'Adobe Dreamweaver'),

-- UI / UX & wireframing
(uuid_generate_v4(), 'Figma'),
(uuid_generate_v4(), 'Sketch'),
(uuid_generate_v4(), 'Balsamiq'),
(uuid_generate_v4(), 'Axure RP'),
(uuid_generate_v4(), 'InVision'),
(uuid_generate_v4(), 'Framer'),
(uuid_generate_v4(), 'ProtoPie'),
(uuid_generate_v4(), 'Marvel App'),
(uuid_generate_v4(), 'Zeplin'),
(uuid_generate_v4(), 'Webflow'),

-- Diagramming & whiteboarding
(uuid_generate_v4(), 'Miro'),
(uuid_generate_v4(), 'Whimsical'),
(uuid_generate_v4(), 'Lucidchart'),
(uuid_generate_v4(), 'Draw.io'),
(uuid_generate_v4(), 'Canva'),

-- Project management & collaboration
(uuid_generate_v4(), 'Jira'),
(uuid_generate_v4(), 'Trello'),
(uuid_generate_v4(), 'Notion'),
(uuid_generate_v4(), 'Confluence'),
(uuid_generate_v4(), 'Asana'),
(uuid_generate_v4(), 'Linear'),
(uuid_generate_v4(), 'ClickUp'),
(uuid_generate_v4(), 'Monday.com'),

-- Additional 3D & CAD
(uuid_generate_v4(), 'Cinema 4D'),
(uuid_generate_v4(), 'Fusion 360'),
(uuid_generate_v4(), 'SketchUp'),
(uuid_generate_v4(), 'ZBrush'),
(uuid_generate_v4(), 'Houdini'),
(uuid_generate_v4(), 'Rhino 3D'),

-- Mobile & platform IDEs
(uuid_generate_v4(), 'Xcode'),
(uuid_generate_v4(), 'Android Studio'),

-- API & testing tools
(uuid_generate_v4(), 'Postman'),
(uuid_generate_v4(), 'Insomnia'),
(uuid_generate_v4(), 'Swagger'),
(uuid_generate_v4(), 'Jest'),
(uuid_generate_v4(), 'Cypress'),
(uuid_generate_v4(), 'Playwright'),
(uuid_generate_v4(), 'Selenium'),
(uuid_generate_v4(), 'JUnit'),

-- Additional version control & CI
(uuid_generate_v4(), 'Bitbucket'),
(uuid_generate_v4(), 'Storybook');
