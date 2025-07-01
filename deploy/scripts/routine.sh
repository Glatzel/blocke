# APT
apt update
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
pixi global update
echo "Cleaning Pixi cache..."
pixi clean cache -y
