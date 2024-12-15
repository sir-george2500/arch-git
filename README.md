# Arch-Git

## Overview

It is generally believed that Linus Torvalds took just 5 days to build Git. While my implementation might take longer due to my skill level in Rust, this project aims to create a version control system inspired by Git.

## What is Arch-Git?

Arch-Git is a version control system similar to Git, with some additional features and improvements. This project is a custom implementation of a Git-like version control system written in Rust.

## Getting Started

### Prerequisites
- Rust programming language
- Cargo (Rust's package manager)

### Installation

1. Clone the repository:
```bash
git clone git@github.com:sir-george2500/arch-git.git
```
### Navigation and Setup


Navigate to the project directory:
```bash
cd arch-git
```
then run the below command to build your project 

```bash 
cargo build 
```

then run this command to run your project

```bash 
 ./target/debug/arch-git init test
```

then `cd` into test directory  and run `git status` you should see something like the below 

```bash 
On branch master No commits yet nothing to commit (create/copy files and use "git add" to track)
```
currently only one feature as been added initializing of empty repository

