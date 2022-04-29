# En-Codex

A command line tool and library for decoding and encoding data as `Base64`, `Base64url`,
`Base32`(todo), `Base32hex`(todo) and `Base16`(todo) encodings as defined in _**RFC 4648**_. For
information about how to use this program please use `cargo doc` from within your project folder.

---
## Todo

- Implement `Base32` en- and decoding
- Implement `Base16` en- and decoding
- Option for adding line feed after a certain amount of characters (_**RFC 4648** - 3.1_)
- Option for conditionally not adding padding bytes? (_**RFC 4648** - 3.2_)
- Option for ignoring non-alphabet characters in base encoded data? (_**RFC 4648** - 3.3_)
- Option to change used alphabet for specific uses (_**RFC 4648** - 3.4_)
- Implement `Base32hex` en- and decoding
- Option to choose between upper- and lowercase for `Base32` and `Base32hex`.
- Implement test vectors:
    * `Base64` (_from **RFC 4648** - 10_)
    * `Base64url`
    * `Base32` (_from **RFC 4648** - 10_)
    * `Base32hex` (_from **RFC 4648** - 10_)
    * `Base16` (_from **RFC 4648** - 10_)
---
###### License
This program is released unter the _GNU Lesser General Public License_. See _[COPYING](./COPYING)_
and _[COPYING.LESSER](./COPYING.LESSER)_ for additional information about the license.

###### Contact
For any other question contact me at
<i>[fabian.moos@moosegamesdev.org](mailto:fabian.moos@moosegamesdev.org)</i>.
