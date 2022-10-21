# rtk-cli

Various tools for use with [Return to Krondor](https://en.wikipedia.org/wiki/Return_to_Krondor) data files.

## Usage

```
Usage: rtk-cli [COMMAND]

Commands:
  compress
  decompress
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

### Decompress

```
Usage: rtk-cli decompress --input <FILE> --output <FILE>

Options:
  -i, --input <FILE>
  -o, --output <FILE>
  -h, --help           Print help information
```

```bash
rtk-cli decompress -i RtkGame.def -o RtkGame.ascii.def
```

### Compress

```
Usage: rtk-cli compress --input <FILE> --output <FILE>

Options:
  -i, --input <FILE>
  -o, --output <FILE>
  -h, --help           Print help information
```

```bash
rtk-cli compress -i RtkGame.ascii.def -o RtkGame.def
```

## Supported Files

Works with files that start with `(c) 1998 PyroTechnix,Inc.` including save and game data.

```
SIERRA\RtK\Books\*\*.rtk
SIERRA\RtK\GameData\Chapter0\Chapter0.def
SIERRA\RtK\GameData\Chapter0\Loc_All.def
SIERRA\RtK\GameData\Chapter1\Chapter1.def
SIERRA\RtK\GameData\Chapter1\Loc_All.def
SIERRA\RtK\GameData\Chapter10\Chapter10.def
SIERRA\RtK\GameData\Chapter10\Loc_All.def
SIERRA\RtK\GameData\Chapter2\Chapter2.def
SIERRA\RtK\GameData\Chapter2\Loc_All.def
SIERRA\RtK\GameData\Chapter3\Chapter3.def
SIERRA\RtK\GameData\Chapter3\Loc_All.def
SIERRA\RtK\GameData\Chapter4\Chapter4.def
SIERRA\RtK\GameData\Chapter4\Loc_All.def
SIERRA\RtK\GameData\Chapter5\Chapter5.def
SIERRA\RtK\GameData\Chapter5\Loc_All.def
SIERRA\RtK\GameData\Chapter6\Chapter6.def
SIERRA\RtK\GameData\Chapter6\Loc_All.def
SIERRA\RtK\GameData\Chapter7\Chapter7.def
SIERRA\RtK\GameData\Chapter7\Loc_All.def
SIERRA\RtK\GameData\Chapter8\Chapter8.def
SIERRA\RtK\GameData\Chapter8\Loc_All.def
SIERRA\RtK\GameData\Chapter9\Chapter9.def
SIERRA\RtK\GameData\Chapter9\Loc_All.def
SIERRA\RtK\GameData\ChapterEvents.tbl
SIERRA\RtK\GameData\Chars.tbl
SIERRA\RtK\GameData\ConversationTracks.tbl
SIERRA\RtK\GameData\HaldonShops.def
SIERRA\RtK\GameData\InventoryItemScript.txt
SIERRA\RtK\GameData\ItemsShop.tbl
SIERRA\RtK\GameData\KrondorShops.def
SIERRA\RtK\GameData\MagicInvItem.txt
SIERRA\RtK\GameData\MagicResult.txt
SIERRA\RtK\GameData\Models.def
SIERRA\RtK\GameData\RtkGame.def
```

## Compressed File Format

![kaitai](https://user-images.githubusercontent.com/913141/197020809-3a562916-5d2e-4cbb-8761-6be4e075ddc8.png)

Files use the signature `(c) 1998 PyroTechnix,Inc.` followed by a [Substitute character](https://en.wikipedia.org/wiki/Substitute_character).

This can also be repesented in hexadecimal as `28 63 29 20 31 39 39 38 20 50 79 72 6f 54 65 63 68 6e 69 78 2c 49 6e 63 2e 1a`

The remainder of the file is [gzip](https://en.wikipedia.org/wiki/Gzip) compressed data.

A simple [Kaitai Struct](https://kaitai.io/) can be seen [here](rtkgz.ksy).

## Other files

`*.t3d` files can be extracted using https://wld-doc.github.io/formats/t3d_fast_file#js-parser
