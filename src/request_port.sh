#!/bin/bash

# VPS login details
VPS_USER="group8"
VPS_HOST="51.120.13.11"

# Port request function
PORT=$(ssh ${VPS_USER}@${VPS_HOST} 'sudo bash ~/assign_port.sh')

# Check if a port was successfully allocated
if [[ $? -eq 0 ]]; then
    echo "Allocated port: $PORT"
    # Use the allocated port as needed in your script
    # For example, establish an SSH tunnel with the allocated port:
    autossh -M 0 -N -R ${PORT}:localhost:80 ${VPS_USER}@${VPS_HOST}
else
    echo "Failed to allocate port"
    exit 1
fi