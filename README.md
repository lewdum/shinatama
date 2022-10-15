<img src="https://wiki.oni2.net/w/images/0/05/TXMPSHINlistening.png" align="left" width="160" style="margin-right: 1em"/>

# Shinatama

A set of useful (*I hope*) patches for **Bungie**'s game [Oni] (2001).

Includes several fixes for the [BungieFrameWork Scripting Language] (BSL).

[Oni]: https://wiki.oni2.net/Main_Page

[BungieFrameWork Scripting Language]: https://wiki.oni2.net/BSL:BFW_Scripting_Language


## Getting started

1. Download `dinput.dll` and `shina.ini` (or an equivalent archive) from the [latest release].
1. Drop both files in your [Anniversary Edition] (AE) directory (**NOT** Oni's base directory).
1. Edit `shina.ini` according to your needs.
1. Start the game.

[latest release]: https://github.com/lewdum/shinatama/releases/latest

[Anniversary Edition]: https://wiki.oni2.net/Anniversary_Edition

[default configuration]: /assets/shina.toml


## Building from source

This project can be built normally with [Cargo]. You must target i686 to interface with Oni:

```
cargo build --target i686-pc-windows-msvc
```

The [build] [scripts] take care of this, but you must run them from the root of the project, and the root itself must be at a folder two levels deeper than the AE, e.g. `Oni/AE/projects/shinatama`. In debug mode, Shinatama automatically creates a console when the game starts, and prints debug messages to it.

[Cargo]: https://doc.rust-lang.org/stable/cargo/

[build]: /scripts/debug.bat

[scripts]: /scripts/release.bat


## The patches

All patches are summarized in the [default configuration] as well.

[default configuration]: /assets/shina.toml

#### `fix_bsl`

**Fix 3 scoping-related bugs in BSL.**

They are documented [here].

*This patch is enabled by default.*

[here]: https://lewdum.notion.site/Assorted-BSL-Fixes-13263d6ca8e84538829a07ad4d318085

#### `two_guns`

**Allow carrying two guns at the same time.**

To pick up a second gun, holster the first one.

#### `keep_guns`

**Prevent guns from despawning when left on the floor.**

Whether this leads to an overflow eventually is unknown at this time. :)

#### `manual_reload`

**Prevent guns from reloading automatically when out of ammo.**

Trying to shoot an empty gun will no longer reload it for you.

#### `hypo_anytime`

**Allow using a Hypo Spray even at full health.**

#### `unlock_doors`

**Unlock all doors, in all levels, at all times.**

Sets a debug flag left in the game by Bungie.

#### `always_dev`

**Always enable [Developer Access] (cheat `x`).**

*This patch is enabled by default.*

[Developer Access]: https://wiki.oni2.net/Developer_Mode

#### `fast_cutscenes`

**Speed up cutscenes by a lot.**

Results in overlapping dialogue, but this could be addressed in the future.

#### `no_black_bars`

**Remove the black bars that appear during cutscenes.**

#### `shut_up`

**Remove debug messages from certain [Daodan] functions.**

In particular, `d_waitforkey` and its variants.

*This patch is enabled by default.*

[Daodan]: https://wiki.oni2.net/Daodan_DLL


## Experimental

The following patches are work in progress.

#### `three_guns` (since 0.1.2)

**Allow carrying yet another gun.**

Oni has a special weapon slot called a "cinematic holster". It is used,
for example, in the opening cutscene of the Syndicate Warehouse (Trial Run),
where it stores the Campbell Equalizer that Konoko will unholster when the
cutscene ends.

This patch repurposes that slot, allowing Konoko to switch between all three of
them freely. As a convenience, until she has at least two guns holstered, the
third slot won't be cycled to. This patch also aims to keep compatibility with
all cutscenes.

Inspired by [this promotional image].

[this promotional image]: https://wiki.oni2.net/Fully_Armed_Konoko
