-- Add down migration script here
DELETE FROM software_tools WHERE name IN (
    -- Adobe Creative Cloud
    'Adobe Photoshop',
    'Adobe Illustrator',
    'Adobe XD',
    'Adobe After Effects',
    'Adobe Premiere Pro',
    'Adobe InDesign',
    'Adobe Lightroom',
    'Adobe Animate',
    'Adobe Audition',
    'Adobe Substance Painter',
    'Adobe Dreamweaver',

    -- UI / UX & wireframing
    'Figma',
    'Sketch',
    'Balsamiq',
    'Axure RP',
    'InVision',
    'Framer',
    'ProtoPie',
    'Marvel App',
    'Zeplin',
    'Webflow',

    -- Diagramming & whiteboarding
    'Miro',
    'Whimsical',
    'Lucidchart',
    'Draw.io',
    'Canva',

    -- Project management & collaboration
    'Jira',
    'Trello',
    'Notion',
    'Confluence',
    'Asana',
    'Linear',
    'ClickUp',
    'Monday.com',

    -- Additional 3D & CAD
    'Cinema 4D',
    'Fusion 360',
    'SketchUp',
    'ZBrush',
    'Houdini',
    'Rhino 3D',

    -- Mobile & platform IDEs
    'Xcode',
    'Android Studio',

    -- API & testing tools
    'Postman',
    'Insomnia',
    'Swagger',
    'Jest',
    'Cypress',
    'Playwright',
    'Selenium',
    'JUnit',

    -- Additional version control & CI
    'Bitbucket',
    'Storybook'
);
