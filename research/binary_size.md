# Option use to reduce binary size

Here a comparison of different build option to try having smaller binary release.

Commit ID = `96a6c92317b2bf04cc1894386adc952c89398944`

| Build options                                         | Size | Strip | UPX  | UPX -9 | Production ready |
| ----------------------------------------------------- | ---- | ----- | ---- | ------ | ---------------- |
| default                                               | 11M  | 5.7M  | 1.7M |        | ✔️               |
| opt = s                                               | 11M  | 5.4M  | 1.6M |        |                  |
| opt = 3, lto = fat                                    | 7.3M | 4.8M  | 1.5M |        |                  |
| opt = 3, lto = fat, codegen = 1                       | 6.9M | 4.6M  | 1.4M |        | ✔️               |
| opt = 3, lto = fat, codegen = 1, panic = abort        | 6.2M | 4.2M  | 1.3M |        | ❌               |
| opt = 3, lto = fat, codegen = 1, panic = abort, xargo | 4.4M | 3.8M  | 1.2M |        | ❌               |
| opt = z, lto = fat, codegen = 1, panic = abort, xargo | 4.0M | 2.8M  | 845K | 831K   | ❌               |

## External links

- [UPX](https://github.com/upx/upx)
