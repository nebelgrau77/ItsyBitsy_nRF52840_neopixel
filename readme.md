### APA102 HUE SHIFT EXAMPLE

NeoPixel changing hue around the colorwheel :) 
Target: Adafruit ItsyBitsy nRF52840 Express (https://learn.adafruit.com/adafruit-itsybitsy-nrf52840-express).

### HOW TO FLASH:

* build the code: ```cargo build --release```
* convert to .hex file: ```arm-none-eabi-objcopy -O ihex target/thumbv7em-none-eabihf/release/blinky_neopixel neopixel.hex```
* create a dfu package: ```adafruit-nrfutil dfu genpkg --dev-type 0x0052 --application neopixel.hex neopixel.zip```
* put the board into bootloader mode (double click on reset button, will show up as _ITSY840BOOT_ or similar)
* flash the firmware: ```adafruit-nrfutil dfu serial --package neopixel.zip -p /dev/ttyACM0 -b 115200```
