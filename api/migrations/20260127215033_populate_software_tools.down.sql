-- Add down migration script here
DELETE FROM software_tools
WHERE name IN (
    -- Programming languages & runtimes
    'C', 'C++', 'C#', 'Java', 'Python', 'Rust', 'Go',
    'JavaScript', 'TypeScript', 'MATLAB', 'R', 'Julia', 'PHP', 'Kotlin', 'Swift',
    -- Web development
    'HTML', 'CSS', 'React', 'Next.js', 'Vue.js', 'Angular',
    'Node.js', 'Express.js', 'ASP.NET Core', 'Blazor',
    'Django', 'Flask', 'Spring Boot', 'Tailwind CSS',
    -- Databases & data tools
    'PostgreSQL', 'MySQL', 'SQLite', 'MongoDB', 'Redis',
    'Microsoft SQL Server', 'Oracle Database',
    'Apache Kafka', 'Apache Spark', 'Elasticsearch',
    -- DevOps & infrastructure
    'Git', 'GitHub', 'GitLab', 'Docker', 'Docker Compose',
    'Kubernetes', 'Nginx', 'Apache HTTP Server',
    'Terraform', 'Ansible', 'CI/CD Pipelines',
    -- Cloud platforms
    'Microsoft Azure', 'AWS', 'Google Cloud Platform',
    'Azure DevOps', 'Firebase',
    -- AI, ML & Data Science
    'TensorFlow', 'PyTorch', 'scikit-learn', 'Keras',
    'Pandas', 'NumPy', 'Jupyter Notebook',
    'Hugging Face Transformers', 'OpenCV', 'MATLAB Simulink',
    -- Cyber security & forensics
    'Wireshark', 'Metasploit', 'Burp Suite', 'Nmap',
    'Kali Linux', 'OpenSSL', 'Hashcat', 'Autopsy',
    -- Game development
    'Unity', 'Unreal Engine', 'Godot', 'Blender', 'Autodesk Maya',
    -- Engineering & electronics
    'AutoCAD', 'SolidWorks', 'ANSYS', 'Altium Designer',
    'Proteus', 'LabVIEW', 'LTspice', 'Multisim',
    -- Research & academic tools
    'LaTeX', 'Overleaf', 'Zotero', 'Mendeley', 'SPSS', 'Stata',
    -- Audio, music & media
    'Pro Tools', 'Logic Pro', 'Ableton Live', 'FL Studio', 'Reaper',
    -- General productivity
    'Microsoft Excel', 'Microsoft Word', 'Microsoft PowerPoint',
    'Visual Studio', 'Visual Studio Code',
    'IntelliJ IDEA', 'PyCharm', 'Eclipse', 'NetBeans'
);
