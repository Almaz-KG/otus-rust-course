## Torrentino


This is a blazingly fast bit torrent client written in pure and ideomatic Rust. Torrentino project is the final part
of  the [Otus Rust Developer Course](https://otus.ru/lessons/rust-developer/).


### The types of deliveries

The result of this project can be delivered to end used in different types. These types will vary by the interaction
with end user, and the envirounment arount the application. There are three types, but only cli options as marked as
`must to have` due the limit of time for project implementation. Optional delivery types might be implemented (or
might not be implemented) depending on remaining time till the end of the course.

1. [must to have] cli client for Macos
1. [optional] desktop application for Macos
1. [optional] web app (with backend and front-end)

### Must to have feature list

As the header of this section states, the following list contains the major features, without them the result of the
project will be useless. These features can be considered as first priority tasks.

1. open and display given *.torrent file internals
1. interact with torrent-server
1. download and save file from torrent-peers

### Nice to have feature list

- [] make parallel downloads
- [] download part of torrent files
- [] act as torrent-peer for the downloaded files
- [] pause and continue download process
- [] continue download process after app close
- [] for cli version provide possibility to install via homebrew (or apt-get) [Instruction](https://docs.brew.sh/Adding-Software-to-Homebrew#casks)

The future list of this project will be updated during the course, and in the process of implementation. It should be
 a complete product, which allows to end user have a possibility to download any data from public servers, which
 support bit-torrent protocol. Many ideas and inspiration was taken from
 [How to make your own bittorrent client](https://allenkim67.github.io/programming/2016/05/04/how-to-make-your-own-bittorrent-client.html#opening-the-torrent-file) guide

### Resources
1. [BitTorrentSpecification](https://wiki.theory.org/BitTorrentSpecification) - Detailed and very readable unofficial
bittorrent specification
1. [UDP Tracker Protocol for BitTorrent](http://www.bittorrent.org/beps/bep_0015.html) - An official specification of
bit torrent protocol. I recommend to read unofficial one first, just because is more understandable from newbies
point of view.
1. [The BitTorrent Protocol](https://www.morehawes.co.uk/old-guides/the-bittorrent-protocol) Another good high level explanation of the BitTorrent protocol
