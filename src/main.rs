use std::cmp::Ordering;
use std::io;
use std::io::Write;
use std::io::Read;

use clap::*;

//use colored::Colorize;

use ftags::{FTLTrait, FTag, FTagList, FTagFile};

#[derive(Parser, Debug)]
#[command(name = "ftags")]
#[command(bin_name = "ftags")]
enum Commands {
	/// List all tags for a given file          (shorthands: `l`, `ls`)
	#[clap(aliases = &["l", "ls"])]
	List { file: std::path::PathBuf },
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
	Search { tags: Vec<FTag> },
}

#[derive(Parser)]
struct Args {
	/// Print script-friendly output.
	#[arg(long, short)]
	script: bool,

	/// Alternative file (path) to use.
	#[arg(long, short)]
	file: Option<std::path::PathBuf>,

	/// Generate shell completions
	#[arg(long, short)]
	completions: bool,

	#[clap(subcommand)]
	cmd: Commands,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = Args::parse();
	let command = args.cmd;
	let ftags_file = if let Some(path) = args.file {
		path
	} else {
		std::path::Path::new(".").read_dir()?.find(|f| 
												   f.as_ref().unwrap().file_name() == ".ftags"
		).ok_or("No .ftags file found!")??.path()
	};

	let tag_delimiter = match args.script {
		true => "\n",
		false => ", ",
	};

	match command {
		Commands::List { file } => {
			let ftags = FTagList::read(&ftags_file);
			let mut ftags_for_file = None;

			for tag in ftags {
				if tag.file == file {
					ftags_for_file = Some(tag);
					break;
				}
			}
			if let Some(file) = ftags_for_file {
				// Print tags
				println!("{}", file)
			} else {
				return Err(format!("No tags for `{}`.", file.display()).into());
			}
		}
		Commands::ListTags => {
			let ftags = FTagList::read(&ftags_file);

			let mut tags_list = Vec::new();

			for tag_file in ftags {
				for tag in tag_file.tags {
					tags_list.push(tag);
				}
			}

			tags_list.sort_by(|a, b| match a.name.cmp(&b.name) {
				Ordering::Equal => a.data.cmp(&b.data),
				other => other,
			});
			tags_list.dedup();

			// Print the list
			println!("{}", join_vec(tags_list, tag_delimiter));
		}
		Commands::Add { file, mut tags } => {
			let mut ftags = FTagList::read(&ftags_file);

			// Find file
			let mut file_found = false;
			for tag_file in &mut ftags {
				if tag_file.file == file {
					file_found = true;
					tag_file.tags.append(&mut tags);
				}
			}

			// Add file if it wasn't found
			if !file_found {
				ftags.push(FTagFile {
					file,
					tags,
				})
			}

			ftags.write(&ftags_file);
		}
		Commands::Remove { file, tags } => {
			let ftags = FTagList::read(&ftags_file);
			let mut ftags_new = ftags.clone();

			// Find file
			let mut file_found = false;
			for (i, tag_file) in ftags.iter().enumerate() {
				if tag_file.file == file {
					file_found = true;
					if tags.is_empty() {
						print!("Remove file from db: `{}`? [y/(N)]> ", file.display());
						io::stdout().flush().unwrap();

						let mut choice = [0_u8];
						io::stdin().read_exact(&mut choice).unwrap();

						if &choice == b"y" || &choice == b"Y" {
							ftags_new.remove(i);
						} else {
							println!("No changes.");
							return Ok(());
						}
					} else {
						let mut tag_found = false;
						for (j, tag) in tag_file.tags.iter().enumerate() {
							if tags.contains(tag) {
								tag_found = true;
								ftags_new[i].tags.remove(j);
							}
						}

						if !tag_found {
							println!("Tags not found: `{}`", join_vec(tags, ", "));
						}
					}
					break;
				}

				// Remove file from db if there are no associated tags.
				if tag_file.tags.is_empty() {
					ftags_new.remove(i);
				}
			}

			if !file_found {
				return Err(format!("File not in database: `{}`", file.display()).into());
			}

			ftags_new.write(&ftags_file);
		}
		Commands::Search { tags } => {
			let ftags = FTagList::read(&ftags_file);
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
		}
	}

	Ok(())
}

//struct TagCount {
//    tag: FTag,
//    count: usize,
//}

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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_join_vec() {
		let elements = vec!["a", "b", "c"];
		assert_eq!(join_vec(elements, ", "), "a, b, c");
	}
}
