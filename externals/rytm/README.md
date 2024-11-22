# rytm

A MaxMSP external to communicate with Analog Rytm MKII through sysex.

- Jump to [API docs](API_DOCS.md).
- Jump to [releases](https://github.com/alisomay/petunia-externals/releases).

This external is powered by [`rytm-rs`](https://github.com/alisomay/rytm-rs) and should be currently considered as beta software. Please report any issue you may encounter through [github issues](https://github.com/alisomay/petunia-externals/issues).

`rytm` provides a rich interface to communicate with Analog Rytm MKII through sysex.

It has verbose and informative error reporting, inherits all the features of `rytm-rs` and provides comprehensive documentation.

All the following documentation below this section is written with the assumption of basic knowledge of the Analog Rytm drum machines and MaxMSP.

## Installation

One can obtain the builds from the [releases](https://github.com/alisomay/petunia-externals/releases) page which contains builds for major operating systems.

If you want to build the external yourself, you will need to install [rust](https://www.rust-lang.org/tools/install). Then, please follow the build and install instructions from [median](https://github.com/Cycling74/median/blob/develop/median/README.md) repository.

## Preparation in your patch

Once you installed the external you may instantiate it in the MaxMSP patch by writing `rytm` in an object box.

Connect the `sysexin` object to `rytm`'s inlet and `rytm`'s leftmost outlet to `midiout` object.

Set your ports for `sysexin` and `midiout` objects so they point to your device and you're ready to go.

The rightmost outlet of `rytm` object is used to respond to get and set queries.

## Context

`rytm` when instantiated contains a twin of a default project on Analog Rytm MKII.

Important thing to know is this is not yet the data on your device.

We'll update parts of the `rytm` by querying the device when needed.

`rytm` is not yet an interface for `cc` or `nrpn` messages which are well documented in the manual.

It is a `sysex` interface where the format is reverse engineered as a community effort see [`rytm-rs`](https://github.com/alisomay/rytm-rs) and these threads from [elektronauts](https://www.elektronauts.com) for details:

- <https://www.elektronauts.com/t/announcing-an-unofficial-sdk-for-analog-rytm-mkii-running-firmware-1-70-written-in-rust/205232>
- <https://www.elektronauts.com/t/rytm-sysex-list/>

`rytm` follows the devices not officially supported and undocumented `sysex` format.

This means that it is not guaranteed to work with future firmware updates unless new versions of this external are released.

Currently the supported firmware version is `1.70` and it is only tested on Analog Rytm MKII.

### The format and the project structure

A `rytm` project is made of

- 128 patterns
- A work buffer pattern
- 128 kits
- A work buffer kit (which includes 12 kit sounds)
- 128 pool sounds
- 12 work buffer sounds
- 4 globals (global settings)
- A work buffer global
- Settings (a single structure for the whole project)

`rytm` enables you to query all these structures from the device, edit them in great detail and send them back to the device.

If you wish you can also edit the default ones which comes with `rytm`'s instantiation and send them to the device.

To work productively with `rytm` an understanding of these structures and the connection between them is necessary.

The following sections will explain the structures briefly.

Note that this is a brief introduction to the structures. For more detailed and complete information please refer to the [api docs](API_DOCS.md), the source code and the community.

#### What is the work buffer?

The device has something called a "work buffer" which points to the pattern you're currently chosen, kit you're currently chosen, etc.

Think of it as a pointer to the currently selected or active structure.

#### Patterns

Think of patterns as the sheet music. They are a set of instructions for the sequencer. They don't contain any settings for the sounds.

This is where your notes, trigs, mutes, locks, retrigs, microtiming, etc. are stored.

Also patterns store the kit number they are associated with.

#### Kits

Kits are associated with patterns. They contain the settings for fx, the sounds they're assigned to them and more.
A pattern can make sound because it is associated with a kit.

#### Sounds

Sounds contain the settings for the fltr, lfo, amp, smpl pages and more. They may be associated with kits but not directly with patterns.

#### Globals

These are the global settings for the device. They represent the settings in the global settings menu.

#### Settings

Settings contain the muted sounds, selected track, selected pattern mode and more.

### Queries

Since the `rytm` contains the twin of the structures on the device we can query them and update the `rytm`'s structures with the data from the device.

This is done by starting our messages with `query` selector. For example `query pattern 1` will query the pattern 1 (2 on device) from the device and update the `rytm`'s pattern 1 with the data from the device.

All indexes are 0 based so the first pattern is 0, the first kit is 0, etc.

Accepted queries are:

- `query pattern <index 0..=127>`
- `query kit <index 0..=127>`
- `query sound <index 0..=11>`
- `query global <index 0..=3>`
- `query settings`
- `query pattern_wb`
- `query kit_wb`
- `query sound_wb <index 0..=11>`
- `query global_wb`

#### Sending data to the device

This is done by starting our messages with `send` selector. For example `send pattern 1` will send the pattern 1 (2 on device) to the device and update the pattern 1 on the device with the data from the `rytm`'s pattern 1.

Accepted formats are:

- `send pattern <index 0..=127>`
- `send kit <index 0..=127>`
- `send sound <index 0..=11>`
- `send global <index 0..=3>`
- `send settings`
- `send pattern_wb`
- `send kit_wb`
- `send sound_wb <index 0..=11>`
- `send global_wb`

#### Getting data from `rytm` external

This is done by starting our messages with `get` selector.
The details of the format and the output format are explained in the [api docs](API_DOCS.md).

#### Setting data in the `rytm` external

This is done by starting our messages with `set` selector.
The details of the format and the input format are explained in the [api docs](API_DOCS.md).

#### Parameter locking

The pattern structure holds the parameter lock pool for that pattern.

Think of it as a pool where you can store parameter locks, set or clear them.

You have a space of 72 different parameter locks for every trig in a pattern.

If you exceed that the memory will be full.

There will be detailed documentation about this in the near future about how it is handled internally and how does sound and kit structures link to a certain pattern's parameter lock pool.

#### Look out

Do not query or send data to the device in a perpetual way without the minimum of a 750-800 ms interval. This is not dangerous but the data is usually large and it is processed in the low priority thread of the device. The device may queue the responses or requests and may not be able to process them in time.

When you send the initial `sysex` message to the device for the first time after power on it usually responds with an irrelevant message and never does it again.

This will appear as an error in the max window called "short read" you may safely ignore this error.

## Todo

Setting, getting and parameter locking machine parameters is not yet implemented.

There are no technical obstacle there but it takes good amount of time due to the large amount of machines.
I'll do it some time later.

Since the hard work is done it is easy to wrap this as a pd external also. I'll do that some time.

## Contributing

I am aware that this is a very niche project, did by a single person and the code base is not very well documented.

On the other hand I still think that an experienced developer in the field may follow it easily.

I'm always open to contributions and I'll do my best to help you understand the code base and the project.

Rust [code of conduct](https://www.rust-lang.org/policies/code-of-conduct) applies to this project. Please be nice and respectful.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
Use it as you wish but if you do something commercial and make a lot of money which is not very likely please consider giving some of that to me also :smile:.
