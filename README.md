[<img alt="github" src="https://img.shields.io/static/v1?label=github&message=wallhaven&color=acb0d0&logo=Github&style=flat-square&logoColor=a9b1d6" height="20">](https://github.com/dax99993/wallhaven)
[<img alt="crates" src="https://img.shields.io/crates/v/wallhaven?logo=rust&logoColor=a9b1d6&style=flat-square&color=fc8d62" height="20">](https://crates.io/crates/wallhaven)
<div align="center">
  <h3>
    A <a href ="https://wallhaven.cc" > WallHaven </a> API CLI client for getting raw/json response and downloading wallpapers, according
    to preferences, with API key support to use account preferences and access NSFW wallpapers.
 </h3>
</div>


## Usage
#### Search wallpaper by query and extra parameters
```sh
wallhaven search --query "+cat +funny" -s VIEWS -c 110 --atleast 1920x1080 --path ~/wallpapers/
```

#### Search wallpaper by query and extra parameters and save wallpapers
```sh
wallhaven search --query "anime +funny" -s VIEWS --atleast 1920x1080 --path ~/wallpapers/
```

#### Search wallpaper by color
```sh
wallhaven search --colors 722f37 
```

#### Search random wallpaper
```sh
wallhaven search --query "" -s RANDOM
```

#### Search random wallpaper with seed
```sh
wallhaven search --query "" -s RANDOM --seed YmdCUP
```

#### Search with api key
```sh
# One time api access with api key
WALLHAVEN_API_KEY="your_api_key" wallhaven search --query "anime +cats" --purity 111 -s TOPLIST --path ~/wallpapers/
# Or
# Use api key always
export WALLHAVEN_API_KEY="your_api_key"
wallhaven search --query "anime +cats" --path ~/wallpapers/
```

#### Get Wallpaper information by id
```sh
wallhaven wallpaper-info 856dlk
```

#### Get Tag info by id
```sh
wallhaven tag-info 15
```

#### Get User Settings **Requires API key**
```sh
wallhaven user-settings
```

#### Get User Settings
```sh
# Get your own collections ** requieres API key **
wallhaven user-collections

# Get user public collections
wallhaven user-collections --username "some_username"
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
- Async
- Download wallpapers
- Download progress bar


## Notes
- Suggestions and bugs to **dax99993@gmail.com**,

## License
[MIT](https://mit-license.org/)
