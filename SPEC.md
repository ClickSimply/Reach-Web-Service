Application Database Server

It's like Nginx with a built in database, event loop, javascript application runtime, logging/error handling, SMTP client & native module system.

- Built in NoSQL database with RocksDB or TiKV backend.
- Multiple ReachOS instances can be linked together with Raft.
- Supports Virtual Hosts, SSL, LetsEncrypt, file hosting and reverse proxy.
- Supports Actions/Events & Filters like Wordpress (in application API)
- Brute force protection for user accounts.
- Static configuration via JS file (parsed by QuickJS).

**Libraries:**
- Javascript Runtime: https://crates.io/crates/libquickjs-sys
- Webassembly Rutnime: https://crates.io/crates/lucet-runtime (implement later)
- Database backend: https://crates.io/crates/rocksdb
- Web server: https://crates.io/crates/warp or https://crates.io/crates/actix
- Native Modules: https://michael-f-bryan.github.io/rust-ffi-guide/dynamic_loading.html
- Initial Native Modules: HTTP Client, Docker Manager, File System, Database
- Core functions: 
    - SMTP (has JS API)
    - Logging (has JS API)
    - Lets Encrypt https://tools.ietf.org/html/rfc8555 https://github.com/serghey-rodin/vesta/blob/master/bin/v-add-letsencrypt-domain
    - Virtual Hosts (JS can request new blank virtual hosts)
    - Every virtual host is expected to have /json endpoint for database requests, a /ws endpont for websockets, every other URL is for rendering.
    - Actions/Events & Filters (JS API)  Should have namespaces for multisite support.
