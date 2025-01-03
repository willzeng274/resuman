use chrono::{DateTime, TimeDelta, Utc};

// This is just architecture, not actual code
// These structs are never used

#[allow(dead_code)]
pub struct Resume {
    pub company: String, // company name
    // INDEX: company_role_time, group is solely for organization purposes
    pub group: String, // work, hackathons, university, etc
    // how groups should work: whenever a new --group is specified, just create the folder if it doesn't exist
    // store in SQL database as "group" text entry
    // default group should be configured
    pub template: String, // template name... OR the name of the .tex file it came from (i.e. used from the copy command)
    // templates folder should be configured as well
    pub position: String, // position; for the HACKATHON edge case just put "hacker"
    pub created_at: DateTime<Utc>, // date created
    pub has_cover_letter: bool, // if applied with a cover letter

    pub metadata: MetaData,
}

#[allow(dead_code)]
pub struct MetaData {
    // the job associated with the resume
    pub applied_time: DateTime<Utc>, // date applied

    pub length: TimeDelta, // length of the job, default 4 months? configurable
    pub location: Option<String>, // location of job

    pub status: String, // status of application
    // may include: applying, applied, oa, interview, rejected, accepted, ghosted, etc
    pub urls: String, // URLs to job posting, company, etc, separated by |
    // may include: job posting, company, linkedin, url to check, etc
    // can be empty
    pub notes: String, // other metadata, can be JSON, default empty
}

#[allow(dead_code)]
impl Resume {
    pub fn new(
        group: String,
        template: String,
        company: String,
        position: String,
        has_cover_letter: bool,
        length: u16,
    ) -> Self {
        Resume {
            group,
            template,
            company,
            position,
            created_at: Utc::now(),
            has_cover_letter,
            metadata: MetaData {
                applied_time: Utc::now(),
                length: TimeDelta::weeks(length.into()), // should be configurable?
                location: None,
                status: "applying".to_string().to_ascii_lowercase(), // standard: lowercase
                urls: "".to_string(),                                // separated by |
                notes: "".to_string(),
            },
        }
    }
}
