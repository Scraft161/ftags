use clap::*;

use ftags::{
    FTLTrait,
    FTagList,
    FTag,
};

#[derive(Parser, Debug)]
#[command(name = "ftags")]
#[command(bin_name = "ftags")]
enum Commands {
    /// List all tags for a given file          (shorthands: `l`, `ls`)
    #[clap(aliases = &["l", "ls"])]
    List {
        file: std::path::PathBuf,
    },
    /// List all tags                           (shorthands: `t`, `lt`)
    #[clap(aliases = &["t", "lt"])]
    ListTags,
    /// Add a tag to a file (Not working)       (shorthands: `a`, `n`)
    #[clap(aliases = &["a", "n"])]
    Add {
        file: std::path::PathBuf,
        tags: Vec<FTag>,
    },
    /// Remove a tag from a file (not working)  (shorthands: `r`, `d`)
    #[clap(aliases = &["r", "d"])]
    Remove {
        file: std::path::PathBuf,
        tags: Vec<FTag>,
    },
    /// Search files for a given tag            (shorthands: `s`, `f`)
    #[clap(aliases = &["s", "f"])]
    Search {
        tags: Vec<FTag>,
    },
}

//struct Args {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ftags_file = match std::path::Path::new(".").read_dir().unwrap().find(|f| {
        //dbg!(f);
        f.as_ref().unwrap().file_name() == ".ftags"
    }) {
        Some(file) => {
            if let Ok(file) = file {
                Some(file)
            } else {
                None
            }
        },
        None => None,
    };
    let command = Commands::parse();
    match command {
        Commands::List{file} => {
            if ftags_file.is_none() {
                return Err("No `.ftags` file found!".into());
            }

            let ftags = FTagList::read(ftags_file);
            let mut ftags_for_file = None;

            for tag in ftags {
                if tag.file == file {
                    ftags_for_file = Some(tag);
                    break;
                }
            };
            if let Some(file) = ftags_for_file {
                // Print tags
                println!("{}", file)
            } else {
                return Err(format!("No tags for `{}`.", file.display()).into());
            }
        },
        Commands::ListTags => {
            if ftags_file.is_none() {
                return Err("No `.ftags` file found!".into());
            }

            let ftags = FTagList::read(ftags_file);

            let mut tags_list = Vec::new();

            for tag_file in ftags {
                for tag in tag_file.tags {
                    tags_list.push(tag);
                }
            }

            // Dedup tags
            //let mut dedup_tags: Vec<TagCount> = Vec::new();
            //for tag in &tags_list {
            //    let mut tag_in_list = false;
            //    for (i, d_tag) in dedup_tags.iter().enumerate() {
            //        if tag == &d_tag.tag {
            //            tag_in_list = true;
            //            d_tag.count += 1;
            //            continue
            //        }

            //        if i == dedup_tags.len() - 1 {
            //            if !tag_in_list {
            //                dedup_tags.push(TagCount {
            //                    tag: tag.clone(),
            //                    count: 1,
            //                })
            //            }
            //        }
            //    }
            //}

            // Print the list
            for (i, tag) in tags_list.iter().enumerate() {
                print!("{}", tag);

                if i != tags_list.len() - 1 {
                    print!(", ");
                }
            }
            println!();
        },
        Commands::Add{file, tags} => {
            dbg!(&file, &tags);
            let ftags = FTagList::read(ftags_file);
            dbg!(&ftags);
        },
        Commands::Remove {file, tags} => {
            todo!();
        }
        Commands::Search {tags} => {
            let ftags = FTagList::read(ftags_file);
            let mut partial_matches = Vec::new();
            let mut full_matches = Vec::new();

            for tag_file in ftags {
                let mut full_match = false;
                let mut partial_match = false;

                for file_tag in &tag_file.tags {
                    for search_tag in &tags {
                        // Name match
                        if file_tag.name == search_tag.name {
                            partial_match = true;

                            // Only match if the user provided tag data to match
                            if let Some(search_tag_data) = &search_tag.data {
                                if let Some(file_tag_data) = &file_tag.data {
                                    if search_tag_data == file_tag_data {
                                        full_match = true;
                                    }
                                }
                            } else {
                                full_match = true;
                            }
                        }
                    }
                }

                if partial_match {
                    partial_matches.push(tag_file.clone());
                }

                if full_match {
                    full_matches.push(tag_file.clone());
                }
            }

            partial_matches.sort();
            full_matches.sort();

            // Print results to the screen
            if !full_matches.is_empty() {
                let matches_pretty = join_vec(full_matches.clone(), "\n    ");
                println!("Full matches:\n    {matches_pretty}");
            }
            if !partial_matches.is_empty() {
                let matches_pretty = join_vec(partial_matches, "\n    ");
                if !full_matches.is_empty() {
                    let full_matches_pretty = join_vec(full_matches.clone(), "\n    ");
                    if full_matches_pretty == matches_pretty {
                        return Ok(());
                    }
                    println!();
                }

                println!("Partial Matches:\n    {matches_pretty}");
            }
        },
    }

    Ok(())
}

struct TagCount {
    tag: FTag,
    count: usize,
}

fn join_vec<T: ToString>(vec: Vec<T>, delimiter: &str) -> String {
    let mut joined = String::new();

    for (i, item) in vec.iter().enumerate() {
        let item_string = item.to_string();
        joined.push_str(&item_string);

        if i != vec.len() - 1 {
            joined.push_str(delimiter);
        }
    }

    joined
}
