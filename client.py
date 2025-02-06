import sys
from scapy.all import IP, Raw, send

dst = "127.0.0.1"
protocol_number = 253

data = ""
if len(sys.argv) > 1:
    data = sys.argv[1]
else:
    if sys.stdin.isatty():
        sys.exit('Usage: python3 client.py [data] OR pipe data to standard input')
    data = sys.stdin.read()

send(IP(dst=dst, proto=protocol_number) / Raw(load=data))