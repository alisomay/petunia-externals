# rytm external API docs

## How to read this documentation?

Words in these wrapped with `<..>` should be read as placeholders for the actual values.

For example `<selector>` should be read as `query`, `send`, `get` or `set`.

Words in these wrapped with `[]` should be read as optional values.

For example `[<parameter>]` should be read as `<parameter>` or nothing.

Words in these wrapped with `<..>` may include attached range of values to denote the valid range of an index or a parameter.

For example `<index 0..=127>` should be read as an integer between 0 and 127.

Ranges can be inclusive or exclusive. An exclusive range is denoted with `..` and an inclusive range is denoted with `..=`.

MaxMSP types will be denoted as `int`, `float`, `symbol` and `list`.

When using the following identifiers and enums with some getters and setters there could be format differences.

For these, please follow common sense and read the errors in the max window, they will guide you to the right direction.

- `<selector>` A symbol which defines an operation. Available selectors are:
  - `query` Queries the device for data.
  - `send` Sends data to the device.
  - `get` Gets data from the `rytm` external.
  - `set` Sets data to the `rytm` external.
- `<object-type>` A symbol which defines the type of the object.
  - `pattern` A pattern.
  - `kit` A kit.
  - `sound` A pool sound.
  - `global` A global setting.
  - `settings` A settings structure.
  - `pattern_wb` The pattern from the work buffer.
  - `kit_wb` The kit from the work buffer.
  - `sound_wb` A sound from the work buffer.
  - `global_wb` The global setting from the work buffer.
- `<identifier>` A symbol which defines a parameter name. Usually followed by a `<parameter>` when setting data.
- `<enum>` A symbol which defines an enumeration. Enumerations are expressed in the format of `<enum-type>:` or `<enum-type>:<enum-value>`.
- `<parameter>` An integer or float parameter.
- `<element>` A sub element of a type.
- `<index>` An integer index.

## Query format

The query format is used to query data from the device.

`query <object-type> [<index>]`

Indexable object types are:

- `pattern`
- `kit`
- `sound`
- `sound_wb`
- `global`

Examples:

- `query pattern 1`
- `query settings`
- `query sound_wb 0`
- `query global_wb`

## Send format

The send format is used to send data to the device.

`send <object-type> [<index>]`

Examples:

- `send pattern 1`
- `send settings`
- `send sound_wb 0`
- `send global_wb`

## Get format

The get format is used to get data from the `rytm` external.

### Output

The default output format for identifier getters:

`<object-index> <identifier-type> <parameter>`

The default output format for enum getters:

`<object-index> <enum-type> <enum-value>`

For some getters an additional parent index is included,

Track getter output format:

`<pattern-index> <track-index> ..`

Trig getter output format:

`<track-index> <trig-index> ..`

For getters which includes an additional element the index of that element is included,

Kit getter with element output format:

`<kit-index> <element-index> ..`

### Pattern

Accepted formats:

- `get pattern <index 0..=127> <identifier>`
- `get pattern <index 0..=127> <enum>`
- `get pattern <index 0..=127> <track-index 0..=12> <identifier>`
- `get pattern <index 0..=127> <track-index 0..=12> <enum>`
- `get pattern <index 0..=127> <track-index 0..=12> <trig-index 0..=63> <identifier>`
- `get pattern <index 0..=127> <track-index 0..=12> <trig-index 0..=63> <enum>`
- `get pattern <index 0..=127> <track-index 0..=12> <trig-index 0..=63> plockget <identifier>`
- `get pattern <index 0..=127> <track-index 0..=12> <trig-index 0..=63> plockget <enum>`

### Pattern work buffer

Accepted formats:

- `get pattern_wb <identifier>`
- `get pattern_wb <enum>`
- `get pattern_wb <track-index 0..=12> <identifier>`
- `get pattern_wb <track-index 0..=12> <enum>`
- `get pattern_wb <track-index 0..=12> <trig-index 0..=63> <identifier>`
- `get pattern_wb <track-index 0..=12> <trig-index 0..=63> <enum>`
- `get pattern_wb <track-index 0..=12> <trig-index 0..=63> plockget <identifier>`
- `get pattern_wb <track-index 0..=12> <trig-index 0..=63> plockget <enum>`

### Kit

Accepted formats:

- `get kit <index 0..=127> <identifier>`
- `get kit <index 0..=127> <enum>`
- `get kit <index 0..=127> <element> <element-index>`
- `get kit <index 0..=127> sound <sound-index 0..=11> <identifier> [<parameter>]`
- `get kit <index 0..=127> sound <sound-index 0..=11> <enum> [<parameter>]`

### Kit work buffer

Accepted formats:

- `get kit_wb <identifier>`
- `get kit_wb <enum>`
- `get kit_wb <element> <element-index>`
- `get kit_wb sound <sound-index 0..=11> <identifier> [<parameter>]`
- `get kit_wb sound <sound-index 0..=11> <enum> [<parameter>]`

Accepted kit elements (`<element>`)

- `tracklevel`
- `trackretrigrate`
- `trackretriglen`
- `trackretrigveloffset`
- `trackretrigalwayson`
- `sound`

### Sound

Accepted formats:

- `get sound <index 0..=127> <identifier> [<parameter>]`
- `get sound <index 0..=127> <enum> [<parameter>]`

### Sound work buffer

Accepted formats:

- `get sound_wb <index 0..=11> <identifier> [<parameter>]`
- `get sound_wb <index 0..=11> <enum> [<parameter>]`

### Global

Accepted formats:

- `get global <index 0..=3> <identifier> [<parameter>]`
- `get global <index 0..=3> <enum> [<parameter>]`

### Global work buffer

Accepted formats:

- `get global_wb <identifier> [<parameter>]`
- `get global_wb <enum>`

### Settings

Accepted formats:

- `get settings <identifier> [<parameter>]`
- `get settings <enum>`

## Set format

The set format is used to send data to the `rytm` external.

### Pattern

Accepted formats:

- `set pattern <index 0..=127> <identifier> <parameter>`
- `set pattern <index 0..=127> <enum>`
- `set pattern <index 0..=127> <track-index 0..=12> <identifier> <parameter>`
- `set pattern <index 0..=127> <track-index 0..=12> <enum>`
- `set pattern <index 0..=127> <track-index 0..=12> <trig-index 0..=63> <identifier> <parameter>`
- `set pattern <index 0..=127> <track-index 0..=12> <trig-index 0..=63> <enum>`
- `set pattern <index 0..=127> <track-index 0..=12> <trig-index 0..=63> plockset <identifier> <parameter>`
- `set pattern <index 0..=127> <track-index 0..=12> <trig-index 0..=63> plockset <enum>`
- `set pattern <index 0..=127> <track-index 0..=12> <trig-index 0..=63> plockclear <identifier>`
- `set pattern <index 0..=127> <track-index 0..=12> <trig-index 0..=63> plockclear <enum>`

### Pattern work buffer

Accepted formats:

- `set pattern_wb <identifier> <parameter>`
- `set pattern_wb <enum>`
- `set pattern_wb <track-index 0..=12> <identifier> <parameter>`
- `set pattern_wb <track-index 0..=12> <enum>`
- `set pattern_wb <track-index 0..=12> <trig-index 0..=63> <identifier> <parameter>`
- `set pattern_wb <track-index 0..=12> <trig-index 0..=63> <enum>`
- `set pattern_wb <track-index 0..=12> <trig-index 0..=63> plockset <identifier> <parameter>`
- `set pattern_wb <track-index 0..=12> <trig-index 0..=63> plockset <enum>`
- `set pattern_wb <track-index 0..=12> <trig-index 0..=63> plockclear <identifier>`
- `set pattern_wb <track-index 0..=12> <trig-index 0..=63> plockclear <enum>`

### Kit

Accepted formats:

- `set kit <index 0..=127> <identifier> <parameter>`
- `set kit <index 0..=127> <enum>`
- `set kit <index 0..=127> <element> <element-index> <enum>`
- `set kit <index 0..=127> <element> <element-index> <parameter>`
- `set kit <index 0..=127> sound <sound-index 0..=11> <identifier> <parameter> [<parameter>]`
- `set kit <index 0..=127> sound <sound-index 0..=11> <enum> [<parameter>]`

### Kit work buffer

Accepted formats:

- `set kit_wb <identifier> <parameter>`
- `set kit_wb <enum>`
- `set kit_wb <element> <element-index> <enum>`
- `set kit_wb <element> <element-index> <parameter>`
- `set kit_wb sound <sound-index 0..=11> <identifier> <parameter> [<parameter>]`
- `set kit_wb sound <sound-index 0..=11> <enum> [<parameter>]`

### Sound

Accepted formats:

- `set sound <index 0..=11> <identifier> <parameter> [<parameter>]`
- `set sound <index 0..=11> <enum> [<parameter>]`

### Sound work buffer

Accepted formats:

- `set sound_wb <index 0..=11> <identifier> <parameter> [<parameter>]`
- `set sound_wb <index 0..=11> <enum> [<parameter>]`

### Global

Accepted formats:

- `set global <index 0..=3> <identifier> <parameter>`
- `set global <index 0..=3> <enum> [<parameter>]`

### Global work buffer

Accepted formats:

- `set global_wb <identifier> <parameter>`
- `set global_wb <enum> [<parameter>]`

### Settings

Accepted formats:

- `set settings <identifier> <parameter>`
- `set settings <enum>`

## Identifiers

## Common identifiers

- `name` -> **symbol**, _Only 15 ascii characters are allowed_ (name of the object)
- `index` -> **int** (index of the object)
- `parentindex` -> **int** (index of the parent object if there is one)
- `version` -> **int** (version of the object)
- `iswb` -> **int**, _0..=1_ (is work buffer object)

### Pattern

- `masterchg` -> **int**, _1..=1024_
- `masterlen` -> **int**, _1..=1024_
- `kitnumber` -> **int**, _0..=127_
- `swingamount` -> **int**, _50..=80_
- `globalquantize` -> **int**, _0..=127_
- `patternbpm` -> **float**, _30.0..=300.0_

### Track

- `deftrignote` -> **int**, _0..=127_
- `deftrigvel` -> **int**, _0..=127_
- `deftrigprob` -> **int**, _0..=100_
- `steps` -> **int**, _1..=64_
- `quantizeamount` -> **int**, _0..=127_
- `sendsmidi` -> **int**, _0..=1_
- `euc` -> **int**, _0..=1_
- `pl2` -> **int**, _0..=63_
- `ro1` -> **int**, _0..=63_
- `ro2` -> **int**, _0..=63_
- `tro` -> **int**, _0..=63_

## Trig

- `enable` -> **int**, _0..=1_
- `retrig` -> **int**, _0..=1_
- `mute` -> **int**, _0..=1_
- `accent` -> **int**, _0..=1_
- `swing` -> **int**, _0..=1_
- `slide` -> **int**, _0..=1_
- `note` -> **int**, _0..=127_
- `vel` -> **int**, _1..=127_
- `retrigveloffset` -> **int**, _-128..=127_
- `soundlock` -> **int**, _0..=127_

## Kit

- `ctrlinmod1amt` -> **int**, _-128..=127_
- `ctrlinmod2amt` -> **int**, _-128..=127_
- `fxdeltime` -> **int**, _0..=127_
- `fxdelpingpong` -> **int**, _0..=1_
- `fxdelstereowidth` -> **int**, _-64..=63_
- `fxdelfeedback` -> **int**, _0..=198_
- `fxdelhpf` -> **int**, _0..=127_
- `fxdellpf` -> **int**, _0..=127_
- `fxdelrevsend` -> **int**, _0..=127_
- `fxdellev` -> **int**, _0..=127_
- `fxrevpredel` -> **int**, _0..=127_
- `fxrevdecay` -> **int**, _0..=127_
- `fxrevfreq` -> **int**, _0..=127_
- `fxrevgain` -> **int**, _0..=127_
- `fxrevhpf` -> **int**, _0..=127_
- `fxrevlpf` -> **int**, _0..=127_
- `fxrevlev` -> **int**, _0..=127_
- `fxcompthr` -> **int**, _0..=127_
- `fxcompgain` -> **int**, _0..=127_
- `fxcompmix` -> **int**, _0..=127_
- `fxcomplev` -> **int**, _0..=127_
- `fxlfospeed` -> **int**, _-64..=63_
- `fxlfofade` -> **int**, _-64..=63_
- `fxlfostartphase` -> **int**, _0..=127_
- `fxlfodepth` -> **float**, _-128.0..=127.99_
- `fxdistdov` -> **int**, _0..=127_
- `fxdistamt` -> **int**, _0..=127_
- `fxdistsym` -> **int**, _-64..=63_
- `fxdistdelpost` -> **int**, _0..=1_
- `fxdistrevpost` -> **int**, _0..=1_

### Elements

- `tracklevel` -> **int**, _0..=127_
  - kit element index range -> **int**, _0..=12_
- `trackretrigveloffset` -> **int**, _-128..=127_
  - kit element index range -> **int**, _0..=12_
- `trackretrigalwayson` -> **int**, _0..=1_
  - kit element index range -> **int**, _0..=12_

## Sound

- `ispool` -> **int**, _0..=1_
- `iskit` -> **int**, _0..=1_
- `kitnumber` -> **int**, _0..=127_
- `accentlev` -> **int**, _0..=127_
- `ampattack` -> **int**, _0..=127_
- `amphold` -> **int**, _0..=127_
- `ampdecay` -> **int**, _0..=127_
- `ampoverdrive` -> **int**, _0..=127_
- `ampdelsend` -> **int**, _0..=127_
- `amprevsend` -> **int**, _0..=127_
- `amppan` -> **int**, _-64..=63_
- `amplev` -> **int**, _0..=127_
- `filtattack` -> **int**, _0..=127_
- `filthold` -> **int**, _0..=127_
- `filtdecay` -> **int**, _0..=127_
- `filtrelease` -> **int**, _0..=127_
- `filtcutoff` -> **int**, _0..=127_
- `filtres` -> **int**, _0..=127_
- `filtenvamt` -> **int**, _-64..=63_
- `lfospeed` -> **int**, _-64..=63_
- `lfofade` -> **int**, _-64..=63_
- `lfostartphase` -> **int**, _0..=127_
- `lfodepth` -> **float**, _-128.0..=127.99_
- `samptune` -> **int**, _-24..=24_
- `sampfinetune` -> **int**, _-64..=63_
- `sampnumber` -> **int**, _0..=127_
- `sampbitreduction` -> **int**, _0..=127_
- `sampstart` -> **float**, _0.0..=120.0_
- `sampend` -> **float**, _0.0..=120.0_
- `samploopflag` -> **int**, _0..=1_
- `samplev` -> **int**, _0..=127_
- `velmodamt` -> **int**, _-127..=128_
- `atmodamt` -> **int**, _-127..=128_
- `envresetfilter` -> **int**, _0..=1_
- `veltovol` -> **int**, _0..=1_
- `legacyfxsend` -> **int**, _0..=1_

## Global

- `kitreloadonchg` -> **int**, _0..=1_
- `quantizeliverec` -> **int**, _0..=1_
- `autotrackswitch` -> **int**, _0..=1_
- `routetomain` -> **int**, _0..=11_
- `sendtofx` -> **int**, _0..=11_
- `clockreceive` -> **int**, _0..=1_
- `clocksend` -> **int**, _0..=1_
- `transportreceive` -> **int**, _0..=1_
- `transportsend` -> **int**, _0..=1_
- `pgmchangereceive` -> **int**, _0..=1_
- `pgmchangesend` -> **int**, _0..=1_
- `receivenotes` -> **int**, _0..=1_
- `receiveccnrpn` -> **int**, _0..=1_
- `turbospeed` -> **int**, _0..=1_
- `metronomeactive` -> **int**, _0..=1_
- `metronomeprerollbars` -> **int**, _0..=16_
- `metronomelev` -> **int**, _0..=127_

## Settings

- `projectbpm` -> **float**, _30.0..=300.0_
- `selectedtrack` -> **int**, _0..=11_
- `selectedpage` -> **int**, _0..=3_
- `mute` -> **int**, _0..=11_
- `unmute` -> **int**, _0..=11_
- `fixedvelocity` -> **int**, _0..=1_
- `fixedvelocityamt` -> **int**, _0..=127_
- `samplerecorderthr` -> **int**, _0..=127_
- `samplerecordermon` -> **int**, _0..=1_

## Enums

## Pattern

### `speed:`

| Variants | &nbsp;   | &nbsp;   |
| -------- | -------- | -------- |
| **1x**   | **3/4x** | **1/8x** |
| **2x**   | **1/2x** |          |
| **3/2x** | **1/4x** |          |

### `timemode:`

| Variants   | &nbsp;       | &nbsp; |
| ---------- | ------------ | ------ |
| **normal** | **advanced** |        |

## Track

### `rootnote:`

| Variants | &nbsp; | &nbsp; |
| -------- | ------ | ------ |
| **c**    | **e**  | **g#** |
| **c#**   | **f**  | **a**  |
| **d**    | **f#** | **bb** |
| **eb**   | **g**  | **b**  |

### `padscale:`

| Variants            | &nbsp;                  | &nbsp;                  |
| ------------------- | ----------------------- | ----------------------- |
| **chromatic**       | **wholetone**           | **majorlocrian**        |
| **ionianmajor**     | **blues**               | **superlocrian**        |
| **dorian**          | **combominor**          | **dorianb2**            |
| **phrygian**        | **persian**             | **lydianaugmented**     |
| **lydian**          | **iwato**               | **lydiandominant**      |
| **mixolydian**      | **insen**               | **doubleharmonicmajor** |
| **aeolianminor**    | **hirajoshi**           | **lydian26**            |
| **locrian**         | **pelog**               | **ultraphrygian**       |
| **pentatonicminor** | **phrygiandominant**    | **hungarianminor**      |
| **pentatonicmajor** | **wholehalfdiminished** | **oriental**            |
| **melodicminor**    | **halfwholediminished** | **ionian25**            |
| **harmonicminor**   | **spanish**             | **locrianbb3bb7**       |

### `defaultnotelen:`

| Variants  | &nbsp;   | &nbsp;    |
| --------- | -------- | --------- |
| **1/128** | **3.63** | **24**    |
| **.188**  | **3.75** | **25**    |
| **1/64**  | **3.88** | **26**    |
| **.313**  | **1/4**  | **27**    |
| **.375**  | **4.25** | **28**    |
| **.438**  | **4.5**  | **29**    |
| **1/32**  | **4.75** | **30**    |
| **.563**  | **5**    | **31**    |
| **.625**  | **5.25** | **32**    |
| **.688**  | **5.5**  | **34**    |
| **.75**   | **5.75** | **36**    |
| **.813**  | **6**    | **38**    |
| **.875**  | **6.25** | **40**    |
| **.938**  | **6.5**  | **42**    |
| **1/16**  | **6.75** | **44**    |
| **1.06**  | **7**    | **46**    |
| **1.13**  | **7.25** | **48**    |
| **1.19**  | **7.5**  | **50**    |
| **1.25**  | **7.75** | **52**    |
| **1.31**  | **1/2**  | **54**    |
| **1.38**  | **8.5**  | **56**    |
| **1.44**  | **9**    | **58**    |
| **1.5**   | **9.5**  | **60**    |
| **1.56**  | **10**   | **62**    |
| **1.63**  | **10.5** | **64**    |
| **1.69**  | **11**   | **68**    |
| **1.75**  | **11.5** | **72**    |
| **1.81**  | **12**   | **76**    |
| **1.88**  | **12.5** | **80**    |
| **1.94**  | **13**   | **84**    |
| **1/8**   | **13.5** | **88**    |
| **2.13**  | **14**   | **92**    |
| **2.25**  | **14.5** | **96**    |
| **2.38**  | **15**   | **100**   |
| **2.5**   | **15.5** | **104**   |
| **2.63**  | **1/1**  | **108**   |
| **2.75**  | **17**   | **112**   |
| **2.88**  | **18**   | **116**   |
| **3**     | **19**   | **120**   |
| **3.13**  | **20**   | **124**   |
| **3.25**  | **21**   | **128**   |
| **3.38**  | **22**   | **inf**   |
| **3.5**   | **23**   | **unset** |

## Trig

### `microtime:`

| Variants    | &nbsp;     | &nbsp;     |
| ----------- | ---------- | ---------- |
| **-23/384** | **-7/384** | **3/128**  |
| **-11/192** | **-1/64**  | **5/192**  |
| **-7/128**  | **-5/384** | **11/384** |
| **-5/96**   | **-1/96**  | **1/32**   |
| **-19/384** | **-1/128** | **13/384** |
| **-3/64**   | **-1/192** | **7/192**  |
| **-17/384** | **-1/384** | **5/128**  |
| **-1/24**   | **ongrid** | **1/24**   |
| **-5/128**  | **1/384**  | **17/384** |
| **-7/192**  | **1/192**  | **3/64**   |
| **-13/384** | **1/128**  | **19/384** |
| **-1/32**   | **1/96**   | **5/96**   |
| **-11/384** | **5/384**  | **7/128**  |
| **-5/192**  | **1/64**   | **11/192** |
| **-3/128**  | **7/384**  | **23/384** |
| **-1/48**   | **1/48**   |            |

### `notelen:`

| Variants  | &nbsp;   | &nbsp;    |
| --------- | -------- | --------- |
| **1/128** | **3.63** | **24**    |
| **.188**  | **3.75** | **25**    |
| **1/64**  | **3.88** | **26**    |
| **.313**  | **1/4**  | **27**    |
| **.375**  | **4.25** | **28**    |
| **.438**  | **4.5**  | **29**    |
| **1/32**  | **4.75** | **30**    |
| **.563**  | **5**    | **31**    |
| **.625**  | **5.25** | **32**    |
| **.688**  | **5.5**  | **34**    |
| **.75**   | **5.75** | **36**    |
| **.813**  | **6**    | **38**    |
| **.875**  | **6.25** | **40**    |
| **.938**  | **6.5**  | **42**    |
| **1/16**  | **6.75** | **44**    |
| **1.06**  | **7**    | **46**    |
| **1.13**  | **7.25** | **48**    |
| **1.19**  | **7.5**  | **50**    |
| **1.25**  | **7.75** | **52**    |
| **1.31**  | **1/2**  | **54**    |
| **1.38**  | **8.5**  | **56**    |
| **1.44**  | **9**    | **58**    |
| **1.5**   | **9.5**  | **60**    |
| **1.56**  | **10**   | **62**    |
| **1.63**  | **10.5** | **64**    |
| **1.69**  | **11**   | **68**    |
| **1.75**  | **11.5** | **72**    |
| **1.81**  | **12**   | **76**    |
| **1.88**  | **12.5** | **80**    |
| **1.94**  | **13**   | **84**    |
| **1/8**   | **13.5** | **88**    |
| **2.13**  | **14**   | **92**    |
| **2.25**  | **14.5** | **96**    |
| **2.38**  | **15**   | **100**   |
| **2.5**   | **15.5** | **104**   |
| **2.63**  | **1/1**  | **108**   |
| **2.75**  | **17**   | **112**   |
| **2.88**  | **18**   | **116**   |
| **3**     | **19**   | **120**   |
| **3.13**  | **20**   | **124**   |
| **3.25**  | **21**   | **128**   |
| **3.38**  | **22**   | **inf**   |
| **3.5**   | **23**   | **unset** |

### `retriglen:`

| Variants  | &nbsp;   | &nbsp;    |
| --------- | -------- | --------- |
| **1/128** | **3.63** | **24**    |
| **.188**  | **3.75** | **25**    |
| **1/64**  | **3.88** | **26**    |
| **.313**  | **1/4**  | **27**    |
| **.375**  | **4.25** | **28**    |
| **.438**  | **4.5**  | **29**    |
| **1/32**  | **4.75** | **30**    |
| **.563**  | **5**    | **31**    |
| **.625**  | **5.25** | **32**    |
| **.688**  | **5.5**  | **34**    |
| **.75**   | **5.75** | **36**    |
| **.813**  | **6**    | **38**    |
| **.875**  | **6.25** | **40**    |
| **.938**  | **6.5**  | **42**    |
| **1/16**  | **6.75** | **44**    |
| **1.06**  | **7**    | **46**    |
| **1.13**  | **7.25** | **48**    |
| **1.19**  | **7.5**  | **50**    |
| **1.25**  | **7.75** | **52**    |
| **1.31**  | **1/2**  | **54**    |
| **1.38**  | **8.5**  | **56**    |
| **1.44**  | **9**    | **58**    |
| **1.5**   | **9.5**  | **60**    |
| **1.56**  | **10**   | **62**    |
| **1.63**  | **10.5** | **64**    |
| **1.69**  | **11**   | **68**    |
| **1.75**  | **11.5** | **72**    |
| **1.81**  | **12**   | **76**    |
| **1.88**  | **12.5** | **80**    |
| **1.94**  | **13**   | **84**    |
| **1/8**   | **13.5** | **88**    |
| **2.13**  | **14**   | **92**    |
| **2.25**  | **14.5** | **96**    |
| **2.38**  | **15**   | **100**   |
| **2.5**   | **15.5** | **104**   |
| **2.63**  | **1/1**  | **108**   |
| **2.75**  | **17**   | **112**   |
| **2.88**  | **18**   | **116**   |
| **3**     | **19**   | **120**   |
| **3.13**  | **20**   | **124**   |
| **3.25**  | **21**   | **128**   |
| **3.38**  | **22**   | **inf**   |
| **3.5**   | **23**   | **unset** |

### `retrigrate:`

| Variants | &nbsp;   | &nbsp;   |
| -------- | -------- | -------- |
| **1/1**  | **1/8**  | **1/32** |
| **1/2**  | **1/10** | **1/40** |
| **1/3**  | **1/12** | **1/48** |
| **1/4**  | **1/16** | **1/64** |
| **1/5**  | **1/20** | **1/80** |
| **1/6**  | **1/24** |          |

### `trigcondition:`

| Variants | &nbsp;      | &nbsp;    |
| -------- | ----------- | --------- |
| **1%**   | **fill**    | **1:6**   |
| **3%**   | **fillnot** | **2:6**   |
| **4%**   | **pre**     | **3:6**   |
| **6%**   | **prenot**  | **4:6**   |
| **9%**   | **nei**     | **5:6**   |
| **13%**  | **neinot**  | **6:6**   |
| **19%**  | **1st**     | **1:7**   |
| **25%**  | **1stnot**  | **2:7**   |
| **33%**  | **1:2**     | **3:7**   |
| **41%**  | **2:2**     | **4:7**   |
| **50%**  | **1:3**     | **5:7**   |
| **59%**  | **2:3**     | **6:7**   |
| **67%**  | **3:3**     | **7:7**   |
| **75%**  | **1:4**     | **1:8**   |
| **81%**  | **2:4**     | **2:8**   |
| **87%**  | **3:4**     | **3:8**   |
| **91%**  | **4:4**     | **4:8**   |
| **94%**  | **1:5**     | **5:8**   |
| **96%**  | **2:5**     | **6:8**   |
| **98%**  | **3:5**     | **7:8**   |
| **99%**  | **4:5**     | **8:8**   |
| **100%** | **5:5**     | **unset** |

## Kit

### `controlinmod1target:`

| Variants           | &nbsp;                 | &nbsp;              |
| ------------------ | ---------------------- | ------------------- |
| **unset**          | **samplebitreduction** | **filterresonance** |
| **lfomultiplier**  | **samplestart**        | **ampattack**       |
| **lfowaveform**    | **sampleend**          | **amphold**         |
| **lfotrigmode**    | **sampleloop**         | **ampdecay**        |
| **lfospeed**       | **samplelevel**        | **ampoverdrive**    |
| **lfofade**        | **filterenvelope**     | **ampvolume**       |
| **lfophase**       | **filterattack**       | **amppan**          |
| **lfodepth**       | **filterdecay**        | **ampaccent**       |
| **sampletune**     | **filtersustain**      | **ampdelaysend**    |
| **samplefinetune** | **filterrelease**      | **ampreverbsend**   |
| **sampleslice**    | **filterfrequency**    |                     |

### `controlinmod2target:`

| Variants           | &nbsp;                 | &nbsp;              |
| ------------------ | ---------------------- | ------------------- |
| **unset**          | **samplebitreduction** | **filterresonance** |
| **lfomultiplier**  | **samplestart**        | **ampattack**       |
| **lfowaveform**    | **sampleend**          | **amphold**         |
| **lfotrigmode**    | **sampleloop**         | **ampdecay**        |
| **lfospeed**       | **samplelevel**        | **ampoverdrive**    |
| **lfofade**        | **filterenvelope**     | **ampvolume**       |
| **lfophase**       | **filterattack**       | **amppan**          |
| **lfodepth**       | **filterdecay**        | **ampaccent**       |
| **sampletune**     | **filtersustain**      | **ampdelaysend**    |
| **samplefinetune** | **filterrelease**      | **ampreverbsend**   |
| **sampleslice**    | **filterfrequency**    |                     |

### `fxlfodest:`

| Variants             | &nbsp;                 | &nbsp;                    |
| -------------------- | ---------------------- | ------------------------- |
| **unset**            | **delayoverdrive**     | **distortionsymmetry**    |
| **delaytime**        | **reverbpredelay**     | **compressorthreshold**   |
| **delaypingpong**    | **reverbdecay**        | **compressorattack**      |
| **delaystereowidth** | **reverbshelvingfreq** | **compressorrelease**     |
| **delayfeedback**    | **reverbshelvinggain** | **compressorratio**       |
| **delayhpfilter**    | **reverbhpfilter**     | **compressorsidechaineq** |
| **delaylpfilter**    | **reverblpfilter**     | **compressormakeupgain**  |
| **delayreverbsend**  | **reverbmixvolume**    | **compressordrywetmix**   |
| **delaymixvolume**   | **distortionamount**   | **compressorvolume**      |

### `fxcompattack:`

| Variants | &nbsp; | &nbsp; |
| -------- | ------ | ------ |
| **0.03** | **1**  | **30** |
| **0.1**  | **3**  |        |
| **0.3**  | **10** |        |

### `fxcomprelease:`

| Variants | &nbsp;  | &nbsp; |
| -------- | ------- | ------ |
| **0.1**  | **0.6** | **A1** |
| **0.2**  | **1**   | **A2** |
| **0.4**  | **2**   |        |

### `fxcompratio:`

| Variants | &nbsp;  | &nbsp; |
| -------- | ------- | ------ |
| **1:2**  | **1:8** |        |
| **1:4**  | **max** |        |

### `fxcompsidechaineq:`

| Variants | &nbsp;  | &nbsp; |
| -------- | ------- | ------ |
| **off**  | **hpf** |        |
| **lpf**  | **hit** |        |

#### `fxdelaytimeonthegrid:`

| Variants       | &nbsp;         | &nbsp;            |
| -------------- | -------------- | ----------------- |
| **128th**      | **16th**       | **quarterdotted** |
| **64th**       | **16thdotted** | **half**          |
| **64thdotted** | **8th**        | **halfdotted**    |
| **32nd**       | **8thdotted**  | **whole**         |
| **32nddotted** | **quarter**    |                   |

### Elements

#### `trackretrigrate:`

| Variants | &nbsp;   | &nbsp;   |
| -------- | -------- | -------- |
| **1/1**  | **1/8**  | **1/32** |
| **1/2**  | **1/10** | **1/40** |
| **1/3**  | **1/12** | **1/48** |
| **1/4**  | **1/16** | **1/64** |
| **1/5**  | **1/20** | **1/80** |
| **1/6**  | **1/24** |          |

#### `trackretriglen:`

| Variants  | &nbsp;   | &nbsp;    |
| --------- | -------- | --------- |
| **1/128** | **3.63** | **24**    |
| **.188**  | **3.75** | **25**    |
| **1/64**  | **3.88** | **26**    |
| **.313**  | **1/4**  | **27**    |
| **.375**  | **4.25** | **28**    |
| **.438**  | **4.5**  | **29**    |
| **1/32**  | **4.75** | **30**    |
| **.563**  | **5**    | **31**    |
| **.625**  | **5.25** | **32**    |
| **.688**  | **5.5**  | **34**    |
| **.75**   | **5.75** | **36**    |
| **.813**  | **6**    | **38**    |
| **.875**  | **6.25** | **40**    |
| **.938**  | **6.5**  | **42**    |
| **1/16**  | **6.75** | **44**    |
| **1.06**  | **7**    | **46**    |
| **1.13**  | **7.25** | **48**    |
| **1.19**  | **7.5**  | **50**    |
| **1.25**  | **7.75** | **52**    |
| **1.31**  | **1/2**  | **54**    |
| **1.38**  | **8.5**  | **56**    |
| **1.44**  | **9**    | **58**    |
| **1.5**   | **9.5**  | **60**    |
| **1.56**  | **10**   | **62**    |
| **1.63**  | **10.5** | **64**    |
| **1.69**  | **11**   | **68**    |
| **1.75**  | **11.5** | **72**    |
| **1.81**  | **12**   | **76**    |
| **1.88**  | **12.5** | **80**    |
| **1.94**  | **13**   | **84**    |
| **1/8**   | **13.5** | **88**    |
| **2.13**  | **14**   | **92**    |
| **2.25**  | **14.5** | **96**    |
| **2.38**  | **15**   | **100**   |
| **2.5**   | **15.5** | **104**   |
| **2.63**  | **1/1**  | **108**   |
| **2.75**  | **17**   | **112**   |
| **2.88**  | **18**   | **116**   |
| **3**     | **19**   | **120**   |
| **3.13**  | **20**   | **124**   |
| **3.25**  | **21**   | **128**   |
| **3.38**  | **22**   | **inf**   |
| **3.5**   | **23**   | **unset** |

## Sound

### `machinetype:`

| Variants      | &nbsp;         | &nbsp;         |
| ------------- | -------------- | -------------- |
| **bdhard**    | **cbclassic**  | **hhbasic**    |
| **bdclassic** | **bdfm**       | **cyride**     |
| **sdhard**    | **sdfm**       | **bdsharp**    |
| **sdclassic** | **utnoise**    | **disable**    |
| **rshard**    | **utimpulse**  | **sydualvco**  |
| **rsclassic** | **chmetallic** | **sychip**     |
| **cpclassic** | **ohmetallic** | **bdacoustic** |
| **btclassic** | **cymetallic** | **sdacoustic** |
| **xtclassic** | **cbmetallic** | **syraw**      |
| **chclassic** | **bdplastic**  | **hhlab**      |
| **ohclassic** | **bdsilky**    | **unset**      |
| **cyclassic** | **sdnatural**  |                |

### `lfodest:`

| Variants           | &nbsp;                 | &nbsp;              |
| ------------------ | ---------------------- | ------------------- |
| **syn1**           | **samplebitreduction** | **filterresonance** |
| **syn2**           | **samplestart**        | **ampattack**       |
| **syn3**           | **sampleend**          | **amphold**         |
| **syn4**           | **sampleloop**         | **ampdecay**        |
| **syn5**           | **samplelevel**        | **ampoverdrive**    |
| **syn6**           | **filterenvelope**     | **ampvolume**       |
| **syn7**           | **filterattack**       | **amppan**          |
| **syn8**           | **filterdecay**        | **ampaccent**       |
| **sampletune**     | **filtersustain**      | **ampdelaysend**    |
| **samplefinetune** | **filterrelease**      | **ampreverb_send**  |
| **sampleslice**    | **filterfrequency**    | **unset**           |

### `velmodtarget:`

| Variants          | &nbsp;                 | &nbsp;              |
| ----------------- | ---------------------- | ------------------- |
| **unset**         | **syn7**               | **filterrelease**   |
| **lfomultiplier** | **syn8**               | **filterfrequency** |
| **lfowaveform**   | **sampletune**         | **filterresonance** |
| **lfotrigmode**   | **samplefinetune**     | **ampattack**       |
| **lfospeed**      | **sampleslice**        | **amphold**         |
| **lfofade**       | **samplebitreduction** | **ampdecay**        |
| **lfophase**      | **samplestart**        | **ampoverdrive**    |
| **lfodepth**      | **sampleend**          | **ampvolume**       |
| **syn1**          | **sampleloop**         | **amppan**          |
| **syn2**          | **samplelevel**        | **ampaccent**       |
| **syn3**          | **filterenvelope**     | **ampdelaysend**    |
| **syn4**          | **filterattack**       | **ampreverbsend**   |
| **syn5**          | **filterdecay**        |                     |
| **syn6**          | **filtersustain**      |                     |

### `atmodtarget:`

| Variants          | &nbsp;                 | &nbsp;              |
| ----------------- | ---------------------- | ------------------- |
| **unset**         | **syn7**               | **filterrelease**   |
| **lfomultiplier** | **syn8**               | **filterfrequency** |
| **lfowaveform**   | **sampletune**         | **filterresonance** |
| **lfotrigmode**   | **samplefinetune**     | **ampattack**       |
| **lfospeed**      | **sampleslice**        | **amphold**         |
| **lfofade**       | **samplebitreduction** | **ampdecay**        |
| **lfophase**      | **samplestart**        | **ampoverdrive**    |
| **lfodepth**      | **sampleend**          | **ampvolume**       |
| **syn1**          | **sampleloop**         | **amppan**          |
| **syn2**          | **samplelevel**        | **ampaccent**       |
| **syn3**          | **filterenvelope**     | **ampdelaysend**    |
| **syn4**          | **filterattack**       | **ampreverbsend**   |
| **syn5**          | **filterdecay**        |                     |
| **syn6**          | **filtersustain**      |                     |

### `filtertype:`

| Variants | &nbsp;  | &nbsp; |
| -------- | ------- | ------ |
| **lp2**  | **hp1** | **pk** |
| **lp1**  | **hp2** |        |
| **bp**   | **bs**  |        |

### `lfomultiplier:`

| Variants | &nbsp;   | &nbsp;   |
| -------- | -------- | -------- |
| **x1**   | **x256** | **.16**  |
| **x2**   | **x512** | **.32**  |
| **x4**   | **x1k**  | **.64**  |
| **x8**   | **x2k**  | **.128** |
| **x16**  | **.1**   | **.256** |
| **x32**  | **.2**   | **.512** |
| **x64**  | **.4**   | **.1k**  |
| **x128** | **.8**   | **.2k**  |

### `lfowaveform:`

| Variants | &nbsp;  | &nbsp;  |
| -------- | ------- | ------- |
| **tri**  | **saw** | **rnd** |
| **sin**  | **exp** |         |
| **sqr**  | **rmp** |         |

### `lfomode:`

| Variants | &nbsp;   | &nbsp;   |
| -------- | -------- | -------- |
| **free** | **hold** | **half** |
| **trig** | **one**  |          |

### `chromaticmode:`

| Variants | &nbsp;       | &nbsp; |
| -------- | ------------ | ------ |
| **off**  | **samp**     |        |
| **syn**  | **syn+samp** |        |

## Global

### `metronometimesig:`

| Variants | &nbsp;   | &nbsp;    |
| -------- | -------- | --------- |
| **1/1**  | **12/2** | **7/8**   |
| **2/1**  | **13/2** | **8/8**   |
| **3/1**  | **14/2** | **9/8**   |
| **4/1**  | **15/2** | **10/8**  |
| **5/1**  | **16/2** | **11/8**  |
| **6/1**  | **1/4**  | **12/8**  |
| **7/1**  | **2/4**  | **13/8**  |
| **8/1**  | **3/4**  | **14/8**  |
| **9/1**  | **4/4**  | **15/8**  |
| **10/1** | **5/4**  | **16/8**  |
| **11/1** | **6/4**  | **1/16**  |
| **12/1** | **7/4**  | **2/16**  |
| **13/1** | **8/4**  | **3/16**  |
| **14/1** | **9/4**  | **4/16**  |
| **15/1** | **10/4** | **5/16**  |
| **16/1** | **11/4** | **6/16**  |
| **1/2**  | **12/4** | **7/16**  |
| **2/2**  | **13/4** | **8/16**  |
| **3/2**  | **14/4** | **9/16**  |
| **4/2**  | **15/4** | **10/16** |
| **5/2**  | **16/4** | **11/16** |
| **6/2**  | **1/8**  | **12/16** |
| **7/2**  | **2/8**  | **13/16** |
| **8/2**  | **3/8**  | **14/16** |
| **9/2**  | **4/8**  | **15/16** |
| **10/2** | **5/8**  | **16/16** |
| **11/2** | **6/8**  |           |

### `autochannel:`

| Variants | &nbsp; | &nbsp; |
| -------- | ------ | ------ |
| **1**    | **7**  | **13** |
| **2**    | **8**  | **14** |
| **3**    | **9**  | **15** |
| **4**    | **10** | **16** |
| **5**    | **11** |        |
| **6**    | **12** |        |

### `trackchannels:`

| Variants | &nbsp; | &nbsp; |
| -------- | ------ | ------ |
| **1**    | **7**  | **13** |
| **2**    | **8**  | **14** |
| **3**    | **9**  | **15** |
| **4**    | **10** | **16** |
| **5**    | **11** |        |
| **6**    | **12** |        |

### `trackfxchannel:`

| Variants | &nbsp; | &nbsp; |
| -------- | ------ | ------ |
| **1**    | **7**  | **13** |
| **2**    | **8**  | **14** |
| **3**    | **9**  | **15** |
| **4**    | **10** | **16** |
| **5**    | **11** |        |
| **6**    | **12** |        |

### `pgmchangeinchannel:`

| Variants | &nbsp; | &nbsp; |
| -------- | ------ | ------ |
| **1**    | **7**  | **13** |
| **2**    | **8**  | **14** |
| **3**    | **9**  | **15** |
| **4**    | **10** | **16** |
| **5**    | **11** |        |
| **6**    | **12** |        |

### `pgmchangeoutchannel:`

| Variants | &nbsp; | &nbsp; |
| -------- | ------ | ------ |
| **1**    | **7**  | **13** |
| **2**    | **8**  | **14** |
| **3**    | **9**  | **15** |
| **4**    | **10** | **16** |
| **5**    | **11** |        |
| **6**    | **12** |        |

### `performancechannel:`

| Variants | &nbsp; | &nbsp; |
| -------- | ------ | ------ |
| **1**    | **7**  | **13** |
| **2**    | **8**  | **14** |
| **3**    | **9**  | **15** |
| **4**    | **10** | **16** |
| **5**    | **11** |        |
| **6**    | **12** |        |

### `outportfunction:`

| Variants | &nbsp;    | &nbsp;    |
| -------- | --------- | --------- |
| **midi** | **din24** | **din48** |

### `thruportfunction:`

| Variants | &nbsp;    | &nbsp;    |
| -------- | --------- | --------- |
| **midi** | **din24** | **din48** |

### `inputfrom:`

| Variants     | &nbsp;       | &nbsp; |
| ------------ | ------------ | ------ |
| **disabled** | **usb**      |        |
| **midi**     | **midi+usb** |        |

### `outputto:`

| Variants     | &nbsp;       | &nbsp; |
| ------------ | ------------ | ------ |
| **disabled** | **usb**      |        |
| **midi**     | **midi+usb** |        |

### `paddest:`

| Variants | &nbsp;      | &nbsp;  |
| -------- | ----------- | ------- |
| **int**  | **int+ext** | **ext** |

### `pressuredest:`

| Variants | &nbsp;      | &nbsp;  |
| -------- | ----------- | ------- |
| **int**  | **int+ext** | **ext** |

### `encoderdest:`

| Variants | &nbsp;      | &nbsp;  |
| -------- | ----------- | ------- |
| **int**  | **int+ext** | **ext** |

### `mutedest:`

| Variants | &nbsp;      | &nbsp;  |
| -------- | ----------- | ------- |
| **int**  | **int+ext** | **ext** |

### `usbtomaindb:`

| Variants | &nbsp;    | &nbsp; |
| -------- | --------- | ------ |
| **0db**  | **+12db** |        |
| **+6db** | **+18db** |        |

### `paramoutput:`

| Variants | &nbsp; | &nbsp; |
| -------- | ------ | ------ |
| **nrpn** | **cc** |        |

### `portsoutputchannel:`

| Variants | &nbsp;    | &nbsp; |
| -------- | --------- | ------ |
| **auto** | **track** |        |

### `usbin:`

| Variants       | &nbsp;           | &nbsp;            |
| -------------- | ---------------- | ----------------- |
| **pre-fx**     | **l:2r:9:10**    | **l:6r:11:12**    |
| **post-fx**    | **l:2r:11:12**   | **l:7:8r:1**      |
| **1**          | **l:3:4r:1**     | **l:7:8r:2**      |
| **2**          | **l:3:4r:2**     | **l:7:8r:3:4**    |
| **3:4**        | **l:3:4r:5**     | **l:7:8r:5**      |
| **5**          | **l:3:4r:6**     | **l:7:8r:6**      |
| **6**          | **l:3:4r:7:8**   | **l:7:8r:9:10**   |
| **7:8**        | **l:3:4r:9:10**  | **l:7:8r:11:12**  |
| **9:10**       | **l:3:4r:11:12** | **l:9:10r:1**     |
| **11:12**      | **l:5r:1**       | **l:9:10r:2**     |
| **l:1r:2**     | **l:5r:2**       | **l:9:10r:3:4**   |
| **l:1r:3:4**   | **l:5r:3:4**     | **l:9:10r:5**     |
| **l:1r:5**     | **l:5r:6**       | **l:9:10r:6**     |
| **l:1r:6**     | **l:5r:7:8**     | **l:9:10r:7:8**   |
| **l:1r:7:8**   | **l:5r:9:10**    | **l:9:10r:11:12** |
| **l:1r:9:10**  | **l:5r:11:12**   | **l:11:12r:1**    |
| **l:1r:11:12** | **l:6r:1**       | **l:11:12r:2**    |
| **l:2r:1**     | **l:6r:2**       | **l:11:12r:3:4**  |
| **l:2r:3:4**   | **l:6r:3:4**     | **l:11:12r:5**    |
| **l:2r:5**     | **l:6r:5**       | **l:11:12r:6**    |
| **l:2r:6**     | **l:6r:7:8**     | **l:11:12r:7:8**  |
| **l:2r:7:8**   | **l:6r:9:10**    | **l:11:12r:9:10** |

### `usbout:`

| Variants       | &nbsp;           | &nbsp;            |
| -------------- | ---------------- | ----------------- |
| **mainout**    | **l:2r:11:12**   | **l:7:8r:1**      |
| **1**          | **l:3:4r:1**     | **l:7:8r:2**      |
| **2**          | **l:3:4r:2**     | **l:7:8r:3:4**    |
| **3:4**        | **l:3:4r:5**     | **l:7:8r:5**      |
| **5**          | **l:3:4r:6**     | **l:7:8r:6**      |
| **6**          | **l:3:4r:7:8**   | **l:7:8r:9:10**   |
| **7:8**        | **l:3:4r:9:10**  | **l:7:8r:11:12**  |
| **9:10**       | **l:3:4r:11:12** | **l:9:10r:1**     |
| **11:12**      | **l:5r:1**       | **l:9:10r:2**     |
| **l:1r:2**     | **l:5r:2**       | **l:9:10r:3:4**   |
| **l:1r:3:4**   | **l:5r:3:4**     | **l:9:10r:5**     |
| **l:1r:5**     | **l:5r:6**       | **l:9:10r:6**     |
| **l:1r:6**     | **l:5r:7:8**     | **l:9:10r:7:8**   |
| **l:1r:7:8**   | **l:5r:9:10**    | **l:9:10r:11:12** |
| **l:1r:9:10**  | **l:5r:11:12**   | **l:11:12r:1**    |
| **l:1r:11:12** | **l:6r:1**       | **l:11:12r:2**    |
| **l:2r:1**     | **l:6r:2**       | **l:11:12r:3:4**  |
| **l:2r:3:4**   | **l:6r:3:4**     | **l:11:12r:5**    |
| **l:2r:5**     | **l:6r:5**       | **l:11:12r:6**    |
| **l:2r:6**     | **l:6r:7:8**     | **l:11:12r:7:8**  |
| **l:2r:7:8**   | **l:6r:9:10**    | **l:11:12r:9:10** |
| **l:2r:9:10**  | **l:6r:11:12**   |                   |

## Settings

### `parametermenuitem:`

| Variants | &nbsp;   | &nbsp;  |
| -------- | -------- | ------- |
| **trig** | **smpl** | **amp** |
| **src**  | **fltr** | **lfo** |

### `fxparametermenuitem:`

| Variants  | &nbsp;     | &nbsp;   |
| --------- | ---------- | -------- |
| **trig**  | **reverb** | **comp** |
| **delay** | **dist**   | **lfo**  |

### `sequencermode:`

| Variants   | &nbsp;    | &nbsp;   |
| ---------- | --------- | -------- |
| **normal** | **chain** | **song** |

### `patternmode:`

| Variants        | &nbsp;         | &nbsp; |
| --------------- | -------------- | ------ |
| **sequential**  | **directjump** |        |
| **directstart** | **tempjump**   |        |

### `samplerecordersrc:`

| Variants   | &nbsp;    | &nbsp;     |
| ---------- | --------- | ---------- |
| **audl+r** | **rs/cp** | **cy/cb**  |
| **audl**   | **bt**    | **main**   |
| **audr**   | **lt**    | **usbl**   |
| **bd**     | **mt/ht** | **usbr**   |
| **sd**     | **ch/oh** | **usbl+r** |

### `samplerecorderrecordinglen:`

| Variants   | &nbsp;      | &nbsp;       |
| ---------- | ----------- | ------------ |
| **1step**  | **8steps**  | **64steps**  |
| **2steps** | **16steps** | **128steps** |
| **4steps** | **32steps** | **max**      |
