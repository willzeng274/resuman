-- Add up migration script here

-- Main Resume Table
CREATE TABLE resumes (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT, -- Serial ID for unique identification
    "group" TEXT NOT NULL,               -- Organizational group
    "template" TEXT NOT NULL,              -- Template name or .tex file name
    "company" TEXT NOT NULL,               -- Company name
    "position" TEXT NOT NULL,              -- Position
    "created_at" DATETIME NOT NULL,        -- Date created
    "has_cover_letter" BOOLEAN NOT NULL,   -- Indicates if applied with a cover letter
    "file_path" TEXT NOT NULL,             -- File path to the resume, should be the directory
    UNIQUE(company, position, created_at) -- Composite unique constraint
);

-- MetaData Table (One-to-One with Resume)
CREATE TABLE metadata (
    "resume_id" INTEGER PRIMARY KEY,      -- Foreign key directly tied to Resume.id
    "applied_time" DATETIME,              -- Date applied, nullable since the resume may not have been sent
    "length" INTEGER NOT NULL,            -- Job length in days (TimeDelta as integer)
    "location" TEXT,                      -- Optional location of job
    "status" TEXT NOT NULL,               -- Application status
    "urls" TEXT,                          -- Pipe-separated URLs
    "notes" TEXT DEFAULT '' NOT NULL,     -- JSON metadata or additional notes, default empty
    FOREIGN KEY (resume_id) REFERENCES resumes(id) ON DELETE CASCADE, -- One-to-one relation
    UNIQUE(resume_id) -- Unique constraint
);
