#!/bin/bash

sudo yum update -y
sudo yum install -y docker.x86_64
sudo systemctl start docker

# set up docker to start at boot
sudo systemctl enable docker.service
sudo systemctl enable containerd.service
mkdir /home/ec2-user/logdata

# Start Seq logging service, reset password immediately
sudo docker run --name seq -d --restart unless-stopped -e ACCEPT_EULA=Y -e SEQ_FIRSTRUN_ADMINPASSWORDHASH="FJ1r9wNyY2OVPHw5gD4v7zIb7xurMq5PWxBPQb7sGMrGoUUd6A==" -v /home/ec2-user/logdata:/data -p 8000:80 -p 5341:5341 datalust/seq