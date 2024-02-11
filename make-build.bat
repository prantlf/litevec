if "%1%"=="" (
  cargo build --release
)
if "%1%"=="WINDOWS_ARM" (
  cargo build --release --target=aarch64-pc-windows-msvc
)
