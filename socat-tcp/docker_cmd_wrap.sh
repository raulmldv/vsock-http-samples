#!/bin/sh

# Start the first process
socat VSOCK-LISTEN:8000,reuseaddr,fork TCP:localhost:5005 &
  
# Start the second process
./server --port 5005
  
# Wait for any process to exit
wait -n
  
# Exit with status of process that exited first
exit $?
