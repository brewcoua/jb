# Changelog

All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

---
## [1.5.4](https://github.com/brewcoua/jb/compare/v1.5.3..1.5.4) - 2024-02-10

### Bug Fixes

- **(auto)** reload daemon when updating services - ([12e4703](https://github.com/brewcoua/jb/commit/12e47037d77e68c03e01c3a805648118254deb5f)) - Brewen Couaran

---
## [1.5.3](https://github.com/brewcoua/jb/compare/v1.5.2..v1.5.3) - 2024-02-10

### Features

- **(refresh)** add --all flag for updating all installed tools - ([1e3e4c8](https://github.com/brewcoua/jb/commit/1e3e4c820864799fbd6662b78c883096e4de31c8)) - Brewen Couaran
- add desktop command for generating desktop entries - ([45a3314](https://github.com/brewcoua/jb/commit/45a33145450935b3c5e82f85f613d35cfeb4c44a)) - Brewen Couaran
- add auto updates through systemd service and timer - ([d84161b](https://github.com/brewcoua/jb/commit/d84161b53a73aaa66f6bc0ea5bec95cc7f4edda8)) - Brewen Couaran

---
## [1.5.2](https://github.com/brewcoua/jb/compare/v1.5.1..v1.5.2) - 2024-02-09

### Bug Fixes

- **(refresh)** add install flag for installing if tool not already installed - ([74e1ba2](https://github.com/brewcoua/jb/commit/74e1ba2ccb42ff0a23be94be3236d33e0ae1651e)) - Brewen Couaran

---
## [1.5.1](https://github.com/brewcoua/jb/compare/v1.5.0..v1.5.1) - 2024-02-09

### Features

- implement notifications and add them to most commands - ([59c94b7](https://github.com/brewcoua/jb/commit/59c94b73f02f4bd4240a53af0d684e84b537a6de)) - Brewen Couaran

---
## [1.5.0](https://github.com/brewcoua/jb/compare/v1.4.3..v1.5.0) - 2024-02-09

### Features

- **(cmds)** add info and meta commands - ([0377975](https://github.com/brewcoua/jb/commit/037797591b713622124bdc5da9728f17c12d52e5)) - Brewen Couaran

---
## [1.4.3](https://github.com/brewcoua/jb/compare/v1.4.2..v1.4.3) - 2024-02-09

### Bug Fixes

- **(kind)** normalize icon and executable path and fix fleet's paths - ([5233935](https://github.com/brewcoua/jb/commit/523393508cec4ada4d647026c4fdc9b558c55a62)) - Brewen Couaran

### Features

- **(cmds)** add cd command and update readme docs - ([66af588](https://github.com/brewcoua/jb/commit/66af5887d7f6192a6355d7fd90e0b9cf54bf53e0)) - Brewen Couaran
- **(cmds)** add refresh command for updating tools - ([a3817ac](https://github.com/brewcoua/jb/commit/a3817ac9a042f3a6c58e0414a5e88d23aa927384)) - Brewen Couaran

---
## [1.4.2](https://github.com/brewcoua/jb/compare/v1.4.1..v1.4.2) - 2024-02-08

### Features

- **(update)** add markdown display for changelog - ([110a8c4](https://github.com/brewcoua/jb/commit/110a8c436643e783df062878b7258f8d62896a10)) - Brewen Couaran

---
## [1.4.1](https://github.com/brewcoua/jb/compare/v1.4.0..v1.4.1) - 2024-02-08

### Features

- **(install)** verify checksum at the end of download - ([73ada06](https://github.com/brewcoua/jb/commit/73ada06b5dcaba0f3730e590b6731211b4937067)) - Brewen Couaran

---
## [1.4.0](https://github.com/brewcoua/jb/compare/v1.3.2..v1.4.0) - 2024-02-07

### Bug Fixes

- **(error)** add unwrap to avoid displaying option - ([9c3160d](https://github.com/brewcoua/jb/commit/9c3160dc9e9d2e34281474690ce480b9fae8196d)) - Brewen Couaran
- **(install)** add folder strip and link step - ([7ecbf57](https://github.com/brewcoua/jb/commit/7ecbf57f45aa43796f1121863174e402259d1044)) - Brewen Couaran
- **(install)** fix linking step, add strip component after extracting and show install message - ([5e7a683](https://github.com/brewcoua/jb/commit/5e7a6838696f8ae32250d32f0875ab2240e802df)) - Brewen Couaran
- **(install)** clean up step with only related tools and avoid uselessly checking link status - ([515803c](https://github.com/brewcoua/jb/commit/515803c26d11cc6137315e0b823421104797c8d2)) - Brewen Couaran

### Features

- **(log)** switch from tracing to using custom logger - ([6f4b207](https://github.com/brewcoua/jb/commit/6f4b207ee8a8e60dfd4964ff847363bb5e0e0303)) - Brewen Couaran
- **(uninstall)** add step by step logs and run concurrently - ([535dc2b](https://github.com/brewcoua/jb/commit/535dc2b5ba4ffad6634b112d982b26e91fbd7026)) - Brewen Couaran

---
## [1.3.2](https://github.com/brewcoua/jb/compare/v1.3.1..v1.3.2) - 2024-02-05

### Bug Fixes

- **(kind)** remove intellij from idea-ultimate and community - ([9da723a](https://github.com/brewcoua/jb/commit/9da723a4ce60fbb65686b2b937b7b9c94fd2478e)) - Brewen Couaran

### Features

- **(bin)** add update command to update cli - ([d3402f7](https://github.com/brewcoua/jb/commit/d3402f7c4b525bf0e24a42d53c77bcbbde558a44)) - Brewen Couaran

---
## [1.3.1](https://github.com/brewcoua/jb/compare/v1.3.0..v1.3.1) - 2024-02-05

### Security

- **(atty)** remove atty and use std as atty is unmaintained - ([6ddf6c7](https://github.com/brewcoua/jb/commit/6ddf6c712b808840e495dc8826f4ec57ed2b4867)) - Brewen Couaran

---
## [1.3.0](https://github.com/brewcoua/jb/compare/v1.2.5..v1.3.0) - 2024-02-05

### Bug Fixes

- **(env)** parse 1 as true for bool - ([676da6f](https://github.com/brewcoua/jb/commit/676da6f0b4b024a91be8e5760d9cf92b28f06e19)) - Brewen Couaran
- tool parsing and link checking - ([88128ef](https://github.com/brewcoua/jb/commit/88128efd5804c5bd5aa4500229762e81cfe8f400)) - Brewen Couaran

### Documentation

- **(cargo)** add usage description for each dependency - ([9c88e60](https://github.com/brewcoua/jb/commit/9c88e604325f4383679c1fb0102013b08be8b2fa)) - Brewen Couaran
- add descriptions to root modules - ([084c038](https://github.com/brewcoua/jb/commit/084c038633fc5c65f463d267716bb9a093fad856)) - Brewen Couaran

### Features

- **(list)** add empty text when no tool is detected - ([6ce7410](https://github.com/brewcoua/jb/commit/6ce7410be60a924ad8beb0b1ee5459ea2032c8e1)) - Brewen Couaran
- add build numbers to tool and refactor practically everything - ([162e3a9](https://github.com/brewcoua/jb/commit/162e3a9949666795b92a84ead5c181703322cfb8)) - Brewen Couaran
- add env settings and use traits with bin commands - ([8f7cc38](https://github.com/brewcoua/jb/commit/8f7cc38c44f8914fd23531e5691d595cd03fdaeb)) - Brewen Couaran

### Refactoring

- **(action)** make traits object safe - ([0345292](https://github.com/brewcoua/jb/commit/03452921668c93b8c965345f99cc805303d4678b)) - Brewen Couaran
- **(unlink)** fix logging - ([442e29b](https://github.com/brewcoua/jb/commit/442e29b61285a66cf4c05789456f12f74dee11cb)) - Brewen Couaran
- move back lib and bin crates together - ([9894b7b](https://github.com/brewcoua/jb/commit/9894b7b34ab6fb85a23d987f0f6d565264326874)) - Brewen Couaran
- remove old files and rename parse mod to api - ([6cc824f](https://github.com/brewcoua/jb/commit/6cc824fbdc3879f3a258b4df516cd9f6739f5a4b)) - Brewen Couaran

---
## [1.2.5](https://github.com/brewcoua/jb/compare/v1.2.4..v1.2.5) - 2024-02-04

### Bug Fixes

- **(tool)** take longest match first - ([06c67df](https://github.com/brewcoua/jb/commit/06c67df651706d638e25a04ff282f805aa969b1d)) - Brewen Couaran

---
## [1.2.4](https://github.com/brewcoua/jb/compare/v1.2.3..v1.2.4) - 2024-02-04

### Bug Fixes

- **(tool)** kind parsing error on tool names with hyphens - ([6eee164](https://github.com/brewcoua/jb/commit/6eee16427940708930a724474e6324f0aba773ca)) - Brewen Couaran

---
## [1.2.3](https://github.com/brewcoua/jb/compare/v1.2.2..v1.2.3) - 2024-02-04

### Bug Fixes

- **(cargo)** set release profile with lto and strip - ([691d833](https://github.com/brewcoua/jb/commit/691d83319c2b2c258edc221a8ab2bc3a84b6bf9e)) - Brewen Couaran

---
## [1.2.1](https://github.com/brewcoua/jb/compare/v1.2.0..v1.2.1) - 2024-02-03

### Bug Fixes

- **(install)** clean up temporary files even if install fails - ([a0a7e53](https://github.com/brewcoua/jb/commit/a0a7e53b0343f4e5d1f382bca64f282251bed55e)) - Brewen Couaran
- **(tls)** switch to rustls-tls to avoid using openssl bindings - ([7d9f660](https://github.com/brewcoua/jb/commit/7d9f660b9aa270767d678e0e8037a8d9e6a358cd)) - Brewen Couaran

---
## [1.2.0](https://github.com/brewcoua/jb/compare/v1.1.1..v1.2.0) - 2024-02-03

### Features

- **(tool)** directly parse tool instead of kind and specify versions directly on the tool - ([b9d0c3a](https://github.com/brewcoua/jb/commit/b9d0c3a064a64e9a9c2ae9769579b7b70301547c)) - Brewen Couaran
- **(uninstall)** add concurrency with multiple args matching - ([2fd5212](https://github.com/brewcoua/jb/commit/2fd521226d911af177599bbe0c38cef32796da50)) - Brewen Couaran
- **(uninstall)** add matching for tools arg - ([a6b260f](https://github.com/brewcoua/jb/commit/a6b260fde0c948c9aae0ecbdf8368f92edab84a0)) - Brewen Couaran

---
## [1.1.1](https://github.com/brewcoua/jb/compare/v1.1.0..v1.1.1) - 2024-02-03

### Miscellaneous Chores

- bump version to 1.1.1 - ([6f155eb](https://github.com/brewcoua/jb/commit/6f155eb8463cf60dfa9a46b7cf3eb5a150d39920)) - brewcoua

---
## [1.1.0](https://github.com/brewcoua/jb/compare/v1.0.1..v1.1.0) - 2024-02-03

### Bug Fixes

- **(log)** prefix colors and set padding for log level - ([4e41069](https://github.com/brewcoua/jb/commit/4e41069a818f792bb9345b1cadc0c91ba78d8156)) - Brewen Couaran

### Documentation

- update licenses and rewrite readme - ([b40426d](https://github.com/brewcoua/jb/commit/b40426d9530bef121b461d3c95634d2eb3c9edd0)) - Brewen Couaran

### Features

- **(install)** add checksum step for file integrity - ([95f0991](https://github.com/brewcoua/jb/commit/95f099182bf560190d574a9274aaae23e18ff905)) - Brewen Couaran
- use env variables to override defaults - ([a497489](https://github.com/brewcoua/jb/commit/a497489bd2f960eb723f594ace2668e34a4160a8)) - Brewen Couaran
- allow concurrent installations - ([8f9ad8f](https://github.com/brewcoua/jb/commit/8f9ad8f0928ce330a1f9724afb7bf39ad94a934a)) - Brewen Couaran
- update logs to support concurrency - ([b84d0a3](https://github.com/brewcoua/jb/commit/b84d0a3c90c82814c037b266c9918331fa0d348f)) - Brewen Couaran
- switch to tracing for logging - ([048409e](https://github.com/brewcoua/jb/commit/048409e89b61dcf963c84313c514f7896a78a4e5)) - Brewen Couaran

### Miscellaneous Chores

- bump dependencies - ([66fd3f7](https://github.com/brewcoua/jb/commit/66fd3f76bc006a1c5eebc7c8163dba4744ee8341)) - Brewen Couaran
- bump dependencies - ([faa202e](https://github.com/brewcoua/jb/commit/faa202e366b074ecd048863655d570f61ab53a1b)) - Brewen Couaran
- bump version to 1.1.0 - ([ca3dc1e](https://github.com/brewcoua/jb/commit/ca3dc1ea5f9f9d01d50425c8edf6f86ec39f803c)) - brewcoua

---
## [1.0.1](https://github.com/brewcoua/jb/compare/v1.0.0..v1.0.1) - 2024-01-27

### Bug Fixes

- all clippy warnings - ([9d9ca96](https://github.com/brewcoua/jb/commit/9d9ca96bb06dcf3adbc5e97242b0d5b5436795f2)) - Brewen Couaran

### Documentation

- add documentation to most lib functions - ([1c2d84c](https://github.com/brewcoua/jb/commit/1c2d84cf2b7d8e5369e5fe6c6f7e1f718e079844)) - Brewen Couaran
- add ci step for deploying to github pages - ([1c73ba3](https://github.com/brewcoua/jb/commit/1c73ba3d70cc951215b571242ae1d3b344a6d833)) - Brewen Couaran

### Features

- **(list)** add checkmark and cross to show link state next to tools - ([bd301b6](https://github.com/brewcoua/jb/commit/bd301b63ce5e7508eab44dbb3f5a6c6e4e7b1e67)) - Brewen Couaran
- add download, extract and symlink steps and complete installer - ([5542cd0](https://github.com/brewcoua/jb/commit/5542cd0afd573a765d009731d131b4f7210a6553)) - Brewen Couaran
- add list command to show all installed tools - ([9d3a9f6](https://github.com/brewcoua/jb/commit/9d3a9f6c30e29648c05803026eb02aa689d6271e)) - Brewen Couaran
- add uninstall and link commands - ([fb25255](https://github.com/brewcoua/jb/commit/fb25255647e8a04949e352ba404014840549d65a)) - Brewen Couaran
- use anyhow for all result and errors & add context to OS errors - ([484fef7](https://github.com/brewcoua/jb/commit/484fef7295de3069a844d8d1d0495f47445d4b76)) - Brewen Couaran
- refactor logs and display backtrace for errors if captured (RUST_BACKTRACE=1) - ([e83e5fa](https://github.com/brewcoua/jb/commit/e83e5fad91d894b74e827055a9e010b6668e8cd4)) - Brewen Couaran
- add unlink command and allow cleaning up old version on install - ([cf8ebc2](https://github.com/brewcoua/jb/commit/cf8ebc2d3bc3226c9db815f93e6cb68cd3e15d94)) - Brewen Couaran
- display whole errors including backtrace & use readonly on tool fields - ([35e8e66](https://github.com/brewcoua/jb/commit/35e8e66a5d385c040db3047faa5b7c08737702f5)) - Brewen Couaran
- rename jb-tool to jb - ([922e3c4](https://github.com/brewcoua/jb/commit/922e3c452c381eb3f495af47368f5604fc0f8871)) - Brewen Couaran

### Miscellaneous Chores

- update dependencies to latest - ([b4bc533](https://github.com/brewcoua/jb/commit/b4bc533d773de491de2949f74043c9ba501076e3)) - Brewen Couaran

### Refactoring

- **(tools)** rename Tool to Kind and use full Tool struct for handling tools - ([899fea0](https://github.com/brewcoua/jb/commit/899fea0f8d0d530f8563831baacae4c1d839cba7)) - Brewen Couaran
- move lib to jb-lib/ & rewrite install and list - ([3bb5c59](https://github.com/brewcoua/jb/commit/3bb5c59559c7f80eb94b5e18c0a89553d6a3df26)) - Brewen Couaran
- rename binary to 'jb' instead of 'jb-tool' - ([7ac2803](https://github.com/brewcoua/jb/commit/7ac2803723ae88989def857f90e18563870c6614)) - Brewen Couaran

### Tests

- add no_run to functions writing files - ([bac0a79](https://github.com/brewcoua/jb/commit/bac0a7911a69bfe9e87d6d73f151c483899727b0)) - Brewen Couaran

---
## [1.0.0] - 2024-01-21

### Init

- first commit - ([4848633](https://github.com/brewcoua/jb/commit/48486331092f4cf8b97509e732e9c35a57807fc3)) - Brewen Couaran

<!-- generated by git-cliff -->
