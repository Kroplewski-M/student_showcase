<div align="center">

<img src="https://huddersfieldshowcase.cloud/favicon.ico" width="64" height="64" alt="SCE Futures logo" />

# SCE Futures '26

### Student Showcase — University of Huddersfield

[![Live Site](https://img.shields.io/badge/Live%20Site-huddersfieldshowcase.cloud-a1e9f0?style=for-the-badge&labelColor=204346)](https://huddersfieldshowcase.cloud/)

</div>

---

A web platform for the University of Huddersfield's School of Computing & Engineering annual showcase event, connecting final-year students with industry professionals and recruiters.

## About

SCE Futures '26 is an event held on **19 June 2026** at the University of Huddersfield, where final-year Computing & Engineering students present their projects to industry partners and potential employers.

The platform serves as both a pre-event information hub and a discovery tool — allowing visitors to browse student profiles and find talent before the event takes place.

## What's on offer

- **Live project demos** — see working software and hardware built by final-year students
- **Poster presentations** — deep-dives into research and technical work
- **Networking opportunities** — meet students, academics, and fellow industry guests

The event runs from **12:00 PM to 5:00 PM** and covers the university's areas of expertise including Engineering, AI, Cyber Security, and Creative Computing.

## Features

- Browse and search student profiles using natural language
- Discover students across technologies, tools, and disciplines
- Student profile editing — name, course, bio, tools, social links, certificates
- Project management — create, edit, and feature projects with images and live links
- User account registration with email verification
- Password reset via email

## Who is it for?

- **Industry professionals & recruiters** looking to spot emerging talent and hire graduates
- **Commercial partners** interested in R&D collaboration with the university
- **Anyone** curious about what the next generation of Computing & Engineering graduates is building

---

## Technical

### Tech Stack

| Layer              | Technology                                                      |
| ------------------ | --------------------------------------------------------------- |
| **Frontend**       | Next.js 16, React 19, TypeScript, Tailwind CSS 4, Framer Motion |
| **Backend**        | Rust, Actix-web 4                                               |
| **Database**       | PostgreSQL 18 + pgvector                                        |
| **Auth**           | JWT (HS256), Argon2, HttpOnly cookies                           |
| **AI / Search**    | FastEmbed, AllMiniLML6V2 (384-dim vectors), pgvector HNSW index |
| **Email**          | Postmark (Tera templates)                                       |
| **Infrastructure** | Docker, Nginx, GitHub Actions                                   |

### Software Architecture

The project follows a **frontend/backend split monorepo** structure with four services orchestrated via Docker Compose. The architecture is documented using the **4+1 model**:

---

#### Logical View

The system's functional decomposition into major subsystems.

```mermaid
graph TD
    subgraph Auth["Authentication"]
        A1[JWT Token]
        A2[Argon2 Hashing]
        A3[Email Verification]
        A4[Password Reset]
    end
    subgraph User["User"]
        U1[Profile Viewing]
        U2[Profile Editing]
        U3[Avatar Upload]
    end
    subgraph Projects["Projects"]
        P1[Project CRUD]
        P2[Featured Project]
        P3[Project Images]
    end
    subgraph Discovery["Discovery"]
        D1[Student Browser]
        D2[Natural Language Search]
    end
    subgraph Content["Content"]
        C1[Student Profiles]
        C2[Reference Data\ncourses · tools · link types]
    end
    subgraph Embedding["Embedding"]
        E1[AllMiniLML6V2 Pool]
        E2[Background Reembedding Job]
    end
    subgraph Notifications["Notifications"]
        N1[Postmark API]
        N2[Tera Email Templates]
    end

    Auth --> User
    Auth --> Notifications
    User --> Content
    User --> Embedding
    Projects --> Content
    Projects --> Embedding
    Discovery --> Content
    D2 -->|vector similarity| C1
    Embedding -->|pgvector| C1
    E2 -->|retry on failure| Embedding
```

---

#### Process View

Runtime communication between services for key flows.

```mermaid
sequenceDiagram
    participant Browser
    participant Nginx
    participant Frontend
    participant API
    participant DB

    Browser->>Nginx: HTTPS request

    alt Page / asset
        Nginx->>Frontend: proxy
        Frontend-->>Browser: HTML / JS / CSS
    else API call (JWT in cookie)
        Nginx->>API: proxy /api/*
        API->>API: JWT middleware
        API->>DB: SQLx query
        DB-->>API: rows
        API-->>Browser: JSON response
    end
```

```mermaid
sequenceDiagram
    participant Browser
    participant API
    participant DB
    participant Embedding

    Browser->>API: PATCH /api/user/update_profile
    API->>API: validate (required fields, links)
    API->>DB: fetch courses & tools by UUID
    API->>Embedding: embed natural language document
    Note over API,Embedding: pool of 2 AllMiniLML6V2 instances
    Embedding-->>API: Vec<f32> 384-dim vector
    API->>DB: BEGIN transaction
    API->>DB: UPDATE users SET ... embedding = $vector
    API->>DB: DELETE + INSERT user_links
    API->>DB: DELETE + INSERT user_tools
    API->>DB: COMMIT
    API-->>Browser: 200 OK

    Note over API,DB: If embedding fails, needs_reembedding=true
    Note over API,DB: Background job retries every 60s
```

---

#### Development View

How the codebase is organised across modules and packages.

```mermaid
graph TD
    subgraph frontend["frontend/"]
        F1["app/ — Next.js App Router"]
        F2["components/ — UI components\nHero · About · Nav · Footer\nSearchStudents · StudentsResult\nGlassCard · ParticleNetwork · GridBackground\nConfirmModal · PasswordStrengthMeter"]
        F3["(auth)/ — Auth route group\nlogin · register · validate-user\nforgot-password · reset-password"]
        F4["profile/ — Profile view & edit\nEditProfileForm · UpdateImageForm\nUserProjects · ProjectCard · UpsertProjectModal"]
        F5["student/[id]/ — Public student profile"]
        F6["context/ — Auth context"]
        F7["lib/ — auth · dtos · helpers"]
        F8["SVGS/ — Inline SVG icon components"]
        F9["public/ — Static assets & event images"]
        F1 --> F2
        F1 --> F3
        F1 --> F4
        F1 --> F5
        F1 --> F6
        F1 --> F7
        F2 --> F8
    end

    subgraph api["api/src/"]
        A1["handler/ — HTTP route handlers\nauth · user · reference"]
        A2["service/ — Business logic\nauth · user · reference"]
        A3["db/ — Repository layer\nauth_repo · user_repo · reference_repo"]
        A4["middleware/ — JWT auth"]
        A5["dtos/ — Data transfer objects\nauth · user · reference"]
        A6["models/ — Domain models\nuser · file"]
        A7["utils/embedding — FastEmbed model pool"]
        A8["utils/file_storage — Disk I/O"]
        A9["utils/images — Image processing"]
        A10["utils/email — Postmark + Tera templates"]
        A11["utils/password — Argon2 hashing"]
        A12["utils/token — JWT creation & validation"]
        A1 --> A2
        A2 --> A3
        A2 --> A7
        A2 --> A8
        A2 --> A9
        A2 --> A10
        A4 --> A1
    end

    subgraph infra["infra"]
        I1["nginx/ — Reverse proxy\ndev.conf · prod.conf"]
        I2["migrations/ — 12 SQLx migrations"]
        I3[".github/workflows/ — CI/CD pipeline\ntest-api · build-frontend · docker-integration"]
    end
```

---

#### Physical View

How the services are deployed on the production host.

```mermaid
graph TD
    Browser["User Browser"] -->|"HTTPS :443"| Nginx

    subgraph host["Cloud Server — Docker network (appnet)"]
        Nginx["Nginx container\nSSL termination · reverse proxy"]
        FE["Frontend container\n:3000 · Next.js SSR"]
        API["API container\n:8080 · Actix-web\n2× AllMiniLML6V2 (~22 MB each)"]
        DB[("PostgreSQL container\n:5432\npgvector · HNSW index")]

        Nginx -->|"pages & assets"| FE
        Nginx -->|"/api/*"| API
        Nginx -->|"/uploads/* (cached 30d)"| API
        API --> DB
    end
```

---

#### Scenarios (+1)

Key use cases that exercise the four views above.

```mermaid
graph LR
    Visitor(["Visitor / Recruiter"])
    Student(["Student"])

    Visitor --> Browse["Browse student profiles"]
    Visitor --> Search["Natural language search\ne.g. 'Rust developer interested in AI'"]
    Visitor --> EventInfo["View event info & schedule"]

    Student --> Register["Register account\n→ email verification"]
    Student --> Login["Log in"]
    Student --> EditProfile["Edit profile\nname · course · bio · tools · links · certificates"]
    Student --> ManageProjects["Manage projects\ncreate · edit · feature · upload images"]

    Register -->|verified| Login
    Login -->|JWT issued| EditProfile
    Login -->|JWT issued| ManageProjects
    EditProfile -->|PATCH + embed| Search
    ManageProjects -->|POST + embed| Search
    Search -->|vector similarity| Browse
```

---
