## [unreleased]

### Miscellaneous

- **(msrv)** remove tests because proptest@1.9.0 requires rustc 1.82 ([8be02a1](https://github.com/m3idnotfree/ircv3_parse/commit/8be02a1b46ac687a9540ed0d7417d2e13fb47dc4))
- add CHANGELOG.md and cliff.toml ([9670339](https://github.com/m3idnotfree/ircv3_parse/commit/9670339c9f2fae62a9ee9d6a22e6dd24bb300721))

## [3.2.0](https://github.com/m3idnotfree/ircv3_parse/compare/v3.1.1..v3.2.0) - 2026-01-12

### Features

- **(derive)** support Commands enum for command attribute ([16852db](https://github.com/m3idnotfree/ircv3_parse/commit/16852db0a52e74f360e803d1e3225ffda4f4db2f))
- **(derive)** infer attribute values from field names ([70b757d](https://github.com/m3idnotfree/ircv3_parse/commit/70b757d7c7d99c6026d4761e58fd55b33943663b))
- **(error)** add MissingCommand for ToMessage ([f4e2654](https://github.com/m3idnotfree/ircv3_parse/commit/f4e2654ab1cdbb7c29ac266018fe82e96b44484a))
- **(commands)** implement FromMessage and ToMessage traits ([f4a7538](https://github.com/m3idnotfree/ircv3_parse/commit/f4a7538bcab5614623033bbbbecfa83b391fc4f4))
- **(derive)** add ToMessage derive macro ([54604e5](https://github.com/m3idnotfree/ircv3_parse/commit/54604e577208974035b0d541713f4f1b5ba3715f))
- **(derive)** expose ToMessage ([d0fc3c0](https://github.com/m3idnotfree/ircv3_parse/commit/d0fc3c0d3aa72c4174d0711e0a8d310acdeeeeec))

### Refactoring

- **(ser)** [**breaking**] remove unnecessary Result wrappers from tags and params ([6e3c62a](https://github.com/m3idnotfree/ircv3_parse/commit/6e3c62a1891fbb07cd17785aa9f35b8a56e62a49))
- **(derive)** parameterize extract_named_fields error messages ([899c35b](https://github.com/m3idnotfree/ircv3_parse/commit/899c35b0b0ab6c1d9eb1dda69b622b5bdb0341af))
- **(ser)** [**breaking**] support multiple component calls ([8c4b979](https://github.com/m3idnotfree/ircv3_parse/commit/8c4b979f388d3d905786da912804d59a9c982fcd))
- **(builder)** accept iterator in Params::extend() ([74ff942](https://github.com/m3idnotfree/ircv3_parse/commit/74ff9422d301ee590583e9991c027a328c9d285f))
- **(builder)** delegate MessageBuilder::add_params() to Params::extend() ([02828fa](https://github.com/m3idnotfree/ircv3_parse/commit/02828fa5358ae5e46609df214401e3a1bb0e9af2))

### Documentation

- add ToMessage documentation ([37fdc0f](https://github.com/m3idnotfree/ircv3_parse/commit/37fdc0f8bbff0e1c917fae554f0aea6dd52e9566))

### Miscellaneous

- **(commands)** allow clippy::len_without_is_empty ([d6bd04f](https://github.com/m3idnotfree/ircv3_parse/commit/d6bd04f923177d84350cd86c2306d7808db91ce7))
- migrate to workspace structure ([08b62e3](https://github.com/m3idnotfree/ircv3_parse/commit/08b62e3fbb68d3cdb80ad293133d19d8bdb33830))
- remove unnecessary comments ([884d62c](https://github.com/m3idnotfree/ircv3_parse/commit/884d62c6f00bda818a91bae49c3715de7b70a505))
- migrate from taplo to tombi for TOML formatting ([754e131](https://github.com/m3idnotfree/ircv3_parse/commit/754e131592f1611a8d84a6a9460028a043565b15))
- add GitHub Actions workflow ([ec3159b](https://github.com/m3idnotfree/ircv3_parse/commit/ec3159b03b5c4f8323261e6d2019ded0241ccf53))

## [ircv3_parse_derive-v3.1.0](https://github.com/m3idnotfree/ircv3_parse/compare/v3.1.0..ircv3_parse_derive-v3.1.0) - 2026-01-04

### Features

- **(derive)** adapt to ircv3_parse 3.1.0 ([71898d2](https://github.com/m3idnotfree/ircv3_parse/commit/71898d2e023c5b20d802be2dc91b39d8e277c6f6))

## [3.1.0](https://github.com/m3idnotfree/ircv3_parse/compare/v3.0.1..v3.1.0) - 2026-01-04

### Features

- **(commands)** add len() method ([bd64667](https://github.com/m3idnotfree/ircv3_parse/commit/bd6466701c0107ea1ca5070ddab6df1758355ef2))
- **(error)** add SourceError::EmptyUser ([ad1294a](https://github.com/m3idnotfree/ircv3_parse/commit/ad1294a4a562df308247f87bebbddbb0e8fb24f5))
- **(lib)** add BANG and EQ byte constants ([7ff8bbc](https://github.com/m3idnotfree/ircv3_parse/commit/7ff8bbcf34cb8c3bf505fd9d1a39383280686f22))
- **(validators)** add param validators and tests ([1937851](https://github.com/m3idnotfree/ircv3_parse/commit/1937851b546b6c0eecf033c2580c8165f573d8b4))
- **(message)** add serialization and order-independent MessageBuilder ([35405ec](https://github.com/m3idnotfree/ircv3_parse/commit/35405ec5d871f82e7bb3483c49171e7324850061))

### Bug Fixes

- **(commands)** implement case-insensitive equality ([06e3ed9](https://github.com/m3idnotfree/ircv3_parse/commit/06e3ed99a11831c1eb6c0eea72004e124714759a))

### Refactoring

- **(components)** re-export Commands ([dca81c2](https://github.com/m3idnotfree/ircv3_parse/commit/dca81c2aa354518d082792fdc82dfd05f1fc5649))
- **(builder)** [**breaking**] move legacy MessageBuilder ([04cd4c2](https://github.com/m3idnotfree/ircv3_parse/commit/04cd4c2ca1ff32db3ff3dc82cc7b2451120113db))
- **(validators)** extract NICK_SPECIAL_CHARS constant ([8c46b53](https://github.com/m3idnotfree/ircv3_parse/commit/8c46b53fc299b8a4edb5b4f045d7fc9614a42e26))
- **(message)** extract Message from components module ([3c29f82](https://github.com/m3idnotfree/ircv3_parse/commit/3c29f821398163aa9e45950862aaf024241733d9))
- **(de)** [**breaking**] rename extract module to de ([269f8be](https://github.com/m3idnotfree/ircv3_parse/commit/269f8bebf24eb88eb2c41474e0b03cfe2265a00d))
- migrate char literals to byte constants ([21dd352](https://github.com/m3idnotfree/ircv3_parse/commit/21dd35241aef940ce942c1f7328004d9e2a5218a))

### Documentation

- **(validators)** add comprehensive documentation ([bf47580](https://github.com/m3idnotfree/ircv3_parse/commit/bf4758094d056da2f954c825529ef147996b01dc))

## [3.0.1](https://github.com/m3idnotfree/ircv3_parse/compare/v3.0.0..v3.0.1) - 2025-12-26

### Features

- **(error)** expose error module publicly ([93852c3](https://github.com/m3idnotfree/ircv3_parse/commit/93852c3f7903f83ce6e0010b20d5c3ce5025b9c3))

### Bug Fixes

- [**breaking**] rename escape-related functions correctly ([15e168f](https://github.com/m3idnotfree/ircv3_parse/commit/15e168fa63d8261eaa72d24114e28b33e65db58f))

### Documentation

- add comprehensive documentation ([15c50ae](https://github.com/m3idnotfree/ircv3_parse/commit/15c50ae9eb2b002e296d2c0da76dc8d200abc6d9))

### Miscellaneous

- **(docs)** configure docs.rs settings ([521d612](https://github.com/m3idnotfree/ircv3_parse/commit/521d612d39f011821405b25d51b418b71da66c66))

## [3.0.0](https://github.com/m3idnotfree/ircv3_parse/compare/ircv3_parse_derive-v0.1.0..v3.0.0) - 2025-12-24

### Features

- **(derive)** add derive feature ([28e49bd](https://github.com/m3idnotfree/ircv3_parse/commit/28e49bd7357889cfe97cdb717afedf506529fb7a))

### Miscellaneous

- **(derive)** add metadata for ircv3_parse_derive ([2a7b82c](https://github.com/m3idnotfree/ircv3_parse/commit/2a7b82c3bbdb99c1a23a192c12d98df1989b148e))
- **(derive)** fix READMD file name ([fc3f620](https://github.com/m3idnotfree/ircv3_parse/commit/fc3f620af5b6d9b229d0af722bfc68db760572bf))

## [ircv3_parse_derive-v0.1.0](https://github.com/m3idnotfree/ircv3_parse/compare/v2.1.0..ircv3_parse_derive-v0.1.0) - 2025-12-24

### Features

- **(derive)** add FromMessage derive macro ([5a5348c](https://github.com/m3idnotfree/ircv3_parse/commit/5a5348c46f2f160f0c20e45980730983d2b58a34))

## [2.1.0](https://github.com/m3idnotfree/ircv3_parse/compare/v2.0.3..v2.1.0) - 2025-12-24

### Features

- **(extract)** add FromMessage trait and ExtractError ([2cd5516](https://github.com/m3idnotfree/ircv3_parse/commit/2cd5516241fae89199dc9f2ecde8de306106e7e0))
- **(error)** enhance ExtractError ([36542c3](https://github.com/m3idnotfree/ircv3_parse/commit/36542c36702792e05a98229fb13a795d882cba46))
- **(commands)** implement case-insensitive command parsing and comparison ([c10328a](https://github.com/m3idnotfree/ircv3_parse/commit/c10328a98c6bcad46950d6563f3625c07882b8e5))

### Bug Fixes

- **(commands)** add lifetime to Command::as_str/as_bytes to avoid incorrect lifetime ([4995df0](https://github.com/m3idnotfree/ircv3_parse/commit/4995df0a10e2670cafbeca443e180b807907c446))
- **(builder)** properly handle empty tag values ([8eb285e](https://github.com/m3idnotfree/ircv3_parse/commit/8eb285e709bbcba8b211923ff0b3a1af95457a93))

### Refactoring

- **(tags)** remove unnecessary self lifetime ([ed589d3](https://github.com/m3idnotfree/ircv3_parse/commit/ed589d3c220fbacec1db519667408f4019e6f49d))
- **(components)** re-export Message ([65fbae8](https://github.com/m3idnotfree/ircv3_parse/commit/65fbae89630bd0d19d5806ce7259da1da1d97221))
- **(params)** use &self in Middles::to_vec() ([8db9034](https://github.com/m3idnotfree/ircv3_parse/commit/8db9034e280f9657439028eb7453cae85324878d))

### Style

- **(error)** use lowercase for error messages ([b44c482](https://github.com/m3idnotfree/ircv3_parse/commit/b44c482405a2a03ed831a229bafd3b148a852a31))

### Other

- use serde?/std and serde?/alloc ([61159d6](https://github.com/m3idnotfree/ircv3_parse/commit/61159d6daa6fc2eb17c0be222a3212bfc13a9d5e))

## [2.0.3](https://github.com/m3idnotfree/ircv3_parse/compare/v2.0.2..v2.0.3) - 2025-11-18

### Bug Fixes

- **(SourceBuilder)** add missing #[cfg(debug_assertions)] ([ec57095](https://github.com/m3idnotfree/ircv3_parse/commit/ec57095786a2bf58ac4dedb7c1b811f2822894d2))

### Refactoring

- **(builder)** restrict BuilderError constructor visibility ([005e05c](https://github.com/m3idnotfree/ircv3_parse/commit/005e05c4c127ab3e3d99b24f33968b7f721d8d38))

## [2.0.2](https://github.com/m3idnotfree/ircv3_parse/compare/v2.0.1..v2.0.2) - 2025-10-19

### Features

- add no_std support ([b57b193](https://github.com/m3idnotfree/ircv3_parse/commit/b57b193f1d565ed6cc1c8fe6d676c220ad02fcf7))

## [2.0.1](https://github.com/m3idnotfree/ircv3_parse/compare/v2.0.0..v2.0.1) - 2025-10-01

### Features

- **(tags)** add Clone/Copy/Hash derives ([2dbc399](https://github.com/m3idnotfree/ircv3_parse/commit/2dbc399e0e9b2dcf699d3e21d189f820004960c8))
- **(serde)** implement Serialize for all components ([cdb9473](https://github.com/m3idnotfree/ircv3_parse/commit/cdb94736ede1fae383aa7e8b5b93662d0b213981))

### Refactoring

- **(source)** implement custom Debug trait ([769c1c5](https://github.com/m3idnotfree/ircv3_parse/commit/769c1c5ec462951329884a2e973b16fd3fafa5fd))
- **(tags)** implement AsRef trait ([8278947](https://github.com/m3idnotfree/ircv3_parse/commit/82789470901788826e4c4cd620609382383b3809))
- **(params)** custom Debug, AsRef, and trait additions ([ac4da7e](https://github.com/m3idnotfree/ircv3_parse/commit/ac4da7e6150c0cf100bbfd50aa8910dd0f70629f))
- **(commands)** add Hash derive and AsRef<str> trait ([cb75eae](https://github.com/m3idnotfree/ircv3_parse/commit/cb75eaea0e9bd9024c18dc714b6e25c0beb1ce16))
- **(message)** custom Display/Debug and Copy trait ([73dd79e](https://github.com/m3idnotfree/ircv3_parse/commit/73dd79e15b1548cbdf7263990f90dc5c9b47f7e5))
- **(parse)** add explicit lifetime parameter ([e21dda9](https://github.com/m3idnotfree/ircv3_parse/commit/e21dda98b905fadb786ab81c82fbd7bde49e4b1f))

### Miscellaneous

- **(license)** add Apache-2.0 as dual license option ([28a7014](https://github.com/m3idnotfree/ircv3_parse/commit/28a70147525174128ef57ed4d9cf88bbb3822d6f))

## [2.0.0](https://github.com/m3idnotfree/ircv3_parse/compare/v1.1.0..v2.0.0) - 2025-06-26

### Features

- **(builder)** add BuildState enforcement and conditional validation to MessageBuilder ([93bb804](https://github.com/m3idnotfree/ircv3_parse/commit/93bb804544a9b0937418d419842267a33f06e0aa))

### Documentation

- improve developer documentation ([51e04df](https://github.com/m3idnotfree/ircv3_parse/commit/51e04df93414f25b88c3a7b792a99e67ef4785d3))

### Performance

- [**breaking**] rewrite parser for zero-copy ([276537f](https://github.com/m3idnotfree/ircv3_parse/commit/276537ff813069c67662fcfed6a2854f31d414bf))

### Miscellaneous

- **(git)** update .gitignore ([8b40b08](https://github.com/m3idnotfree/ircv3_parse/commit/8b40b08109913bbcb47938bf8e80da95dc302fa6))

## [1.1.0](https://github.com/m3idnotfree/ircv3_parse/compare/v1.0.1..v1.1.0) - 2025-05-15

### Features

- **(parsers)** Implement IRCv3 tag value escaping conversion ([e2bb627](https://github.com/m3idnotfree/ircv3_parse/commit/e2bb627fbd4a95d1a51f31145a683d98cb9dcc76))
- Add CharValidator trait for flexible IRC protocol validation ([b1b4c21](https://github.com/m3idnotfree/ircv3_parse/commit/b1b4c218a3f2f7c0501827c090f4a57101090869))
- Implement IRCv3Error ([78bac0c](https://github.com/m3idnotfree/ircv3_parse/commit/78bac0c0466bd2df94feaef8625f7a345b7b3a0e))

### Miscellaneous

- update nom 8.0.0 and ircv3_tags 2.0.0 ([e2305a0](https://github.com/m3idnotfree/ircv3_parse/commit/e2305a018cd8ee61a3152f21944c47ae8aaf7ecc))

## [1.0.0](https://github.com/m3idnotfree/ircv3_parse/compare/v0.2.3..v1.0.0) - 2024-12-20

### Refactoring

- **(lib)** make module exports more explicit ([c54e40e](https://github.com/m3idnotfree/ircv3_parse/commit/c54e40e5e5215b058faf319b604b311a1f6c9525))

### Style

- add fn main to example ([39830fe](https://github.com/m3idnotfree/ircv3_parse/commit/39830fe876ea68a86903b58b50e0962e4b4d22fc))

### Other

- MIT-only ([7b81bc1](https://github.com/m3idnotfree/ircv3_parse/commit/7b81bc1d552c2c214aea9c858c2a4366efd1c5c6))

## [0.2.3](https://github.com/m3idnotfree/ircv3_parse/compare/v0.2.2..v0.2.3) - 2024-10-11

### Miscellaneous

- **(README)** update ([ad20951](https://github.com/m3idnotfree/ircv3_parse/commit/ad20951a79bd57dd5d1841d2fcea2c9fe122fdaf))

## [0.2.1](https://github.com/m3idnotfree/ircv3_parse/compare/v0.2.0..v0.2.1) - 2024-10-11

### Miscellaneous

- **(lib)** update doc ([d08db89](https://github.com/m3idnotfree/ircv3_parse/commit/d08db8932ed4198d08b682c61572262d9256de6b))

## [0.2.0](https://github.com/m3idnotfree/ircv3_parse/compare/v0.1.4..v0.2.0) - 2024-10-11

### Features

- **(command)** add command parser ([eda0db4](https://github.com/m3idnotfree/ircv3_parse/commit/eda0db4123965095db20635832d8c4bbcac416c0))
- **(builder)** new feature IRCv3Builder ([8059f8a](https://github.com/m3idnotfree/ircv3_parse/commit/8059f8ae0eea5ae602b63fdc8ab1eb8f0cf15803))
- **(params)** add ParamsParse trait etc ([e6b366d](https://github.com/m3idnotfree/ircv3_parse/commit/e6b366dcdc023ea0dfa94a4e2c17870b72476789))
- **(message)** base and generic struct ([df791c9](https://github.com/m3idnotfree/ircv3_parse/commit/df791c9952e7f8cbe0d0d31d9a669e0b309c5c4a))
- **(lib)** add IRCv3 struct ([398c463](https://github.com/m3idnotfree/ircv3_parse/commit/398c463abfd3ba348f386e7b198e316b8279b706))

### Bug Fixes

- **(lib)** return Option<IRCv3Prefix> ([8b18397](https://github.com/m3idnotfree/ircv3_parse/commit/8b183979b736a518307c638c45e20c46ddcc9d58))

### Refactoring

- **(prefix)** parse move fn ([04cc76e](https://github.com/m3idnotfree/ircv3_parse/commit/04cc76ec605f6fcf9a1406a36f4c8aefff3a7405))
- **(prefix)** IRCv3Prefix::parse() move fn prefix_parse ([bc73f51](https://github.com/m3idnotfree/ircv3_parse/commit/bc73f51108872e73efeef5e2604c96573a0a4d15))
- **(params)** all new ([bf82441](https://github.com/m3idnotfree/ircv3_parse/commit/bf824419edfbc74ce8affcb56d39349be898edd7))
- **(params)** remove channel field and parser ([7977280](https://github.com/m3idnotfree/ircv3_parse/commit/797728003a8e67b3407b58112afc4d9d7796c49f))

### Style

- format ([354e202](https://github.com/m3idnotfree/ircv3_parse/commit/354e2020790c52b47fee134dd27fc4fe3a0d9433))

### Tests

- **(prefix)** update ([933d9c3](https://github.com/m3idnotfree/ircv3_parse/commit/933d9c3bcb7d5a62addf21ca416632df7465a711))
- **(prefix)** update ([f64402a](https://github.com/m3idnotfree/ircv3_parse/commit/f64402adb409fb2a48f38d7e3710c56f233d4d49))
- **(command)** add command tests ([6b860ed](https://github.com/m3idnotfree/ircv3_parse/commit/6b860edf98e61f7d226c35e8c87663a67ae2ad53))
- **(all)** update ([f8451ef](https://github.com/m3idnotfree/ircv3_parse/commit/f8451efeac35c27a2d12fee0cc88362dfe9d9e1d))
- **(all)** update ([bd8fe95](https://github.com/m3idnotfree/ircv3_parse/commit/bd8fe95daddd7eb7238c144d638e624e58b5eb5a))
- update ([1671049](https://github.com/m3idnotfree/ircv3_parse/commit/1671049c21069b5f6fe82769d33397b99220fbb4))

### Miscellaneous

- **(cargo)** update ([8cf2776](https://github.com/m3idnotfree/ircv3_parse/commit/8cf2776c21cc7825c3bddbdb04968e46f770812e))
- **(rename)** prefix -> sourse ([d3c2f27](https://github.com/m3idnotfree/ircv3_parse/commit/d3c2f273f45c7a0c310999c7d2398fb2fe876203))
- **(prefix)** prefix -> source ([3843262](https://github.com/m3idnotfree/ircv3_parse/commit/3843262bdbaf06a9f59b0e295a5e9aed68e82c64))
- **(source)** rename ([da30919](https://github.com/m3idnotfree/ircv3_parse/commit/da3091937355c1acc3e1e516a3bd5735c09fe652))
- **(lib)** remove unused import ([f54ce65](https://github.com/m3idnotfree/ircv3_parse/commit/f54ce65911c9b0f32ed0eaccc039b11fce8fb7da))
- **(README)** update ([35448d2](https://github.com/m3idnotfree/ircv3_parse/commit/35448d22f9ce9f1dc9d0a5129e48a172d2f10d0e))

### Other

- **(parse)** IRCv3Tags return Option ([28736a6](https://github.com/m3idnotfree/ircv3_parse/commit/28736a67f891c84ea6ca6890127fe152b896e1bb))
- **(prefix)** return Option IRCv3Prefix ([0e1125d](https://github.com/m3idnotfree/ircv3_parse/commit/0e1125dbf21a03515a8c7bdd34f752f6f13b7fd6))
- **(prefix)** use String instead str ([4b99bd4](https://github.com/m3idnotfree/ircv3_parse/commit/4b99bd4c1aba944c2e0bfdd210072410773f8d68))
- **(prefix)** struct update ([a7e8c25](https://github.com/m3idnotfree/ircv3_parse/commit/a7e8c253cfd70431f5325b7c14663aecf7116f62))
- **(lib)** all new ([ad7a7d0](https://github.com/m3idnotfree/ircv3_parse/commit/ad7a7d01d5aded387743f543d1df812bc5b57093))

## [0.1.4](https://github.com/m3idnotfree/ircv3_parse/compare/v0.1.3..v0.1.4) - 2024-05-07

### Style

- format ([80ab194](https://github.com/m3idnotfree/ircv3_parse/commit/80ab1942569c974b405f4844577d2a6ca72af0db))

### Miscellaneous

- **(gitignore)** remove some file ([d54fe1b](https://github.com/m3idnotfree/ircv3_parse/commit/d54fe1b604981dbd0ba7d6c1a78d95a7ef5a5e8a))

## [0.1.3](https://github.com/m3idnotfree/ircv3_parse/compare/v0.1.2..v0.1.3) - 2024-03-16

### Refactoring

- **(IRCv3Prefix)** [**breaking**] add parse method ([c9d16af](https://github.com/m3idnotfree/ircv3_parse/commit/c9d16afe50fa09101586f17f970f9f8ca47b5f96))
- **(params)** use parse method ([5863002](https://github.com/m3idnotfree/ircv3_parse/commit/58630023ad1f2d8c411dd144fbe30b7619e9d37b))

## [0.1.2](https://github.com/m3idnotfree/ircv3_parse/compare/v0.1.1..v0.1.2) - 2024-03-12

### Features

- **(params)** change type to Option ([2b60276](https://github.com/m3idnotfree/ircv3_parse/commit/2b602767d1f337e3b50a7d1456dc74608a59c197))

### Miscellaneous

- tests update ([df08e69](https://github.com/m3idnotfree/ircv3_parse/commit/df08e692180ab94eadb943a689af3f26478cdb8a))

## [0.1.1](https://github.com/m3idnotfree/ircv3_parse/compare/v0.1.0..v0.1.1) - 2024-03-12

### Features

- pub use IRCv3Tags ([6b23959](https://github.com/m3idnotfree/ircv3_parse/commit/6b239599c5eef5c478fc3e4071f8225a34061d31))

## [0.1.0](https://github.com/m3idnotfree/ircv3_parse/compare/v0.0.8..v0.1.0) - 2024-03-12

### Features

- more flexible lib.rs ([1ae2d08](https://github.com/m3idnotfree/ircv3_parse/commit/1ae2d08bf22fb392208b948b212f2bac24a60457))

### Tests

- update ([58ced72](https://github.com/m3idnotfree/ircv3_parse/commit/58ced72ed0719a3cdf8867249959f1bf13a7d13a))

### Other

- prefix,params ([9ec5f08](https://github.com/m3idnotfree/ircv3_parse/commit/9ec5f08da5fd50eba26e4d8725a8c11ba6dc39e2))

## [0.0.8](https://github.com/m3idnotfree/ircv3_parse/compare/v0.0.7..v0.0.8) - 2024-03-02

### Bug Fixes

- **(Ircv3Params)** rename, add methods ([767454b](https://github.com/m3idnotfree/ircv3_parse/commit/767454b1e9bd0d98d66bd1c9ba49767adad7d4d9))
- **(Ircv3Prefix)** rename, add methods ([d466e60](https://github.com/m3idnotfree/ircv3_parse/commit/d466e603555aeb2671e938b49c7b9adcc89507a2))
- rename struct ChannelNMsg, MiddleNMsg ([89cb51e](https://github.com/m3idnotfree/ircv3_parse/commit/89cb51e1417e248071d2a0c006ad268714d3919d))

### Refactoring

- **(parse)** restructure parsing methods ([fad80c7](https://github.com/m3idnotfree/ircv3_parse/commit/fad80c76b394fb1fea693d1cd3bfc28026d662f4))

## [0.0.7](https://github.com/m3idnotfree/ircv3_parse/compare/v0.0.6..v0.0.7) - 2024-03-01

### Features

- **(params)** add methods channel ([43a1ae3](https://github.com/m3idnotfree/ircv3_parse/commit/43a1ae37dc51434ec6cb23b241476b9390c849e1))

## [0.0.6](https://github.com/m3idnotfree/ircv3_parse/compare/v0.0.5..v0.0.6) - 2024-03-01

### Bug Fixes

- add derive: Copy, Clone ([f393b18](https://github.com/m3idnotfree/ircv3_parse/commit/f393b188c94aadc59a60284f0e50389cc249fe90))

## [0.0.5](https://github.com/m3idnotfree/ircv3_parse/compare/v0.0.4..v0.0.5) - 2024-03-01

### Bug Fixes

- **(field)** change fields type from &str to String ([d1178f0](https://github.com/m3idnotfree/ircv3_parse/commit/d1178f0cb43f21102c0ed1167b268be29c6de563))

## [0.0.4](https://github.com/m3idnotfree/ircv3_parse/compare/v0.0.3..v0.0.4) - 2024-02-29

### Features

- **(channelNMsg)** add derive clone ([fb9a96f](https://github.com/m3idnotfree/ircv3_parse/commit/fb9a96f3d875c92d6d387c0e16d44b07562816a2))

## [0.0.3](https://github.com/m3idnotfree/ircv3_parse/compare/v0.0.2..v0.0.3) - 2024-02-29

### Bug Fixes

- add params struct ([2581564](https://github.com/m3idnotfree/ircv3_parse/commit/2581564fc076869e052b965567eb05d50f2c8959))

## [0.0.2] - 2024-02-29

### Features

- **(docs)** add README.md ([eae5b90](https://github.com/m3idnotfree/ircv3_parse/commit/eae5b90121c834aa516be09d3a8cb613e4236d8b))

### Bug Fixes

- tags, prefix type change ([5a14c6d](https://github.com/m3idnotfree/ircv3_parse/commit/5a14c6d436c7b5ebbd3ac67c71aa0dbb79dbea1f))
- method name change: string -> to_string ([38e5e87](https://github.com/m3idnotfree/ircv3_parse/commit/38e5e879e388495fd21111e48b9c52319c748711))
- command: String -> &str ([9d053c5](https://github.com/m3idnotfree/ircv3_parse/commit/9d053c5b3511acba1abe28acf2b8f3fa4a387f5b))

### Miscellaneous

- **(license)** add LICENSE-APACHE ([fbb15ad](https://github.com/m3idnotfree/ircv3_parse/commit/fbb15adf20d9ee79d46df1f92591b9feb49afbf3))
- **(license)** add LICENSE-MIT ([a1d23e0](https://github.com/m3idnotfree/ircv3_parse/commit/a1d23e0a1919ee292ee434217d73c14912f0eb75))
- **(cargo)** tags: path -> git ([74ff743](https://github.com/m3idnotfree/ircv3_parse/commit/74ff743510ff8a0afae8adfe80c7a29cac3ff12c))
- first commit ([b2e9a2d](https://github.com/m3idnotfree/ircv3_parse/commit/b2e9a2d7277b6249e610cf0f81b46dea3a2925e0))
- format, ([1e3152c](https://github.com/m3idnotfree/ircv3_parse/commit/1e3152c72a72e0726b816050ad1745f2f088e54d))
- format ([cb7690c](https://github.com/m3idnotfree/ircv3_parse/commit/cb7690ce7c0e99369ecef252962c3af19c6ca04e))
