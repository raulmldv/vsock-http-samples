#!/bin/sh

# Start the first process
./server --port 5005 &
  
# Start the second process
./vsock-proxy 8000 127.0.0.1 5005 --config vsock-proxy.yaml
  
# now we bring the primary process back into the foreground
# and leave it there
fg %1
