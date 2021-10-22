![PR](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat&logo=github)
![Open Source](https://badges.frapsoft.com/os/v2/open-source.svg?v=103)
![Visitors](https://visitor-badge.glitch.me/badge?page_id=IEEE-VIT.IEEE-VIT)
![Issues](https://img.shields.io/github/issues/IEEE-VIT/templa-rs)
![License](https://img.shields.io/github/license/IEEE-VIT/templa-rs)
![Stars](https://img.shields.io/github/stars/IEEE-VIT/templa-rs)
![Forks](https://img.shields.io/github/forks/IEEE-VIT/templa-rs)
![Last Commit](https://img.shields.io/github/last-commit/IEEE-VIT/templa-rs)
![Size](https://img.shields.io/github/repo-size/IEEE-VIT/templa-rs)
![Maintained](https://img.shields.io/maintenance/yes/2021)
[![UI](https://img.shields.io/badge/User%20Interface-Link%20to%20UI-orange?style=flat-square&logo=appveyor)](https://www.figma.com/file/2VfxFGTk2FqLJiMExB7u9P/templa-rs?node-id=2%3A2)

<p align="center"><img width="80%" src="https://i.imgur.com/tLvPX2F.png"/></p>

<p align="center">One Stop Solution for all boilerplate needs!</p>
<p align="center">Consider leaving a :star: if you found the project helpful.</p>

# Templa-rs
Templa-rs is a one-of-a-kind TUI tool written in Rust, which helps you generate boilerplate templates for various types of projects and architectures in a matter of seconds!

![Templa-rs](https://i.imgur.com/H2YjBSr.gif)

## Usage
1) Download templa-rs [here](https://github.com/IEEE-VIT/templa-rs/releases)
2) [Run templa-rs](#running-templa-rs)
3) Find the boilerplate you want
4) Press [ENTER] to select it
5) Start Coding

You can exit the program at any time using `Ctrl+C`

### Running templa-rs
#### Windows
If you only wish to use Live Search you can run templa-rs on Windows by simply double clicking the `exe` file.
If you wish to use CLI queries:
1) Open a Terminal Window in the directory that the executable is in 
2) Run `./templa-rs.exe [OPTIONS]`

#### Linux and macOS
1) Open a Terminal window in the directory that templa-rs is in
2) Run `tar -zxvf {templa-file-name}.tar.gz`
6) Then run `./templa-rs [OPTIONS]`

## Features

### Live Search
Live Search is a tool that lets you narrow down your search for the boilerplate you need, while also remaining inside the TUI at all times saving you time.

#### Live Search Features:
* One way to use Live Search is to simply type a search term such as `php`, this will then show you all the boilerplates with `php` in their name
* `TAG:` Tag allows you to search each boilerplate for a specific catagory of boilerplates (e.g. `TAG:backend`), using multiple tags may narrow down your search further

More methods to narrow down your search may be coming in the future so keep your eyes out!

### Command Line Queries
You can also use queries directly from the command line to refine your search:

![image](https://i.imgur.com/tTBXG6A.png)

### Template Preview
The Template Preview will show you a tree of the files that will be created when loading the boilerplate.

![image](https://i.imgur.com/4I1htAe.png)

## Built With
* [Rust](https://www.rust-lang.org/)
* [tui-rs](https://github.com/fdehau/tui-rs)

## Getting Started Contributing
Got a great new feature or a boilerplate you want to add? Why not contribute to the project:
1) Fork it.
2) Clone your forked repo and move inside it:
```
git clone https://github.com/{your-username}/templa-rs.git && cd templa-rs
```
3) Checkout to a new branch to work on an issue:
```
git checkout -b my-amazing-feature
```
4) Run It Locally
```
cargo run
```
5) Once you're all done coding, it's time to open a PR :)
Run the following commands from the root of the project directory:
```
git add .
```
```
git commit -m "A short description about the feature."
```
```
git push origin <my-amazing-feature>
```

Open your forked repo in your browser and then raise a PR to the `master` branch of this repository!


## Contributing
To start contributing, check out [CONTRIBUTING.md](https://github.com/IEEE-VIT/Templa-rs/blob/master/CONTRIBUTING.md). New contributors are always welcome to support this project. If you want something gentle to start with, check out issues labelled as `difficulty-easy` or `good-first-issue`. Check out issues labelled as `hacktoberfest` if you are up for some grabs! :) 

## License
This project is licensed under [MIT](https://github.com/IEEE-VIT/Templa-rs/blob/master/LICENSE.md).
