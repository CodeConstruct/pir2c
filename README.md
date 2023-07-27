Bus Pirate I2C Rust embedded-hal 
==

This allows using Bus Pirate 
[binary scripting I2C](http://dangerousprototypes.com/docs/I2C_%28binary%29) with Rust embedded-hal.
The Bus Pirate protocol side is thanks to Geoff Cant's
[ruspirate](https://github.com/archaelus/ruspirate)

It currently targets embedded-hal 1.0.0-alpha.10 

Performance is fairly slow, but is usable for some circumstances.

To get out of BBIO mode when you reattach a serial terminal, send *break* then *ctrl-o*

Bus Pirate
== 

Tested with:

```
Bus Pirate v3.a
Firmware v6.1 r1676 Bootloader v4.1
```

```
Bus Pirate v3.5
Community Firmware v7.0 - goo.gl/gCzQnW [HiZ UART I2C SPI] Bootloader v4.4
```


