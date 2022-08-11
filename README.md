# searche-rs

A small project to learn how cli apps work with rust.
This app lets you search things on the internet by typing it on your terminal, followed by opening your browser, going to a search engine and putting in your search text in there.

## Dependencies
xdg-utils
```bash
#Debian/Ubuntu based:
sudo apt-get install -y xdg-utils

#Arch based:
sudo pacman -S xdg-utils
```

## Installation
After cloning the repo, you can run the install script.
For the lazy you can just copy paste this following command in your favorite terminal:
```bash
git clone https://github.com/ogi28/searche-rs.git ; cd searche-rs ; chmod +x ./install.sh ; ./install.sh
```

## Usage
the name of the app is searchers.
searchers -h for more information.
```bash
searchers how cold will it be tomorrow?
```

