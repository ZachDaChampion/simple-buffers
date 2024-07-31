# Introduction

SimpleBuffers is a schema language and compiler for data serialization. Like
[protobuf](https://protobuf.dev/), it is used to generate code in various languages that encodes and
decodes data structures in a common format. Unlike protobuf, SimpleBuffers is designed for
consistent APIs in resource-constrained environments. It forgoes some backwards-compatibility in
order to increase efficiency in both storage density and encoding/decoding speed. In fact,
SimpleBuffers data can be decoded lazily, often in constant-time. For more information about how
this is done, see [Serialization Format](serialization_format.md).

SimpleBuffers has an extremely similar serialization scheme to
[Cap'n Proto](https://capnproto.org/). I made this project independently before investigating Cap'n
Proto's inner workings, and while I prefer some aspects of my C++ API, I highly recommend using
Cap'n Proto over SimpleBuffers for any serious project. It is more established, more complete, and
will be far better supported.
