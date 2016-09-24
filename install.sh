compressed_filename="imgprev-linux.tar.gz"
tmp_folder="/tmp"
compressed_file="$tmp_folder/$compressed_filename"

extract_folder="/usr/local/bin"
executable_name="imgprev"
executable_path="$extract_folder/$executable_name"

version="v0.0.2"
url="https://github.com/wikiti/imgprev/releases/download/$version/$compressed_filename"

echo "Downloading release $version from $url ..."
curl -L -o $compressed_file $url

echo "Extracting executable into $extract_folder ..."
sudo tar -xvzf $compressed_file -C $extract_folder >> /dev/null

echo "Adding executable permissions to $executable_path ..."
sudo chmod +x $executable_path

printf "\n\n"
$executable_name --help
