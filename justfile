set dotenv-load

replace-package:
  rm -rf ~/Documents/Max\ 8/Packages/petunia
  cp -r {{justfile_directory()}}/petunia ~/Documents/Max\ 8/Packages/petunia

install:
   cargo make --profile release install
package-all:
   cargo make --profile release package-all

# Environment variables needed for notarization
APPLE_ID := env_var('APPLE_ID')
APP_PASSWORD := env_var('APP_PASSWORD')
TEAM_ID := env_var('TEAM_ID')

notarize:
    #!/usr/bin/env bash
    set -euo pipefail
    cd {{justfile_directory()}}/petunia/externals
    rm -f rytm.zip
    zip -r rytm.zip rytm.mxo
    rm -f rytm_notarization.log
    xcrun notarytool submit rytm.zip \
        --apple-id "{{APPLE_ID}}" \
        --password "{{APP_PASSWORD}}" \
        --team-id "{{TEAM_ID}}" \
        --wait \
        --output-format json \
        > rytm_notarization.log

    STATUS=$(cat rytm_notarization.log | grep -o '"status":"[^"]*"' | cut -d'"' -f4)
    if [ "$STATUS" = "Accepted" ]; then
        echo "✅ Notarization succeeded"
    else
        echo "❌ Notarization failed with status: $STATUS"
        cat rytm_notarization.log
        exit 1
    fi

    cd {{justfile_directory()}}

    xcrun stapler staple petunia/externals/rytm.mxo
    xcrun stapler validate petunia/externals/rytm.mxo