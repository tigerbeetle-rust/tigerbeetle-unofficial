`tigerbeetle-unofficial` changelog
==================================

All user visible changes to `tigerbeetle-unofficial`, `tigerbeetle-unofficial-core` and `tigerbeetle-unofficial-sys` crates will be documented in this file. This project uses [Semantic Versioning 2.0.0].




## [0.5.0+0.16.11] · 2024-12-02
[0.5.0+0.16.11]: /../../tree/v0.5.0%2B0.16.11

[Diff](/../../compare/v0.4.1%2B0.15.3...v0.5.0%2B0.16.11) | [Milestone](/../../milestone/1)

### BC Breaks

- Upgraded [`tb_client` C library] to [0.16.1 version][tb-0.16.1]. ([#24], [#19], [#18])
- Removed `concurrency_max` argument from `Client::new()`, `Client::with_callback()` and `Client::with_callback_unchecked()` methods. ([#24], [#19])
- Replaced `Client::acquire()` and `ClientHandle::acquire()` methods with `Client::packet()` and `Packet::new()`. ([#24], [#19], [#34])
- Removed `error::AcquirePacketError` type. ([#24], [#19])

### Added

- `TIGERBEETLE_LOG_LEVEL` env var for setting `config-log-level` when building (default is `info`). ([#24], [#19])
- `QueryFilter` and `query_filter::Raw` types. ([#26])
- `Client::query_accounts()` and `Client::query_transfers()` methods. ([#26])

### Fixed

- Broken builds inside environments without [Git] (like [Docker] image). ([#23], [#20])

[#18]: /../../issues/18
[#19]: /../../pull/19
[#20]: /../../issues/20
[#23]: /../../pull/23
[#24]: /../../pull/24
[#26]: /../../pull/26
[#34]: /../../pull/34
[tb-0.16.1]: https://github.com/tigerbeetle/tigerbeetle/blob/0.16.11/CHANGELOG.md#tigerbeetle-01611




## [0.4.1+0.15.3] · 2024-07-28
[0.4.1+0.15.3]: /../../tree/v0.4.1%2B0.15.3

[Diff](/../../compare/v0.4.0%2B0.15.4...v0.4.1%2B0.15.3)

See the [release notes][release-0.4.1+0.15.3].

[release-0.4.1+0.15.3]: /../../releases/tag/v0.4.1%2B0.15.3




## [0.4.0+0.15.3] · 2024-07-13
[0.4.0+0.15.3]: /../../tree/v0.4.0%2B0.15.4

[Diff](/../../compare/v0.3.0%2B0.13.133...v0.4.0%2B0.15.4)

See the [release notes][release-0.4.0+0.15.3].

[release-0.4.0+0.15.3]: /../../releases/tag/v0.4.0%2B0.15.4




## [0.3.0+0.13.133] and prior
[0.3.0+0.13.133]: /../../tree/v0.3.0%2B0.13.133

See [Git log](/../../compare/a4994b2da3914352b8d64adae0535189b4bc7b27...v0.3.0%2B0.13.133).




[`tb_client` C library]: https://github.com/tigerbeetle/tigerbeetle/tree/main/src/clients/c
[Docker]: https://www.docker.com
[Git]: https://git-scm.com
[Semantic Versioning 2.0.0]: https://semver.org
