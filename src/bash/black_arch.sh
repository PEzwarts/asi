if [ ! -f /etc/pacman.d/blackarch-mirrorlist]; then
    curl -O https://blackarch.org/strap.sh

    # SHA1 hash might change !

    echo bbf0a0b838aed0ec05fff2d375dd17591cbdf8aa strap.sh | sha1sum -c
    chmod u+x ./strap.sh
    sudo ./strap.sh
fi
