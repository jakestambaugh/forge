# Forge

New design for a software development environment

## Notes

### `/var/run/forge/forged.sock` permissions

Much like Docker and `docker.sock`, there is a permissions issue with the `/var/run` directory. We could solve this by opening up permissions on `/var/run/forge/` or by creating a Unix group for Forge users. I think that since this is a single-user system right now (just me on my own machine) I'm going to open up the permissions on this directory.

## Devlog

### Aug 12

The goal for today is to create a protocol definition for the unix socket connection. We will keep it really simple for now.

It's pretty hardcoded, based in serde_json serialization, but I'm happy with it. There is no response for now because I'm not sure what I want the response to do yet. I think it will need to stream logs back to the user which might be complex.


### Aug 10

The goal for today is to get the CLI process to communicate with the daemon over Unix sockets. I think that I need to:

- Start a new thread to listen for socket connections
- Create a loop to listen
- Create the function to send bytes into the server
- Send some bytes back
