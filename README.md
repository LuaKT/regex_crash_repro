Output
```bash
[lua@desktop regex_crash2]$ RUST_BACKTRACE=full cargo run --release
    Finished `release` profile [optimized + debuginfo] target(s) in 0.00s
     Running `target/release/regex_crash`
Match { start: 287919, end: 287954, bytes: "chromedriver.storage.googleapis.com" }
Match { start: 608672, end: 608711, bytes: "selenium-release.storage.googleapis.com" }
Match { start: 2279253, end: 2279288, bytes: "chromedriver.storage.googleapis.com" }
Match { start: 2610045, end: 2610084, bytes: "selenium-release.storage.googleapis.com" }
Match { start: 6669144, end: 6669179, bytes: "chromedriver.storage.googleapis.com" }
Match { start: 6991107, end: 6991146, bytes: "selenium-release.storage.googleapis.com" }
Match { start: 108206541, end: 108206583, bytes: "storage.googleapis.com/chrome_hats_staging" }
thread 'main' panicked at /home/lua/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-automata-0.4.7/src/hybrid/dfa.rs:2601:9:
invalid 'from' id: LazyStateID(134217664)
stack backtrace:
   0:     0x64ae56644a05 - std::backtrace_rs::backtrace::libunwind::trace::hd0a431ec4286eec2
                               at /rustc/bcf94dec5ba6838e435902120c0384c360126a26/library/std/src/../../backtrace/src/backtrace/libunwind.rs:116:5
   1:     0x64ae56644a05 - std::backtrace_rs::backtrace::trace_unsynchronized::h55b7147bbdcae103
                               at /rustc/bcf94dec5ba6838e435902120c0384c360126a26/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x64ae56644a05 - std::sys::backtrace::_print_fmt::h5ba811d5fa76665c
                               at /rustc/bcf94dec5ba6838e435902120c0384c360126a26/library/std/src/sys/backtrace.rs:68:5
   3:     0x64ae56644a05 - <std::sys::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc7ed570e71131300
                               at /rustc/bcf94dec5ba6838e435902120c0384c360126a26/library/std/src/sys/backtrace.rs:44:22
   4:     0x64ae56662c9b - core::fmt::rt::Argument::fmt::hd5f04252949343fb
                               at /rustc/bcf94dec5ba6838e435902120c0384c360126a26/library/core/src/fmt/rt.rs:165:63
   5:     0x64ae56662c9b - core::fmt::write::h525edfe35df838b2
                               at /rustc/bcf94dec5ba6838e435902120c0384c360126a26/library/core/src/fmt/mod.rs:1168:21
   6:     0x64ae5664280f - std::io::Write::write_fmt::h7ca43b42bd752649
                               at /rustc/bcf94dec5ba6838e435902120c0384c360126a26/library/std/src/io/mod.rs:1835:15
   7:     0x64ae566447de - std::sys::backtrace::_print::h5374f6e608e081ce
                               at /rustc/bcf94dec5ba6838e435902120c0384c360126a26/library/std/src/sys/backtrace.rs:47:5
   8:     0x64ae566447de - std::sys::backtrace::print::ha7453f9fb23375ba
                               at /rustc/bcf94dec5ba6838e435902120c0384c360126a26/library/std/src/sys/backtrace.rs:34:9
   9:     0x64ae56645b79 - std::panicking::default_hook::{{closure}}::hf9505cfe32528ed7
  10:     0x64ae5664591c - std::panicking::default_hook::ha7962b8515cde474
                               at /rustc/bcf94dec5ba6838e435902120c0384c360126a26/library/std/src/panicking.rs:292:9
  11:     0x64ae56646071 - std::panicking::rust_panic_with_hook::hea4c878dc07df55b
                               at /rustc/bcf94dec5ba6838e435902120c0384c360126a26/library/std/src/panicking.rs:796:13
  12:     0x64ae56645f67 - std::panicking::begin_panic_handler::{{closure}}::hca858b01da723ad6
                               at /rustc/bcf94dec5ba6838e435902120c0384c360126a26/library/std/src/panicking.rs:670:13
  13:     0x64ae56644ec9 - std::sys::backtrace::__rust_end_short_backtrace::h94b285c94ce95409
                               at /rustc/bcf94dec5ba6838e435902120c0384c360126a26/library/std/src/sys/backtrace.rs:171:18
  14:     0x64ae56645c44 - rust_begin_unwind
                               at /rustc/bcf94dec5ba6838e435902120c0384c360126a26/library/std/src/panicking.rs:661:5
  15:     0x64ae56661ae3 - core::panicking::panic_fmt::h47b1e0ddaafb858d
                               at /rustc/bcf94dec5ba6838e435902120c0384c360126a26/library/core/src/panicking.rs:74:14
  16:     0x64ae5658d7d8 - regex_automata::hybrid::dfa::Lazy::set_transition::ha7e8fe70de53f06e
                               at /home/lua/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-automata-0.4.7/src/hybrid/dfa.rs:2601:9
  17:     0x64ae5658adb9 - regex_automata::hybrid::dfa::Lazy::cache_next_state::h4565c8371fc6bb4d
                               at /home/lua/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-automata-0.4.7/src/hybrid/dfa.rs:2146:9
  18:     0x64ae56599841 - regex_automata::hybrid::dfa::DFA::next_state::hfda518ade44ed042
                               at /home/lua/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-automata-0.4.7/src/hybrid/dfa.rs:1228:9
  19:     0x64ae56599841 - regex_automata::hybrid::search::find_fwd_imp::h417d6ad5d3651efd
                               at /home/lua/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-automata-0.4.7/src/hybrid/search.rs:227:23
  20:     0x64ae56599841 - regex_automata::hybrid::search::find_fwd::ha9b1fafadfb2f610
                               at /home/lua/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-automata-0.4.7/src/hybrid/search.rs:44:13
  21:     0x64ae56555410 - regex_automata::hybrid::dfa::DFA::try_search_fwd::h31ce27b32f25e474
                               at /home/lua/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-automata-0.4.7/src/hybrid/dfa.rs:595:24
  22:     0x64ae56555410 - regex_automata::hybrid::regex::Regex::try_search::h3e8007c5ad48f367
                               at /home/lua/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-automata-0.4.7/src/hybrid/regex.rs:448:25
  23:     0x64ae5655ca83 - regex_automata::meta::wrappers::HybridEngine::try_search::h817b982eeee28167
                               at /home/lua/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-automata-0.4.7/src/meta/wrappers.rs:649:13
  24:     0x64ae5655ca83 - <regex_automata::meta::strategy::Core as regex_automata::meta::strategy::Strategy>::search::h7bf587f4abf1b6c7
                               at /home/lua/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-automata-0.4.7/src/meta/strategy.rs:720:19
  25:     0x64ae5654fe78 - regex_automata::meta::regex::Regex::search_with::h920b6cbd2afe6241
                               at /home/lua/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-automata-0.4.7/src/meta/regex.rs:1248:9
  26:     0x64ae5654fe78 - <regex_automata::meta::regex::FindMatches as core::iter::traits::iterator::Iterator>::next::{{closure}}::h1e9d5ea2ba222227
                               at /home/lua/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-automata-0.4.7/src/meta/regex.rs:2079:31
  27:     0x64ae5654fe78 - regex_automata::util::iter::Searcher::try_advance::h897f76b290e8182e
                               at /home/lua/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-automata-0.4.7/src/util/iter.rs:431:27
  28:     0x64ae5654fe78 - regex_automata::util::iter::Searcher::advance::h5f28eacd01f05802
                               at /home/lua/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-automata-0.4.7/src/util/iter.rs:380:15
  29:     0x64ae5654fe78 - <regex_automata::meta::regex::FindMatches as core::iter::traits::iterator::Iterator>::next::h8af52fa2ecc532e2
                               at /home/lua/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-automata-0.4.7/src/meta/regex.rs:2079:9
  30:     0x64ae5654fe78 - <regex::regex::bytes::Matches as core::iter::traits::iterator::Iterator>::next::h4fc4b178a20a862e
                               at /home/lua/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-1.10.5/src/regex/bytes.rs:2175:14
  31:     0x64ae5654fe78 - regex_crash::main::h0f20340e0f7739f4
                               at /home/lua/playground/regex_crash2/src/main.rs:18:14
  32:     0x64ae5654dbb3 - core::ops::function::FnOnce::call_once::hcfa16cfe5c110f77
                               at /rustc/bcf94dec5ba6838e435902120c0384c360126a26/library/core/src/ops/function.rs:250:5
  33:     0x64ae5654dbb3 - std::sys::backtrace::__rust_begin_short_backtrace::h15fc2c2d6e8c6342
                               at /rustc/bcf94dec5ba6838e435902120c0384c360126a26/library/std/src/sys/backtrace.rs:155:18
  34:     0x64ae5654db29 - std::rt::lang_start::{{closure}}::h125c61b3ac5c222e
                               at /rustc/bcf94dec5ba6838e435902120c0384c360126a26/library/std/src/rt.rs:159:18
  35:     0x64ae5663f7d2 - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once::ha1963bc38011d7b6
                               at /rustc/bcf94dec5ba6838e435902120c0384c360126a26/library/core/src/ops/function.rs:284:13
  36:     0x64ae5663f7d2 - std::panicking::try::do_call::h5d039f454b38d38a
                               at /rustc/bcf94dec5ba6838e435902120c0384c360126a26/library/std/src/panicking.rs:553:40
  37:     0x64ae5663f7d2 - std::panicking::try::hc88e0647db6108c4
                               at /rustc/bcf94dec5ba6838e435902120c0384c360126a26/library/std/src/panicking.rs:517:19
  38:     0x64ae5663f7d2 - std::panic::catch_unwind::h6f5b0543366c4144
                               at /rustc/bcf94dec5ba6838e435902120c0384c360126a26/library/std/src/panic.rs:350:14
  39:     0x64ae5663f7d2 - std::rt::lang_start_internal::{{closure}}::h32c08947bc761bcb
                               at /rustc/bcf94dec5ba6838e435902120c0384c360126a26/library/std/src/rt.rs:141:48
  40:     0x64ae5663f7d2 - std::panicking::try::do_call::hf1b8b9ec58cf92dd
                               at /rustc/bcf94dec5ba6838e435902120c0384c360126a26/library/std/src/panicking.rs:553:40
  41:     0x64ae5663f7d2 - std::panicking::try::hede6d521829455d9
                               at /rustc/bcf94dec5ba6838e435902120c0384c360126a26/library/std/src/panicking.rs:517:19
  42:     0x64ae5663f7d2 - std::panic::catch_unwind::hb6f666a2e0db4b91
                               at /rustc/bcf94dec5ba6838e435902120c0384c360126a26/library/std/src/panic.rs:350:14
  43:     0x64ae5663f7d2 - std::rt::lang_start_internal::h7065a2d21fb2089d
                               at /rustc/bcf94dec5ba6838e435902120c0384c360126a26/library/std/src/rt.rs:141:20
  44:     0x64ae5655023c - main
  45:     0x7bc983ca9c88 - <unknown>
  46:     0x7bc983ca9d4c - __libc_start_main
  47:     0x64ae5654d885 - _start
  48:                0x0 - <unknown>
```
