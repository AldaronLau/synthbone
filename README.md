# Electric Trombone
Code to run an electric trombone.

## Setup Your Computer
```bash
rustup target install arm-unknown-linux-gnueabihf

```

## Setup Pi
### Step 1
Get the newest full raspbian download and write it onto the SD card.

### Step 2
Make ssh folder in boot, edit wpa\_supplicant.conf to connect to your wifi.

### Step 3
Put into the pi 0.

### Step 4
ssh raspberrypi.local & raspi-config
Change the hostname to electric-trombone
Turn on ssh
reboot
ssh & poweroff

### Step 5
Install the rt kernel.
