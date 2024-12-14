#!/usr/bin/env bash

set -e

latestReleaseInfo=$(curl -s https://api.github.com/repos/tigerbeetle/tigerbeetle/releases/latest | jq .)

newVersion=$(echo "$latestReleaseInfo" | jq -r '.tag_name')
if [[ "$newVersion" = "null" ]]; then
  echo -e "Wrong JSON response: no \`tag_name\` field found."
  exit 1
fi
newCommit=$(echo "$latestReleaseInfo" | jq -r '.target_commitish')
if [[ "$newVersion" = "null" ]]; then
  echo -e "Wrong JSON response: no \`target_commitish\` field found."
  exit 1
fi

echo "version=$newVersion"
echo "commit=$newCommit"
if [[ ! -z "$GITHUB_OUTPUT" ]]; then
  echo "version=$newVersion" >> $GITHUB_OUTPUT
  echo "commit=$newCommit" >> $GITHUB_OUTPUT
fi

if [[ -z $(echo "$latestReleaseInfo" | jq -r 'select(.draft == false)') ]]; then
  echo "Latest TigerBeetle $newVersion release is in draft yet. Skipping..."
  exit 0
fi
if [[ -z $(echo "$latestReleaseInfo" | jq -r 'select(.prerelease == false)') ]]; then
  echo "Latest TigerBeetle $newVersion version is pre-release. Skipping..."
  exit 0
fi

sed -i.bk -e "s/^const TIGERBEETLE_RELEASE: \&str =.*$/const TIGERBEETLE_RELEASE: \&str = \"$newVersion\";/g" \
          -e "s/^const TIGERBEETLE_COMMIT: \&str =.*$/const TIGERBEETLE_COMMIT: \&str = \"$newCommit\";/g" \
    ./sys/build.rs
if cmp -s ./sys/build.rs ./sys/build.rs.bk; then
  echo "TigerBeetle $newVersion version is already the latest one. Skipping... "
fi
