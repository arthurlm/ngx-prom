# Option use to reduce binary size

default
=> 11M

opt = s
=> 11M

opt = 3, lto = fat
=> 7.3M
strip
=> 4.3M

opt = 3, lto = fat, codegen = 1
=> 6.9M
strip
=> 4.6M

opt = 3, lto = fat, codegen = 1, panic = abort
=> 6.2M
strip
=> 4.2M

opt = 3, lto = fat, codegen = 1, panic = abort + xargo
=> 4.4M
strip
=> 3.8M

opt = z, lto = fat, codegen = 1, panic = abort + xargo
=> 4.0M
strip
=> 2.8M
