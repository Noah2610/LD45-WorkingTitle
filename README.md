<h1 align="center" bgcolor="black" cell>
  <img
   src="./thumbnail.png"
   alt="Working Title"
   width="500px" />
</h1>

Our [Ludum Dare 45 game jam entry][ludumdare].
The theme...
> Start with nothing

## Screenshots
__TODO!__

## Description
_Experience a work in progress._  

Play through the development process of a 2D platformer game,  
and watch your environment transform as you progress...

## Controls
| Action | Keyboard        |
| :----- | :-------------- |
| Move   | `W`,`S`,`A`,`D` |
| Jump   | `Space` / `K`   |

## Downloads
Download the build for your platform from [itch.io][itch].  
Supported platforms: __Windows__ and __Linux__.

## Notes
### Requires Vulkan support to run!
If the game doesn't run, it's likely that you're missing Vulkan API support.  
Unless you're using a very old graphics card,  
updating your graphics drivers should fix the issue.  

If you're on Linux, you may need to install an appropriate  
Vulkan driver package for your graphics card.  
For Intel graphics on Arch Linux (or arch-based distros),  
the package [vulkan-intel] did the trick for me.  

Sorry about this inconvenience, I'm not sure how to circumvent this.

## Fonts Used
Used [undefined-medium] for the text at the end.

## Development
[![Build Status][Travis-CI-SVG]][Travis-CI]

### Tools Used
- __[Rust]__, programming language
- __[Amethyst]__, engine
- __[Deathframe]__, framework
- __[Vim]__, code editor
- __[GitHub]__, git repository hosting
- __[Gimp]__, tile/background graphics
- __[Aseprite]__, sprite graphics
- __[FL Studio][FLStudio]__, music
- __[Tiled]__, level design
- __[Travis-CI]__, Windows/Linux executable building

### Compiling from source
You need to have `cargo` installed (preferably `rustup`).  
Run `cargo run --release` to build and run the game with your default toolchain.  
To run the development build with some debug stuff enabled,  
run the provided `bin/run` script; you'll need the rust toolchain  
`nightly-2019-08-13` installed for the development run script.

### Making levels
We use [Tiled] to create levels.  
You'll need to install [Tiled] and [Python].  
We use a custom Tiled export script, which requires Python.  
To use the custom export script, do the following:  
- Enable the python plugin (`Edit -> Preferences -> Plugins`)
- create a new directory under `~/.tiled`  
  `mkdir -p ~/.tiled`
- copy the export script `tiled-export-script.py` to the new directory  
  `cp tiled-export-script.py ~/.tiled/working-title-export.py`
- restart Tiled

The export script should now appear in the drop-down menu when exporting a level in Tiled.

To use the level in-game, copy the exported `.json` level file to  
`resources/levels/level.json`, and copy all `*.ron` files to  
`resources/spritesheets/`, associated with the spritesheet `.png` files used.  
You can use any spritesheets, as long as the `.png` and `.ron` files are  
in the `resources/spritesheets` directory.

The file [DESIGN.md] describes all tile/object properties and object types  
you can use in Tiled, which have meaning in the game.

## License
Licensed under the terms of the [MIT license][mit].

[ludumdare]:        https://ldjam.com/events/ludum-dare/45/working-title
[itch]:             https://noahro.itch.io/working-title
[vulkan-intel]:     https://www.archlinux.org/packages/extra/x86_64/vulkan-intel/
[undefined-medium]: https://github.com/andirueckel/undefined-medium
[mit]:              https://github.com/Noah2610/LD45/blob/master/LICENSE
[DESIGN.md]:        https://github.com/Noah2610/LD45/blob/master/DESIGN.md
[Rust]:             https://www.rust-lang.org/
[Amethyst]:         https://amethyst.rs/
[Deathframe]:       https://github.com/Noah2610/deathframe
[Vim]:              https://www.vim.org/
[GitHub]:           https://github.com/Noah2610/LD45
[Gimp]:             https://www.gimp.org/
[Aseprite]:         https://www.aseprite.org/
[FLStudio]:         https://www.image-line.com/flstudio
[Tiled]:            https://www.mapeditor.org/
[Travis-CI]:        https://travis-ci.org/Noah2610/LD45-WorkingTitle
[Travis-CI-SVG]:    https://travis-ci.org/Noah2610/LD45-WorkingTitle.svg?branch=master
[Python]:           https://www.python.org/
