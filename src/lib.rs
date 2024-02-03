use std::path::PathBuf;
use std::str::FromStr;
use std::fs::DirEntry;
use std::fs;
use std::fmt;

pub trait FTLTrait {
    fn read(_: Option<DirEntry>) -> Self;

    fn write(_: Option<DirEntry>);

    fn from_string(_: String) -> Self;
}

pub type FTagList = Vec<FTagFile>;

impl FTLTrait for FTagList {
    /// Read ftags from a file
    ///
    /// `panic!()`s when file is `None`
    fn read(file: Option<DirEntry>) -> Self {
        if file.is_none() { panic!() }

        let contents = fs::read_to_string(file.unwrap().file_name()).unwrap();
        Self::from_string(contents)
    }

    fn write(file: Option<DirEntry>) {
        todo!()
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
            tags_str.split(',').for_each(|s| tags.push(FTag::from_str(s.trim()).unwrap()));
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
    pub data: Option<FTagData>
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
                },
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
                } else if s.starts_with('[') {
                    data_list.push(s[1..].trim().to_string())
                } else if s.ends_with(']') {
                    data_list.push(s[0..s.len() - 1].trim().to_string())
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
