tcpdump -vi lo
tcpdump: listening on lo, link-type EN10MB (Ethernet), snapshot length 262144 bytes
16:07:41.504650 IP (tos 0x0, ttl 64, id 6322, offset 0, flags [DF], proto ICMP (1), length 59)
    localhost > localhost: ICMP type-#1, length 39 (wrong icmp cksum 1a4 (->6313)!)
16:07:41.517143 IP (tos 0x0, ttl 64, id 11783, offset 0, flags [DF], proto IGMP (2), length 59)
    localhost > localhost: igmp-1 bad igmp cksum 1a4!
16:07:41.528165 IP (tos 0x0, ttl 64, id 17690, offset 0, flags [DF], proto unknown (3), length 59)
    localhost > localhost:  ggp 39
16:07:41.541665 IP (tos 0x0, ttl 64, id 48089, offset 0, flags [DF], proto IPIP (4), length 59)
    localhost > localhost: IP0 (invalid)
16:07:41.554536 IP (tos 0x0, ttl 64, id 40064, offset 0, flags [DF], proto unknown (5), length 59)
    localhost > localhost:  st 39
16:07:41.567363 IP (tos 0x0, ttl 64, id 52893, offset 0, flags [DF], proto TCP (6), length 59)
    localhost.420 > localhost.420: Flags [FS.UE], cksum 0x7364 (incorrect -> 0xd4fd), seq 405040490:405040505, ack 3461544128, win 25697, urg 24947, options [[bad opt]
16:07:41.578176 IP (tos 0x0, ttl 64, id 12426, offset 0, flags [DF], proto unknown (7), length 59)
    localhost > localhost:  cbt 39
16:07:41.591639 IP (tos 0x0, ttl 64, id 18649, offset 0, flags [DF], proto EGP (8), length 59)
    localhost > localhost: EGPv1, length 39[version 1]
16:07:41.604522 IP (tos 0x0, ttl 64, id 46894, offset 0, flags [DF], proto IGRP (9), length 59)
    localhost > localhost: igrp: update V0 edit=164 AS=420 (6180/28010/53385) checksum=0xfcd6 *.97.115.100 d=63865320 b=1 r=103 l=102 M=12773064 mtu=24947 in 113 hops (invalid)
16:07:41.617545 IP (tos 0x0, ttl 64, id 41170, offset 0, flags [DF], proto unknown (10), length 59)
    localhost > localhost:  bbn-rcc-mon 39
16:07:41.630543 IP (tos 0x0, ttl 64, id 31722, offset 0, flags [DF], proto unknown (11), length 59)
    localhost > localhost:  nvp-ii 39
16:07:41.643019 IP (tos 0x0, ttl 64, id 57451, offset 0, flags [DF], proto unknown (12), length 59)
    localhost > localhost:  pup 39
16:07:41.656673 IP (tos 0x0, ttl 64, id 53360, offset 0, flags [DF], proto unknown (13), length 59)
    localhost > localhost:  argus 39
16:07:41.669503 IP (tos 0x0, ttl 64, id 59189, offset 0, flags [DF], proto unknown (14), length 59)
    localhost > localhost:  emcon 39
16:07:41.682118 IP (tos 0x0, ttl 64, id 489, offset 0, flags [DF], proto unknown (15), length 59)
    localhost > localhost:  xnet 39
16:07:41.695650 IP (tos 0x0, ttl 64, id 55531, offset 0, flags [DF], proto unknown (16), length 59)
    localhost > localhost:  chaos 39
16:07:41.708516 IP (tos 0x0, ttl 64, id 33900, offset 0, flags [DF], proto UDP (17), length 59)
    localhost.420 > localhost.420: UDP, bad length 6172 > 31
16:07:41.721553 IP (tos 0x0, ttl 64, id 46520, offset 0, flags [DF], proto unknown (18), length 59)
    localhost > localhost:  mux 39
16:07:41.734537 IP (tos 0x0, ttl 64, id 29246, offset 0, flags [DF], proto unknown (19), length 59)
    localhost > localhost:  dcn-meas 39
16:07:41.747544 IP (tos 0x0, ttl 64, id 43432, offset 0, flags [DF], proto unknown (20), length 59)
    localhost > localhost:  hmp 39
16:07:41.760528 IP (tos 0x0, ttl 64, id 11521, offset 0, flags [DF], proto unknown (21), length 59)
    localhost > localhost:  prm 39
16:07:41.773554 IP (tos 0x0, ttl 64, id 41179, offset 0, flags [DF], proto unknown (22), length 59)
    localhost > localhost:  xns-idp 39
16:07:41.786118 IP (tos 0x0, ttl 64, id 10124, offset 0, flags [DF], proto unknown (23), length 59)
    localhost > localhost:  trunk-1 39
16:07:41.799654 IP (tos 0x0, ttl 64, id 25954, offset 0, flags [DF], proto unknown (24), length 59)
    localhost > localhost:  trunk-2 39
16:07:41.812493 IP (tos 0x0, ttl 64, id 43967, offset 0, flags [DF], proto unknown (25), length 59)
    localhost > localhost:  leaf-1 39
16:07:41.825563 IP (tos 0x0, ttl 64, id 21185, offset 0, flags [DF], proto unknown (26), length 59)
    localhost > localhost:  leaf-2 39
16:07:41.838535 IP (tos 0x0, ttl 64, id 530, offset 0, flags [DF], proto unknown (27), length 59)
    localhost > localhost:  rdp 39
16:07:41.849659 IP (tos 0x0, ttl 64, id 64925, offset 0, flags [DF], proto unknown (28), length 59)
    localhost > localhost:  irtp 39
16:07:41.862503 IP (tos 0x0, ttl 64, id 12639, offset 0, flags [DF], proto unknown (29), length 59)
    localhost > localhost:  iso-tp4 39
16:07:41.875559 IP (tos 0x0, ttl 64, id 21600, offset 0, flags [DF], proto unknown (30), length 59)
    localhost > localhost:  netblt 39
16:07:41.887627 IP (tos 0x0, ttl 64, id 49447, offset 0, flags [DF], proto unknown (31), length 59)
    localhost > localhost:  mfe-nsp 39
16:07:41.900527 IP (tos 0x0, ttl 64, id 28477, offset 0, flags [DF], proto unknown (32), length 59)
    localhost > localhost:  merit-inp 39
16:07:41.913541 IP (tos 0x0, ttl 64, id 60088, offset 0, flags [DF], proto DCCP (33), length 59)
    localhost.420 > localhost.420: DCCP (CCVal 2, CsCov 4, cksum 0x6d6a (incorrect -> 0x7123)) DCCP-Response (service=1684108135) (ack=7562337) 
16:07:41.926539 IP (tos 0x0, ttl 64, id 10095, offset 0, flags [DF], proto unknown (34), length 59)
    localhost > localhost:  3pc 39
16:07:41.939536 IP (tos 0x0, ttl 64, id 30295, offset 0, flags [DF], proto unknown (35), length 59)
    localhost > localhost:  idpr 39
16:07:41.952545 IP (tos 0x0, ttl 64, id 27930, offset 0, flags [DF], proto unknown (36), length 59)
    localhost > localhost:  xtp 39
16:07:41.965529 IP (tos 0x0, ttl 64, id 32922, offset 0, flags [DF], proto unknown (37), length 59)
    localhost > localhost:  ddp 39
16:07:41.978552 IP (tos 0x0, ttl 64, id 28086, offset 0, flags [DF], proto unknown (38), length 59)
    localhost > localhost:  idpr-cmtp 39
16:07:41.989170 IP (tos 0x0, ttl 64, id 44088, offset 0, flags [DF], proto unknown (39), length 59)
    localhost > localhost:  tp++ 39
16:07:42.002622 IP (tos 0x0, ttl 64, id 31981, offset 0, flags [DF], proto unknown (40), length 59)
    localhost > localhost:  il 39
16:07:42.015508 IP (tos 0x0, ttl 64, id 58162, offset 0, flags [DF], proto IPv6 (41), length 59)
    localhost > localhost:  [|ip6]
16:07:42.028544 IP (tos 0x0, ttl 64, id 7758, offset 0, flags [DF], proto unknown (42), length 59)
    localhost > localhost:  sdrp 39
16:07:42.041531 IP (tos 0x0, ttl 64, id 55837, offset 0, flags [DF], proto Routing (43), length 59)
    localhost > localhost:  ipv6-route 39
16:07:42.054531 IP (tos 0x0, ttl 64, id 29145, offset 0, flags [DF], proto Fragment (44), length 59)
    localhost > localhost:  ipv6-frag 39
16:07:42.067536 IP (tos 0x0, ttl 64, id 60462, offset 0, flags [DF], proto unknown (45), length 59)
    localhost > localhost:  idrp 39
16:07:42.080538 IP (tos 0x0, ttl 64, id 63434, offset 0, flags [DF], proto RSVP (46), length 59)
    localhost > localhost: ERROR: RSVP version 0 packet not supported
16:07:42.091778 IP (tos 0x0, ttl 64, id 43956, offset 0, flags [DF], proto GRE (47), length 59)
    localhost > localhost: GREv4 ERROR: unknown-version
16:07:42.103527 IP (tos 0x0, ttl 64, id 19249, offset 0, flags [DF], proto unknown (48), length 59)
    localhost > localhost:  dsr 39
16:07:42.115690 IP (tos 0x0, ttl 64, id 7408, offset 0, flags [DF], proto unknown (49), length 59)
    localhost > localhost:  bna 39
16:07:42.128500 IP (tos 0x0, ttl 64, id 59990, offset 0, flags [DF], proto ESP (50), length 59)
    localhost > localhost: ESP(spi=0x01a401a4,seq=0x18246d6a), length 39
16:07:42.141547 IP (tos 0x0, ttl 64, id 63980, offset 0, flags [DF], proto AH (51), length 59)
    localhost > localhost: AH(length=164(664-bytes),reserved=0x1a4[MustBeZero],spi=0x18246d6a,seq=0xf08c5983,icv=0x61736461736461736461736766716561676561677461657465610a [|ah]
16:07:42.154541 IP (tos 0x0, ttl 64, id 20094, offset 0, flags [DF], proto unknown (52), length 59)
    localhost > localhost:  i-nlsp 39
16:07:42.167532 IP (tos 0x0, ttl 64, id 31259, offset 0, flags [DF], proto unknown (53), length 59)
    localhost > localhost:  swipe 39
16:07:42.180532 IP (tos 0x0, ttl 64, id 19054, offset 0, flags [DF], proto unknown (54), length 59)
    localhost > localhost:  narp 39
16:07:42.191236 IP (tos 0x0, ttl 64, id 57898, offset 0, flags [DF], proto Mobile IP (55), length 59)
    localhost > localhost: mobile: [S] 243.130.139.1 > d24-36-109-106.home1.cgocable.net (oproto=1) (bad checksum 420)
16:07:42.203609 IP (tos 0x0, ttl 64, id 27225, offset 0, flags [DF], proto unknown (56), length 59)
    localhost > localhost:  tlsp 39
16:07:42.215538 IP (tos 0x0, ttl 64, id 22113, offset 0, flags [DF], proto unknown (57), length 59)
    localhost > localhost:  skip 39
16:07:42.226173 IP (tos 0x0, ttl 64, id 30259, offset 0, flags [DF], proto ICMPv6 (58), length 59)
    localhost > localhost: [ICMPv6 requires IPv6] (invalid)
16:07:42.239630 IP (tos 0x0, ttl 64, id 30888, offset 0, flags [DF], proto unknown (59), length 59)
    localhost > localhost: no next header
16:07:42.252511 IP (tos 0x0, ttl 64, id 20117, offset 0, flags [DF], proto unknown (60), length 59)
    localhost > localhost:  ipv6-opts 39
16:07:42.263171 IP (tos 0x0, ttl 64, id 62300, offset 0, flags [DF], proto unknown (61), length 59)
    localhost > localhost:  ip-proto-61 39
16:07:42.276621 IP (tos 0x0, ttl 64, id 42926, offset 0, flags [DF], proto Mobile IP (old) (62), length 59)
    localhost > localhost:  cftp 39
16:07:42.289503 IP (tos 0x0, ttl 64, id 34075, offset 0, flags [DF], proto unknown (63), length 59)
    localhost > localhost:  ip-proto-63 39
16:07:42.300169 IP (tos 0x0, ttl 64, id 14125, offset 0, flags [DF], proto unknown (64), length 59)
    localhost > localhost:  sat-expak 39
16:07:42.313661 IP (tos 0x0, ttl 64, id 16825, offset 0, flags [DF], proto unknown (65), length 59)
    localhost > localhost:  kryptolan 39
16:07:42.326542 IP (tos 0x0, ttl 64, id 64517, offset 0, flags [DF], proto unknown (66), length 59)
    localhost > localhost:  rvd 39
16:07:42.339568 IP (tos 0x0, ttl 64, id 57697, offset 0, flags [DF], proto unknown (67), length 59)
    localhost > localhost:  ippc 39
16:07:42.351695 IP (tos 0x0, ttl 64, id 50671, offset 0, flags [DF], proto unknown (68), length 59)
    localhost > localhost:  ip-proto-68 39
16:07:42.364158 IP (tos 0x0, ttl 64, id 7380, offset 0, flags [DF], proto unknown (69), length 59)
    localhost > localhost:  sat-mon 39
16:07:42.377668 IP (tos 0x0, ttl 64, id 47120, offset 0, flags [DF], proto unknown (70), length 59)
    localhost > localhost:  visa 39
16:07:42.390537 IP (tos 0x0, ttl 64, id 48903, offset 0, flags [DF], proto unknown (71), length 59)
    localhost > localhost:  ipcv 39
16:07:42.401201 IP (tos 0x0, ttl 64, id 21456, offset 0, flags [DF], proto unknown (72), length 59)
    localhost > localhost:  cpnx 39
16:07:42.412397 IP (tos 0x0, ttl 64, id 55203, offset 0, flags [DF], proto unknown (73), length 59)
    localhost > localhost:  rspf 39
16:07:42.425609 IP (tos 0x0, ttl 64, id 2502, offset 0, flags [DF], proto unknown (74), length 59)
    localhost > localhost:  wsn 39
16:07:42.438551 IP (tos 0x0, ttl 64, id 35021, offset 0, flags [DF], proto unknown (75), length 59)
    localhost > localhost:  pvp 39
16:07:42.449172 IP (tos 0x0, ttl 64, id 18673, offset 0, flags [DF], proto unknown (76), length 59)
    localhost > localhost:  br-sat-mon 39
16:07:42.462664 IP (tos 0x0, ttl 64, id 2697, offset 0, flags [DF], proto unknown (77), length 59)
    localhost > localhost:  nd 39
16:07:42.475540 IP (tos 0x0, ttl 64, id 37724, offset 0, flags [DF], proto unknown (78), length 59)
    localhost > localhost:  wb-mon 39
16:07:42.486170 IP (tos 0x0, ttl 64, id 6264, offset 0, flags [DF], proto unknown (79), length 59)
    localhost > localhost:  wb-expak 39
16:07:42.499661 IP (tos 0x0, ttl 64, id 52398, offset 0, flags [DF], proto unknown (80), length 59)
    localhost > localhost:  iso-ip 39
16:07:42.511536 IP (tos 0x0, ttl 64, id 26578, offset 0, flags [DF], proto unknown (81), length 59)
    localhost > localhost:  vmtp 39
16:07:42.524573 IP (tos 0x0, ttl 64, id 49911, offset 0, flags [DF], proto unknown (82), length 59)
    localhost > localhost:  secure-vmtp 39
16:07:42.537564 IP (tos 0x0, ttl 64, id 29565, offset 0, flags [DF], proto unknown (83), length 59)
    localhost > localhost:  vines 39
16:07:42.550561 IP (tos 0x0, ttl 64, id 16521, offset 0, flags [DF], proto unknown (84), length 59)
    localhost > localhost:  ttp 39
16:07:42.561647 IP (tos 0x0, ttl 64, id 46427, offset 0, flags [DF], proto unknown (85), length 59)
    localhost > localhost:  nsfnet-igp 39
16:07:42.572193 IP (tos 0x0, ttl 64, id 32089, offset 0, flags [DF], proto unknown (86), length 59)
    localhost > localhost:  dgp 39
16:07:42.585645 IP (tos 0x0, ttl 64, id 10007, offset 0, flags [DF], proto unknown (87), length 59)
    localhost > localhost:  tcf 39
16:07:42.598531 IP (tos 0x0, ttl 64, id 47003, offset 0, flags [DF], proto EIGRP (88), length 59)
    localhost > localhost: EIGRP version 1 packet not supported
16:07:42.611557 IP (tos 0x0, ttl 64, id 3241, offset 0, flags [DF], proto OSPF (89), length 59)
    localhost > localhost: OSPFv1, unknown LS-type 164, length 39
16:07:42.624544 IP (tos 0x0, ttl 64, id 60596, offset 0, flags [DF], proto unknown (90), length 59)
    localhost > localhost:  sprite-rpc 39
16:07:42.637550 IP (tos 0x0, ttl 64, id 28859, offset 0, flags [DF], proto unknown (91), length 59)
    localhost > localhost:  larp 39
16:07:42.650553 IP (tos 0x0, ttl 64, id 16356, offset 0, flags [DF], proto unknown (92), length 59)
    localhost > localhost:  mtp 39
16:07:42.661171 IP (tos 0x0, ttl 64, id 21637, offset 0, flags [DF], proto unknown (93), length 59)
    localhost > localhost:  ax.25 39
16:07:42.674666 IP (tos 0x0, ttl 64, id 48117, offset 0, flags [DF], proto unknown (94), length 59)
    localhost > localhost:  ipip 39
16:07:42.687512 IP (tos 0x0, ttl 64, id 18090, offset 0, flags [DF], proto unknown (95), length 59)
    localhost > localhost:  micp 39
16:07:42.698169 IP (tos 0x0, ttl 64, id 7470, offset 0, flags [DF], proto unknown (96), length 59)
    localhost > localhost:  scc-sp 39
16:07:42.711649 IP (tos 0x0, ttl 64, id 7430, offset 0, flags [DF], proto unknown (97), length 59)
    localhost > localhost:  etherip 39
16:07:42.724515 IP (tos 0x0, ttl 64, id 51449, offset 0, flags [DF], proto unknown (98), length 59)
    localhost > localhost:  encap 39
16:07:42.737573 IP (tos 0x0, ttl 64, id 32940, offset 0, flags [DF], proto unknown (99), length 59)
    localhost > localhost:  ip-proto-99 39
16:07:42.750542 IP (tos 0x0, ttl 64, id 25889, offset 0, flags [DF], proto unknown (100), length 59)
    localhost > localhost:  gmtp 39
16:07:42.763555 IP (tos 0x0, ttl 64, id 16047, offset 0, flags [DF], proto unknown (101), length 59)
    localhost > localhost:  ifmp 39
16:07:42.775778 IP (tos 0x0, ttl 64, id 5159, offset 0, flags [DF], proto unknown (102), length 59)
    localhost > localhost:  pnni 39
16:07:42.788482 IP (tos 0x0, ttl 64, id 45948, offset 0, flags [DF], proto PIM (103), length 59)
    localhost > localhost: PIMv0, length 39
16:07:42.801570 IP (tos 0x0, ttl 64, id 38615, offset 0, flags [DF], proto unknown (104), length 59)
    localhost > localhost:  aris 39
16:07:42.812793 IP (tos 0x0, ttl 64, id 4336, offset 0, flags [DF], proto unknown (105), length 59)
    localhost > localhost:  scps 39
16:07:42.825504 IP (tos 0x0, ttl 64, id 965, offset 0, flags [DF], proto unknown (106), length 59)
    localhost > localhost:  qnx 39
16:07:42.838561 IP (tos 0x0, ttl 64, id 45647, offset 0, flags [DF], proto unknown (107), length 59)
    localhost > localhost:  a/n 39
16:07:42.849173 IP (tos 0x0, ttl 64, id 24621, offset 0, flags [DF], proto Compressed IP (108), length 59)
    localhost > localhost: IPComp(cpi=0x01a4)
16:07:42.862647 IP (tos 0x0, ttl 64, id 39811, offset 0, flags [DF], proto unknown (109), length 59)
    localhost > localhost:  snp 39
16:07:42.875521 IP (tos 0x0, ttl 64, id 3208, offset 0, flags [DF], proto unknown (110), length 59)
    localhost > localhost:  compaq-peer 39
16:07:42.886173 IP (tos 0x0, ttl 64, id 29306, offset 0, flags [DF], proto unknown (111), length 59)
    localhost > localhost:  ipx-in-ip 39
16:07:42.899661 IP (tos 0x0, ttl 64, id 3171, offset 0, flags [DF], proto VRRP (112), length 59)
    localhost > localhost: VRRPv0, Advertisement, (ttl 64)
16:07:42.912532 IP (tos 0x0, ttl 64, id 23490, offset 0, flags [DF], proto PGM (113), length 59)
    localhost > localhost: localhost.420 > localhost.420: PGM, length 25697 0x1e80a67c6173 UNKNOWN type 0x18 [39]
16:07:42.925561 IP (tos 0x0, ttl 64, id 34386, offset 0, flags [DF], proto unknown (114), length 59)
    localhost > localhost:  ip-proto-114 39
16:07:42.938550 IP (tos 0x0, ttl 64, id 53190, offset 0, flags [DF], proto unknown (115), length 59)
    localhost > localhost:  l2tp 39
16:07:42.949173 IP (tos 0x0, ttl 64, id 25757, offset 0, flags [DF], proto unknown (116), length 59)
    localhost > localhost:  ddx 39
16:07:42.962660 IP (tos 0x0, ttl 64, id 2893, offset 0, flags [DF], proto unknown (117), length 59)
    localhost > localhost:  iatp 39
16:07:42.975519 IP (tos 0x0, ttl 64, id 20519, offset 0, flags [DF], proto unknown (118), length 59)
    localhost > localhost:  stp 39
16:07:42.988560 IP (tos 0x0, ttl 64, id 29143, offset 0, flags [DF], proto unknown (119), length 59)
    localhost > localhost:  srp 39
16:07:43.001555 IP (tos 0x0, ttl 64, id 14864, offset 0, flags [DF], proto unknown (120), length 59)
    localhost > localhost:  uti 39
16:07:43.013174 IP (tos 0x0, ttl 64, id 33794, offset 0, flags [DF], proto unknown (121), length 59)
    localhost > localhost:  smp 39
16:07:43.138168 IP (tos 0x0, ttl 64, id 15294, offset 0, flags [DF], proto unknown (131), length 59)
    localhost > localhost:  pipe 39
16:07:43.151636 IP (tos 0x0, ttl 64, id 50635, offset 0, flags [DF], proto SCTP (132), length 59)
    localhost.420 > localhost.420: sctp [|sctp]
16:07:43.164521 IP (tos 0x0, ttl 64, id 49015, offset 0, flags [DF], proto unknown (133), length 59)
    localhost > localhost:  fc 39
16:07:43.177551 IP (tos 0x0, ttl 64, id 42212, offset 0, flags [DF], proto unknown (134), length 59)
    localhost > localhost:  rsvp-e2e-ignore 39
16:07:43.190530 IP (tos 0x0, ttl 64, id 13548, offset 0, flags [DF], proto Mobility (135), length 59)
    localhost > localhost:  mobility-header 39
16:07:43.202711 IP (tos 0x0, ttl 64, id 44832, offset 0, flags [DF], proto unknown (136), length 59)
    localhost > localhost:  udplite 39
16:07:43.214142 IP (tos 0x0, ttl 64, id 51362, offset 0, flags [DF], proto unknown (137), length 59)
    localhost > localhost:  mpls-in-ip 39
16:07:43.214149 IP (tos 0xc0, ttl 64, id 6858, offset 0, flags [none], proto ICMP (1), length 87)
    localhost > localhost: ICMP localhost protocol 137 port 420 unreachable, length 67
	IP (tos 0x0, ttl 64, id 51362, offset 0, flags [DF], proto unknown (137), length 59)
    localhost > localhost:  mpls-in-ip 39
16:07:43.227638 IP (tos 0x0, ttl 64, id 27297, offset 0, flags [DF], proto unknown (138), length 59)
    localhost > localhost:  manet 39
16:07:43.239798 IP (tos 0x0, ttl 64, id 55150, offset 0, flags [DF], proto unknown (139), length 59)
    localhost > localhost:  hip 39
16:07:43.250178 IP (tos 0x0, ttl 64, id 30773, offset 0, flags [DF], proto unknown (140), length 59)
    localhost > localhost:  shim6 39
16:07:43.263633 IP (tos 0x0, ttl 64, id 24812, offset 0, flags [DF], proto unknown (141), length 59)
    localhost > localhost:  wesp 39
16:07:43.276518 IP (tos 0x0, ttl 64, id 26414, offset 0, flags [DF], proto unknown (142), length 59)
    localhost > localhost:  rohc 39
16:07:43.289541 IP (tos 0x0, ttl 64, id 38776, offset 0, flags [DF], proto Ethernet (143), length 59)
    localhost > localhost: [Ethernet requires IPv6] (invalid)
16:07:43.302532 IP (tos 0x0, ttl 64, id 11324, offset 0, flags [DF], proto unknown (144), length 59)
    localhost > localhost:  ip-proto-144 39
16:07:43.315541 IP (tos 0x0, ttl 64, id 10276, offset 0, flags [DF], proto unknown (145), length 59)
    localhost > localhost:  ip-proto-145 39
16:07:43.328539 IP (tos 0x0, ttl 64, id 17945, offset 0, flags [DF], proto unknown (146), length 59)
    localhost > localhost:  ip-proto-146 39
16:07:43.341542 IP (tos 0x0, ttl 64, id 56337, offset 0, flags [DF], proto unknown (147), length 59)
    localhost > localhost:  ip-proto-147 39
16:07:43.352175 IP (tos 0x0, ttl 64, id 40896, offset 0, flags [DF], proto unknown (148), length 59)
    localhost > localhost:  ip-proto-148 39
16:07:43.365632 IP (tos 0x0, ttl 64, id 57533, offset 0, flags [DF], proto unknown (149), length 59)
    localhost > localhost:  ip-proto-149 39
16:07:43.378515 IP (tos 0x0, ttl 64, id 34793, offset 0, flags [DF], proto unknown (150), length 59)
    localhost > localhost:  ip-proto-150 39
16:07:43.391541 IP (tos 0x0, ttl 64, id 18845, offset 0, flags [DF], proto unknown (151), length 59)
    localhost > localhost:  ip-proto-151 39
16:07:43.404538 IP (tos 0x0, ttl 64, id 47304, offset 0, flags [DF], proto unknown (152), length 59)
    localhost > localhost:  ip-proto-152 39
16:07:43.415175 IP (tos 0x0, ttl 64, id 39805, offset 0, flags [DF], proto unknown (153), length 59)
    localhost > localhost:  ip-proto-153 39
16:07:43.428622 IP (tos 0x0, ttl 64, id 48711, offset 0, flags [DF], proto unknown (154), length 59)
    localhost > localhost:  ip-proto-154 39
16:07:43.441505 IP (tos 0x0, ttl 64, id 22248, offset 0, flags [DF], proto unknown (155), length 59)
    localhost > localhost:  ip-proto-155 39
16:07:43.453120 IP (tos 0x0, ttl 64, id 5043, offset 0, flags [DF], proto unknown (156), length 59)
    localhost > localhost:  ip-proto-156 39
16:07:43.466644 IP (tos 0x0, ttl 64, id 6365, offset 0, flags [DF], proto unknown (157), length 59)
    localhost > localhost:  ip-proto-157 39
16:07:43.479519 IP (tos 0x0, ttl 64, id 37279, offset 0, flags [DF], proto unknown (158), length 59)
    localhost > localhost:  ip-proto-158 39
16:07:43.492527 IP (tos 0x0, ttl 64, id 59423, offset 0, flags [DF], proto unknown (159), length 59)
    localhost > localhost:  ip-proto-159 39
16:07:43.505538 IP (tos 0x0, ttl 64, id 10899, offset 0, flags [DF], proto unknown (160), length 59)
    localhost > localhost:  ip-proto-160 39
16:07:43.518537 IP (tos 0x0, ttl 64, id 39583, offset 0, flags [DF], proto unknown (161), length 59)
    localhost > localhost:  ip-proto-161 39
16:07:43.529171 IP (tos 0x0, ttl 64, id 57947, offset 0, flags [DF], proto unknown (162), length 59)
    localhost > localhost:  ip-proto-162 39
16:07:43.542620 IP (tos 0x0, ttl 64, id 9828, offset 0, flags [DF], proto unknown (163), length 59)
    localhost > localhost:  ip-proto-163 39
16:07:43.553413 IP (tos 0x0, ttl 64, id 38394, offset 0, flags [DF], proto unknown (164), length 59)
    localhost > localhost:  ip-proto-164 39
16:07:43.566574 IP (tos 0x0, ttl 64, id 40587, offset 0, flags [DF], proto unknown (165), length 59)
    localhost > localhost:  ip-proto-165 39
16:07:43.579528 IP (tos 0x0, ttl 64, id 57579, offset 0, flags [DF], proto unknown (166), length 59)
    localhost > localhost:  ip-proto-166 39
16:07:43.592546 IP (tos 0x0, ttl 64, id 38785, offset 0, flags [DF], proto unknown (167), length 59)
    localhost > localhost:  ip-proto-167 39
16:07:43.605530 IP (tos 0x0, ttl 64, id 22188, offset 0, flags [DF], proto unknown (168), length 59)
    localhost > localhost:  ip-proto-168 39
16:07:43.616185 IP (tos 0x0, ttl 64, id 678, offset 0, flags [DF], proto unknown (169), length 59)
    localhost > localhost:  ip-proto-169 39
16:07:43.629619 IP (tos 0x0, ttl 64, id 17546, offset 0, flags [DF], proto unknown (170), length 59)
    localhost > localhost:  ip-proto-170 39
16:07:43.642516 IP (tos 0x0, ttl 64, id 57763, offset 0, flags [DF], proto unknown (171), length 59)
    localhost > localhost:  ip-proto-171 39
16:07:43.655194 IP (tos 0x0, ttl 64, id 23838, offset 0, flags [DF], proto unknown (172), length 59)
    localhost > localhost:  ip-proto-172 39
16:07:43.668438 IP (tos 0x0, ttl 64, id 44100, offset 0, flags [DF], proto unknown (173), length 59)
    localhost > localhost:  ip-proto-173 39
16:07:43.681449 IP (tos 0x0, ttl 64, id 32660, offset 0, flags [DF], proto unknown (174), length 59)
    localhost > localhost:  ip-proto-174 39
16:07:43.694448 IP (tos 0x0, ttl 64, id 36670, offset 0, flags [DF], proto unknown (175), length 59)
    localhost > localhost:  ip-proto-175 39
16:07:43.705172 IP (tos 0x0, ttl 64, id 43691, offset 0, flags [DF], proto unknown (176), length 59)
    localhost > localhost:  ip-proto-176 39
16:07:43.718650 IP (tos 0x0, ttl 64, id 30929, offset 0, flags [DF], proto unknown (177), length 59)
    localhost > localhost:  ip-proto-177 39
16:07:43.731527 IP (tos 0x0, ttl 64, id 36440, offset 0, flags [DF], proto unknown (178), length 59)
    localhost > localhost:  ip-proto-178 39
16:07:43.743503 IP (tos 0x0, ttl 64, id 44796, offset 0, flags [DF], proto unknown (179), length 59)
    localhost > localhost:  ip-proto-179 39
16:07:43.756562 IP (tos 0x0, ttl 64, id 24026, offset 0, flags [DF], proto unknown (180), length 59)
    localhost > localhost:  ip-proto-180 39
16:07:43.768891 IP (tos 0x0, ttl 64, id 24298, offset 0, flags [DF], proto unknown (181), length 59)
    localhost > localhost:  ip-proto-181 39
16:07:43.782716 IP (tos 0x0, ttl 64, id 55796, offset 0, flags [DF], proto unknown (182), length 59)
    localhost > localhost:  ip-proto-182 39
16:07:43.795510 IP (tos 0x0, ttl 64, id 44225, offset 0, flags [DF], proto unknown (183), length 59)
    localhost > localhost:  ip-proto-183 39
16:07:43.808577 IP (tos 0x0, ttl 64, id 5227, offset 0, flags [DF], proto unknown (184), length 59)
    localhost > localhost:  ip-proto-184 39
16:07:43.821547 IP (tos 0x0, ttl 64, id 48092, offset 0, flags [DF], proto unknown (185), length 59)
    localhost > localhost:  ip-proto-185 39
16:07:43.834548 IP (tos 0x0, ttl 64, id 15577, offset 0, flags [DF], proto unknown (186), length 59)
    localhost > localhost:  ip-proto-186 39
16:07:43.847554 IP (tos 0x0, ttl 64, id 53968, offset 0, flags [DF], proto unknown (187), length 59)
    localhost > localhost:  ip-proto-187 39
16:07:43.860568 IP (tos 0x0, ttl 64, id 30539, offset 0, flags [DF], proto unknown (188), length 59)
    localhost > localhost:  ip-proto-188 39
16:07:43.872820 IP (tos 0x0, ttl 64, id 10157, offset 0, flags [DF], proto unknown (189), length 59)
    localhost > localhost:  ip-proto-189 39
16:07:43.885487 IP (tos 0x0, ttl 64, id 26635, offset 0, flags [DF], proto unknown (190), length 59)
    localhost > localhost:  ip-proto-190 39
16:07:43.898568 IP (tos 0x0, ttl 64, id 26635, offset 0, flags [DF], proto unknown (191), length 59)
    localhost > localhost:  ip-proto-191 39
16:07:43.911550 IP (tos 0x0, ttl 64, id 37895, offset 0, flags [DF], proto unknown (192), length 59)
    localhost > localhost:  ip-proto-192 39
16:07:43.924550 IP (tos 0x0, ttl 64, id 1281, offset 0, flags [DF], proto unknown (193), length 59)
    localhost > localhost:  ip-proto-193 39
16:07:43.937548 IP (tos 0x0, ttl 64, id 11749, offset 0, flags [DF], proto unknown (194), length 59)
    localhost > localhost:  ip-proto-194 39
16:07:43.950549 IP (tos 0x0, ttl 64, id 19044, offset 0, flags [DF], proto unknown (195), length 59)
    localhost > localhost:  ip-proto-195 39
16:07:43.961864 IP (tos 0x0, ttl 64, id 36155, offset 0, flags [DF], proto unknown (196), length 59)
    localhost > localhost:  ip-proto-196 39
16:07:43.972942 IP (tos 0x0, ttl 64, id 8638, offset 0, flags [DF], proto unknown (197), length 59)
    localhost > localhost:  ip-proto-197 39
16:07:43.983174 IP (tos 0x0, ttl 64, id 53844, offset 0, flags [DF], proto unknown (198), length 59)
    localhost > localhost:  ip-proto-198 39
16:07:43.996637 IP (tos 0x0, ttl 64, id 9275, offset 0, flags [DF], proto unknown (199), length 59)
    localhost > localhost:  ip-proto-199 39
16:07:44.009514 IP (tos 0x0, ttl 64, id 57072, offset 0, flags [DF], proto unknown (200), length 59)
    localhost > localhost:  ip-proto-200 39
16:07:44.022554 IP (tos 0x0, ttl 64, id 52151, offset 0, flags [DF], proto unknown (201), length 59)
    localhost > localhost:  ip-proto-201 39
16:07:44.035535 IP (tos 0x0, ttl 64, id 28057, offset 0, flags [DF], proto unknown (202), length 59)
    localhost > localhost:  ip-proto-202 39
16:07:44.048547 IP (tos 0x0, ttl 64, id 12928, offset 0, flags [DF], proto unknown (203), length 59)
    localhost > localhost:  ip-proto-203 39
16:07:44.061540 IP (tos 0x0, ttl 64, id 15476, offset 0, flags [DF], proto unknown (204), length 59)
    localhost > localhost:  ip-proto-204 39
16:07:44.072163 IP (tos 0x0, ttl 64, id 56889, offset 0, flags [DF], proto unknown (205), length 59)
    localhost > localhost:  ip-proto-205 39
16:07:44.085632 IP (tos 0x0, ttl 64, id 54734, offset 0, flags [DF], proto unknown (206), length 59)
    localhost > localhost:  ip-proto-206 39
16:07:44.098510 IP (tos 0x0, ttl 64, id 25176, offset 0, flags [DF], proto unknown (207), length 59)
    localhost > localhost:  ip-proto-207 39
16:07:44.111545 IP (tos 0x0, ttl 64, id 28695, offset 0, flags [DF], proto unknown (208), length 59)
    localhost > localhost:  ip-proto-208 39
16:07:44.124539 IP (tos 0x0, ttl 64, id 51448, offset 0, flags [DF], proto unknown (209), length 59)
    localhost > localhost:  ip-proto-209 39
16:07:44.137550 IP (tos 0x0, ttl 64, id 42751, offset 0, flags [DF], proto unknown (210), length 59)
    localhost > localhost:  ip-proto-210 39
16:07:44.150531 IP (tos 0x0, ttl 64, id 18496, offset 0, flags [DF], proto unknown (211), length 59)
    localhost > localhost:  ip-proto-211 39
16:07:44.161173 IP (tos 0x0, ttl 64, id 476, offset 0, flags [DF], proto unknown (212), length 59)
    localhost > localhost:  ip-proto-212 39
16:07:44.174642 IP (tos 0x0, ttl 64, id 27915, offset 0, flags [DF], proto unknown (213), length 59)
    localhost > localhost:  ip-proto-213 39
16:07:44.187421 IP (tos 0x0, ttl 64, id 18694, offset 0, flags [DF], proto unknown (214), length 59)
    localhost > localhost:  ip-proto-214 39
16:07:44.200563 IP (tos 0x0, ttl 64, id 40896, offset 0, flags [DF], proto unknown (215), length 59)
    localhost > localhost:  ip-proto-215 39
16:07:44.213544 IP (tos 0x0, ttl 64, id 51215, offset 0, flags [DF], proto unknown (216), length 59)
    localhost > localhost:  ip-proto-216 39
16:07:44.226542 IP (tos 0x0, ttl 64, id 34337, offset 0, flags [DF], proto unknown (217), length 59)
    localhost > localhost:  ip-proto-217 39
16:07:44.237173 IP (tos 0x0, ttl 64, id 36759, offset 0, flags [DF], proto unknown (218), length 59)
    localhost > localhost:  ip-proto-218 39
16:07:44.250645 IP (tos 0x0, ttl 64, id 51460, offset 0, flags [DF], proto unknown (219), length 59)
    localhost > localhost:  ip-proto-219 39
16:07:44.263519 IP (tos 0x0, ttl 64, id 51502, offset 0, flags [DF], proto unknown (220), length 59)
    localhost > localhost:  ip-proto-220 39
16:07:44.274173 IP (tos 0x0, ttl 64, id 21160, offset 0, flags [DF], proto unknown (221), length 59)
    localhost > localhost:  ip-proto-221 39
16:07:44.286176 IP (tos 0x0, ttl 64, id 64970, offset 0, flags [DF], proto unknown (222), length 59)
    localhost > localhost:  ip-proto-222 39
16:07:44.297179 IP (tos 0x0, ttl 64, id 29637, offset 0, flags [DF], proto unknown (223), length 59)
    localhost > localhost:  ip-proto-223 39
16:07:44.308499 IP (tos 0x0, ttl 64, id 43906, offset 0, flags [DF], proto unknown (224), length 59)
    localhost > localhost:  ip-proto-224 39
16:07:44.321543 IP (tos 0x0, ttl 64, id 23398, offset 0, flags [DF], proto unknown (225), length 59)
    localhost > localhost:  ip-proto-225 39
16:07:44.334543 IP (tos 0x0, ttl 64, id 28715, offset 0, flags [DF], proto unknown (226), length 59)
    localhost > localhost:  ip-proto-226 39
16:07:44.347540 IP (tos 0x0, ttl 64, id 64016, offset 0, flags [DF], proto unknown (227), length 59)
    localhost > localhost:  ip-proto-227 39
16:07:44.358169 IP (tos 0x0, ttl 64, id 24914, offset 0, flags [DF], proto unknown (228), length 59)
    localhost > localhost:  ip-proto-228 39
16:07:44.371639 IP (tos 0x0, ttl 64, id 44246, offset 0, flags [DF], proto unknown (229), length 59)
    localhost > localhost:  ip-proto-229 39
16:07:44.383792 IP (tos 0x0, ttl 64, id 35005, offset 0, flags [DF], proto unknown (230), length 59)
    localhost > localhost:  ip-proto-230 39
16:07:44.394179 IP (tos 0x0, ttl 64, id 29219, offset 0, flags [DF], proto unknown (231), length 59)
    localhost > localhost:  ip-proto-231 39
16:07:44.407620 IP (tos 0x0, ttl 64, id 58720, offset 0, flags [DF], proto unknown (232), length 59)
    localhost > localhost:  ip-proto-232 39
16:07:44.418639 IP (tos 0x0, ttl 64, id 30043, offset 0, flags [DF], proto unknown (233), length 59)
    localhost > localhost:  ip-proto-233 39
16:07:44.431508 IP (tos 0x0, ttl 64, id 45521, offset 0, flags [DF], proto unknown (234), length 59)
    localhost > localhost:  ip-proto-234 39
16:07:44.444541 IP (tos 0x0, ttl 64, id 2122, offset 0, flags [DF], proto unknown (235), length 59)
    localhost > localhost:  ip-proto-235 39
16:07:44.457542 IP (tos 0x0, ttl 64, id 17220, offset 0, flags [DF], proto unknown (236), length 59)
    localhost > localhost:  ip-proto-236 39
16:07:44.470529 IP (tos 0x0, ttl 64, id 47120, offset 0, flags [DF], proto unknown (237), length 59)
    localhost > localhost:  ip-proto-237 39
16:07:44.483538 IP (tos 0x0, ttl 64, id 60080, offset 0, flags [DF], proto unknown (238), length 59)
    localhost > localhost:  ip-proto-238 39
16:07:44.495486 IP (tos 0x0, ttl 64, id 39339, offset 0, flags [DF], proto unknown (239), length 59)
    localhost > localhost:  ip-proto-239 39
16:07:44.508541 IP (tos 0x0, ttl 64, id 46281, offset 0, flags [DF], proto unknown (240), length 59)
    localhost > localhost:  ip-proto-240 39
16:07:44.521532 IP (tos 0x0, ttl 64, id 44448, offset 0, flags [DF], proto unknown (241), length 59)
    localhost > localhost:  ip-proto-241 39
16:07:44.534545 IP (tos 0x0, ttl 64, id 10417, offset 0, flags [DF], proto unknown (242), length 59)
    localhost > localhost:  ip-proto-242 39
16:07:44.547534 IP (tos 0x0, ttl 64, id 22454, offset 0, flags [DF], proto unknown (243), length 59)
    localhost > localhost:  ip-proto-243 39
16:07:44.560544 IP (tos 0x0, ttl 64, id 37039, offset 0, flags [DF], proto unknown (244), length 59)
    localhost > localhost:  ip-proto-244 39
16:07:44.573531 IP (tos 0x0, ttl 64, id 64265, offset 0, flags [DF], proto unknown (245), length 59)
    localhost > localhost:  ip-proto-245 39
16:07:44.584172 IP (tos 0x0, ttl 64, id 37050, offset 0, flags [DF], proto unknown (246), length 59)
    localhost > localhost:  ip-proto-246 39
16:07:44.595764 IP (tos 0x0, ttl 64, id 7836, offset 0, flags [DF], proto unknown (247), length 59)
    localhost > localhost:  ip-proto-247 39
16:07:44.608481 IP (tos 0x0, ttl 64, id 41995, offset 0, flags [DF], proto unknown (248), length 59)
    localhost > localhost:  ip-proto-248 39
16:07:44.619677 IP (tos 0x0, ttl 64, id 51041, offset 0, flags [DF], proto unknown (249), length 59)
    localhost > localhost:  ip-proto-249 39
16:07:44.632494 IP (tos 0x0, ttl 64, id 26536, offset 0, flags [DF], proto unknown (250), length 59)
    localhost > localhost:  ip-proto-250 39
16:07:44.645548 IP (tos 0x0, ttl 64, id 55685, offset 0, flags [DF], proto unknown (251), length 59)
    localhost > localhost:  ip-proto-251 39
16:07:44.658531 IP (tos 0x0, ttl 64, id 39671, offset 0, flags [DF], proto unknown (252), length 59)
    localhost > localhost:  ip-proto-252 39
16:07:44.671534 IP (tos 0x0, ttl 64, id 33753, offset 0, flags [DF], proto unknown (253), length 59)
    localhost > localhost:  exptest-253 39
16:07:44.684539 IP (tos 0x0, ttl 64, id 11444, offset 0, flags [DF], proto unknown (254), length 59)
    localhost > localhost:  exptest-254 39
16:07:44.696627 IP (tos 0x0, ttl 64, id 45396, offset 0, flags [DF], proto unknown (255), length 59)
    localhost > localhost:  reserved 39
16:07:44.709545 IP (tos 0x0, ttl 64, id 1692, offset 0, flags [DF], proto Options (0), length 59)
    localhost > localhost:  hopopt 39
16:07:44.709550 IP (tos 0xc0, ttl 64, id 7163, offset 0, flags [none], proto ICMP (1), length 87)
    localhost > localhost: ICMP localhost protocol 0 unreachable, length 67
	IP (tos 0x0, ttl 64, id 1692, offset 0, flags [DF], proto Options (0), length 59)
    localhost > localhost:  hopopt 39
