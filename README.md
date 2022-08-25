[<img alt="github" src="https://img.shields.io/static/v1?label=github&message=wallhaven&color=acb0d0&logo=Github&style=flat-square&logoColor=a9b1d6" height="20">](https://github.com/dax99993/wallhaven)
[<img alt="crates" src="https://img.shields.io/crates/v/wallhaven?logo=rust&logoColor=a9b1d6&style=flat-square&color=fc8d62" height="20">](https://crates.io/crates/ansinator)
<div align="center">

  <h3>
Wallhaven is CLI for downloading quickly and easily wallpaper from <a href ="https://wallhaven.cc" > Wallhaven </a> using their API
  </h3>
</div>

<div align="center">
  <img src="https://github.com/dax99993/wallhaven/blob/main/demo/demo.gif" width='1024' height='768'>
  <br/><br/>
</div>

## Usage
#### Set API key
```sh
wallhaven --set-api "asdjj188371jjasdGGJASUUEWQJ8" 
```
#### Search without API key and query
```sh
wallhaven -q "anime +cat" -s VIEWS -x 110 -a 1920x1080 -p ~/.Pictures/wallpapers
```
#### Search without API key and color
```sh
wallhaven -C 0066cc -s FAVORITES -a 1920x1080 -p ~/.Pictures/wallpapers
```

#### Search with API user preferences
```sh
wallhaven -q "nature -mountain" -s TOPLIST -p ~/.Pictures/wallpapers
```

#### Search overwriting API user preferences with given preferences (one use)
```sh
wallhaven -q "mountain" -n -c 101 -p ~/.Pictures/wallpapers
```

#### Search as non-user ignoring API stored user
```sh
wallhaven -q "mountain" -n -c 101 -p ~/.Pictures/wallpapers
```

## Installation
#### Cargo:
You can install the binary crate directly
```sh
cargo install wallhaven 
```

#### Manual Installation:
you can clone **wallhaven** repo and build it locally
```sh
git clone https://github.com/dax99993/wallhaven
cd wallhaven 
cargo install --path .
```

## Features
- API key support
- Downloading progress bar
- Overwrite API preferences but keep NSFW from API. 
- Made with love <3


## Todo
- [ ] Replace the love in exchange for good code.
- [ ] Refactor Code
- [ ] Add more helpful argument description and examples 
- [ ] Learn about async code to implement it correctly.


## Notes
- I'm new in Rust so, a lot of unclean and ugly code.
- If you create a good version similar to this program please notify me at **dax99993@gmail.com**, i want to see a good code example of this program.

## License
[MIT](https://mit-license.org/)
