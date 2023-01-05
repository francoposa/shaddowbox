---
**Components**
1. API/Frontend/Gateway server
2. Storage server

**API/Frontend/Gateway**
- serve API requests
- handle placement algorithm to select storage nodes
- handle striping & replication
- ensure consistency before returning
- interface with primary metadata store (TBD primary store model)

**Storage Server**
- write strips to disk (TBD filesystem or raw block)
- checksum strips
- maintain local metadata store
  - for filesystem disk backing, probably just checksums
  - for raw block disk backing, actual strip location pointers
  - should be able to rebuild the primary metadata store from the separate disk metadata stores
