# ftags

create and view tags for files and directories

tags are extra metadata that can be used to describe the file.

tags are stored in a `.ftags` file in the same directory

## Installation


## .ftags specification

the .ftags file is a binary text file containing the name of the file (in UTF-8), followed by `: ` and a comma separated list of tags.
tags can not have spaces in them; but they may contain further tag data.
tag data starts after the `: ` and either contains a single data point or multiple data points inside `[]` as a space-separated list.

```ftags
FILENAME: tag1, tag2:TagData, tag3
```

## ftags cli

**Important! This part is still very WIP**

- [x] list tags for file
- [x] list all tags
- [ ] add tags
- [ ] remove tags
- [x] list files with tag
- [ ] glob matching for files?

### Parsing & display

|    feature    | parse | display |
| ------------- | ----- | ------- |
| file name     | yes   | yes     |
| tag (no data) | yes   | yes     |
| tag (single)  | yes   | yes     |
| tag (list)    | yes   | yes     |

## 0.1 release roadmap
- [x] list tags for file
- [x] list all tags
- [x] list files with tag
- [x] add initial set of tests

## 0.2 release roadmap
- [x] finish file write logic (technically already here; just want to make sure it fully works first)
- [x] implement adding/removing tags from files
- [x] implement adding/removing files from the database
    - [x] remove a file once all tags have been removed.
- [ ] set up shell completions
- [ ] allow alternative file paths

## 0.3 release roadmap
- [ ] implement glob matching for files
- [ ] implement better tag matching (`a` should match `a:b`, and `a:[b c]`)
