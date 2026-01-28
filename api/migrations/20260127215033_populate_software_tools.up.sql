-- Add up migration script here
INSERT INTO software_tools (id, name)
VALUES
-- Programming languages & runtimes
(uuid_generate_v4(), 'C'),
(uuid_generate_v4(), 'C++'),
(uuid_generate_v4(), 'C#'),
(uuid_generate_v4(), 'Java'),
(uuid_generate_v4(), 'Python'),
(uuid_generate_v4(), 'Rust'),
(uuid_generate_v4(), 'Go'),
(uuid_generate_v4(), 'JavaScript'),
(uuid_generate_v4(), 'TypeScript'),
(uuid_generate_v4(), 'MATLAB'),
(uuid_generate_v4(), 'R'),
(uuid_generate_v4(), 'Julia'),
(uuid_generate_v4(), 'PHP'),
(uuid_generate_v4(), 'Kotlin'),
(uuid_generate_v4(), 'Swift'),

-- Web development
(uuid_generate_v4(), 'HTML'),
(uuid_generate_v4(), 'CSS'),
(uuid_generate_v4(), 'React'),
(uuid_generate_v4(), 'Next.js'),
(uuid_generate_v4(), 'Vue.js'),
(uuid_generate_v4(), 'Angular'),
(uuid_generate_v4(), 'Node.js'),
(uuid_generate_v4(), 'Express.js'),
(uuid_generate_v4(), 'ASP.NET Core'),
(uuid_generate_v4(), 'Blazor'),
(uuid_generate_v4(), 'Django'),
(uuid_generate_v4(), 'Flask'),
(uuid_generate_v4(), 'Spring Boot'),
(uuid_generate_v4(), 'Tailwind CSS'),

-- Databases & data tools
(uuid_generate_v4(), 'PostgreSQL'),
(uuid_generate_v4(), 'MySQL'),
(uuid_generate_v4(), 'SQLite'),
(uuid_generate_v4(), 'MongoDB'),
(uuid_generate_v4(), 'Redis'),
(uuid_generate_v4(), 'Microsoft SQL Server'),
(uuid_generate_v4(), 'Oracle Database'),
(uuid_generate_v4(), 'Apache Kafka'),
(uuid_generate_v4(), 'Apache Spark'),
(uuid_generate_v4(), 'Elasticsearch'),

-- DevOps & infrastructure
(uuid_generate_v4(), 'Git'),
(uuid_generate_v4(), 'GitHub'),
(uuid_generate_v4(), 'GitLab'),
(uuid_generate_v4(), 'Docker'),
(uuid_generate_v4(), 'Docker Compose'),
(uuid_generate_v4(), 'Kubernetes'),
(uuid_generate_v4(), 'Nginx'),
(uuid_generate_v4(), 'Apache HTTP Server'),
(uuid_generate_v4(), 'Terraform'),
(uuid_generate_v4(), 'Ansible'),
(uuid_generate_v4(), 'CI/CD Pipelines'),

-- Cloud platforms
(uuid_generate_v4(), 'Microsoft Azure'),
(uuid_generate_v4(), 'AWS'),
(uuid_generate_v4(), 'Google Cloud Platform'),
(uuid_generate_v4(), 'Azure DevOps'),
(uuid_generate_v4(), 'Firebase'),

-- AI, ML & Data Science
(uuid_generate_v4(), 'TensorFlow'),
(uuid_generate_v4(), 'PyTorch'),
(uuid_generate_v4(), 'scikit-learn'),
(uuid_generate_v4(), 'Keras'),
(uuid_generate_v4(), 'Pandas'),
(uuid_generate_v4(), 'NumPy'),
(uuid_generate_v4(), 'Jupyter Notebook'),
(uuid_generate_v4(), 'Hugging Face Transformers'),
(uuid_generate_v4(), 'OpenCV'),
(uuid_generate_v4(), 'MATLAB Simulink'),

-- Cyber security & forensics
(uuid_generate_v4(), 'Wireshark'),
(uuid_generate_v4(), 'Metasploit'),
(uuid_generate_v4(), 'Burp Suite'),
(uuid_generate_v4(), 'Nmap'),
(uuid_generate_v4(), 'Kali Linux'),
(uuid_generate_v4(), 'OpenSSL'),
(uuid_generate_v4(), 'Hashcat'),
(uuid_generate_v4(), 'Autopsy'),

-- Game development
(uuid_generate_v4(), 'Unity'),
(uuid_generate_v4(), 'Unreal Engine'),
(uuid_generate_v4(), 'Godot'),
(uuid_generate_v4(), 'Blender'),
(uuid_generate_v4(), 'Autodesk Maya'),

-- Engineering & electronics
(uuid_generate_v4(), 'AutoCAD'),
(uuid_generate_v4(), 'SolidWorks'),
(uuid_generate_v4(), 'ANSYS'),
(uuid_generate_v4(), 'Altium Designer'),
(uuid_generate_v4(), 'Proteus'),
(uuid_generate_v4(), 'LabVIEW'),
(uuid_generate_v4(), 'LTspice'),
(uuid_generate_v4(), 'Multisim'),

-- Research & academic tools
(uuid_generate_v4(), 'LaTeX'),
(uuid_generate_v4(), 'Overleaf'),
(uuid_generate_v4(), 'Zotero'),
(uuid_generate_v4(), 'Mendeley'),
(uuid_generate_v4(), 'SPSS'),
(uuid_generate_v4(), 'Stata'),

-- Audio, music & media
(uuid_generate_v4(), 'Pro Tools'),
(uuid_generate_v4(), 'Logic Pro'),
(uuid_generate_v4(), 'Ableton Live'),
(uuid_generate_v4(), 'FL Studio'),
(uuid_generate_v4(), 'Reaper'),

-- General productivity
(uuid_generate_v4(), 'Microsoft Excel'),
(uuid_generate_v4(), 'Microsoft Word'),
(uuid_generate_v4(), 'Microsoft PowerPoint'),
(uuid_generate_v4(), 'Visual Studio'),
(uuid_generate_v4(), 'Visual Studio Code'),
(uuid_generate_v4(), 'IntelliJ IDEA'),
(uuid_generate_v4(), 'PyCharm'),
(uuid_generate_v4(), 'Eclipse'),
(uuid_generate_v4(), 'NetBeans');
