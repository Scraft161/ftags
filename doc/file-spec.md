# .ftags file specification

This document goes over the file specification of the `.ftags` index file.

## Index

- tl;dr
- file matching
    - globbing
- tags
    - tag data
    

# tl;dr

the .ftags file is in the root of the search path and contains a list of all tagged files with their tags.

```ftags
file: tag1, tag2:tag2_data, tag3:[tag 3 data]
```

The above snippet defines 3 tags for a file named `file`.  
`tag1`: a tag without tag data.  
`tag2`: a tag with `tag2_data` as tag data.  
`tag3`: a tag with a list of tag data containing `tag`, `3`¸ and `data`.

## File matching

`ftags(1)` only keeps an index of
