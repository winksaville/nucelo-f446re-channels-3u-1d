# nucleo-f446re-channeld-3u-1d

This is basically [all-the-channels](https://probe.rs/docs/tools/cargo-embed/#all-the-channels!)
from probe-rs docs. I've tweaked it slightly adding delay, although ATM I was expecting to see
the logs written to "Up two" tab, but it's blank?

# Pre-requisites

TODO: Add pre-requisites

# Build and run using cargo-embed

Run `cargo embed with_rtt` to build and flash the program to the target
and the result should be the initial blank screen with 3 tabs:

![cargo-embed with_rtt initial screen](Up-zero-initial-screen.png)

I then type "First line" but don't hit enter, so in the "blue" input line
at the bottom of the screen you see "First line":
![cargo-embed with_rtt up zero typed frist line no return](Up-zero-typed-first-line-no-return.png)

I then hit return and in "Up zero" we see "First line":
![cargo-embed with_rtt up zero frist line](Up-zero-first-line.png)

Next I type "Second line" and hit return in the "blue" input line, so in the "Up zero" we now
see the First and Second line:
![cargo-embed with_rtt up zero second line](Up-zero-second-line.png)

If you click on the F2 (function key, on my keyboard I press Ctrl-F2) the tab will switch to "Up one":
![cargo-embed with_rtt up one](Up-one.png)

**Note for some reason the "Up two" tab, which should be F3 cannot be selected :(**
But you can switch back and forth between "Up zero" and "Up one" using F1 and F2 (Ctrl-F1 & Ctrl-F2).

After hitting `Ctrl-C` to stop the program, the output should look like this:
```
wink@3900x 24-01-21T21:21:28.440Z:~/prgs/rust/myrepos/nucleo-f446re-channels-3u-1d (main)
$ cargo embed with_rtt
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
      Config with_rtt
      Target /home/wink/prgs/rust/myrepos/nucleo-f446re-channels-3u-1d/target/thumbv7em-none-eabi/debug/channels-3u-1d
     Erasing sectors ✔ [00:00:00] [#######################] 16.00 KiB/16.00 KiB @ 40.15 KiB/s (eta 0s )
 Programming pages   ✔ [00:00:00] [#######################] 13.00 KiB/13.00 KiB @ 37.54 KiB/s (eta 0s )    Finished flashing in 0.763s
Shutting down.
wink@3900x 24-01-21T21:25:31.293Z:~/prgs/rust/myrepos/nucleo-f446re-channels-3u-1d (main)```
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
