meta:
  id: rtkgz
  imports:
    - /archive/gzip

seq:
  - id: magic
    contents:
      - (c) 1998 PyroTechnix,Inc.
      - 0x1a
  - id: compressed
    type: gzip
