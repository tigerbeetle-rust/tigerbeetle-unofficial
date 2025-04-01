`tigerbeetle-unofficial` changelog
==================================

All user visible changes to `tigerbeetle-unofficial`, `tigerbeetle-unofficial-core` and `tigerbeetle-unofficial-sys` crates will be documented in this file. This project uses [Semantic Versioning 2.0.0].




## [0.10.1+0.16.34] · 2025-04-01
[0.10.1+0.16.34]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/tree/v0.10.1%2B0.16.34

[Diff](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/compare/v0.10.0%2B0.16.33...v0.10.1%2B0.16.34) | [Milestone](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/milestone/17)

### Changed

- Upgraded [`tb_client` C library] to [0.16.34 version][tb-0.16.34]. ([#58])

[#58]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/pull/58
[tb-0.16.34]: https://github.com/tigerbeetle/tigerbeetle/blob/0.16.34/CHANGELOG.md#tigerbeetle-01634




## [0.10.0+0.16.33] · 2025-03-25
[0.10.0+0.16.33]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/tree/v0.10.0%2B0.16.33

[Diff](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/compare/v0.9.3%2B0.16.32...v0.10.0%2B0.16.33) | [Milestone](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/milestone/16)

### BC Breaks

- `sys` crate:
    - Removed `TB_CREATE_TRANSFER_AMOUNT_MUST_NOT_BE_ZERO` value from `TB_CREATE_TRANSFER_RESULT` enumeration. ([#57], [tigerbeetle/tigerbeetle#2824])
    - `generated_safe`:
        - Removed `AmountMustNotBeZero` variant from `CreateTransferErrorKind` enum. ([#57], [tigerbeetle/tigerbeetle#2824])
- `core` crate:
    - Removed `AmountMustNotBeZero` variant from `error::CreateTransferErrorKind` enum. ([#57], [tigerbeetle/tigerbeetle#2824])
- Main crate:
    - Removed `AmountMustNotBeZero` variant from `error::CreateTransferErrorKind` enum. ([#57], [tigerbeetle/tigerbeetle#2824])

### Added

- `sys` crate:
    - `generated_safe`:
        - `EXCLUDED_CREATE_TRANSFER_ERROR_CODES` constant holding the code of the removed `CreateTransferErrorKind::AmountMustNotBeZero` variant. ([#57])

### Changed

- Upgraded [`tb_client` C library] to [0.16.33 version][tb-0.16.33]. ([#57])

[#57]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/pull/57
[tb-0.16.33]: https://github.com/tigerbeetle/tigerbeetle/blob/0.16.33/CHANGELOG.md#tigerbeetle-01633
[tigerbeetle/tigerbeetle#2824]: https://github.com/tigerbeetle/tigerbeetle/pull/2824




## [0.9.3+0.16.32] · 2025-03-20
[0.9.3+0.16.32]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/tree/v0.9.3%2B0.16.32

[Diff](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/compare/v0.9.2%2B0.16.31...v0.9.3%2B0.16.32) | [Milestone](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/milestone/15)

### Changed

- Upgraded [`tb_client` C library] to [0.16.32 version][tb-0.16.32]. ([#56])

[#56]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/pull/56
[tb-0.16.32]: https://github.com/tigerbeetle/tigerbeetle/blob/0.16.32/CHANGELOG.md#tigerbeetle-01632




## [0.9.2+0.16.31] · 2025-03-20
[0.9.2+0.16.31]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/tree/v0.9.2%2B0.16.31

[Diff](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/compare/v0.9.1%2B0.16.30...v0.9.2%2B0.16.31) | [Milestone](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/milestone/14)

### Changed

- Upgraded [`tb_client` C library] to [0.16.31 version][tb-0.16.31]. ([#54])

[#54]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/pull/54
[tb-0.16.31]: https://github.com/tigerbeetle/tigerbeetle/blob/0.16.31/CHANGELOG.md#tigerbeetle-01631




## [0.9.1+0.16.30] · 2025-03-20
[0.9.1+0.16.30]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/tree/v0.9.1%2B0.16.30

[Diff](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/compare/v0.9.0%2B0.16.29...v0.9.1%2B0.16.30) | [Milestone](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/milestone/13)

### Changed

- Upgraded [`tb_client` C library] to [0.16.30 version][tb-0.16.30]. ([#52])

[#52]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/pull/52
[tb-0.16.30]: https://github.com/tigerbeetle/tigerbeetle/blob/0.16.30/CHANGELOG.md#tigerbeetle-01630




## [0.9.0+0.16.29] · 2025-03-20
[0.9.0+0.16.29]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/tree/v0.9.0%2B0.16.29

[Diff](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/compare/v0.8.0%2B0.16.28...v0.9.0%2B0.16.29) | [Milestone](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/milestone/12)

### BC Breaks

- `sys` crate:
    - Renamed `TB_STATUS` enumeration as `TB_INIT_STATUS`. ([#49], [tigerbeetle/tigerbeetle#2742])
    - Transformed `tb_client_t` into struct with `opaque` field. ([#49], [tigerbeetle/tigerbeetle#2742])
    - `tb_packet_t`: ([#49], [tigerbeetle/tigerbeetle#2742])
        - Renamed `tag` field as `user_tag`.
        - Renamed `reserved` field as `opaque`.
    - `tb_client_deinit()`: ([#49], [tigerbeetle/tigerbeetle#2742])
        - Changed return type to `TB_CLIENT_STATUS`.
        - Changed `client` argument to `*mut tb_client_t`.
    - `tb_client_submit()`: ([#49], [tigerbeetle/tigerbeetle#2742])
        - Changed return type to `TB_CLIENT_STATUS`.
        - Changed `client` argument to `*mut tb_client_t`.
    - `tb_client_completion_context()`: ([#49], [tigerbeetle/tigerbeetle#2742])
        - Changed return type to `TB_CLIENT_STATUS`.
        - Changed `client` argument to `*mut tb_client_t`.
        - Added `completion_ctx_out: *mut usize` argument.
    - `tb_client_init()` and `tb_client_init_echo()`: ([#49], [tigerbeetle/tigerbeetle#2742])
        - Removed second `tb_client_t` argument from `completion_callback` (was `on_completion`) argument.
- `core` crate:
    - Renamed `Callbacks::on_completion()` as `Callbacks::completion()` to match [`tb_client` C library] naming. ([#49])
    - Removed lifetime parameter from `Packet`. ([#49])
    - Removed `ClientHandle`, `Client::handle()`, `Packet::client_handle()` and `handle` argument of `Packet::new()`. ([#49])
    - Remade `Packet::submit()` into `Client::submit()`. ([#49])
    - Removed `Client::packet()` (use `Packet::new()` instead). ([#49])
- Main crate:
    - Removed lifetime parameter from `Packet`. ([#49]) 

### Added

- `sys` crate:
    - `TB_OPERATION_GET_EVENTS` value to `TB_OPERATION` enumeration. ([#49], [tigerbeetle/tigerbeetle#2507])
    - `TB_CLIENT_STATUS` enumeration. ([#49], [tigerbeetle/tigerbeetle#2742])
    - `TB_REGISTER_LOG_CALLBACK_STATUS` enumeration. ([#49], [tigerbeetle/tigerbeetle#2742])
    - `TB_LOG_LEVEL` enumeration. ([#49], [tigerbeetle/tigerbeetle#2742])
    - `tb_client_register_log_callback()` function. ([#49], [tigerbeetle/tigerbeetle#2742])
- `core` crate:
    - `OperationKind::GetEvents` variant. ([#49])

### Changed

- Upgraded [`tb_client` C library] to [0.16.29 version][tb-0.16.29]. ([#49])

[#49]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/pull/49
[tb-0.16.29]: https://github.com/tigerbeetle/tigerbeetle/blob/0.16.29/CHANGELOG.md#tigerbeetle-01629
[tigerbeetle/tigerbeetle#2507]: https://github.com/tigerbeetle/tigerbeetle/pull/2507
[tigerbeetle/tigerbeetle#2742]: https://github.com/tigerbeetle/tigerbeetle/pull/2742




## [0.8.0+0.16.28] · 2025-02-18
[0.8.0+0.16.28]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/tree/v0.8.0%2B0.16.28

[Diff](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/compare/v0.7.5%2B0.16.27...v0.8.0%2B0.16.28) | [Milestone](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/milestone/11)

### BC Breaks

- `sys` crate:
    - `tb_packet_t`: ([#48], [tigerbeetle/tigerbeetle#2728])
        - `next`, `batch_next`, `batch_tail`, `batch_size` and `batch_allowed` fields were hidden into opaque `reserved` field.
        - Added `tag` field.

### Changed

- Upgraded [`tb_client` C library] to [0.16.28 version][tb-0.16.28]. ([#48])

[#48]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/pull/48
[tb-0.16.28]: https://github.com/tigerbeetle/tigerbeetle/blob/0.16.28/CHANGELOG.md#tigerbeetle-01628
[tigerbeetle/tigerbeetle#2728]: https://github.com/tigerbeetle/tigerbeetle/pull/2728




## [0.7.5+0.16.27] · 2025-02-11
[0.7.5+0.16.27]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/tree/v0.7.5%2B0.16.27

[Diff](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/compare/v0.7.4%2B0.16.26...v0.7.5%2B0.16.27) | [Milestone](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/milestone/10)

### Changed

- Upgraded [`tb_client` C library] to [0.16.27 version][tb-0.16.27]. ([#47])

[#47]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/pull/47
[tb-0.16.27]: https://github.com/tigerbeetle/tigerbeetle/blob/0.16.27/CHANGELOG.md#tigerbeetle-01627




## [0.7.4+0.16.26] · 2025-02-04
[0.7.4+0.16.26]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/tree/v0.7.4%2B0.16.26

[Diff](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/compare/v0.7.3%2B0.16.25...v0.7.4%2B0.16.26) | [Milestone](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/milestone/9)

### Changed

- Upgraded [`tb_client` C library] to [0.16.26 version][tb-0.16.26]. ([#46])

[#46]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/pull/46
[tb-0.16.26]: https://github.com/tigerbeetle/tigerbeetle/blob/0.16.26/CHANGELOG.md#tigerbeetle-01626




## [0.7.3+0.16.25] · 2025-01-28
[0.7.3+0.16.25]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/tree/v0.7.3%2B0.16.25

[Diff](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/compare/v0.7.2%2B0.16.23...v0.7.3%2B0.16.25) | [Milestone](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/milestone/8)

### Changed

- Upgraded [`tb_client` C library] to [0.16.25 version][tb-0.16.25]. ([#45])

[#45]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/pull/45
[tb-0.16.25]: https://github.com/tigerbeetle/tigerbeetle/blob/0.16.25/CHANGELOG.md#tigerbeetle-01625




## [0.7.2+0.16.23] · 2025-01-21
[0.7.2+0.16.23]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/tree/v0.7.2%2B0.16.23

[Diff](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/compare/v0.7.1%2B0.16.21...v0.7.2%2B0.16.23) | [Milestone](https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/milestone/7)

### Changed

- Upgraded [`tb_client` C library] to [0.16.23 version][tb-0.16.23]. ([#44])

[#44]: https://github.com/tigerbeetle-rust/tigerbeetle-unofficial/pull/44
[tb-0.16.23]: https://github.com/tigerbeetle/tigerbeetle/blob/0.16.23/CHANGELOG.md#tigerbeetle-01623




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
