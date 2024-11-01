@echo off
cd ..
cd node_modules
for /R %%d in (.) do (
    if exist "%%d\package.json" (
        echo Installing in %%d
        cd %%d
        npm install
        cd /d %~dp0
    )
)
echo Done!