<div align="center">

<img src="https://huddersfieldshowcase.cloud/favicon.ico" width="64" height="64" alt="C&E Futures logo" />

# C&E Futures '26

### Student Showcase — University of Huddersfield

[![Live Site](https://img.shields.io/badge/Live%20Site-huddersfieldshowcase.cloud-a1e9f0?style=for-the-badge&labelColor=204346)](https://huddersfieldshowcase.cloud/)

</div>

---

A web platform for the University of Huddersfield's School of Computing & Engineering annual showcase event, connecting final-year students with industry professionals and recruiters.

## About

C&E Futures '26 is an event held on **19 June 2026** at the University of Huddersfield, where final-year Computing & Engineering students present their projects to industry partners and potential employers.

The platform serves as both a pre-event information hub and a discovery tool — allowing visitors to browse student projects and register their interest in attending before the event takes place.

## What's on offer

- **Live project demos** — see working software and hardware built by final-year students
- **Poster presentations** — deep-dives into research and technical work
- **Networking opportunities** — meet students, academics, and fellow industry guests

The event runs from **12:00 PM to 5:00 PM** and covers the university's areas of expertise including Engineering, AI, Cyber Security, and Creative Computing.

## Features

- Browse student profiles and their final-year projects
- Filter and discover projects across a range of technologies and disciplines
- Register interest in attending the event
- User account registration and login

## Who is it for?

- **Industry professionals & recruiters** looking to spot emerging talent and hire graduates
- **Commercial partners** interested in R&D collaboration with the university
- **Anyone** curious about what the next generation of Computing & Engineering graduates is building

---

## Technical

### Tech Stack

| Layer | Technology |
|---|---|
| **Frontend** | Next.js 16, React 19, TypeScript, Tailwind CSS 4 |
| **Backend** | Rust, Actix-web 4 |
| **Database** | PostgreSQL 18 + pgvector |
| **Auth** | JWT, Argon2 |
| **AI / Search** | FastEmbed (text embeddings) |
| **Infrastructure** | Docker, Nginx, GitHub Actions |

### Software Architecture

The project follows a **frontend/backend split monorepo** structure with three main services orchestrated via Docker Compose. The architecture is documented using the **4+1 model**:

---

#### Logical View
The system's functional decomposition into major subsystems.

```mermaid
graph TD
    subgraph Auth["Authentication"]
        A1[JWT Token]
        A2[Argon2 Hashing]
    end
    subgraph User["User"]
        U1[Profile]
        U2[File Uploads]
    end
    subgraph Discovery["Discovery"]
        D1[Project Browser]
        D2[Filter & Sort]
        D3[Vector Search]
    end
    subgraph Content["Content"]
        C1[Projects]
        C2[Reference Data]
    end

    Auth --> User
    User --> Content
    Discovery --> Content
    D3 -->|FastEmbed embeddings| C1
```

---

#### Process View
Runtime communication between services for a typical request.

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

---

#### Development View
How the codebase is organised across modules and packages.

```mermaid
graph TD
    subgraph frontend["frontend/"]
        F1["app/ — Next.js App Router"]
        F2["components/ — UI components"]
        F3["context/ — React context"]
        F4["lib/ — utilities"]
        F1 --> F2
        F1 --> F3
        F1 --> F4
    end

    subgraph api["api/src/"]
        A1["handler/ — HTTP route handlers"]
        A2["service/ — business logic"]
        A3["db/ — repository layer"]
        A4["middleware/ — JWT auth"]
        A5["models/ & dtos/"]
        A1 --> A2 --> A3
        A4 --> A1
    end

    subgraph infra["infra"]
        I1["nginx/ — reverse proxy config"]
        I2["migrations/ — SQL migrations"]
        I3[".github/workflows/ — CI/CD"]
    end
```

---

#### Physical View
How the services are deployed on the production host.

```mermaid
graph TD
    Browser["User Browser"] -->|"HTTPS :443"| Nginx

    subgraph host["Cloud Server — Docker network (appnet)"]
        Nginx["Nginx container"]
        FE["Frontend container\n:3000"]
        API["API container\n:8080"]
        DB[("PostgreSQL container\n:5432")]

        Nginx -->|pages & assets| FE
        Nginx -->|"/api/*"| API
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

    Visitor --> Browse["Browse projects"]
    Visitor --> FilterP["Filter by tech / discipline"]
    Visitor --> Register["Register event interest"]

    Student --> Login["Log in"]
    Student --> Profile["Edit profile"]
    Student --> Upload["Upload project assets"]

    Login -->|JWT issued| Profile
    Login -->|JWT issued| Upload
    Browse -->|vector search| FilterP
```

---

<div align="center">

*Built by [Mateusz Kroplewski](https://github.com/Kroplewski-M)*

</div>
