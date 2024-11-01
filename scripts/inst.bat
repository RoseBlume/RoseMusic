@echo off
cd ..
cd node_modules
for /D %%d in (*) do (
    echo Installing in %%d
    cd %%d
    npm install
    cd ..
)
echo Done!