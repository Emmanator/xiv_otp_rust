# xiv-otp

Automatically enters your Square Enix one-time password in XIVLauncher.

## Setup

Requires [Rust](https://rustup.rs).

1. Install: 
   ```
   cargo install --git https://github.com/Emmanator/xiv_otp_rust --locked
   ```
2. In XIVLauncher's settings, enable **XL Authenticator app/OTP macro support**.
3. Save your OTP secret follow prompt (run again to replace secret):

   ```
   xiv-otp --setup
   ```

## Usage

Start it with no arguments:

```
xiv-otp
```

To start it at login, press `Win + R`, type `shell:startup`, and put a shortcut to `xiv-otp.exe` (in `%USERPROFILE%\.cargo\bin`) in that folder.

To stop it, end `xiv-otp.exe` in Task Manager.
