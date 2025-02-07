# HDP Specification
```plaintext
   0                   1                   2                   3  
   0 1 2 3 4 5 6 7 8 9 A B C D E F 0 1 2 3 4 5 6 7 8 9 A B C D E F  
  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+  
  |         Source Port          |       Destination Port         |  
  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+  
  |                   Timestamp (High Precision)                  |  
  |                                                               |  
  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+  
  |                           Payload ...                         |  
  |                                                               |  
  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+  
```

Note: Payload is encrypted in base64 or base128 format randomly, or not encrypted at all, that's to increase the security and confidentiality of the data—however the authors of the protocol later realized that this was not a good idea...!