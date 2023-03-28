
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
- [x] Global properties
- [x] Different delimiters for properties: 
    + [x] `<space>` 
    + [x] `:` 
- [x] Hierarchy: Nesting the section
    + [x] `[section.subsection]` 
    + [x] `[.subsection]` where nesting under `section` is inferred
- [x] Variants of comments:
    + [x] `#`,
- [x] Duplicate properties with multiple parsing strategies:
    + [x] Ignore: Ignore the latest conflicting duplicate.
    + [x] Overwrite: Overwrite the original property with the new one.
    + [x] Allow multiple: Allow multiple values to co-exist.
