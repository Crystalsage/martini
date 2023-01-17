
# Martini
Martini is a Rust based INI file parser. It was born during my recreational
programming sessions.

## About INI files
INI files are configuration files used for computer software configuration 
(typically Windows). Each file describes several named sections
. These sections have key-value pairs inside of them. 

The file format isn't too formally defined. There are some things that are 
standard and some things which vary. The goals section defines all specifications
(to be) implemented in this project.

## Goals
We'd like to achieve parsing for following features and properties. 
Some of these are implicitly guaranteed.

### Basic features
- [x] Sections (`[section]`)
- [x] Keys/Properties (`key=value`)
- [x] Case sensitivity
- [x] Comments (`; is a comment`)
- [x] Irrelevant ordering of sections 

### Varying features
- [x] Blank properties (There's no value. e.g. `key=`)
- [ ] Global properties
- [ ] Different delimiters: `<space>` and `:` 
- [ ] Hierarchy: Nesting the section (`[section.subsection]` and `[.subsection]` where nesting under `section` is inferred)
- [ ] Variants of comments: `#`, comments after properties may begin with multiple markers.
- [ ] Duplicate properties with multiple parsing strategies: 
    + [ ] Ignore: Ignore the latest conflicting duplicate.
    + [ ] Overwrite: Overwrite the original property with the new one.
    + [ ] Allow multiple: Allow multiple values to co-exist.
- [ ] Escape characters (maybe)
