# maidata-rs

Parses [the `maidata.txt` format][format] of the [simai] application, popular
in the maimai mapping community.

解析 [simai] 的 [`maidata.txt` 文件格式][format]的库。simai 是 maimai 自制谱谱师群体所常用的工具。

[simai]: https://w.atwiki.jp/simai/
[format]: https://w.atwiki.jp/simai/pages/25.html

Currently very much WIP, expect a lot of breakages. Don't use in production.

项目目前处于很早的阶段，尚未施工完成，不能保证兼容，不要用。

## Supported `maidata.txt` features

General format features:

* [x] basic metadata fields
* [ ] comments `||xxx\n`
* [ ] escape sequences `\＆ \＋ \％ \￥`
* [ ] active message fields

Map definition instructions:

* [x] BPM spec `(float)`
* [x] beat divisor spec `{4}`
* [ ] end mark `E`
* [x] TAP `B,`
    - [x] simplified BOTH/EACH TAP form (`16` `38` etc.; `123` and such are also allowed)
    - [x] BREAK modifier `Bb,`
    - [ ] star-shape modifier `B$,` `Bb$, B$b,` `B$$,`
* [x] HOLD `Bh[length],`
    - [x] normal duration spec `[x:y]`
    - [ ] absolute duration spec `[#float]`
* [x] SLIDE `FxE[length],`
    - [x] all track shapes `- ^ < > v p q s z pp qq V w`
    - [x] multiple tracks sharing one start `1-3[4:1]*-4[4:1]`
    - [ ] absolute duration specs
        - `[#1.5]`
        - `[160#2]`
        - `[3##1.5]`
* [x] BOTH/EACH `note/note,`
    - [x] arbitrary number of concurrent notes allowed (3simai)

maimai DX (3simai) features are largely currently not implemented.
