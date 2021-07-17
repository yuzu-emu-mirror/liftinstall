#!/bin/bash -ex

# the UID for the container yuzu user is 1027
sudo chown -R 1027 ./
docker run -v $(pwd):/liftinstall -t yuzuemu/build-environments:linux-liftinstall /bin/bash /liftinstall/.travis/build.sh
sudo chown -R $(id -u):$(id -g) ./
