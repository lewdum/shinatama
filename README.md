<img src="https://wiki.oni2.net/w/images/0/05/TXMPSHINlistening.png" align="left" width="160" style="margin-right: 1em"/>

# Shinatama

A set of useful (*I hope*) patches for **Bungie**'s game [Oni] (2001).

Includes several fixes for the [BungieFrameWork Scripting Language] (BSL).

[Oni]: https://wiki.oni2.net/Main_Page

[BungieFrameWork Scripting Language]: https://wiki.oni2.net/BSL:BFW_Scripting_Language


## Getting Started

1. Download `dinput.dll` and `shina.ini` (or an equivalent archive) from the [latest release].
1. Drop both files in your [Anniversary Edition] (AE) directory (**NOT** Oni's base directory).
1. Edit `shina.ini` according to your needs.
1. Start the game.

[latest release]: https://github.com/lewdum/shinatama/releases/latest

[Anniversary Edition]: https://wiki.oni2.net/Anniversary_Edition


### Building

This project can be built normally with [Rust]. You must target i686 (i.e. x86) to interface with Oni.

*In debug mode, Shinatama automatically creates a console and prints messages to it.*

[Rust]: https://www.rust-lang.org/


## Patches

#### `fix_bsl`

Fix 3 bugs in BSL.

They are documented [here].

*This patch is enabled by default.*

[here]: https://lewdum.notion.site/Assorted-BSL-Fixes-13263d6ca8e84538829a07ad4d318085

#### `two_guns`

Allow carrying two guns at the same time.

To pick up a second gun, holster the first one.

#### `keep_guns`

Prevent guns from despawning when left on the floor.

Whether this leads to an overflow eventually is unknown at this time. :)

#### `manual_reload`

Prevent guns from reloading automatically when out of ammo.

Trying to shoot will no longer reload a gun for you.

#### `hypo_anytime`

Allow using a Hypo Spray even at full health.

#### `unlock_doors`

Unlock all doors, in all levels, at all times.

Sets a debug flag left in the game by Bungie.

#### `always_dev`

Always enable [Developer Access] (cheat `x`).

*This patch is enabled by default.*

[Developer Access]: https://wiki.oni2.net/Developer_Mode

#### `fast_cutscenes`

Speed up cutscenes by a lot.

Results in overlapping dialogue, but this could be addressed in the future.

#### `no_black_bars`

Remove the black bars that appear during cutscenes.

#### `shut_up`

Remove debug messages from certain [Daodan] functions.

In particular, `d_waitforkey` and its variants.

*This patch is enabled by default.*

[Daodan]: https://wiki.oni2.net/Daodan_DLL
