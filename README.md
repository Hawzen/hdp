# What would happen if we didn't use TCP or UDP? 
Switches, bridges, routers, load balancers, firewalls—these network boxes keep the internet running. Routing, blocking, mirroring, duplicating and deduplicating traffic in ways most people never think about. Without them, this document wouldn’t have reached you

But the network is just one layer. The OS has its own way of handling packets—classifying, queuing, enforcing firewall rules, translating addresses, deciding what gets through and what gets dropped without a trace. Every part plays by its own rules, shaping what’s “allowed” and what's not

At some point, I wondered—*what if I sent a packet using a transport protocol that didn’t exist?* Not TCP, not UDP, not even ICMP—something completely made up. Would the OS let it through? Would it get stopped before it even left my machine? Would routers ignore it, or would some middlebox kill it on sight? Could it actually move faster by slipping past common firewall rules?

No idea.

So I had to try.

First, I sent the packets to myself, just to see how my own machine handled the poison I made up. Then, I sent them across continents to a remote Linux machine to see if they’d actually make it.
# Some background first
> [!NOTE]
> Feel free to skip this section if you already know how the internet works. Otherwise, continue reading on

But wait—what exactly is a transport layer protocol?

The internet isn’t magic. It just looks that way. Underneath, it’s a stack of protocols, each one shoving data to the next until it reaches its destination. At the application level, you send a request—loading a website, streaming a video, or whatever you do. That request gets wrapped by the OS in multiple layers of metadata, addresses, and headers, until it’s nothing but raw bits flying through the network

It kinda works like this:
<p align="center">  <img src="./readme_assets/internet_protocols.png" alt="a visual guide to how the internet works, it kinda sucks but that why i like it."> </p>
<p align="center"><sub>The diagram is 100% correct and should be included in all networking textbooks.</sub></p>

At the top, apps—browsers, games, whatever—generate requests (Load this page, Send this message, Connect to this game server). Then the requests start their descent through the network stack, getting wrapped, encoded, and addressed at each layer, until all that’s left is a stream of bits flying into the void

Each layer plays a role. IP assigns addresses and makes sure packets know where they’re going. The link layer handles the actual transmission—Wi-Fi, Ethernet, fiber optics, whatever. There’s more to it, but we’re not going down that rabbit hole right now. What matters is the layer that makes network communication actually usable

The **transport layer** is where networking personally starts to get interesting. It’s the first truly complex protocol layer. It doesn’t just move packets—it manages connections, makes sure multiple applications can share the same machine, and decides how data should flow.

This is where **TCP**, **UDP**, and their weird cousins live. The **IP Protocol** defines a field called `Protocol`. Setting this field to 6 means the encapsulated packet is TCP, 17 is UDP, and [there are others defined](https://en.wikipedia.org/wiki/List_of_IP_protocol_numbers) but some numbers are deliberately left out for future use

But what if we used those *unused* numbers?
# Experiment #1: Sending traffic.. to me!
There are simply too many variables to this experiment. My OS, my router, the receiver's OS, and god knows how many middle boxes are littered on the open internet. It's hard to extrapolate conclusions from experimentation with all these moving parts—so I thought of the following: To begin, I'll send the packets to *my own machine*, this guarantees that any results are solely due to my OS's behaviour

First, I designed a [simple protocol](./hdp_specification.md): **HDP**. The specifics don’t matter—what matters is that it doesn’t resemble any known protocol. It’s an outsider, something the OS and network stack weren’t expecting

Next, I built a [server, or a listener](./src/server/main.rs), whatever you call it. The machine running this code will be patiently waiting for any packets. Then I wrote a [client](./src/client/main.rs), the machine running this code will send HDP packets to the server

Finally, here are the steps I'll attempt
1. Startup an HDP server
	- Which will ask the OS to forward any packets with the protocol 255 to a socket it controls
2. Run the HDP client, sending packets to my local machine
	- The client will ask the OS to nicely deliver the packets to 127.0.01
		- The OS is configured to hand packets with that target address to the loopback [network interface](https://en.wikipedia.org/wiki/Network_interface_controller)
			- The loopback interface should realize: "uhhh.. this packet should go right back in?", and send it back to my own machine
3. The OS delivers them to the HDP server unmodified..?? 🤞

Let's do it

I opened two shells—one was the server:
```haskell
$ sudo cargo run --bin server
```

And in another shell I opened the client
```haskell
$ fortune | cowsay | sudo cargo run --bin client 127.0.0.1
```

Alright, let's send the packet via the client. 3, 2, 1, and.. 

The server got the message!
```haskell
$ sudo cargo run --bin server
~~~ IP Header ~~~
Version: 4
IHL: 5
DSCP: 0
ECN: 0
Total Length: 58625
Identification: 36455
Flags: 0
Fragment Offset: 0
TTL: 64
Protocol: 255
Header Checksum: 0
Source IP: [127, 0, 0, 1]
Destination IP: [127, 0, 0, 1]


~~~ HDP Header & Data ~~~
Source Port: 420
Destination Port: 420
Timestamp: 1739640243546134000
Data:  _________________________________________
/ Marriage is not merely sharing the      \
| fettucine, but sharing the burden of    |
| finding the fettucine restaurant in the |
| first place.                            |
|                                         |
\ -- Calvin Trillin                       /
 -----------------------------------------
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||
```

Success! The OS accepted my protocol, looped it back, and delivered it to the server with no shenanigans happening, unexpected!. But before calling it a day, I had another question:

What would happen if we repeated this experiment, whilst changing the protocol number defined in the IP packet? 

My initial choice of **255** was arbitrary—it was an unused protocol number. But what if I tried something more… unconventional? I decided to test different protocol numbers, including:
- 6, the number assigned to **TCP** packets
- Or 2, which is the protocol number used for **ICMP** (i.e., the thing powering `ping`)
- Or even 256, an index beyond the defined boundaries of the IP Protocol
Would they make it? Would the OS freak out?

Let's see:
```haskell
fortune | cowsay | sudo cargo run --bin client 127.0.0.1 # This time looping over protocol numbers
```

**Results**:

| Protocol Number | Source IP (Server) | Byte Sum (Server) | Received (Server) | Succeeded (Client) | Byte sum (Client) | Failure reason (Client)                          | Time difference (μs) |
| --------------: | :----------------- | ----------------: | :---------------- | :----------------- | :---------------- | :----------------------------------------------- | -------------------: |
|               0 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   70 |
|               1 | nan                |               nan | 🤯                | 🫡                 | 373               | -                                                |                  nan |
|               2 | nan                |               nan | 🤯                | 🫡                 | 373               | -                                                |                  nan |
|               3 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   61 |
|               4 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   52 |
|               5 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   54 |
|               6 | nan                |               nan | 🤯                | 🫡                 | 373               | -                                                |                  nan |
|               7 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   77 |
|               8 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   63 |
|               9 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   66 |
|              10 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   52 |
|              11 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   52 |
|              12 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   63 |
|              13 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   63 |
|              14 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   50 |
|              15 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   80 |
|              16 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   64 |
|              17 | nan                |               nan | 🤯                | 🫡                 | 373               | -                                                |                  nan |
|              18 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   42 |
|              19 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   82 |
|              20 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   71 |
|              21 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   59 |
|              22 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   50 |
|              23 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   51 |
|              24 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   54 |
|              25 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   46 |
|              26 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   48 |
|              27 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   43 |
|              28 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   46 |
|              29 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   66 |
|              30 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   56 |
|              31 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   65 |
|              32 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   56 |
|              33 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   49 |
|              34 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   47 |
|              35 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   48 |
|              36 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   59 |
|              37 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   47 |
|              38 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   45 |
|              39 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   52 |
|              40 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   57 |
|              41 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   56 |
|              42 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   51 |
|              43 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   45 |
|              44 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   58 |
|              45 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   52 |
|              46 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   50 |
|              47 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   46 |
|              48 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   51 |
|              49 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   84 |
|              50 | nan                |               nan | 🤯                | 🤯                 | -                 | Operation not supported on socket (os error 102) |                  nan |
|              51 | nan                |               nan | 🤯                | 🤯                 | -                 | Operation not supported on socket (os error 102) |                  nan |
|              52 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   92 |
|              53 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  115 |
|              54 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   81 |
|              55 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   83 |
|              56 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   96 |
|              57 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   71 |
|              58 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   69 |
|              59 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   80 |
|              60 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   84 |
|              61 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  105 |
|              62 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  109 |
|              63 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   97 |
|              64 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  100 |
|              65 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   94 |
|              66 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  124 |
|              67 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  101 |
|              68 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  100 |
|              69 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   87 |
|              70 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   95 |
|              71 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  101 |
|              72 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   97 |
|              73 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  111 |
|              74 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  104 |
|              75 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  115 |
|              76 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   96 |
|              77 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   77 |
|              78 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   65 |
|              79 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   54 |
|              80 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  150 |
|              81 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   95 |
|              82 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   97 |
|              83 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   74 |
|              84 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   93 |
|              85 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   71 |
|              86 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   77 |
|              87 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   70 |
|              88 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   49 |
|              89 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   59 |
|              90 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   74 |
|              91 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   78 |
|              92 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   61 |
|              93 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   59 |
|              94 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   55 |
|              95 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   46 |
|              96 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   59 |
|              97 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   94 |
|              98 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   66 |
|              99 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   54 |
|             100 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   53 |
|             101 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   91 |
|             102 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  148 |
|             103 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  111 |
|             104 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  119 |
|             105 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   75 |
|             106 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   52 |
|             107 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   53 |
|             108 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   52 |
|             109 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   44 |
|             110 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   59 |
|             111 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   51 |
|             112 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   45 |
|             113 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   75 |
|             114 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   91 |
|             115 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   85 |
|             116 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   84 |
|             117 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   64 |
|             118 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   24 |
|             119 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   46 |
|             120 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   62 |
|             121 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   48 |
|             122 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   50 |
|             123 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   50 |
|             124 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   49 |
|             125 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   74 |
|             126 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   54 |
|             127 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   46 |
|             128 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  103 |
|             129 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   73 |
|             130 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   57 |
|             131 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   49 |
|             132 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   62 |
|             133 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   43 |
|             134 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   47 |
|             135 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   90 |
|             136 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  112 |
|             137 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   94 |
|             138 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   53 |
|             139 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   57 |
|             140 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   74 |
|             141 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   64 |
|             142 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   77 |
|             143 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   77 |
|             144 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   75 |
|             145 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   77 |
|             146 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   88 |
|             147 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   96 |
|             148 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  106 |
|             149 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   72 |
|             150 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   80 |
|             151 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   77 |
|             152 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   78 |
|             153 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   91 |
|             154 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   75 |
|             155 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   80 |
|             156 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   96 |
|             157 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  110 |
|             158 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  105 |
|             159 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   83 |
|             160 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   89 |
|             161 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   95 |
|             162 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  111 |
|             163 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  103 |
|             164 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   97 |
|             165 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   91 |
|             166 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   95 |
|             167 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   84 |
|             168 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   57 |
|             169 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   50 |
|             170 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   65 |
|             171 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   75 |
|             172 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   80 |
|             173 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   78 |
|             174 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   67 |
|             175 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   55 |
|             176 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   60 |
|             177 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   85 |
|             178 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   78 |
|             179 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   73 |
|             180 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   79 |
|             181 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   91 |
|             182 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   96 |
|             183 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   88 |
|             184 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   95 |
|             185 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   91 |
|             186 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   74 |
|             187 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   92 |
|             188 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   79 |
|             189 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   75 |
|             190 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   81 |
|             191 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   96 |
|             192 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   95 |
|             193 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   91 |
|             194 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   88 |
|             195 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   92 |
|             196 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   99 |
|             197 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   90 |
|             198 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   90 |
|             199 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  100 |
|             200 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   96 |
|             201 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   89 |
|             202 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  100 |
|             203 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   92 |
|             204 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  109 |
|             205 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  104 |
|             206 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  108 |
|             207 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   95 |
|             208 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   96 |
|             209 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   71 |
|             210 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   76 |
|             211 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   71 |
|             212 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   78 |
|             213 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   91 |
|             214 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   97 |
|             215 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   97 |
|             216 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   93 |
|             217 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  105 |
|             218 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   97 |
|             219 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   91 |
|             220 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   98 |
|             221 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   90 |
|             222 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  108 |
|             223 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   92 |
|             224 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  104 |
|             225 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  109 |
|             226 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   94 |
|             227 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   99 |
|             228 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   94 |
|             229 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   79 |
|             230 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   84 |
|             231 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   79 |
|             232 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  102 |
|             233 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  101 |
|             234 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  113 |
|             235 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   95 |
|             236 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  100 |
|             237 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   91 |
|             238 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  106 |
|             239 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   92 |
|             240 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   97 |
|             241 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   89 |
|             242 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   99 |
|             243 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   90 |
|             244 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   98 |
|             245 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   93 |
|             246 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   94 |
|             247 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   91 |
|             248 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   94 |
|             249 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   94 |
|             250 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   90 |
|             251 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   88 |
|             252 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   94 |
|             253 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   96 |
|             254 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   76 |
|             255 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                  nan |
|             255 | 127.0.0.1          |               373 | 🫡                | 🫡                 | 373               | -                                                |                   71 |
|             256 | nan                |               nan | 🤯                | 🤯                 | -                 | Invalid argument (os error 22)                   |                  nan |
## What’s up with these failures?
Most protocol numbers worked fine—the OS saw the packet, looped it back, and my server received it without an issue. But a few of them outright _failed_ at different points in the stack
- **Protocols 1, 2, and 6 failed at the server side**. Meaning: the client successfully sent them, but the server never saw them
- **Protocols 50 and 51 failed at the client side**. The OS refused to even send them
- **Protocol 256 didn't even make it past the `socket()` call**

But *why?* What’s making the OS treat these packets differently?
## Syscalls: What actually matters
One of the most useful debugging techniques I learnt debugging this stuff is, when dealing with low-level code, trace the *system calls* a process is making

A [system call](https://en.wikipedia.org/wiki/System_call) for the uninitiated is just a function that allows applications to request privileged resources from the OS—whether that’s opening a file, allocating memory, or, in our case, sending a packet over the network

In my Rust code I use a library called [`socket2`](https://docs.rs/socket2/latest/socket2/index.html) which implements a pretty wrapper over the system calls provided by my OS. And to send a packet, I request a socket—which you can think of as just a special file my code can write in to communicate over the network

Here's what the client would do:
```c
int sockfd = socket(
    AF_INET,    // Domain: ARPA Internet protocols. This tells the OS that we're interested in the IP protocols
    SOCK_RAW,   // Type: Raw socket. The OS normally handles the transport layer, but this gives us full control.
    255         // Protocol: We looped over this field.
);
```
## Revisiting the failures
**1, 2, and 6: The Server Never Sees Them**  
These packets were successfully transmitted from the client, but they were intercepted before my server had a chance to look at them. That suggests something inside the OS intercepted them

Originally, I assumed my server would capture any raw IP packet it received. The socket looked like this:
```c
int sockfd = socket(
    AF_INET,    // Internet domain
    SOCK_RAW,   // Raw socket: should give us full control
    0           // Let the OS decide the protocol
);
```

I expected 0 to mean:
*"Give me everything—TCP, UDP, whatever it is, forward it"*  

For context, I ran these experiments on my Mac, which runs Darwin. Looking at the [documentation](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/socket.2.html), there is really nothing mentioning the Protocol Number = 0 trick

Under the hood, Darwin is just like BSD but with a ton of makeup, meaning it inherits BSD’s socket behaviour and network stack quirks. And on a whim I checked the **[BSD socket documentation](https://man.openbsd.org/socket.2)**, and I found this frustratingly vague line:  

> "A value of 0 for `protocol` will let the system select an appropriate protocol for the requested socket type."  

So instead of delivering **all** raw packets, my OS was silently (and haphazardly) filtering them. My server never even saw the ICMP (1), IGMP (2), or TCP (6) packets—because Darwin likely deemed my socket not appropriate to receive those protocols.. or something?

**50 and 51: The Client Can’t Even Send Them**  
Here, the OS flat-out refused to send the packets. These aren’t just arbitrary numbers—they’re part of **IPSec (ESP and AH)**, which is used for encrypted VPN traffic. I'm not sure _why_ the OS blocked them, but I imagine it's a security feature of sorts in Darwin

**256: The `socket()` Call Fails Immediately**  
This one is simple:  
- The IPv4 protocol field is 8 bits meaning valid values range from 0 to 255
- 256 is simply too large—the OS rejects it outright as an invalid argument

No surprises here. But what *was* surprising is what happened when I tried the same experiment on Linux..

After seeing these inconsistencies, I was curious as to how Linux would behave. So I spun up a Linux VM and re-ran the experiment. Right away, the behaviour was very different

Running the server I quickly noticed that Linux does not allow binding a raw socket to protocol `0`—Some invalid protocol numbers like 256 *worked*. For reference, I logged the results in [`results_no_server_linux_client_loopback`](./samples/results_no_server_linux_client_loopback.md). I was satisfied that at least _some_ of the protocol numbers were working as expected
## Lessons learned
Custom transport-layer protocols are doable, buuuuut the OS isn’t exactly welcoming. The networking stack has so many assumptions baked in, and raw sockets aren’t as raw as you’d expect

I imagine this is why most new protocols live at the application layer instead. Instead of fighting the OS, engineers just build on top of existing transport protocols. QUIC, for example, runs over UDP and avoids these issues entirely

And if you're ever working with raw sockets, *please* test across multiple OSes. If Darwin lets you do something, Linux might shut it down. If Linux is fine with it, Windows might pretend it doesn’t exist. There’s really no universal behaviour, even if they claim to _implement the POSIX standard_

## Next step: What happens outside loopback?
So far, these packets never left my machine. Now, I want to send HDP over the public internet:
- Will routers forward it, or will they drop it?
- Will firewalls let it through, or flag it as an attack?
- Will it have different latency compared to TCP?
- Will I accidentally brick DigitalOcean’s network? :D
Time to find out
# Experiment #2: 
At first I expected this experiment to be straight-forward (spoilers: it was NOT). How could it not..? 

I planned to deploy my server on a machine using a cheap cloud provider like Digital Ocean—then I'd send all sorts of packets to it, UDP, TCP, my own protocol, you name it. Gathering statistics about packet drop, latency, whatever, then I'd make conclusions about the feasibility of not using UDP/TCP

Simple!

But oh it was not, not at all. It wasn't that the experiment was difficult to setup—but what weirded me out was the results.. they weren't anything I expected or was prepared to deal with. Keep reading to see why
## Setting up the server
I rented the the cheapest VPS on Digital Ocean I could find, then set up my server and all the tooling I needed. Nice!

Let's see where the server is..
```haskell
root@debian-s-1vcpu-512mb-10gb-fra1-01:~# curl myip.wtf
161.35.222.56
root@debian-s-1vcpu-512mb-10gb-fra1-01:~# curl ipinfo.io/161.35.222.56
{
  "ip": "161.35.222.56",
  "city": "Frankfurt am Main",
  "region": "Hesse",
  "country": "DE",
  "loc": "50.1155,8.6842",
  "org": "AS14061 DigitalOcean, LLC",
  "postal": "60306",
  "timezone": "Europe/Berlin",
  "readme": "https://ipinfo.io/missingauth"
}
```

Alright, looks like the experiment will span continents given that I'm running my client on Saudi Arabia, and the server is hosted in Frankfurt

Before running any deep analysis, I wanted to check that there is a network path between my Mac and the server, so I `ping`'ed the server from my Mac
```haskell
❯ ping 161.35.222.56
PING 161.35.222.56 (161.35.222.56): 56 data bytes
64 bytes from 161.35.222.56: icmp_seq=0 ttl=47 time=125.364 ms
64 bytes from 161.35.222.56: icmp_seq=1 ttl=47 time=128.061 ms
64 bytes from 161.35.222.56: icmp_seq=2 ttl=47 time=177.931 ms
64 bytes from 161.35.222.56: icmp_seq=3 ttl=47 time=225.798 ms
64 bytes from 161.35.222.56: icmp_seq=4 ttl=47 time=130.101 ms
64 bytes from 161.35.222.56: icmp_seq=5 ttl=47 time=194.563 ms
64 bytes from 161.35.222.56: icmp_seq=6 ttl=47 time=159.518 ms
64 bytes from 161.35.222.56: icmp_seq=7 ttl=47 time=134.343 ms
64 bytes from 161.35.222.56: icmp_seq=8 ttl=47 time=501.139 ms
64 bytes from 161.35.222.56: icmp_seq=9 ttl=47 time=153.672 ms
64 bytes from 161.35.222.56: icmp_seq=10 ttl=47 time=137.927 ms
64 bytes from 161.35.222.56: icmp_seq=11 ttl=47 time=355.672 ms
64 bytes from 161.35.222.56: icmp_seq=12 ttl=47 time=138.777 ms
64 bytes from 161.35.222.56: icmp_seq=13 ttl=47 time=166.116 ms
64 bytes from 161.35.222.56: icmp_seq=14 ttl=47 time=288.758 ms
64 bytes from 161.35.222.56: icmp_seq=15 ttl=47 time=151.458 ms
64 bytes from 161.35.222.56: icmp_seq=16 ttl=47 time=164.025 ms
64 bytes from 161.35.222.56: icmp_seq=17 ttl=47 time=170.132 ms
64 bytes from 161.35.222.56: icmp_seq=18 ttl=47 time=279.034 ms
^C
--- 161.35.222.56 ping statistics ---
19 packets transmitted, 19 packets received, 0.0% packet loss
```

It seems it's quite far, but looks fine to me, let's send some packets using our new protocol!

First let's start the server in our Digital Ocean machine
```haskell
root@debian-s-1vcpu-512mb-10gb-fra1-01:~/hdp/hdp# sudo cargo run --bin server
Listening on protocol 255
```

And now we can send a packet from my Mac

```haskell
❯ fortune | cowsay | sudo cargo run --bin client 161.35.222.56
| Protocol Number | Succeeded (Client) | Time (μs) (Client) | Byte sum (Client) | Failure reason (Client) |
| 255 | 🫡 | timestamp | 563 | - |
```

Packet sent. Let's check the server again

```haskell
root@debian-s-1vcpu-512mb-10gb-fra1-01:~/hdp/hdp# sudo cargo run --bin server
Listening on protocol 255
| Protocol Number | Time (μs) (Server) | Source IP (Server) | Byte Sum (Server) |
| --- | --- | --- |
| 255 | timestamp | my_ip | 563 |
```

Excellent. It seems that all went well, or so I thought. In-fact, all went downhill starting here. I took a quick break then came back. Let's try sending the packet again..

```haskell
| Protocol Number | Time (μs) (Server) | Source IP (Server) | Byte Sum (Server) |
| --- | --- | --- |
| 255 | timestamp | my_ip | 563 |
```

It's stuck? I can't see the second packet

I `Ctrl+C` and attempt doing it again. No results..? That can't be right, could it be a client side bug? Let's use `tcpdump` to see all outgoing packets from my device
```haskell
❯ sudo tcpdump -i any 'ip[9] == 255'
tcpdump: data link type PKTAP
tcpdump: verbose output suppressed, use -v[v]... for full protocol decode
listening on any, link-type PKTAP (Apple DLT_PKTAP), snapshot length 524288 bytes
IP mac > 161.35.222.56:  reserved 427
IP mac > 161.35.222.56:  reserved 427
IP mac > 161.35.222.56:  reserved 427
IP mac > 161.35.222.56:  reserved 427
IP mac > 161.35.222.56:  reserved 427
IP mac > 161.35.222.56:  reserved 427
IP mac > 161.35.222.56:  reserved 427
IP mac > 161.35.222.56:  reserved 427
```

They're definitely leaving my Mac. What about doing the same thing on the receiving end?

```haskell
root@debian-s-1vcpu-512mb-10gb-fra1-01:~/hdp# tcpdump -i any 'ip[9] > 17'
tcpdump: data link type LINUX_SLL2
tcpdump: verbose output suppressed, use -v[v]... for full protocol decode
listening on any, link-type LINUX_SLL2 (Linux cooked v2), snapshot length 262144 bytes

```

Nothing appeared

I began doubting my earlier results, there they are in my shell. The timestamps and byte sums match. Was I imagining them? Is Linus Torvalds himself gaslighting me??

Wait..? How did my ISP's [NATing box](https://simple.wikipedia.org/wiki/Network_address_translation) forward the packet? NAT'ing relies on ports—but my protocol is just black magic to them

I'm confused

Very confused

After digging a bit in, I found that Digital Ocean doesn't support non-standard IP Protocols

![digital_ocean_sucks](./readme_assets/ihatedigitalocean.png)

This still doesn't explain it. How did one packet survive? There really is no way to know, and I was banging my head against the wall trying to figure it out

### One. Last. Try
if any cloud provider would support non-standard IP Protocols, it'd be AWS

I provisioned two machines. Set them up. Server. Client. It works.. !
```haskell
admin@ip-172-31-13-218:~/hdp$ sudo cargo run --bin server 255
Server is listening on SockAddr { ss_family: 2, len: 16 }, protocol: 255
| Protocol Number | Time (μs) (Server) | Source IP (Server) | Byte Sum (Server) |
| --- | --- | --- |
| 255 | timestamp | 54.153.13.186 | 33 |
| 255 | timestamp | 54.153.13.186 | 34 |
| 255 | timestamp | 54.153.13.186 | 35 |
| 255 | timestamp | 54.153.13.186 | 36 |

```

Granted, the server was just two hops away from the client, and it didn't have to pass through the scary sea of the internet

<img src="./readme_assets/latency_difference_between_hdp_and_udp.png" alt="Description" style="border-radius: 15px; width: 100%;">
<p align="center"><sub>The latency is in the microseconds due to both machines being in the same datacenter.</sub></p>

The latency difference between the HDP & UDP was a consistent, but negligible 20μs across various benchmarks

#### But what about the internet?
I tried sending packets from my Mac to the AWS server, and I reproduced the same one packet behaviour above. I left a sample of the results in [`tcpdump_tokyo_server_mac_client.md`](./samples/tcpdump_tokyo_sever_mac_client.md). I sent 1 packet for all protocols, and all of them stopped working after the first packet except UDP/TCP/ICMP

And as expected, sending or recieving packets from the Digital Ocean to AWS didn't work

There's no way to know for sure.

# Lessons learned
Technically *yes*, you could use your own IP protocol. But unless you're a masochist, I do not suggest it
- Your code won't be portable, and you'll need to support various operating systems
- Your protocol will be randomly dropped at NAT gateways & firewalls. It might work on your own network, but I gaurentee it won't work on the internet
- From my testing, there's no latency improvements from using a non-standard IP protocol

TL;DR: ***Use UDP or TCP***
# Resources
- The [UDP protocol specification](https://datatracker.ietf.org/doc/html/rfc768) is so minimal it is almost funny
- [IP Protocol numbers that are assigned for testing](https://datatracker.ietf.org/doc/html/rfc3692#section-2.1)
- [The list of protocols](https://en.wikipedia.org/wiki/List_of_IP_protocol_numbers) supported under the IP protocol is pretty interesting
- [This](https://hackaday.com/2024/09/21/when-raw-network-sockets-arent-raw-raw-sockets-in-macos-and-linux/) article speaks about some differences between raw sockets in Linux & FreeBSD
- How would you implement NAT on something other than UDP or TCP? [This](https://superuser.com/a/1108226) answer is pretty insightful