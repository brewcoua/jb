# Changelog

All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

---
## [1.1.0](https://github.com/brewcoua/jb/compare/v1.0.1..1.1.0) - 2024-02-03

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

- **(cargo)** update meta with license and repo - ([468fbaa](https://github.com/brewcoua/jb/commit/468fbaa2d35a82695e0ec1cee3b73b64ffdeb762)) - Brewen Couaran
- **(clippy)** fix all errors and warnings - ([67607e0](https://github.com/brewcoua/jb/commit/67607e0f85696f1eee73a5ec7918347f2fba5ce1)) - Brewen Couaran
- bump dependencies - ([66fd3f7](https://github.com/brewcoua/jb/commit/66fd3f76bc006a1c5eebc7c8163dba4744ee8341)) - Brewen Couaran
- bump dependencies - ([faa202e](https://github.com/brewcoua/jb/commit/faa202e366b074ecd048863655d570f61ab53a1b)) - Brewen Couaran

### Ci

- **(release)** fix typo in git cliff download link - ([55b4f6b](https://github.com/brewcoua/jb/commit/55b4f6b4d2ba35355cc5353b4e15d6db233ba92a)) - Brewen Couaran
- **(release)** fix another typo - ([1f4292b](https://github.com/brewcoua/jb/commit/1f4292b2cfbc885840a29ef81543dc10863fbe79)) - Brewen Couaran
- **(release)** add strip component to tar - ([b665e89](https://github.com/brewcoua/jb/commit/b665e89e45175950b63173e722b41a7ab7c0f23b)) - Brewen Couaran
- **(release)** add fetch depth to allow generating changelogs - ([707e440](https://github.com/brewcoua/jb/commit/707e4406b3a70faa5984b67b6ead5f8d36b28510)) - Brewen Couaran
- **(release)** add cargo bump and fix release body version - ([7dcf8a8](https://github.com/brewcoua/jb/commit/7dcf8a80fabc290507ddf14e66c7428968c4b36c)) - Brewen Couaran
- **(release)** set tag for release manually - ([c4d41df](https://github.com/brewcoua/jb/commit/c4d41df0c3b3419c1de8f03ca606318b102f77bc)) - Brewen Couaran
- add steps for bumping version & releasing - ([cf697d3](https://github.com/brewcoua/jb/commit/cf697d3542e3e3e7e55fb1278919689925c48d39)) - Brewen Couaran
- use only one target for now - ([ac0822b](https://github.com/brewcoua/jb/commit/ac0822bbce9e67346b8e20f651c36267b01b1e04)) - Brewen Couaran

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

### Ci

- **(docs)** add index.html to root with redirect - ([373a261](https://github.com/brewcoua/jb/commit/373a2613f1325aaea6e626fef40ce9875aba3a24)) - Brewen Couaran
- add ci steps for test, build, clippy and outdated - ([2aae0a9](https://github.com/brewcoua/jb/commit/2aae0a9ae781c6217cca142f384fb714af2a8823)) - Brewen Couaran
- add more permissions to docs workflow - ([e525d2a](https://github.com/brewcoua/jb/commit/e525d2a7766206a2cc3e558349c00ab92ae9becd)) - Brewen Couaran
- split docs deploy in 2 steps - ([bd9fcd1](https://github.com/brewcoua/jb/commit/bd9fcd1a89bef64b65ab22f1bd6cd3010b0eaae3)) - Brewen Couaran
- update artifact id and remove cache for docs - ([2453916](https://github.com/brewcoua/jb/commit/2453916ba35246d6c8d255dd3fe9ecc4a3938f31)) - Brewen Couaran
- fix typo in action name - ([d74b463](https://github.com/brewcoua/jb/commit/d74b4634eee9c736e340d53229fffd84bfe11c64)) - Brewen Couaran
- update docs environment & add workflow_dispatch - ([ff81daa](https://github.com/brewcoua/jb/commit/ff81daa2850c8b56dfd47dd495ebabe4412daebf)) - Brewen Couaran
- publish to branch instead of actions (since it does not work) - ([d423057](https://github.com/brewcoua/jb/commit/d423057fafad29a20536d02945fc17c6e5854904)) - Brewen Couaran
- update permissions for docs workflow - ([e5bb1ea](https://github.com/brewcoua/jb/commit/e5bb1ea891132f1e7cb2eda6624e3eb4d9ccc9d7)) - Brewen Couaran

---
## [1.0.0] - 2024-01-21

### Init

- first commit - ([4848633](https://github.com/brewcoua/jb/commit/48486331092f4cf8b97509e732e9c35a57807fc3)) - Brewen Couaran

<!-- generated by git-cliff -->