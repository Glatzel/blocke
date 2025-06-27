apt update
apt upgrade

apt install -y --no-install-recommends \
    curl \
    stress-ng \
    fio \
    bc \
    libncurses5

curl -fsSL https://pixi.sh/install.sh | sh
