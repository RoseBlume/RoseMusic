yarn run tauri icon Logo.png
yarn run tauri android build
zipalign -f -v 4 src-tauri\gen\android\app\build\outputs\apk\universal\release\app-universal-release-unsigned.apk Rose-Music.apk
apksigner sign --ks rosary.keystore --ks-key-alias rosary Rose-Music.apk
