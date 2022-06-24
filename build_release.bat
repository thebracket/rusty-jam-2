REM Setup the Folders
mkdir release
cd release
mkdir itch
cd itch
mkdir assets
cd ..
mkdir wasm
cd wasm
mkdir assets
cd ..
mkdir linux
cd linux
mkdir assets
cd ..
cd ..
copy .\assets\* .\release\itch\assets
copy .\assets\* .\release\wasm\assets
copy .\assets\* .\release\linux\assets

REM Build for Windows
cargo build --release
copy target\release\rust_jam_chicken_dog.exe release\itch

REM Build for Linux
rem cargo build --release --target x86_64-pc-windows-gnu
copy .\target\release\rust_jam_chicken_dog release\linux

REM Build for WASM and Upload
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir .\release\wasm --target web .\target\wasm32-unknown-unknown\release\rust_jam_chicken_dog.wasm
copy template.html .\release\wasm\index.html
.\upload.bat
