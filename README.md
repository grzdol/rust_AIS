## rust_AIS
This is sort of a rust implementation of [AIS system](https://pl.wikipedia.org/wiki/Automatic_Identification_System). Purpose of this system is to have a method of passing position and other maritime data between sailboats in fleet. Currently most of civil charter sailboats don't have AIS transmitter, so during [KŻUW](https://klubzeglarski.uw.edu.pl/) voyages, when there are more than two boats in fleet and we loose internet on sea, we don't know anything about other boats and other boats don't know anything about us.  
### Client
Every Client communicates with Server through two Senders: WeakSender and StrongSender.  
Why two? Because we would like to be able to send to server the whole history of client, so we can't affor losing data, while also maximizing number of packets received by Server. So for instance if we'd use simply TCP, we would achieve first goal, however in high packet loss environment we could really struggle to send any information. On the other hand if we'd use only UDP we could loose a lot of data.  
That's why we have two senders. Currently there are two concrete implementations of Client: TcpUdpClient and BrokenClient. BrokenCLient is testing utility, Client that can't send data through either Strong or Weak sender, it can only communicate through broacdcaster.
### Broadcaster
Consider situation where we have two Clients A, B close to each other, in range of radio communication, however Client A doesn't have internet connection. We would like to have a mechanism through which client A could pass data to B, which could publish data to Server. That's the purpose of Broadcaster. Every Client, except two Senders, also has a Broadcaster. It's purpose is to send data to all available clients. It broadcasts data received both from Client-owner and from all Broadcasters (cycle handling is done). Currently there is one concrete implementation of Broadcaster, BroadcasterMockup which is simply broadcast channel from tokio. I've also started to write UdpBroadcaster but I bugged it as hell and it doesn't work yet. Either way it would be just another testing utility. Goal is to have a Broadcaster that works through radio, but that's a topic I haven't started working on yet.  
### Server
Handles Clients and publishes data further. The goal is to have some django service for hosting the data with endpoints for real-time data and history. I didn't do it yet but either way it's not something interesting from rust point of view. Right now server simply passes data to locally run tcp servers, one of which is OpenCpn hence simulation.  
### BoatState
This is imply a data provider for Client. There are currently two implementations: Mockup which simply calculates position based on time, initial position and velocity, and BoatStateUdp which receives data to socket and passes to client through tokio channel.  
### What is currently done?
In summary:  
1) The whole architecture described above is implemented abstractly. Adding a component which uses a different interface in certain part of communication would simply mean writing a Sender and a Receiver for this interface and implementing some boilerplate.  
2) Concrete implementation of Server: TcpUdpServer
3) Concrete implementation of Client: TcpUdpClient and testing utility BrokenClient (currently they run with BroadcasterMockup)
4) Concrete implementation of Broadcaster: BroadcasterMockup
  
What was shown in the simulation is bin/runner_tcp_udp_server.rs. It runs a TcpUdpServer and 7 clients, some are TcpUdpClients and some are BrokenClients. Simulation was suppsoed to show that this architecture actually works, and clients that don't have internet are also visible in simulation.  
### What's left?
I guess everything I assumed at the beginning was done. To make this work in reality I'd have to implement RadioBroadcaster but it requires some hardware. Also the mention hosting service but it's not rust job.
