# Releasing

## Building the container
- `podman build -f Containerfile`
- - This spits out a container id such as `e9e09b548237`
- `podman run -it -d -v /outside-musr-dir:/app/mus_dir -v /outside-library.db:/app/library.db -p 8080:8080 <container id>`
- When the container is running, get the running container id with `podman ps`. It should look something like `38e60ffbb258`
- Name the container musrs with `podman commit 38e60ffbb258 musrs`
- Login to the local container registry using `podman login <url>` and enter username and password.
- Push to the container registry with `podman push <registry-host>/<registry_name>/musrs:latest`

## Deploy container
- On the target host, stop the existing service. `systemctl stop musrs.service`
- Pull the new image `podman pull <registry-host>/<registry-url>/musrs:latest`
- Run the new image temporarily with `sudo podman run -it -d -p 3300:8080 -v /var/fs/storage/media/music:/app/mus_dir -v /usr/local/musrs/library.db:/app/library.db <registry-host>/<registry-url>/musrs`
- Get the name of the container with `podman ps`
- Generate a systemd unit for the container with `sudo podman generate systemd <container-name> > musrs.service`
- Stop the container `podman stop <container-name>`
- Install the new unit `sudo cp musrs.service /etc/systemd/system/`
- Reload systemd `systemctl daemon-reload`
- Start and enable the new service with `systemctl enable --now musrs.service`
