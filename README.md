# What would happen if we didn't use TCP or UDP? 
Switches, bridges, routers, load balancers, firewalls—these are the unsung gatekeepers of the internet, silently directing, blocking, mirroring, and reshaping traffic to make everything work. Without them, this very document wouldn’t have reached you

But the magic doesn’t stop at the network. The operating system has its own invisible choreography—classifying packets, queuing them, enforcing firewall rules, translating addresses, and deciding which data gets the green light and which gets silently discarded. It’s a layered activity, with each component playing by its own rules, shaping what’s “allowed” and what never sees the light of day

One day, I had a thought: **what if I sent a packet using a transport protocol that didn’t exist?** Not TCP, not UDP, not even ICMP—just something I made up. Would my OS play along, or shut it down before it even left the machine? Would it slip through routers unnoticed? Would some middlebox, somewhere, sniff it out and discard it as an abomination? Could it, by some strange quirk of network filtering, actually travel _faster_ by avoiding common firewall rules?

I had no clue

So, naturally, I had to try

To find out, I ran two experiments. First, I sent my custom packets from my machine to itself—just to see how the OS would handle them. Then, I raised the stakes: I launched them across continents to a remote Linux server, waiting to see if they’d survive the journey
# Some background first
> [!NOTE]
> Feel free to skip this section if you already know how the internet works. Otherwise, continue reading on

But wait—what exactly is a transport layer protocol?

The internet isn’t magic (though it often feels like it). It's a tower of protocols, each playing a distinct role in getting messages from one machine to another. At the application level, you send a request—maybe ordering food, loading a website, or streaming a video. That request then gets wrapped in multiple layers of instructions, addresses, and metadata, until it finally turns into raw bits ready to be launched across the network

It kinda works like this:
<p align="center">  <img src="./readme_assets/internet_protocols.png" alt="a visual guide to how the internet works, it kinda sucks but that why i like it."> </p>
<p align="center"><sub>The diagram is 100% correct and should be included in all networking textbooks.</sub></p>

At the top, applications like browsers, games, or messaging services generate requests (like “*Load this website*”, “*Send this message*”, or “*Connect me to this game server*”). These requests then begin their descent through the network stack, getting wrapped, encoded, and addressed at each layer until they’re nothing but streams of raw bits, ready to be sent off into the void that is the internet

Each layer plays a role in this journey. **IP**—the Internet Protocol—is what gives every machine a unique address, making it possible to route data across networks. The **link layer** takes care of actually moving bits between physical devices, whether over Wi-Fi, Ethernet, or even fiber optics. There are more layers, each with its own responsibilities, but we won’t get into all of them now. Instead, let’s focus on the layer that makes network communication truly practical

The **transport layer** is where networking starts to get interesting. It’s the first truly complex protocol layer, responsible for more than just moving packets—it enables multiple applications to share the same machine, ensures reliability when needed, and defines how data flows between systems

This is where **TCP**, **UDP**, and friends come into play. The **IP Protocol** defines a field called `Protocol`. Setting this field to 6 means the encapsulated packet is TCP, 17 is UDP, and [there are others defined](https://en.wikipedia.org/wiki/List_of_IP_protocol_numbers) but some numbers are deliberately left out for future use

But what if we used those *unused* numbers?
# Experiment #1: Sending traffic.. to me!
First, I designed a [simple protocol](./hdp_specification.md): **HDP**. The specifics don’t matter—what matters is that it doesn’t resemble any known protocol. It’s an outsider, something the OS and network stack weren’t expecting

Next, I built a [server](./src/server/main.rs) and a [client](./src/client/main.rs). The client crafts an HDP packet (with an IP Protocol of 255) and hands it off to the OS, trusting it to deliver the data to 127.0.0.1—the loopback interface. This sends the packet full circle, back to the OS, which (if all goes well) will recognize it, process it, and hand it off to the waiting server..! 

I opened two shells—one was the client:
```haskell
$ fortune | cowsay | sudo cargo run --bin client 127.0.0.1
```

And in another shell I opened the server
```haskell
$ sudo cargo run --bin server
```

Alright, let's send the packet via the client. 3, 2, 1, and.. the server got the message!
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

Success! The OS accepted my protocol, looped it back, and delivered it to the server as if nothing was unusual. But before calling it a day, I had another question:

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

### What’s up with these failures?
Most protocol numbers worked fine—the OS saw the packet, looped it back, and my server received it without an issue. But a few of them outright failed at different points in the stack
- **Protocols 1, 2, and 6 failed at the server side**. Meaning: the client successfully sent them, but the server never saw them
- **Protocols 50 and 51 failed at the client side**. The OS refused to even send them
- **Protocol 256 didn't even make it past the `socket()` call**

But *why?* What’s making the OS treat these packets differently?
### Syscalls: What actually matters
One of the most useful debugging techniques I learnt early on when dealing with low-level code is to trace the *system calls* a process is making

A [system call](https://en.wikipedia.org/wiki/System_call) for the uninitiated is just a function that allows applications to request privileged resources from the OS—whether that’s opening a file, allocating memory, or, in our case, sending a packet over the network

In my Rust code I use a library called [`socket2`](https://docs.rs/socket2/latest/socket2/index.html) which implements a pretty wrapper over the system calls provided by my OS. And to send a packet, I request a socket—which you can think of as just a special file descriptor my code can use to write in to communicate over the network

Here's what the client would do:
```c
int sockfd = socket(
    AF_INET,    // Domain: ARPA Internet protocols. This tells the OS that we're interested in the IP protocols
    SOCK_RAW,   // Type: Raw socket. The OS normally handles the transport layer, but this gives us full control.
    255         // Protocol: We looped over this field.
);
```
### Revisiting the failures
**1, 2, and 6: The Server Never Sees Them**  
These packets were successfully transmitted from the client, but they were intercepted before my server had a chance to look at them. That suggests something inside the OS intercepted them

At first, I naively expected my server to capture any raw IP packets it received. The initialization looked like this:  
```c
int sockfd = socket(
    AF_INET,    // Internet domain
    SOCK_RAW,   // Raw socket: should give us full control
    0           // Let the OS decide the protocol
);
```

I assumed that passing `0` as the protocol meant:  
*"Give me everything—TCP, UDP, whatever it is, forward it"*  

For context, I ran these experiments on my Mac, which runs Darwin. Looking at the [documentation](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/socket.2.html), there is really nothing mentioning the Protocol Number = 0 trick

Under the hood, Darwin is derives much of its networking stack from another OS, BSD, meaning it inherits BSD’s socket behaviour and network stack quirks. And on a whim I checked the **[BSD socket documentation](https://man.openbsd.org/socket.2)**, and I found this frustratingly vague line:  

> "A value of 0 for `protocol` will let the system select an appropriate protocol for the requested socket type."  

So instead of delivering **all** raw packets, my OS was silently (and haphazardly) filtering them. My server never even saw the ICMP (1), IGMP (2), or TCP (6) packets—because Darwin likely deemed my socket not appropriate to receive those protocols..

**50 and 51: The Client Can’t Even Send Them**  
Here, the OS flat-out refused to send the packets. These aren’t just arbitrary numbers—they’re part of **IPSec (ESP and AH)**, which is used for encrypted VPN traffic

**256: The `socket()` Call Fails Immediately**  
This one is simple:  
- The IPv4 protocol field is 8 bits meaning valid values range from 0 to 255.  
- 256 is simply too large—the OS rejects it outright as an invalid argument.

No surprises here. But what *was* surprising is what happened when I tried the same experiment on Linux..

After seeing these inconsistencies, I was curious as to how Linux would behave. So I spun up a Linux VM and re-ran the experiment. Right away, the behaviour was very different

Running the server I quickly noticed that Linux does not allow binding a raw socket to protocol `0`—Some invalid protocol numbers like 256 *worked*. For reference, I logged the results in [`results_no_server_linux_client_loopback`](./samples/results_no_server_linux_client_loopback.md).  
### Lessons learned
Custom transport-layer protocols are doable, but the OS isn’t exactly welcoming. The networking stack has assumptions baked in, and raw sockets aren’t as raw as you’d expect

I imagine this is why most new protocols live at the application layer instead. Instead of fighting the OS, engineers just build on top of existing transport protocols. QUIC, for example, runs over UDP and avoids these issues entirely

And if you're ever working with raw sockets, *please* test across multiple OSes. If Darwin lets you do something, Linux might shut it down. If Linux is fine with it, Windows might pretend it doesn’t exist. There’s no universal behaviour, even if they claim to implement the POSIX standard
### Next step: What happens outside loopback?
So far, I’ve only tested loopback traffic—packets never left my machine. What happens when I try sending HDP over the public internet?
- Will routers forward it, or will they drop it?
- Will firewalls let it through, or will they see it as an attack?
- Will its there be any latency differences when using a custom protocol as opposed to something like TCP?
# Experiment #2: 
Now let's do it with a digital ocean droplet
# Sharing
I'm thinking of creating a README / blog post where I report my findings
# Resources
- The [UDP protocol specification](https://datatracker.ietf.org/doc/html/rfc768) is so minimal it is almost funny
- [IP Protocol numbers that are assigned for testing](https://datatracker.ietf.org/doc/html/rfc3692#section-2.1)
- [The list of protocols](https://en.wikipedia.org/wiki/List_of_IP_protocol_numbers) supported under the IP protocol is pretty interesting
- [This](https://hackaday.com/2024/09/21/when-raw-network-sockets-arent-raw-raw-sockets-in-macos-and-linux/) article speaks about some differences between raw sockets in Linux & FreeBSD