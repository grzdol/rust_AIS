### rust_AIS

Everything sent through socket or channel is supposed to be tokio::Bytes

## Client architecture
Client works with 2 threads.  
1) Weak sender- a thread that sends packages without any confirmations to Server through UDP
2) Strong sender

## Sending NMEA messages through TCP conveniently, robustly and efficiently
We will use tokio::utils::codec::LinesCodec.  
According to wikipedia(xd) NMEA messages shouldn't contain endline so it should work.  
Also I'd like to add timestamp to every message. (Fun fact AIS messages do not support timestamps).  
We add timestamp to message by addint "TIMESTAMP" + timestamp string for easy message decomposition