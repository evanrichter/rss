#![no_main]
use libfuzzer_sys::fuzz_target;
use rss::validation::Validate;

fuzz_target!(|data: &[u8]| {
    let data = std::io::Cursor::new(data);
    if let Ok(channel) = rss::Channel::read_from(data) {
        let _ = channel.validate();
    }
});
