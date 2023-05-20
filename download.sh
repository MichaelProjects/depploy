#!/bin/bash

# Set the binary name and destination directory
BINARY_NAME="your-binary"
DESTINATION_DIR="$HOME/bin"

# Fetch the latest release URL from GitHub API
API_URL="https://api.github.com/repos/{owner}/{repo}/releases/latest"
URL=$(curl -s "$API_URL" | grep "browser_download_url" | cut -d '"' -f 4)

# Download the latest binary release
echo "Downloading $BINARY_NAME..."
curl -L# "$URL" -o "$BINARY_NAME"

# Make the binary executable
chmod +x "$BINARY_NAME"

# Move the binary to the destination directory
mv "$BINARY_NAME" "$DESTINATION_DIR/$BINARY_NAME"

# Add the destination directory to the PATH in the user's profile
echo "export PATH=\"$DESTINATION_DIR:\$PATH\"" >> "$HOME/.bashrc"

# Export the PATH in the current session
export PATH="$DESTINATION_DIR:$PATH"

echo "Installation complete. You can now execute $BINARY_NAME from any folder."
