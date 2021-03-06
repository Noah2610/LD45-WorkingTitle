<h1 align="center" bgcolor="black" cell>
  <img
   src="./thumbnail.png"
   alt="Working Title"
   width="500px" />
</h1>

Our [Ludum Dare 45 game jam entry][ludumdare].
The theme...
> Start with nothing

---

<details>
<summary>
<strong>
Table of Contents
</strong>
</summary>

- [Screenshots](#screenshots)
- [Description](#description)
- [Controls](#controls)
  - [Ingame](#ingame)
  - [Main Menu](#main-menu)
- [Downloads](#downloads)
- [Data Directory](#data-directory)
- [Notes](#notes)
  - [Requires Vulkan support to run!](#requires-vulkan-support-to-run)
- [Change-Log](#change-log)
  - [Jam Versions](#jam-versions)
  - [Post-Jam Versions](#post-jam-versions)
    - [Change-Log v1.1.0](#change-log-v110)
    - [Change-Log v1.1.1](#change-log-v111)
    - [Change-Log v1.1.2](#change-log-v112)
    - [Change-Log v1.1.3](#change-log-v113)
    - [Change-Log v1.2.0](#change-log-v120)
    - [Change-Log v1.3.0](#change-log-v130)
- [Fonts Used](#fonts-used)
- [Development](#development)
  - [Credits](#credits)
  - [Tools Used](#tools-used)
  - [Compiling from source](#compiling-from-source)
  - [Making levels](#making-levels)
- [License](#license)
</details>

## Screenshots
<p align="center">
  <img
   width="400px"
   src="./screenshots/mid.png"
   alt="Screenshot of the middle section of the level" />
  <img
   width="400px"
   src="./screenshots/end.png"
   alt="Screenshot of the ending section of the level" />
</p>

## Description
_Experience a work in progress._  

Play through the development process of a 2D platformer game,  
and watch your environment transform as you progress...

The further you get, the more features are added, including  
menacing enemies, destructive spikes, and adaptive music.

## Controls
### Ingame
| Action                         | Keyboard                                     |
| :----------------------------- | :------------------------------------------- |
| Move                           | `W`,`S`,`A`,`D` / `Up`,`Down`,`Left`,`Right` |
| Jump                           | `Space` / `K`                                |
| Toggle Pause                   | `Escape` / `P`                               |
| To Main Menu (from pause menu) | `Q` / `Backspace`                            |
| Quit Game                      | `Shift+Escape` / `Shift+Q`                   |

### Main Menu
| Action                             | Keyboard                       |
| :--------------------------------- | :----------------------------- |
| Start Selected                     | `Enter` / `Space`              |
| Start Selected (without save data) | `X` / `Delete`                 |
| Select Next                        | `S` / `Down` / `J` / `Tab`     |
| Select Previous                    | `W` / `Up` / `K` / `Shift+Tab` |
| Quit Game                          | `Escape` / `Shift+Q`           |

## Downloads
Download the build for your platform from [itch.io][itch].  
Supported platforms: __Windows__ and __Linux__.

## Data Directory
The game's data directory is where the `savefile.json`,  
and the `panic.log` (if an error should occur) files will be saved to.  
It's location depends on the platform:

| Platform    | Path                                                |
| ----------: | :-------------------------------------------------- |
| __Windows__ | `C:\Users\<USER>\AppData\Local\ld45-working-title\` |
| __Linux__   | `$HOME/.local/share/ld45-working-title/`            |

Location is determined by the [`dirs` crate's `data_local_dir()` function](https://docs.rs/dirs/2.0.2/dirs/fn.data_local_dir.html).

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

## Change-Log
### Jam Versions
<details>
<summary>
    <strong>v1.0.0 - v1.0.2</strong>
</summary>

| Version | Download | Release Date | Notes |
| :-----: | :------: | :----------: | :---- |
| __1.0.0__ | [Windows][v1.0.0-windows]<br />[Linux][v1.0.0-linux] | 2019-10-08<br />_(jam deadline)_               | Initial Ludum Dare jam release. <br /> Has some major bugs, which prevent the game from being completed: <br /> When dying and respawning at a checkpoint, after getting the high-jump feature <br /> you lose your high-jump, which prevents the game from being completed <br /> (unless you beat it without dying). <br /> The order in which the music tracks are played are messed-up; <br /> you won't experience the intended flow of the music in this build. |
| __1.0.1__ | [Windows][v1.0.1-windows]<br />[Linux][v1.0.1-linux] | 2019-10-08<br />_(~20 minutes after deadline)_ | Partially fixes the music playing-order bug from v1.0.0 (still has some bugs). <br /> Was released about 20 minutes after the deadline. |
| __1.0.2__ | [Windows][v1.0.2-windows]<br />[Linux][v1.0.2-linux] | 2019-10-08<br />_(~5 hours after deadline)_    | Properly fixes the music playing-order bug. <br /> Fixes the bug where player's high-jump is taken away after respawning, making the game beatable. <br /> Adds frame rate counter system, which prints FPS to the console. <br /> Adds the `WorkingTitle.desktop` file for Linux file manager GUIs. <br /> Some minor, meta stuff. <br /> Was released about 5 hours after the deadline. |
</details>

### Post-Jam Versions
<details>
<summary>
    <strong>v1.1.0</strong>
</summary>

| Download | Release Date |
| :------: | :----------: |
| [Windows][v1.1.0-windows] / [Linux][v1.1.0-linux] | 2019-10-30 |

#### Change-Log v1.1.0
- Change settings to be more low-gravity and feel more like space.
- Add a hover mechanic, where the player can hold down the jump button  
  to slowly hover downwards for the space-y gravity sections.
  - [#14 Add proper hover](https://github.com/Noah2610/LD45-WorkingTitle/issues/14)
- Add four proper levels, each for a different difficulty, from _easy_ to _absurd_.  
  Starts with the _easy_ level and goes to the next difficulty when the level is beaten.
- Add savefile system.  
  When you hit a checkpoint your progress is saved to a savefile,  
  which is loaded when you start the game.
  - [#17 Persistent savefile](https://github.com/Noah2610/LD45-WorkingTitle/issues/17)
- Add red-rectangle ("programmer art") sprites for enemies.
  - [#11 Add no\_sprite animations for enemies](https://github.com/Noah2610/LD45-WorkingTitle/issues/11)
- Adjust animations.
- Bug fixes.
</details>

<details>
<summary>
    <strong>v1.1.1</strong>
</summary>

| Download | Release Date |
| :------: | :----------: |
| [Windows][v1.1.1-windows] / [Linux][v1.1.1-linux] | 2019-10-30 |

#### Change-Log v1.1.1
- Add a player death counter, which is printed to the console at regular intervals.
- Improve performance from previous version.
- Bug fixes, level adjustments, settings tweaks.
</details>

<details>
<summary>
    <strong>v1.1.2</strong>
</summary>

| Download | Release Date |
| :------: | :----------: |
| [Windows][v1.1.2-windows] / [Linux][v1.1.2-linux] | 2019-10-31 |

#### Change-Log v1.1.2
- Update background image, less distracting.
- Level, textures, settings adjustments.
</details>

<details>
<summary>
    <strong>v1.1.3</strong>
</summary>

| Download | Release Date |
| :------: | :----------: |
| [Windows][v1.1.3-windows] / [Linux][v1.1.3-linux] | 2019-11-03 |

#### Change-Log v1.1.3
- Add indicators for when certain features are added (jump, low-gravity).
  - [#16 Add Indicator object, which becomes visible when touched](https://github.com/Noah2610/LD45-WorkingTitle/issues/16)
  - [#18 Add indicator animations](https://github.com/Noah2610/LD45-WorkingTitle/issues/18)
- Fix savefile issues.
- Add pause menu. Toggle with `P` or `Escape`.
  - [#19 Pause menu](https://github.com/Noah2610/LD45-WorkingTitle/issues/19)
- Update levels.
- Add plants tiles.
- Add speedrun timer (kinda). Time is printed to the console.
  - [#12 Speedrun timer](https://github.com/Noah2610/LD45-WorkingTitle/issues/12)
- Animation adjustments.
- Settings tweaks.
</details>

<details>
<summary>
    <strong>v1.2.0</strong>
</summary>

| Download | Release Date |
| :------: | :----------: |
| [Windows][v1.2.0-windows] / [Linux][v1.2.0-linux] | 2019-11-06 |

#### Change-Log v1.2.0
- Add main menu, where you can select the difficulty you want to play.  
  Use _keyboard_ or _mouse_ to select a difficulty.  
  Each difficulty has its own save.
  - [#21 Difficulty select menu](https://github.com/Noah2610/LD45-WorkingTitle/issues/21)
  - [#26 Select difficulty with keyboard in DifficultySelect menu](https://github.com/Noah2610/LD45-WorkingTitle/issues/26)
- Save best level clear times for each level.  
  Display an in-game timer after the level has been beaten once if the timer is running.  
  Display the best time for each level, in-game next to the running timer.
  - [#23 Save best time to savefile](https://github.com/Noah2610/LD45-WorkingTitle/issues/23)
  - [#28 Display timer in-game](https://github.com/Noah2610/LD45-WorkingTitle/issues/28)
  - [#31 Display best time with timer](https://github.com/Noah2610/LD45-WorkingTitle/issues/31)
- Minor paused menu UI changes.
- Reset saved player deaths after beating a level.  
  _(Player deaths are only printed to the console.)_
- The window can be resized by the user.  
  It's recommended to keep it at a 1:1 size ratio (square window).
- Display a different "level complete" text for each level.
- Level changes.
- Tile spritesheets changes.
</details>

<details>
<summary>
    <strong>v1.3.0</strong>
</summary>

| Download | Release Date |
| :------: | :----------: |
| [Windows][v1.3.0-windows] / [Linux][v1.3.0-linux] | 2019-11-24 |

#### Change-Log v1.3.0
- Add dynamic tile animations; animations, that are triggered by the player or enemies.  
  Used in grass/plant tiles, which play a swaying animation when the player collides with them.
  - [#24 Dynamic tile animations](https://github.com/Noah2610/LD45-WorkingTitle/issues/24)
- Use `savefile.json` from the user's data directory.  
  Also saves a `panic.log` file to the directory, in case a crash happens.  
  See the [Data Directory](#data-directory) section.
  - [#35 Use savefile from a user-space directory](https://github.com/Noah2610/LD45-WorkingTitle/issues/35)
- Add key bindings to start the selected difficulty with a new save for the level.  
  Bindings as of v1.3.0: `X`, `Delete`  
  See the [Controls](#controls) section.
  - [#25 Delete save for difficulty from DifficultySelect menu](https://github.com/Noah2610/LD45-WorkingTitle/issues/25)
- Add arrow key bindings for player movement.
- Checkpoints now have fixed respawn positions.  
  This means the player will always spawn in an ideal position,  
  instead of somewhere in the air or even spawning inside a solid tile.
  - [#22 Make player spawn on ground by checkpoints](https://github.com/Noah2610/LD45-WorkingTitle/issues/22)
- Display your best time for the selected level under the button text.
  - [#29 Display best time in DifficultySelect state](https://github.com/Noah2610/LD45-WorkingTitle/issues/29)
- Rename level difficulties, as shown ingame.  
  For development the old code names are still used.  
  __Code names to ingame names table:__  
  ```
  CODE NAME -> INGAME/DISPLAYED NAME
  VeryEasy  -> "Easy"
  Easy      -> "Normal"
  Normal    -> "Advanced"
  Hard      -> "Hard"
  Absurd    -> "Absurd"
  ```
- Add a new `VeryEasy` level (now called _"Easy"_ ingame).  
  Intended to showcase the game's features, without too much challenge.
  - [#38 Add "Very Easy" difficulty button to the main menu](https://github.com/Noah2610/LD45-WorkingTitle/issues/38)
- Add difficulty descriptions to the main menu.  
  A description of the selected level is shown on the right side of the main menu.
  - [#37 Display a description of the selected level in DifficultySelect menu](https://github.com/Noah2610/LD45-WorkingTitle/issues/37)
- Lock levels. Unlock a locked level by beating its prior difficulty.  
  Levels `VeryEasy` and `Easy` start unlocked, the others start locked.  
  Add an alternative locked description for each locked level.
  - [#40 Make harder difficulties unlockable by beating previous difficulties](https://github.com/Noah2610/LD45-WorkingTitle/issues/40)
- Level changes.
- UI changes.
- Show version number in the bottom-right corner of the main menu.
- Camera tries to scale to the window size.
- Print welcome message to stdout when the game starts.
- Add completion text. Beat the game to see it :)
- Fix multi-tile animations de-sync.
  - [#41 Add always_loaded property to multi-tile animations in levels](https://github.com/Noah2610/LD45-WorkingTitle/issues/41)
</details>

[v1.0.0-windows]: https://drive.google.com/open?id=1KVx1OpiFyv8DIjm8x0AYwkH_wrwRz_BT
[v1.0.0-linux]:   https://drive.google.com/open?id=1ELAu_Xnh_CRx41qACSeJjiExs5I5Pziu
[v1.0.1-windows]: https://drive.google.com/open?id=1trgy3J-jKcHcjwICZeNqIR1PuSDjjDnb
[v1.0.1-linux]:   https://drive.google.com/open?id=1IqgaudYNlWc_npcTbaSceo0xReDhQn6W
[v1.0.2-windows]: https://drive.google.com/open?id=1thVKQmxqulxDKRW7RW2tMQMt2Cej52VV
[v1.0.2-linux]:   https://drive.google.com/open?id=1aX_bZkpaJTXFlQlgM1Pz4fQY9uryUDYo
[v1.1.0-windows]: https://drive.google.com/open?id=1e1rDkDrPYHpggYph-z2fC0DrMuo5iMKb
[v1.1.0-linux]:   https://drive.google.com/open?id=1yK2RCumiGPkA8FDCibbvzTvcPD_RuYxR
[v1.1.1-windows]: https://drive.google.com/open?id=1BPOKnfXPQITsAkGithX__c1WKvDGlxSQ
[v1.1.1-linux]:   https://drive.google.com/open?id=1Ksz0DiKAGJg3-fNFeLiEggzTc0qUD6o_
[v1.1.2-windows]: https://drive.google.com/open?id=1BjWpFp7PCYf_YvrAeTaLaUFa1a4evrLJ
[v1.1.2-linux]:   https://drive.google.com/open?id=1aiA-f2wL2Wqwd5ugmXM4IJbAxj8MEZJA
[v1.1.3-windows]: https://drive.google.com/open?id=1ekcwnutBPTs11RE8UHmipn-hcf4Tr5Ib
[v1.1.3-linux]:   https://drive.google.com/open?id=1XQGDSQPD2HFGayEryASPSKbUgeILJCqa
[v1.2.0-windows]: https://drive.google.com/open?id=1TtbqwltL4EWI-jgrt7kvdZZrJ_hP7ni0
[v1.2.0-linux]:   https://drive.google.com/open?id=1thHtNAEytx8kSb1vDJlDXYcwnMwpB_4C
[v1.3.0-windows]: https://drive.google.com/open?id=17goo1o3Br7hcfY9VGDnaDwdjBi2v0NSn
[v1.3.0-linux]:   https://drive.google.com/open?id=1klspsvuQraEtA8kz7s8xxC3sQYlrZ8ge

## Fonts Used
Used [undefined-medium] for the text at the end.

## Development
[![Build Status][Travis-CI-SVG]][Travis-CI]

### Credits

| User       | Role                                   |
| :--------- | :------------------------------------- |
| [dimling]  | Music, character sprites               |
| [hoichael] | Level design, tile/background graphics |
| [Noah2610] | Programming                            |

[dimling]:  https://github.com/dimling
[hoichael]: https://github.com/hoichael
[Noah2610]: https://github.com/Noah2610

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
