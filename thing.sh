cargo build --release
scp ./target/armv5te-unknown-linux-musleabi/release/robocode robot@10.99.118.128:/home/robot/
ssh robot@10.99.118.128 "/home/robot/robocode"
