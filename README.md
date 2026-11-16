# xiv-otp

Automatically enters your Square Enix one-time password in XIVLauncher. This is provided as is, for my own personal use mostly, you can use it if you want to. Do whatever with it really, I don't really care.
This is a convenience tool, if a bad actor gets access to your PC, it can steal the key. Important to be aware of that I think.

## Setup

Requires [Rust](https://rustup.rs).

1. Install: 
   ```
   cargo install --git https://github.com/Emmanator/xiv_otp_rust
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

In theory it should work on linux too, but I haven't verified it myself.
