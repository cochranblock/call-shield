# call-shield

call-shield is sub-millisecond call screening — a 48 KB binary that blocks robocalls and spam without any cloud dependency. It classifies caller intent as spam, legitimate, or unknown in under 1ms using a 38-pattern classifier; no audio ever leaves the device. Ships as a Rust CLI, iOS static lib, Android CallScreeningService, and PWA — all running the same core classifier. Public domain (Unlicense).
