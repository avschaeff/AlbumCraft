# Colour icons

The icons of the Bright style (`src/color_icons.rs`), drawn instead of the Material Symbols
glyphs of `src/icons.rs` where one is listed here. From Microsoft's
[Fluent UI System Icons](https://github.com/microsoft/fluentui-system-icons) (MIT, see
`LICENSE`), taken from the Iconify sets `fluent-color` (version 1.1.346) and `fluent`, 24 px.

| File | Fluent icon | For |
|---|---|---|
| add.svg | Color `add-circle` | `ADD` |
| album.svg | Color `book` | `ALBUM` |
| auto-fix.svg | Color `bot-sparkle` | `AUTO_FIX` |
| calendar.svg | Color `calendar` | `CALENDAR` |
| collage.svg | Color `board` | `COLLAGE` |
| desktop.svg | Color `laptop` | `DESKTOP` |
| drive.svg | Color `arrow-square-down` | `DRIVE` (Import) |
| edit.svg | Color `edit` | `EDIT` |
| export.svg | Color `share-ios` | `EXPORT` |
| flag.svg | Color `flag` | `FLAG` |
| folder.svg | Color `document-folder` | `FOLDER` |
| location.svg | Color `location-ripple` | `LOCATION` |
| mail.svg | Color `mail` | `MAIL` |
| movie.svg | Color `video` | `MOVIE` |
| music.svg | Color `headphones` | `MUSIC` |
| palette.svg | Color `design-ideas` | `PALETTE` |
| person.svg | Color `person` | `PERSON` |
| photos.svg | Color `image` | `PHOTOS` |
| search.svg | Color `search-sparkle` | `SEARCH` |
| star.svg | Color `star` | `STAR` |
| sunny.svg | Color `weather-sunny-low` | `WB_SUNNY` |
| tree.svg | Color `org` | `TREE` |
| web.svg | Color `globe` | `WEB` |
| info.svg | `info-24-filled`, blue gradient added | `INFO` |
| map.svg | `map-24-filled`, green gradient added | `MAP` |
| play.svg | `play-circle-24-filled`, green gradient added | `PLAY` |
| print.svg | `print-24-filled`, blue gradient added | `PRINT` |
| tag.svg | `tag-24-filled`, orange gradient added | `TAG` |

The colour set has no print, tag, map, play or info icon: those five are the plain
(one-colour) Fluent icons with `fill="currentColor"` replaced by a two-stop vertical gradient in
the colour set's tones. To add an icon, take its SVG from the same sets (an Iconify icon's
`body` inside `<svg viewBox="0 0 24 24">`), add the file here and a line to `color_icons::SVG`.
