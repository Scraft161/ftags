use std::{
	fmt,
	fs,
	path::{Path, PathBuf},
	str::FromStr,
};

pub trait FTLTrait {
	fn read(_: &Path) -> Self;

	fn write(&self, _: &Path);

	fn from_string(_: String) -> Self;

	fn to_string(&self) -> String;
}

pub type FTagList = Vec<FTagFile>;

impl FTLTrait for FTagList {
	/// Read ftags from a file
	///
	/// `panic!()`s when file is `None`
	fn read(file: &Path) -> Self {
		let contents = fs::read_to_string(file).unwrap();
		Self::from_string(contents)
	}

	fn write(&self, file: &Path) {
		fs::write(file, self.to_string() + "\n").unwrap();
	}

	fn from_string(string: String) -> Self {
		let mut ftags = Vec::new();

		for tagged_file in string.split('\n') {
			if tagged_file.is_empty() {
				continue;
			}

			ftags.push(FTagFile::from_str(tagged_file).unwrap());
		}

		ftags
	}

	fn to_string(&self) -> String {
		let mut str = String::new();

		for (i, file) in self.iter().enumerate() {
			str.push_str(&format!("{}", file));

			if i != self.len() - 1 {
				str.push('\n');
			}
		}

		str
	}
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct FTagFile {
	pub file: PathBuf,
	pub tags: Vec<FTag>,
}

impl FromStr for FTagFile {
	type Err = String;

	fn from_str(str: &str) -> Result<Self, Self::Err> {
		if let Some((file, tags_str)) = str.split_once(':') {
			let mut tags = Vec::new();
			tags_str
				.split(',')
				.for_each(|s| tags.push(FTag::from_str(s.trim()).unwrap()));
			Ok(Self {
				file: PathBuf::from_str(file).unwrap(),
				tags,
			})
		} else {
			Err("ɑ".to_string())
		}
	}
}
impl fmt::Display for FTagFile {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		if self.tags.is_empty() {
			return Ok(());
		}

		let mut tags_str = String::new();

		for (i, tag) in self.tags.iter().enumerate() {
			if i != self.tags.len() && i != 0 {
				tags_str.push_str(", ");
			}

			tags_str.push_str(&format!("{}", tag));
		}

		write!(f, "{}: {}", self.file.display(), tags_str)
	}
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct FTag {
	pub name: String,
	pub data: Option<FTagData>,
}

impl FromStr for FTag {
	type Err = String;

	fn from_str(str: &str) -> Result<Self, Self::Err> {
		if !str.contains(':') {
			Ok(Self {
				name: str.to_string(),
				data: None,
			})
		} else {
			if let Some((name, tag_data)) = str.split_once(':') {
				let data = FTagData::from_str(tag_data);

				Ok(Self {
					name: name.to_string(),
					data: Some(data?),
				})
			} else {
				Err(format!("Could not parse `{}` into a valid tag.", str))
			}
		}
	}
}

impl fmt::Display for FTag {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		if let Some(data) = &self.data {
			match data {
				FTagData::Single(tag_data) => write!(f, "{}:{}", self.name, tag_data),
				FTagData::List(tag_list) => {
					let mut data_str = String::new();

					for (i, list_item) in tag_list.iter().enumerate() {
						if i != tag_list.len() && i != 0 {
							data_str.push(' ');
						}

						data_str.push_str(list_item);
					}

					write!(f, "{}:[{}]", self.name, data_str)
				}
			}
		} else {
			write!(f, "{}", self.name)
		}
	}
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum FTagData {
	Single(String),
	List(Vec<String>),
}

impl FromStr for FTagData {
	type Err = String;

	fn from_str(str: &str) -> Result<Self, Self::Err> {
		if str.starts_with('[') && str.ends_with(']') {
			let mut data_list = Vec::new();
			str.split(' ').for_each(|s| {
				// TODO: clean this mess TwT
				if s.starts_with('[') && s.ends_with(']') {
					data_list.push(s[1..s.len() - 1].trim().to_string())
				} else if let Some(stripped) = s.strip_prefix('[') {
					data_list.push(stripped.trim().to_string())
				} else if let Some(stripped) = s.strip_suffix(']') {
					data_list.push(stripped.trim().to_string())
				} else {
					data_list.push(s.trim().to_string())
				}
			});
			Ok(FTagData::List(data_list))
		} else {
			Ok(FTagData::Single(str.to_string()))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_ftags() {
		let ftags_str = "a: tag, tag2
foo/b.jpg: file_type:jpg, img_tags:[1girl solo standing long_hair]";
		let ftags_proper: FTagList = vec![
			FTagFile {
				file: String::from("a").into(),
				tags: vec![
					FTag {
						name: String::from("tag"),
						data: None,
					},
					FTag {
						name: String::from("tag2"),
						data: None,
					},
				],
			},
			FTagFile {
				file: String::from("foo/b.jpg").into(),
				tags: vec![
					FTag {
						name: String::from("file_type"),
						data: Some(FTagData::Single(String::from("jpg"))),
					},
					FTag {
						name: String::from("img_tags"),
						data: Some(FTagData::List(vec![
							String::from("1girl"),
							String::from("solo"),
							String::from("standing"),
							String::from("long_hair"),
						])),
					},
				],
			},
		];

		assert_eq!(FTagList::from_string(ftags_str.to_string()), ftags_proper);
	}

	#[test]
	fn test_stringify_ftags() {
		let ftags: FTagList = vec![FTagFile {
			file: String::from(".cargo/bin").into(),
			tags: vec![
				FTag {
					name: String::from("directory"),
					data: None,
				},
				FTag {
					name: String::from("child_content_type"),
					data: Some(FTagData::Single(String::from("binary"))),
				},
				FTag {
					name: String::from("misc_info"),
					data: Some(FTagData::List(vec![
						String::from("rust"),
						String::from("cargo"),
					])),
				},
			],
		}];
		let ftags_as_string = String::from(
			".cargo/bin: directory, child_content_type:binary, misc_info:[rust cargo]",
		);

		assert_eq!(ftags.to_string(), ftags_as_string);
	}

	#[test]
	fn test_full_parse_and_stringify() {
		let str = String::from(
			".cargo/bin: directory, child_content_type:binary, misc_info:[rust cargo]",
		);

		assert_eq!(FTagList::from_string(str.clone()).to_string(), str);
	}
}
