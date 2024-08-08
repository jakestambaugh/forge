### `/var/run/forge/forged.sock` permissions

Much like Docker and `docker.sock`, there is a permissions issue with the `/var/run` directory. We could solve this by opening up permissions on `/var/run/forge/` or by creating a Unix group for Forge users. I think that since this is a single-user system right now (just me on my own machine) I'm going to open up the permissions on this directory.  