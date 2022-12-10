Trying to get something like
[SoftwareSerial](https://docs.arduino.cc/learn/built-in-libraries/software-serial)
to work in order to use my ESP8266 (ESP01?). `bitbang-hal` seems buggy since
there's two PRs and one issue about its serial implementation. With one of the
PRs applied I can send all bytes `0x00..=0xFF` to the connected computer over
the serial connection (`cargo run` if you wish to try yourself) so at least the
timing code seems to work haha.

Also the behaviour changes between nightly-2022-11-19 and nightly-2022-11-20.
