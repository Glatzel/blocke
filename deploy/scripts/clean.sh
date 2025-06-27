# APT cache cleanup
echo "Cleaning APT cache..."
sudo apt-get clean
sudo apt-get autoclean
sudo apt-get autoremove -y

# Clean user cache and trash
echo "Cleaning user cache..."
rm -rf ~/.cache/*
rm -rf ~/.local/share/Trash/*

# Clean /tmp and /var/tmp
echo "Cleaning /tmp and /var/tmp..."
sudo rm -rf /tmp/*
sudo rm -rf /var/tmp/*

# Pixi cache cleanup
echo "Cleaning Pixi cache..."
pixi clean cache -y
