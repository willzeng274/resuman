use crate::config::Config;
use anyhow::Result;
use clap::Parser;
use sqlx::SqlitePool;

#[derive(Parser, Debug)]
#[command(name = "find", about = "Find a resume and return its id", visible_aliases = ["search", "query"])]
pub struct FindCommand {
    // filters
    #[clap(short, long, help = "Filter by company")]
    company: Option<String>,

    #[clap(short, long, help = "Filter by group")]
    group: Option<String>,

    #[clap(short, long, help = "Filter by template")]
    template: Option<String>,

    #[clap(short, long, help = "Filter by position")]
    position: Option<String>,

    #[clap(long = "letter", help = "Filter by cover letter status")]
    has_cover_letter: Option<bool>,

    #[clap(long, help = "Filter by created date")]
    created_at: Option<String>,

    #[clap(long, help = "Filter by date applied")]
    applied_time: Option<String>,

    #[clap(long, help = "Filter by length of job")]
    length: Option<u16>,

    #[clap(short, long, help = "Filter by location")]
    location: Option<String>,

    #[clap(short, long, help = "Filter by status")]
    status: Option<String>,

    #[clap(short, long, help = "Filter by file path")]
    file_path: Option<String>,

    #[clap(long, help = "Filter by copied from")]
    copied_from: Option<String>,

    #[clap(short, long, help = "Filter by all")]
    all: Option<String>,
}

pub async fn execute(_cfg: Config, args: &FindCommand, pool: &SqlitePool) -> Result<()> {
    log::debug!("Find arguments:\n{:#?}", args);

    // run sql queries to find resumes from db

    if let Some(a) = &args.all {
        // filter every field with a
        // doesn't work with bools
        let pattern = format!("%{}%", a);
        let resumes = sqlx::query!(
            "SELECT id FROM resumes 
    JOIN metadata ON metadata.resume_id = resumes.id 
    WHERE company LIKE ? 
        OR \"group\" LIKE ? 
        OR template LIKE ? 
        OR position LIKE ? 
        OR created_at LIKE ? 
        OR applied_time LIKE ? 
        OR length = ? 
        OR location LIKE ? 
        OR status LIKE ? 
        OR file_path LIKE ? 
        OR copied_from LIKE ?",
            pattern,
            pattern,
            pattern,
            pattern,
            pattern,
            pattern,
            a,
            pattern,
            pattern,
            pattern,
            pattern
        )
        .fetch_all(pool)
        .await?;
        // output all ids
        for r in resumes {
            println!("{}", r.id);
        }
        return Ok(());
    }

    // each arg should be AND not OR

    let mut query =
        "SELECT id FROM resumes JOIN metadata ON metadata.resume_id = resumes.id WHERE 1 = 1"
            .to_string();

    if let Some(c) = &args.company {
        query.push_str(&format!(" AND company LIKE '%{}%'", c));
    }

    if let Some(g) = &args.group {
        query.push_str(&format!(" AND \"group\" LIKE '%{}%'", g));
    }

    if let Some(t) = &args.template {
        query.push_str(&format!(" AND template LIKE '%{}%'", t));
    }

    if let Some(p) = &args.position {
        query.push_str(&format!(" AND position LIKE '%{}%'", p));
    }

    if let Some(h) = &args.has_cover_letter {
        query.push_str(&format!(" AND has_cover_letter = {}", h));
    }

    if let Some(c) = &args.created_at {
        query.push_str(&format!(" AND created_at LIKE '%{}%'", c));
    }

    if let Some(a) = &args.applied_time {
        query.push_str(&format!(" AND applied_time LIKE '%{}%'", a));
    }

    if let Some(l) = &args.length {
        query.push_str(&format!(" AND length = {}", l));
    }

    if let Some(l) = &args.location {
        query.push_str(&format!(" AND location LIKE '%{}%'", l));
    }

    if let Some(s) = &args.status {
        query.push_str(&format!(" AND status LIKE '%{}%'", s));
    }

    if let Some(f) = &args.file_path {
        query.push_str(&format!(" AND file_path LIKE '%{}%'", f));
    }

    if let Some(c) = &args.copied_from {
        query.push_str(&format!(" AND copied_from LIKE '%{}%'", c));
    }

    let resumes = sqlx::query_as::<_, (i32,)>(query.as_str())
        .fetch_all(pool)
        .await?;

    for r in resumes {
        println!("{}", r.0);
    }

    // below is if each arg should be OR not AND

    // if let Some(c) = &args.company {
    //     let pattern = format!("%{}%", c);
    //     let resumes = sqlx::query!("SELECT id FROM resumes WHERE company LIKE ?", pattern)
    //         .fetch_all(pool)
    //         .await?;
    //     for r in resumes {
    //         println!("{}", r.id.unwrap());
    //     }
    // }
    //
    // if let Some(g) = &args.group {
    //     let pattern = format!("%{}%", g);
    //     let resumes = sqlx::query!("SELECT id FROM resumes WHERE \"group\" LIKE ?", pattern)
    //         .fetch_all(pool)
    //         .await?;
    //     for r in resumes {
    //         println!("{}", r.id);
    //     }
    // }
    //
    // if let Some(t) = &args.template {
    //     let pattern = format!("%{}%", t);
    //     let resumes = sqlx::query!("SELECT id FROM resumes WHERE template LIKE ?", pattern)
    //         .fetch_all(pool)
    //         .await?;
    //     for r in resumes {
    //         println!("{}", r.id);
    //     }
    // }
    //
    // if let Some(p) = &args.position {
    //     let pattern = format!("%{}%", p);
    //     let resumes = sqlx::query!("SELECT id FROM resumes WHERE position LIKE ?", pattern)
    //         .fetch_all(pool)
    //         .await?;
    //     for r in resumes {
    //         println!("{}", r.id.unwrap());
    //     }
    // }
    //
    // if let Some(h) = &args.has_cover_letter {
    //     let resumes = sqlx::query!("SELECT id FROM resumes WHERE has_cover_letter = ?", h)
    //         .fetch_all(pool)
    //         .await?;
    //     for r in resumes {
    //         println!("{}", r.id);
    //     }
    // }
    //
    // if let Some(c) = &args.created_at {
    //     let pattern = format!("%{}%", c);
    //     let resumes = sqlx::query!("SELECT id FROM resumes WHERE created_at LIKE ?", pattern)
    //         .fetch_all(pool)
    //         .await?;
    //     for r in resumes {
    //         println!("{}", r.id.unwrap());
    //     }
    // }
    //
    // if let Some(a) = &args.applied_time {
    //     let pattern = format!("%{}%", a);
    //     let resumes = sqlx::query!(
    //         "SELECT resume_id FROM metadata WHERE applied_time LIKE ?",
    //         pattern
    //     )
    //     .fetch_all(pool)
    //     .await?;
    //     for r in resumes {
    //         println!("{}", r.resume_id);
    //     }
    // }
    //
    // if let Some(l) = &args.length {
    //     let resumes = sqlx::query!("SELECT resume_id FROM metadata WHERE length = ?", l)
    //         .fetch_all(pool)
    //         .await?;
    //     for r in resumes {
    //         println!("{}", r.resume_id);
    //     }
    // }
    //
    // if let Some(l) = &args.location {
    //     let pattern = format!("%{}%", l);
    //     let resumes = sqlx::query!(
    //         "SELECT resume_id FROM metadata WHERE location LIKE ?",
    //         pattern
    //     )
    //     .fetch_all(pool)
    //     .await?;
    //     for r in resumes {
    //         println!("{}", r.resume_id);
    //     }
    // }
    //
    // if let Some(s) = &args.status {
    //     let pattern = format!("%{}%", s);
    //     let resumes = sqlx::query!(
    //         "SELECT resume_id FROM metadata WHERE status LIKE ?",
    //         pattern
    //     )
    //     .fetch_all(pool)
    //     .await?;
    //     for r in resumes {
    //         println!("{}", r.resume_id);
    //     }
    // }
    //
    // if let Some(f) = &args.file_path {
    //     let pattern = format!("%{}%", f);
    //     let resumes = sqlx::query!("SELECT id FROM resumes WHERE file_path LIKE ?", pattern)
    //         .fetch_all(pool)
    //         .await?;
    //     for r in resumes {
    //         println!("{}", r.id);
    //     }
    // }
    //
    // if let Some(c) = &args.copied_from {
    //     let pattern = format!("%{}%", c);
    //     let resumes = sqlx::query!(
    //         "SELECT resume_id FROM metadata WHERE copied_from LIKE ?",
    //         pattern
    //     )
    //     .fetch_all(pool)
    //     .await?;
    //     for r in resumes {
    //         println!("{}", r.resume_id);
    //     }
    // }

    Ok(())
}
