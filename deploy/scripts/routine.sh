# APT
echo "Update APT..."
apt update
echo "Upgrade APT..."
apt upgrade

echo "Cleaning APT cache..."
apt-get clean
apt-get autoclean
apt-get autoremove -y

# Clean user cache and trash
echo "Cleaning user cache..."
rm -rf ~/.cache/*
rm -rf ~/.local/share/Trash/*

# Clean /tmp and /var/tmp
echo "Cleaning /tmp and /var/tmp..."
rm -rf /tmp/*
rm -rf /var/tmp/*

# Pixi
echo "Attempting pixi self-update..."
if pixi self-update; then
    echo "pixi successfully updated."
else
    echo "pixi not found or update failed — installing fresh copy..."
    curl -fsSL https://pixi.sh/install.sh | sh
fi

echo "copy pixi-global.toml to manifests..."
mkdir -p ~/.pixi/manifests
cp ~/config/pixi-global.toml ~/.pixi/manifests/

pixi global update
