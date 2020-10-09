### APA102 WAVE EXAMPLE

Nice wave effect ported from an example made for Adafruit PyBadge/Edgebadge:
https://github.com/atsamd-rs/atsamd/blob/master/boards/edgebadge/examples/neopixel_easing.rs 
ported to the Adafruit ItsyBitsy nRF52840 Express (https://learn.adafruit.com/adafruit-itsybitsy-nrf52840-express).

#### __sine\_ease\_in__ function by atsamd-rs team ####

![ItsyBitsy Neopixel](neopixel_wave.gif)

Uses micromath and SmallRng random numbers generator.

TO DO: add ADC reading to obtain a different seed each time.



### HOW TO FLASH:

* build the code: ```cargo build --release```
* convert to .hex file: ```arm-none-eabi-objcopy -O ihex target/thumbv7em-none-eabihf/release/blinky_neopixel neopixel.hex```
* create a dfu package: ```adafruit-nrfutil dfu genpkg --dev-type 0x0052 --application neopixel.hex neopixel.zip```
* put the board into bootloader mode (double click on reset button, will show up as _ITSY840BOOT_ or similar)
* flash the firmware: ```adafruit-nrfutil dfu serial --package neopixel.zip -p /dev/ttyACM0 -b 115200```
