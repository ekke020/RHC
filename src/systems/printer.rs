pub mod print {
    const SHA_224_TEST: &str = "d14a028c2a3a2bc9476102bb288234c415a2b01f828ea62ac5b3e42f";
    const SHA_256_TEST: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    const SHA_384_TEST: &str = "38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b";
    const SHA_512_TEST: &str = "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e";
    const SHA_512_224_TEST: &str = "6ed0dd02806fa89e25de060c19d3ac86cabb87d6a0ddd05c333b84f4";
    const SHA_512_256_TEST: &str = "c672b8d1ef56ed28ab87c3622c5114069bdd3ad7b8f9737498d0c01ecef0967a";

    pub fn help() {
        println!("Usage: [hash]... [options] ...");
        println!("Tries to reveal the original content of a hashed value.");
        println!("A hash is always mandatory and should be provided as the first argument.");
        println!("{:>5}{}", "", "-h, --help \t Show this message.");
        println!("{:>5}{}", "", "-l, --length \t The length of the hashed value.");
        println!("{:>5}{}", "", "-t, --type \t Specifies the algorithm used to generate the hash.");
        println!("{:>5}{}", "", "-a, --all  \t List all possible algorithms.");
    }

    pub fn available_algorithms() {
        println!("SHA_224");
        println!("SHA_256");
        println!("SHA_384");
        println!("SHA_512");
        println!("SHA_512/224");
        println!("SHA_512/256");
    }

    pub fn tests() {
        println!("SHA 224: {}", SHA_224_TEST.as_bytes().len() * 4);
        println!("SHA 256: {}", SHA_256_TEST.as_bytes().len() * 4);
        println!("SHA 384: {}", SHA_384_TEST.as_bytes().len() * 4);
        println!("SHA 512: {}", SHA_512_TEST.as_bytes().len() * 4);
        println!("SHA 512/224: {}", SHA_512_224_TEST.as_bytes().len() * 4);
        println!("SHA 512/256: {}", SHA_512_256_TEST.as_bytes().len() * 4);
    }

    pub fn elapsed_time(elapsed: u64) {
        let seconds = (elapsed % 3600) % 60;
        let minutes = (elapsed % 3600 - seconds) / 60;
        let hours = (elapsed - minutes * 60 + seconds) / 3600;
        println!("H: {}, M: {}, S: {}", hours, minutes, seconds);
    }
}
