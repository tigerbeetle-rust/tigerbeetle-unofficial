`tigerbeetle-unofficial` changelog
==================================

All user visible changes to `tigerbeetle-unofficial`, `tigerbeetle-unofficial-core` and `tigerbeetle-unofficial-sys` crates will be documented in this file. This project uses [Semantic Versioning 2.0.0].




## [0.7.1+0.16.21] · 2025-01-14
[0.7.1+0.16.21]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/tree/v0.7.1%2B0.16.21

[Diff](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/compare/v0.7.0%2B0.16.20...v0.7.1%2B0.16.21) | [Milestone](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/milestone/6)

### Changed

- Upgraded [`tb_client` C library] to [0.16.21 version][tb-0.16.21]. ([#43])

[#43]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/pull/43
[tb-0.16.21]: https://github.com/tigerbeetle/tigerbeetle/blob/0.16.21/CHANGELOG.md#tigerbeetle-01621




## [0.7.0+0.16.20] · 2024-12-30
[0.7.0+0.16.20]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/tree/v0.7.0%2B0.16.20

[Diff](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/compare/v0.6.1%2B0.16.19...v0.7.0%2B0.16.20) | [Milestone](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/milestone/4)

### BC Breaks

- Removed MIT license. ([#41])

### Changed

- Upgraded [`tb_client` C library] to [0.16.20 version][tb-0.16.20]. ([#42])

[#41]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/pull/41
[#42]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/pull/42
[tb-0.16.20]: https://github.com/tigerbeetle/tigerbeetle/blob/0.16.20/CHANGELOG.md#tigerbeetle-01620




## [0.6.1+0.16.19] · 2024-12-25
[0.6.1+0.16.19]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/tree/v0.6.1%2B0.16.19

[Diff](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/compare/v0.6.0%2B0.16.17...v0.6.1%2B0.16.19) | [Milestone](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/milestone/3)

### Added

- `core` and main crates:
    - `error::SendErrorKind::ClientReleaseTooLow` and `error::SendErrorKind::ClientReleaseTooHigh` variants. ([#40], [tigerbeetle/tigerbeetle#2552])
- `sys` crate:
    - `TB_PACKET_STATUS::TB_PACKET_CLIENT_RELEASE_TOO_LOW` and `TB_PACKET_STATUS::TB_PACKET_CLIENT_RELEASE_TOO_HIGH` constants. ([#40], [tigerbeetle/tigerbeetle#2552])
    - `PacketStatusErrorKind::ClientReleaseTooLow` and `PacketStatusErrorKind::ClientReleaseTooHigh` variants. ([#40], [tigerbeetle/tigerbeetle#2552])

### Changed

- Upgraded [`tb_client` C library] to [0.16.19 version][tb-0.16.19]. ([#40])

[#40]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/pull/40
[tb-0.16.19]: https://github.com/tigerbeetle/tigerbeetle/blob/0.16.19/CHANGELOG.md#tigerbeetle-01619
[tigerbeetle/tigerbeetle#2552]: https://github.com/tigerbeetle/tigerbeetle/pull/2552




## [0.6.0+0.16.17] · 2024-12-19
[0.6.0+0.16.17]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/tree/v0.6.0%2B0.16.17

[Diff](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/compare/v0.5.0%2B0.16.11...v0.6.0%2B0.16.17) | [Milestone](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/milestone/2)

### BC Breaks

- Upgraded [`tb_client` C library] to [0.16.17 version][tb-0.16.17]: ([#38])
    - Replaced `payload` argument with `reply` in `core::Callbacks::on_competion()` to provide cluster `timestamp` of `Reply` generation. ([tigerbeetle/tigerbeetle#2481])
    - Replaced `TIGERBEETLE_LOG_LEVEL` build time env var with `TB_CLIENT_DEBUG` one, since `config-log-level` build option was removed, but no FFI yet added for configuring runtime log filtering. ([tigerbeetle/tigerbeetle#2539])

### Added

- `SendErrorKind::ClientEvicted` variant. ([#38], [tigerbeetle/tigerbeetle#2484])
- `id()` function generating [TigerBeetle Time-Based Identifiers](https://github.com/tigerbeetle/tigerbeetle/blob/0.16.17/docs/coding/data-modeling.md#tigerbeetle-time-based-identifiers-recommended). ([#39])

[#38]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/pull/38
[#39]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/pull/39
[tb-0.16.17]: https://github.com/tigerbeetle/tigerbeetle/blob/0.16.17/CHANGELOG.md#tigerbeetle-01617
[tigerbeetle/tigerbeetle#2539]: https://github.com/tigerbeetle/tigerbeetle/pull/2539
[tigerbeetle/tigerbeetle#2481]: https://github.com/tigerbeetle/tigerbeetle/pull/2481
[tigerbeetle/tigerbeetle#2484]: https://github.com/tigerbeetle/tigerbeetle/pull/2484




## [0.5.0+0.16.11] · 2024-12-02
[0.5.0+0.16.11]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/tree/v0.5.0%2B0.16.11

[Diff](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/compare/v0.4.1%2B0.15.3...v0.5.0%2B0.16.11) | [Milestone](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/milestone/1)

### BC Breaks

- Upgraded [`tb_client` C library] to [0.16.11 version][tb-0.16.11]. ([#24], [#19], [#18])
- Removed `concurrency_max` argument from `Client::new()`, `Client::with_callback()` and `Client::with_callback_unchecked()` methods. ([#24], [#19])
- Replaced `Client::acquire()` and `ClientHandle::acquire()` methods with `Client::packet()` and `Packet::new()`. ([#24], [#19], [#34])
- Removed `error::AcquirePacketError` type. ([#24], [#19])

### Added

- `TIGERBEETLE_LOG_LEVEL` env var for setting `config-log-level` when building (default is `info`). ([#24], [#19])
- `QueryFilter` and `query_filter::Raw` types. ([#26])
- `Client::query_accounts()` and `Client::query_transfers()` methods. ([#26])

### Fixed

- Broken builds inside environments without [Git] (like [Docker] image). ([#23], [#20])

[#18]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/issues/18
[#19]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/pull/19
[#20]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/issues/20
[#23]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/pull/23
[#24]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/pull/24
[#26]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/pull/26
[#34]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/pull/34
[tb-0.16.11]: https://github.com/tigerbeetle/tigerbeetle/blob/0.16.11/CHANGELOG.md#tigerbeetle-01611




## [0.4.1+0.15.3] · 2024-07-28
[0.4.1+0.15.3]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/tree/v0.4.1%2B0.15.3

[Diff](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/compare/v0.4.0%2B0.15.4...v0.4.1%2B0.15.3)

See the [release notes][release-0.4.1+0.15.3].

[release-0.4.1+0.15.3]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/releases/tag/v0.4.1%2B0.15.3




## [0.4.0+0.15.3] · 2024-07-13
[0.4.0+0.15.3]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/tree/v0.4.0%2B0.15.4

[Diff](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/compare/v0.3.0%2B0.13.133...v0.4.0%2B0.15.4)

See the [release notes][release-0.4.0+0.15.3].

[release-0.4.0+0.15.3]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/releases/tag/v0.4.0%2B0.15.4




## [0.3.0+0.13.133] and prior
[0.3.0+0.13.133]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/tree/v0.3.0%2B0.13.133

See [Git log](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/compare/a4994b2da3914352b8d64adae0535189b4bc7b27...v0.3.0%2B0.13.133).




[`tb_client` C library]: https://github.com/tigerbeetle/tigerbeetle/tree/main/src/clients/c
[Docker]: https://www.docker.com
[Git]: https://git-scm.com
[MSRV]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field
[Semantic Versioning 2.0.0]: https://semver.org
