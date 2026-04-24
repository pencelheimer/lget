aur_path := "packaging/aur-bin"

# Prepare lget-bin for publishing to the AUR. Usage: just publish-aur 0.1.1
prepare-aur version:
    #!/usr/bin/env bash
    set -euo pipefail

    if ! grep -q 'ID=arch' /etc/os-release 2>/dev/null; then
        echo "Error: This recipe requires Arch Linux (makepkg/updpkgsums are missing)."
        exit 1
    fi

    echo "Updating AUR package to v{{ version }}..."

    sed -i "s/^pkgver=.*/pkgver={{ version }}/" {{ aur_path }}/PKGBUILD
    sed -i "s/^pkgrel=.*/pkgrel=1/" {{ aur_path }}/PKGBUILD

    cd {{ aur_path }}
    updpkgsums
    rm -f lget-{{ version }} LICENSE-{{ version }}
    makepkg --printsrcinfo > .SRCINFO

    git add PKGBUILD .SRCINFO
    if git diff-index --quiet HEAD; then
        echo "No changes detected in the AUR package files."
    else
        git commit -m "release: v{{ version }}"
    fi
