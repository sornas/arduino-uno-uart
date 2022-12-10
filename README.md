Trying to get something like
[SoftwareSerial](https://docs.arduino.cc/learn/built-in-libraries/software-serial)
to work in order to use my ESP8266 (ESP01?). `bitbang-hal` seems buggy since
there's two PRs and one issue about its serial implementation. With one of the
PRs applied I can send all bytes `0x00..=0xFF` to the connected computer over
the serial connection (`cargo run` if you wish to try yourself) so at least the
timing code seems to work haha.

Also the behaviour changes between nightly-2022-11-19 and nightly-2022-11-20.

If reads are fixed, there's a few other things that would be nice:

- Handle more baudrates. At least some standard ones, but it would be cool to
  work with arbitrary ones.
- Can we work with any counter?
- Can we be more generic? For example, a crate that wraps an arbitrary timer and
  gives a (slower) arbitrary timer, handling "fractions" correctly.
- A buffered reader that works with interrupts and a (const generic-sized?)
  buffer.
