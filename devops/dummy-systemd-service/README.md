# Dummy Systemd Service

Create a long-running systemd service that logs to a file.

## Objective

The goal of this project is to get familiar with systemd.

- creating and enabling a service
- checking the status
- keeping an eye on the logs
- starting and stopping the service, etc.

## Example script

```shell
bash -c 'cat <<EOF > /tmp/example.sh
#!/bin/bash

while true; do
  echo "Dummy service is running..." >> /var/log/dummy-service.log
  sleep 10
done
EOF'
```

## Create a systemd service

```shell
sudo vim /etc/systemd/system/dummy.service
```

```dummy.service
[Unit]
Description=Dummy Service
After=network.target

[Service]
ExecStart=/tmp/dummy.sh
Restart=always
User=dummy

[Install]
WantedBy=multi-user.target
```

```shell
sudo useradd --system --shell /sbin/nologin dummy
sudo chown dummy:dummy /tmp/dummy.sh
sudo chmod +x /tmp/dummy.sh
sudo touch /var/log/dummy-service.log
sudo chown dummy:dummy /var/log/dummy-service.log
```

# Interact with the service with systemd

```shell
# Interacting with the service
sudo systemctl start dummy
sudo systemctl stop dummy
sudo systemctl enable dummy
sudo systemctl disable dummy
sudo systemctl status dummy

# Check the logs
sudo journalctl -u dummy -f
```