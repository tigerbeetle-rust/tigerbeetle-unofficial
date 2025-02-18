## Upgrading to new [TigerBeetle] version

First, checkout Git submodule to the desired version:
```bash
cd sys/tigerbeetle
git fetch origin 'refs/tags/*:refs/tags/*'
git checkout 0.15.3
```

Then, update `TIGERBEETLE_RELEASE` and `TIGERBEETLE_VERSION` in the [build script](./sys/build.rs).
```rust
pub const TIGERBEETLE_RELEASE: &str = "0.15.3";
pub const TIGERBEETLE_COMMIT: &str = "73bbc1a32ba2513e369764680350c099fe302285";
```

Finally, provide changes supporting new [TigerBeetle] version.




[TigerBeetle]: https://tigerbeetle.com
